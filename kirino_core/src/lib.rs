#![no_std]

extern crate alloc;
extern crate serde;
extern crate vsdbsled;
//#[cfg(doc)]
// extern crate kiririn;

mod config;
mod errors;

use alloc::{
  boxed::Box,
  format,
  string::{String, ToString},
  vec::Vec,
};

pub use config::{Config, Library, Paths, Web};
use serde::Serialize;
use vsdbsled::Db;

/// Handle to interact with Kirino (as a plugin)
///
/// Passed to plugins in kiririn::PluginImpl::run()
#[derive(Clone)]
pub struct Handle {
  pub plugins: Vec<Plugin>,
  pub db: Db,
}
impl Handle {
  /// Add [Item] **`item`** to database
  pub fn add(&mut self, item: Box<dyn Fn(u64) -> Item>) {
    let id = self.db.generate_id().unwrap();

    let item = item(id);

    let items = self.db.open_tree("items").unwrap();
    items
      .insert(format!("item:{id}").as_bytes(), item.name.as_bytes())
      .unwrap();
    items
      .insert(format!("{id}:path").as_bytes(), item.path.as_bytes())
      .unwrap();
    items
      .insert(format!("{id}:img").as_bytes(), item.img.as_bytes())
      .unwrap();
  }
  pub fn item(&mut self, id: u64) -> Item {
    let tr = self.db.open_tree("items").unwrap();
    Item {
      name: core::str::from_utf8(
        tr.get(format!("item:{id}"))
          .unwrap()
          .unwrap()
          .to_vec()
          .as_slice(),
      )
      .unwrap()
      .to_string(),
      path: core::str::from_utf8(
        tr.get(format!("{id}:path"))
          .unwrap()
          .unwrap()
          .to_vec()
          .as_slice(),
      )
      .unwrap()
      .to_string(),
      img: core::str::from_utf8(
        tr.get(format!("{id}:img"))
          .unwrap()
          .unwrap()
          .to_vec()
          .as_slice(),
      )
      .unwrap()
      .to_string(),
    }
  }
  /// Get all [`items`](Item)
  pub fn items(&mut self) -> Vec<String> {
    self
      .db
      .open_tree("items")
      .unwrap()
      .scan_prefix("item:")
      .map(|x| {
        core::str::from_utf8(x.unwrap().0.to_vec().as_slice())
          .unwrap()
          .strip_prefix("item:")
          .unwrap()
          .to_string()
      })
      .collect::<Vec<String>>()
  }
}

#[derive(Clone, Serialize)]
pub struct Plugin {
  pub name: String,
  pub description: String,
}

#[derive(Clone, Serialize)]
pub struct Item {
  pub name: String,
  pub path: String,
  pub img: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
  Service,
  Storage,
  Metadata,
  Fetcher,
}
