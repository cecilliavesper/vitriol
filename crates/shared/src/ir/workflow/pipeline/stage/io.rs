/// Input types for a stage.
pub enum Input {
    /// A file pattern to be resolved before sending the manifest.
    /// It is referenced by its name.
    Glob {
        /// Input binding name.
        name: String,
        /// Glob or URI path.
        path: String,
    },
}

/// Output types for a stage.
pub enum Output {
    /// A produced file pattern to be resolved before sending the completion notification.
    /// It is referenced by its name.
    Glob {
        /// Output binding name.
        name: String,
        /// Glob or URI path.
        path: String,
    },
}
