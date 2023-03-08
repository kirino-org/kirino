use alloc::{boxed::Box, string::String, vec::Vec};
use std::{
  error::Error,
  fs::{create_dir, File},
  io::Read,
  path::Path,
  println,
};

use kirino_core::Config;
use owo_colors::OwoColorize;

use crate::task;

const CONFIG_PATHS: &'static [&'static str] = &["/etc/kirino.toml", "kirino.toml"];

pub fn init() -> Result<Config, Box<dyn Error>> {
  let done = task("Initializing server");

  // If `KIRINO_CONFIG` env var is set, use it as the config path.
  // Otherwise, iterate over possible config files until finding one that exists
  // (if any)
  let cpath: String = if let Ok(c) = std::env::var("KIRINO_CONFIG") {
    Some(c.clone())
  } else {
    let mut cpath = None;
    for c in CONFIG_PATHS {
      if Path::new(c).exists() {
        cpath = Some(String::from(*c));
        break;
      }
    }
    cpath
  }
  .expect("couldn't find config");

  // Open config for reading
  let mut f = File::open(cpath)?;
  let mut cfg: Config = {
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;

    // Attempt to deserialize config
    toml::from_slice(&buf).expect("invalid config")
  };

  // Fill undefined config paths
  let mut paths = cfg.paths.clone().unwrap_or_default();
  paths.fill();
  cfg.paths = Some(paths.clone());

  // Create required dirs if they don't already exist
  for path in &[
    paths.base,
    paths.db.unwrap(),
    paths.cache.unwrap(),
    paths.thumbnails.unwrap(),
  ] {
    if !Path::new(&path).exists() {
      create_dir(path.clone())
        .or_else(|x| {
          println!(
            "{}{}{}",
            "failed to create dir '".red(),
            path.red().bold(),
            "'".red()
          );
          Err(x.into_inner().unwrap())
        })
        .unwrap();
    }
  }

  done();

  Ok(cfg)
}
