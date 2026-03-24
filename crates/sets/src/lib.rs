//! View composition and resolution for GraphClaw.
//!
//! Types: SetDefinition, BoundSet, ResolvedSet, SetExport.
//! Operations: create/update/read templates, bind, compose, resolve, export (llm_compact, llm_explained).

mod error;
mod export;
mod service;
mod types;

pub use error::SetsError;
pub use export::{export_for_llm, export_llm_compact, export_llm_explained};
pub use memgraph::{GraphEdge, GraphNode, Subgraph};
pub use service::SetsService;
pub use types::{
    BoundSet, ResolvedSet, SetDefinition, SetExport, SetKind, SetOperation, SetSelectors,
};
