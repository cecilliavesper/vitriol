//! Pipeline subtree.
//!
//! Maps to `pipeline { stage ... }` in KDL.

pub mod stage;

/// Workflow pipeline node containing execution stages.
pub struct Pipeline {
    /// Stage declarations in logical pipeline order.
    pub stages: Vec<stage::Stage>,
}
