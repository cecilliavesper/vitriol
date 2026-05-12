/// An execution environment declaration.
///
/// Maps to `environments { env "<name>" { image ... resources ... } }` in KDL.
pub struct Env {
    /// Environment reference name.
    pub name: String,
    /// The image definition to use on this environment.
    pub image: Image,
}

/// An image definition that will be used to create runtimes.
pub struct Image {
    /// The image's tag, often in the format of `org/image-name:version`.
    pub tag: String,
    /// Image digest or checksum to pin immutable content.
    pub digest: String,
    /// Minimum required resources to allocate to the runtime.
    pub resources: Resources,
}

/// A resource requirement definition.
pub struct Resources {
    /// Number of CPUs, between 1 and 255.
    pub cpu: u8,
    /// Memory requirement.
    pub memory: Memory,
}

/// A memory amount representation.
pub enum Memory {
    /// Memory in megabytes.
    Megabytes(u64),
    /// Memory in gigabytes.
    Gigabytes(u64),
}
