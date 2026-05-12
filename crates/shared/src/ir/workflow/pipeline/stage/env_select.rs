/// Environment selection for a stage.
///
/// It can be either a reference to a workflow-level environment or
/// a conditional resolve that selects the environment dynamically.
pub enum UseEnv {
    /// Uses a reference to a declared environment.
    EnvRef(String),
    /// A conditional environment selector.
    Conditional {
        /// Condition entries evaluated in order.
        conditions: Vec<ConditionExp>,
        /// Fallback environment if no condition matches.
        default: String,
    },
}

/// Conditional expression forms for environment selection.
pub enum ConditionExp {
    /// String literal.
    String(String),
    /// Number literal.
    Number(f64),
    /// If expression: when `cond` is truthy, resolve to `then`.
    If {
        /// Condition expression.
        cond: Box<ConditionExp>,
        /// Result expression.
        then: Box<ConditionExp>,
    },
}
