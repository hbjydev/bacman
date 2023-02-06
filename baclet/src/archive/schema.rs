use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArchiveJobCompression {
    #[serde(rename = "xz")]
    XZ,

    #[serde(rename = "gzip")]
    GZip,
}

impl Default for ArchiveJobCompression {
    fn default() -> Self {
        ArchiveJobCompression::XZ
    }
}

/// A specification for an archive job
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveJobSpec {
    /// The file or folder path the archive should be created from.
    pub src: String,

    /// The file or folder path the archive should be saved to.
    pub dest: String,

    /// The algorithm to compress the archive with.
    #[serde(default)]
    pub compression: ArchiveJobCompression,
}
