/// Error-handling policy attached to a stage.
///
/// Example from `workflow.kdl`:
/// `max_retries 2` with `on_exit 137 env="gatk-high-mem"`.
pub struct ErrorHandling {
    /// Retry timing and limits.
    pub retry: RetryStrategy,
    /// Conditions that map failures to actions.
    pub condition: Vec<RetryCondition>,
}

/// Retry strategy for failed attempts.
pub enum RetryStrategy {
    /// Fixed retry delays.
    Linear {
        /// Delay sequence between retries.
        retries: Vec<f64>,
    },
    /// Exponential backoff.
    Exponential {
        /// Maximum number of retries.
        max_retries: u8,
        /// Exponential backoff multiplier.
        backoff_factor: f64,
    },
}

/// Retry condition selectors.
pub enum RetryCondition {
    /// Match a process exit code.
    OnExit {
        /// Exit status code.
        error_code: u8,
        /// Action to execute when matched.
        action: RetryAction,
    },
}

/// Retry action when a condition matches.
pub enum RetryAction {
    /// Retry with a different environment reference.
    SwitchEnv(String),
    /// Abort stage execution, optionally with a reason.
    Abort(Option<String>),
}
