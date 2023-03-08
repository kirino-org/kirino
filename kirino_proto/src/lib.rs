#![no_std]
extern crate prost;

pub mod proto {
  /// Re-export of [prost::Message]
  pub use prost::Message;
  include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
}

pub use kirino::*;
pub use proto::*;
