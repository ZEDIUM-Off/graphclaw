# Web Pages Context

## Scope

Route-level screens for the operational dashboard.

## File Map

- `Dashboard.tsx` - default landing page
- `AgentChat.tsx` - agent interaction view
- `Tools.tsx`, `Cron.tsx`, `Integrations.tsx`, `Memory.tsx`, `Config.tsx` - operational control pages
- `Cost.tsx`, `Logs.tsx`, `Doctor.tsx` - diagnostics and observability views

## Routing

`web/src/App.tsx` maps `/` to `Dashboard`, `/agent` to `AgentChat`, `/tools` to `Tools`, `/cron` to `Cron`, `/integrations` to `Integrations`, `/memory` to `Memory`, `/config` to `Config`, `/cost` to `Cost`, `/logs` to `Logs`, and `/doctor` to `Doctor`.

## Current State

Pages are organized around inherited gateway/runtime operations instead of a new GraphClaw-specific domain model.

## GraphClaw Relevance

This is where user-facing migration eventually becomes visible, so context docs should clearly separate current operational pages from future architecture aspirations.

## Cautions

- Keep reusable view logic in `components/` or `hooks/`, not spread across page files.
- Do not document routes that are not actually mounted in `App.tsx`.

## Agent Guidance

- Treat page files as route composition points.
- If a page starts sharing logic with another route, extract the shared part downward instead of widening both pages.
