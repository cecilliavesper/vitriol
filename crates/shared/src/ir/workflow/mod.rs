//! Workflow subtree of the IR.
//!
//! Mirrors the top-level KDL shape:
//! - `workflow { ... }`
//! - `parameters { ... }`
//! - `environments { ... }`
//! - `pipeline { ... }`

pub mod environment;
mod node;
pub mod parameter;
pub mod pipeline;

pub use node::Workflow;
pub use parameter::{GlobSort, LiteralValue, Param};
