/// Expansion strategy for generating data-plane task attempts from one stage declaration.
///
/// Examples in `workflow.kdl`:
/// - `strategy "matrix"` for caller x tumor/normal pair combinations
/// - `strategy "scatter" batch_size=1` for per-input fanout
pub enum ExecutionStrategy {}
