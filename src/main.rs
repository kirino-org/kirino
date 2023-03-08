// Yes, I use libstd.
// Yes, I use #![no_std].
//
// Yes, I know what I'm doing here.
#![no_std]

extern crate alloc;
extern crate std;

extern crate kiririn;
extern crate serde;
extern crate tokio;
extern crate toml;
extern crate vsdbsled;

#[cfg(feature = "web")]
mod web;
// mod server;

//#[cfg(feature = "image")]
// mod image;

mod init;

use alloc::{boxed::Box, format, vec::Vec};
use std::{print, println};

use kirino_indexer::Indexer;
use owo_colors::OwoColorize;
use tokio::time::Instant;

pub(crate) fn task(text: &str) -> impl Fn() {
  print!("{}{}", text.cyan(), "...".cyan());
  || println!("{}", " done!".green())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Crappy startup banner
  println!(
    "\n {}\n  {}\n\n  {}\n  {}\n",
    "Kirino Media Server".bright_green().bold(),
    format!("v{}", env!("CARGO_PKG_VERSION")).green(),
    "https://kirino.io/kirino/kirino".green(),
    "https://github.com/kirino-org/kirino".green(),
  );

  let cfg = init::init()?;

  let dbp = cfg.paths.clone().unwrap().db.unwrap();
  let db = vsdbsled::open(dbp)?;

  let mut sc = Indexer::new();
  // sc.push("/mnt/music/dl".to_string());

  if let Some(libs) = cfg.clone().library {
    for l in libs {
      for p in l.paths {
        sc.push(p.clone());
      }
    }
  }

  let st = Instant::now();
  kirino_indexer::index(
    sc,
    kirino_core::Handle {
      plugins: Vec::new(),
      db: db.clone(),
    },
    cfg,
  );
  println!("{:?}", st.elapsed());

  // Load plugins
//  let mut ld = kiririn::Loader::new("./plugins");
//  ld.load_all().unwrap();
//  ld.run_all().unwrap();
//  for pl in ld.all() {
//    println!("{}", pl.name());
//  }

  let mut h = kirino_core::Handle {
    plugins: Vec::new(),
    db: db.clone(),
  };
  //  for item in h.items() {
  //    let d = h.item(item.parse()?);
  //    println!("- {item}\n  '- {} ({})", d.name, d.img);
  //  }

  #[cfg(feature = "web")]
  {
    let done = task("Starting web");
    let jh = web::main(kirino_core::Handle {
      plugins: Vec::new(),
      db: db.clone(),
    });
    done();
    jh.await?;
  }

  #[cfg(not(feature = "web"))]
  {
    // Park current thread
    std::thread::park();
  }

  Ok(())
}
