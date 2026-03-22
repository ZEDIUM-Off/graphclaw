//! WebSocket agent chat handler.
//!
//! Protocol:
//! ```text
//! Client -> Server: {"type":"message","content":"Hello"}
//! Server -> Client: {"type":"session_start","session_id":"...","resumed":false,"message_count":0}
//! Server -> Client: {"type":"done","full_response":"Hi!"}
//! ```

use super::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    http::{header, HeaderMap},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;

/// The sub-protocol we support for the chat WebSocket.
const WS_PROTOCOL: &str = "zeroclaw.v1";

/// Prefix used in `Sec-WebSocket-Protocol` to carry a bearer token.
const BEARER_SUBPROTO_PREFIX: &str = "bearer.";

/// Gateway session key prefix to avoid collisions with channel sessions.
const GW_SESSION_PREFIX: &str = "gw_";

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: Option<String>,
    pub session_id: Option<String>,
}

/// Extract a bearer token from WebSocket-compatible sources.
///
/// Precedence:
/// 1. `Authorization: Bearer <token>` header
/// 2. `Sec-WebSocket-Protocol: bearer.<token>` subprotocol
/// 3. `?token=<token>` query parameter
fn extract_ws_token<'a>(headers: &'a HeaderMap, query_token: Option<&'a str>) -> Option<&'a str> {
    if let Some(t) = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .filter(|t| !t.is_empty())
    {
        return Some(t);
    }

    if let Some(t) = headers
        .get("sec-websocket-protocol")
        .and_then(|v| v.to_str().ok())
        .and_then(|protos| {
            protos
                .split(',')
                .map(|p| p.trim())
                .find_map(|p| p.strip_prefix(BEARER_SUBPROTO_PREFIX))
        })
        .filter(|t| !t.is_empty())
    {
        return Some(t);
    }

    query_token.filter(|t| !t.is_empty())
}

/// GET /ws/chat — WebSocket upgrade for agent chat
pub async fn handle_ws_chat(
    State(state): State<AppState>,
    Query(params): Query<WsQuery>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    if state.pairing.require_pairing() {
        let token = extract_ws_token(&headers, params.token.as_deref()).unwrap_or("");
        if !state.pairing.is_authenticated(token) {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                "Unauthorized — provide Authorization header, Sec-WebSocket-Protocol bearer, or ?token= query param",
            )
                .into_response();
        }
    }

    let ws = if headers
        .get("sec-websocket-protocol")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|protos| protos.split(',').any(|p| p.trim() == WS_PROTOCOL))
    {
        ws.protocols([WS_PROTOCOL])
    } else {
        ws
    };

    let session_id = params
        .session_id
        .filter(|id| !id.trim().is_empty())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    ws.on_upgrade(move |socket| handle_socket(socket, state, session_id))
        .into_response()
}

async fn handle_socket(socket: WebSocket, state: AppState, session_id: String) {
    let (mut sender, mut receiver) = socket.split();
    let session_key = format!("{GW_SESSION_PREFIX}{session_id}");

    let resumed_count = state
        .session_backend
        .as_ref()
        .map(|backend| backend.load(&session_key).len())
        .unwrap_or(0);

    let session_start = serde_json::json!({
        "type": "session_start",
        "session_id": session_id,
        "resumed": resumed_count > 0,
        "message_count": resumed_count,
    });
    let _ = sender
        .send(Message::Text(session_start.to_string().into()))
        .await;

    while let Some(msg) = receiver.next().await {
        let msg = match msg {
            Ok(Message::Text(text)) => text,
            Ok(Message::Ping(payload)) => {
                let _ = sender.send(Message::Pong(payload)).await;
                continue;
            }
            Ok(Message::Close(_)) | Err(_) => break,
            _ => continue,
        };

        let parsed: serde_json::Value = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(_) => {
                let err = serde_json::json!({"type": "error", "message": "Invalid JSON"});
                let _ = sender.send(Message::Text(err.to_string().into())).await;
                continue;
            }
        };

        if parsed["type"].as_str().unwrap_or("") != "message" {
            continue;
        }

        let content = parsed["content"].as_str().unwrap_or("").to_string();
        if content.is_empty() {
            continue;
        }

        process_chat_message(&state, &mut sender, &content, &session_key).await;
    }
}

async fn process_chat_message(
    state: &AppState,
    sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    content: &str,
    session_key: &str,
) {
    let provider_label = state
        .config
        .lock()
        .default_provider
        .clone()
        .unwrap_or_else(|| "unknown".to_string());

    let _ = state.event_tx.send(serde_json::json!({
        "type": "agent_start",
        "provider": provider_label,
        "model": state.model,
    }));

    let system_prompt = {
        let config_guard = state.config.lock();
        crate::channels::build_system_prompt(
            &config_guard.workspace_dir,
            &state.model,
            &[],
            &[],
            Some(&config_guard.identity),
            None,
        )
    };

    let mut messages = vec![crate::providers::ChatMessage::system(system_prompt)];
    if let Some(ref backend) = state.session_backend {
        messages.extend(backend.load(session_key));
    }
    messages.push(crate::providers::ChatMessage::user(content));

    let multimodal_config = state.config.lock().multimodal.clone();
    let prepared =
        match crate::multimodal::prepare_messages_for_provider(&messages, &multimodal_config).await
        {
            Ok(p) => p,
            Err(e) => {
                let err = serde_json::json!({
                    "type": "error",
                    "message": format!("Multimodal prep failed: {e}")
                });
                let _ = sender.send(Message::Text(err.to_string().into())).await;
                return;
            }
        };

    match state
        .provider
        .chat_with_history(&prepared.messages, &state.model, state.temperature)
        .await
    {
        Ok(response) => {
            if let Some(ref backend) = state.session_backend {
                let user_msg = crate::providers::ChatMessage::user(content);
                let assistant_msg = crate::providers::ChatMessage::assistant(&response);
                let _ = backend.append(session_key, &user_msg);
                let _ = backend.append(session_key, &assistant_msg);
            }

            let done = serde_json::json!({
                "type": "done",
                "full_response": response,
            });
            let _ = sender.send(Message::Text(done.to_string().into())).await;

            let _ = state.event_tx.send(serde_json::json!({
                "type": "agent_end",
                "provider": provider_label,
                "model": state.model,
            }));
        }
        Err(e) => {
            let sanitized = crate::providers::sanitize_api_error(&e.to_string());
            let err = serde_json::json!({
                "type": "error",
                "message": sanitized,
            });
            let _ = sender.send(Message::Text(err.to_string().into())).await;

            let _ = state.event_tx.send(serde_json::json!({
                "type": "error",
                "component": "ws_chat",
                "message": sanitized,
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn extract_ws_token_from_authorization_header() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", "Bearer zc_test123".parse().unwrap());
        assert_eq!(extract_ws_token(&headers, None), Some("zc_test123"));
    }

    #[test]
    fn extract_ws_token_from_subprotocol() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "sec-websocket-protocol",
            "zeroclaw.v1, bearer.zc_sub456".parse().unwrap(),
        );
        assert_eq!(extract_ws_token(&headers, None), Some("zc_sub456"));
    }

    #[test]
    fn extract_ws_token_from_query_param() {
        let headers = HeaderMap::new();
        assert_eq!(
            extract_ws_token(&headers, Some("zc_query789")),
            Some("zc_query789")
        );
    }
}
