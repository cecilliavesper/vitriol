/// The stage executable payload run by the data-plane runtime.
///
/// KDL's `exec { script ... }` maps to [`Exec::Script`].
pub enum Exec {
    /// Execute a single command line.
    Command(String),
    /// Execute a multiline script block.
    Script(String),
    /// Execute multiple command lines in sequence.
    List(Vec<String>),
}
