//! Current binary Signal contract for owner mirror operations.
//!
//! The contract carries mirror configuration and store policy only. Mirror
//! operations live in `signal-mirror`; runtime decisions live in `mirror`.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against a fresh
//! generation. `examples/canonical.datom` is the authored wire-text witness,
//! actualized line by line by `tests/canonical.rs`.
//!
//! The portable rkyv frame and its three kinds come from `signal` and are
//! re-exported here, so an owner mirror frame is the same type as every other
//! contract's frame.
pub mod generated;
pub use generated::*;

pub use signal_standard::{ByteViewable, Restorable, Signal, Signalizable};

/// The authored Ethos source of this contract.
pub const META_MIRROR_SIGNAL_SOURCE: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`META_MIRROR_SIGNAL_SOURCE`].
pub const META_MIRROR_SIGNAL_RUST: &str = include_str!("generated/signal.rs");
