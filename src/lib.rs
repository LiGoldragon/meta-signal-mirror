//! Owner Mirror configuration and store-policy Interface.
//!
//! `ethos/interface.ethos` is the sole schema authority. Its checked Rust
//! projection exposes encoded identities; this crate adds structural Signal
//! behavior and the binary configuration archive adapter.

pub mod bootstrap_manifest;
pub mod schema;

pub use schema::lib::*;

pub const META_MIRROR_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const META_MIRROR_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");

impl z2VPES {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }

    pub fn as_path(&self) -> &std::path::Path {
        std::path::Path::new(self.as_str())
    }
}

impl z2VYru {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }

    pub fn as_path(&self) -> &std::path::Path {
        std::path::Path::new(self.as_str())
    }
}

impl z2VQot {
    pub fn into_u32(self) -> Result<u32, SocketModeRangeError> {
        u32::try_from(self.into_payload()).map_err(|_| SocketModeRangeError)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("socket mode does not fit the current substrate's 32-bit representation")]
pub struct SocketModeRangeError;

impl z2VXab {
    /// Decode the daemon's single binary startup argument.
    pub fn from_binary_path(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, ConfigurationArchiveError> {
        let bytes = std::fs::read(path).map_err(ConfigurationArchiveError::Read)?;
        Self::from_binary_bytes(&bytes)
    }

    pub fn from_binary_bytes(bytes: &[u8]) -> Result<Self, ConfigurationArchiveError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|_| ConfigurationArchiveError::Decode)
    }

    pub fn to_binary_bytes(&self) -> Result<Vec<u8>, ConfigurationArchiveError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| ConfigurationArchiveError::Encode)
    }

    /// Write the binary startup file consumed by the current daemon adapter.
    pub fn write_binary_file(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), ConfigurationArchiveError> {
        std::fs::write(path, self.to_binary_bytes()?).map_err(ConfigurationArchiveError::Write)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigurationArchiveError {
    #[error("failed to read binary mirror configuration: {0}")]
    Read(std::io::Error),

    #[error("failed to write binary mirror configuration: {0}")]
    Write(std::io::Error),

    #[error("failed to encode binary mirror configuration")]
    Encode,

    #[error("failed to decode binary mirror configuration")]
    Decode,
}
