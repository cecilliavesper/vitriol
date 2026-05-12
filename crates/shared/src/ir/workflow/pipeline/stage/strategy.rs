/// Expansion strategy for generating data-plane task attempts from one stage declaration.
///
/// Examples in `workflow.kdl`:
/// - `strategy "matrix"` for caller x tumor/normal pair combinations
/// - `strategy "scatter" batch_size=1` for per-input fanout
pub enum ExecutionStrategy {
    /// Expand a stage into one attempt per matrix combination.
    Matrix(MatrixStrategy),
    /// Expand a stage into batches of the declared size.
    Scatter(ScatterStrategy),
}

/// Matrix expansion settings for a stage.
pub struct MatrixStrategy {
    /// Matrix axes evaluated to produce combinations.
    pub axes: Vec<MatrixAxis>,
}

/// One matrix axis definition.
pub enum MatrixAxis {
    /// Axis with a single expression that yields values.
    Values {
        /// Axis name used in templates, such as `caller`.
        name: String,
        /// Expression or reference that produces the axis values.
        values: String,
    },
    /// Axis that zips multiple inputs together positionally.
    Zip {
        /// Axis name used in templates, such as `pair`.
        name: String,
        /// Ordered inputs that should be zipped together.
        values: Vec<String>,
    },
}

/// Scatter execution settings.
pub struct ScatterStrategy {
    /// Number of items to place in each batch.
    pub batch_size: usize,
}
