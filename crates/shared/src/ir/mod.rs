//! Intermediate Representation (IR) for workflow.kdl files.
//!
//! This module models a workflow file as a **control-plane contract**:
//! - what to run (`workflow::Workflow`, `workflow::pipeline::stage::Stage`),
//! - where to run (`workflow::environment::Env`, `workflow::pipeline::stage::UseEnv`),
//! - how to recover (`workflow::pipeline::stage::ErrorHandling`),
//! - and what data crosses stage boundaries
//!   (`workflow::Param`, `workflow::pipeline::stage::Input`, `workflow::pipeline::stage::Output`).
//!
//! In a workflow-file based genomic engine:
//! - the control-plane parses and validates KDL into this IR;
//! - the data-plane executes each stage attempt in containers and materializes declared outputs.

pub mod common;
pub mod workflow;
