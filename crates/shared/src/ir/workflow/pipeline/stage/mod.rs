use crate::ir::common::Description;

pub mod env_select;
pub mod exec;
pub mod io;
pub mod retry;
pub mod strategy;

pub use env_select::{ConditionExp, UseEnv};
pub use exec::Exec;
pub use io::{Input, Output};
pub use retry::{ErrorHandling, RetryAction, RetryCondition, RetryStrategy};
pub use strategy::ExecutionStrategy;

/// A stage in the workflow.
///
/// The stage is identified by `name`, which must be unique within one workflow.
///
/// In control-plane terms, a stage is a schedulable unit with declared contracts
/// (inputs, env, retries, outputs). In data-plane terms, each stage produces one or
/// more concrete task attempts that execute `exec`.
pub struct Stage {
    /// The name of the stage.
    pub name: String,
    /// An optional description to communicate stage purpose.
    pub description: Option<Description>,
    /// Stage expansion strategy, defaults to simple.
    pub strategy: Option<ExecutionStrategy>,
    /// Template-capable stage inputs.
    pub inputs: Vec<Input>,
    /// Declared stage outputs.
    pub outputs: Vec<Output>,
    /// Environment binding or conditional selection.
    pub use_env: UseEnv,
    /// Error handling strategy, defaults to no retry.
    pub error_handling: Option<ErrorHandling>,
    /// Executable payload for this stage.
    pub exec: Exec,
}
