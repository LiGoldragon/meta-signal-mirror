//! Schema-derived meta policy contract for the sema version-control
//! mirror.
//!
//! The wire vocabulary is generated from `schema/lib.schema`; this file
//! re-exports the generated nouns and attaches the binary configuration
//! archive surface the daemon's startup path decodes.

#[rustfmt::skip]
pub mod schema;

use thiserror::Error;

pub use schema::lib::*;

impl StoreName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl WirePath {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }

    pub fn as_path(&self) -> &std::path::Path {
        std::path::Path::new(self.payload().as_str())
    }
}

impl SocketMode {
    pub fn into_u32(self) -> u32 {
        self.into_payload() as u32
    }
}

impl ListenAddress {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl DaemonConfiguration {
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

    /// Write the binary startup file a deploy tool hands the daemon.
    pub fn write_binary_file(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), ConfigurationArchiveError> {
        std::fs::write(path, self.to_binary_bytes()?).map_err(ConfigurationArchiveError::Write)
    }
}

#[derive(Debug, Error)]
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
