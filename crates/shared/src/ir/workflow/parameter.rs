/// A parameter of workflow execution.
/// It can represent a glob or a literal value.
///
/// ### Example
/// ```kdl
/// parameters {
///     param "name" path="file://here/*.zip" sort="name"
/// }
/// ```
pub enum Param {
    /// A glob or file path.
    ///
    /// ### Example
    /// ```kdl
    /// param "name" path="file://here/*.zip" sort="name"
    /// ```
    Glob {
        /// Parameter name.
        name: String,
        /// Glob or URI path.
        path: String,
        /// Sort strategy.
        sort: GlobSort,
    },
    /// A param holding a literal static value.
    ///
    /// ### Example
    /// ```kdl
    /// param "name" {
    ///     - 1
    ///     - abc
    /// }
    /// ```
    Literal {
        /// Parameter name.
        name: String,
        /// Parameter value.
        value: LiteralValue,
    },
}

/// A literal value, either a string, a number or a list of literal values.
pub enum LiteralValue {
    /// String literal.
    String(String),
    /// Number literal.
    Number(f64),
    /// Recursive list literal.
    List(Vec<LiteralValue>),
}

/// Sorting options supported by glob parameters.
pub enum GlobSort {
    /// Stable lexical ordering (`sort="name"` in KDL).
    Name,
    /// Size-based ordering (`sort="size"` in KDL).
    Size,
}
