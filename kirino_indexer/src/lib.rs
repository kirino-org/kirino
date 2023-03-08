#![doc = include_str!("../README.md")]

extern crate burrow;
#[cfg(feature = "ffthumb")]
extern crate ffthumb;
extern crate infer;
extern crate rayon;
#[cfg(feature = "symphonia")]
extern crate symphonia;

extern crate kirino_core;

use std::{
  io::Read,
  path::Path,
  sync::{Arc, Mutex},
};

use burrow::Burrow;
#[cfg(feature = "ffthumb")]
use ffthumb::Thumbnailer;
use infer::MatcherType as Type;
use kirino_core::Handle;
use rayon::prelude::*;
#[cfg(feature = "symphonia")]
use symphonia::{
  core::{
    formats::FormatOptions,
    io::{MediaSourceStream, MediaSourceStreamOptions},
    meta::MetadataOptions,
    probe::Hint,
  },
  default::get_probe,
};

#[derive(Clone, Debug)]
pub struct File(pub MediaType, pub String);

pub type Indexer = Vec<String>;

/// Index `sc`
pub fn index(sc: Indexer, mut handle: Handle, cfg: kirino_core::Config) -> Vec<File> {
  let files: Arc<Mutex<Vec<File>>> = Arc::new(Mutex::new(Vec::new()));
  let paths = cfg.paths.clone().unwrap();

  for dir in sc {
    let walker = Burrow::index(&dir, true).unwrap().iter();

    walker.into_par_iter().for_each(|p| {
      let p = p.clone();
      let mut handle = handle.clone();

      let mut f = std::fs::File::open(p.clone()).unwrap();
      let ty = magic_nums(&mut f);

      if let Some(ty) = ty {
        match ty {
          Type::Video => {
            #[cfg(feature = "ffthumb")]
            {
              let thumbp = paths.clone().thumbnails.unwrap();
              handle.add(Box::new(move |id| {
                let mut th = Thumbnailer::new();
                th.set_size(512, 0);
                let out = format!("{}/{id}.png", thumbp);
                th.generate_to_file(&p, &out);

                println!("{p}");
                kirino_core::Item {
                  name: Path::new(&p)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                  path: p.clone(),
                  img: out,
                }
              }));
            }
            MediaType::Video
          }
          Type::Audio => {
            //#[cfg(feature = "symphonia")]
            // audio_meta(&p);
            MediaType::Audio
          }
          Type::Image => MediaType::Photo,
          Type::Archive => MediaType::Gallery,
          _ => MediaType::Other,
        };
      } else {
        MediaType::Other;
      }
    });
  }

  Arc::try_unwrap(files).unwrap().into_inner().unwrap()
}

/// Determine media type based on magic numbers
fn magic_nums(f: &mut std::fs::File) -> Option<Type> {
  // Read the first 128 bytes into a buffer
  let mut buf = Vec::with_capacity(128);
  f.take(128).read_to_end(&mut buf).unwrap();

  // -`, Magic Numbers `,-
  let ty = infer::get(&buf);
  if let Some(ty) = ty {
    Some(ty.matcher_type())
  } else {
    None
  }
}

/// Extract audio metadata from `p` with [symphonia]
///
/// > **Requires the `symphonia` feature**
#[cfg(feature = "symphonia")]
fn audio_meta(p: &str) {
  let f = std::fs::File::open(p).unwrap();
  // Probe for format
  if let Ok(mut fmt) = get_probe().format(
    &Hint::default(),
    MediaSourceStream::new(Box::new(f), MediaSourceStreamOptions::default()),
    &FormatOptions::default(),
    &MetadataOptions::default(),
  ) {
    // Get metadata
    if let Some(meta) = fmt.format.metadata().current() {
      // Iterate over tags
      for t in meta.tags() {
        println!("{}: {}", t.key, t.value);
      }
      // Iterate over visuals (attached images)
      for (i, v) in meta.visuals().iter().enumerate() {
        let dim = v.dimensions.unwrap();
        println!("{}x{}", dim.width, dim.height);

        // std::fs::remove_file(format!(
        // "{p}.{i}.{}",
        // v.media_type.split_once('/').unwrap().1
        // ))
        // .unwrap();
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
  Video,
  Audio,
  Photo,
  Gallery,

  Other,
}
