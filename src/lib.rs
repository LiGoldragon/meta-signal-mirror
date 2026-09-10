//! Current binary Signal contract for owner mirror operations.
pub mod generated;
pub use generated::*;
pub const META_MIRROR_SIGNAL_SOURCE: &str = include_str!("../ethos/signal.ethos");
pub const META_MIRROR_SIGNAL_RUST: &str = include_str!("generated/signal.rs");
