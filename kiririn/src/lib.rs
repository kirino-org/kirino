//! # Kiririn
//!
//! WebAssembly-powered plugin runtime for Rust
//!
//! ```rust
//! let rt = Loader::new("./plugins")?;
//! rt.load_all()?;
//! rt.run_all()?;
//!
//! rt.only(proto::plugin::Type::Metadata, |_pl| {
//!   // Do something
//! })
//! ```

#![no_std]
extern crate kirino_proto;

pub mod doc;

#[cfg(feature = "rt")]
mod runtime;

#[cfg(not(feature = "rt"))]
pub use kirino_proto::*;
#[cfg(feature = "rt")]
pub mod api;
#[cfg(feature = "rt")]
pub use runtime::*;
