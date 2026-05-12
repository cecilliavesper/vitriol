/// A simple description node.
///
/// ### Example
/// `description "a description of this block"`
pub struct Description {
    /// Free-form human-readable text retained by the control-plane for UX/logging.
    pub value: String,
}
