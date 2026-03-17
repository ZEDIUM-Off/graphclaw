//! View composition and resolution for GraphClaw.
//!
//! Types: ViewTemplate, BoundView, ResolvedView, ResolvedViewExport.
//! Operations: create/update/read templates, bind, compose, resolve, export (llm_compact, llm_explained).

mod error;
mod export;
mod service;
mod types;

pub use error::ViewsError;
pub use export::{export_for_llm, export_llm_compact, export_llm_explained};
pub use memgraph::{GraphEdge, GraphNode, Subgraph};
pub use service::ViewsService;
pub use types::{
    BoundView, ResolvedView, ResolvedViewExport, Selectors, ViewKind, ViewOperation, ViewTemplate,
};
