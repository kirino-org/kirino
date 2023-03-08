use alloc::{format, string::String, vec::Vec};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub paths: Option<Paths>,
  pub library: Option<Vec<Library>>,
  pub web: Option<Web>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Paths {
  pub base: String,
  pub db: Option<String>,
  pub cache: Option<String>,
  pub thumbnails: Option<String>,
}
impl Paths {
  pub fn fill(&mut self) {
    macro_rules! fill {
      ( $( $f:ident ),* ) => {
        $(
        if self.$f.is_none() {
          self.$f = Some(format!("{}/{}", self.base, stringify!($f)))
        }
        )*
      };
    }
    fill!(db, cache, thumbnails);
  }
}
impl Default for Paths {
  fn default() -> Self {
    Self {
      base: String::from("/var/lib/kirino"),
      db: None,
      cache: None,
      thumbnails: None,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Library {
  pub name: String,
  pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Web {
  pub listen_addr: String,
}
impl Default for Web {
  fn default() -> Self {
    Self {
      listen_addr: String::from("127.0.0.1:8698"),
    }
  }
}
