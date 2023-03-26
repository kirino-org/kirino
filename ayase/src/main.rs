use eframe::egui::mutex::Mutex;
use kirino_proto::Message;
use std::{collections::BTreeMap, sync::Arc};

use ayase::{library, preferences, Ayase, Mode};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_os = "android"))]
#[tokio::main]
async fn main() -> std::io::Result<()> {
  let buf = {
    if let Ok(mut res) = surf::get("http://localhost:8080/meta/archive.pb")
      .send()
      .await
    {
      res.body_bytes().await.unwrap()
    } else {
      Vec::new()
    }
  };
  let data = kirino_proto::kirino::Archive::decode(buf.as_slice()).unwrap_or_default();

  eframe::run_native(
    "Ayase",
    eframe::NativeOptions {
      ..Default::default()
    },
    Box::new(|_| {
      Box::new(Ayase {
        mode: Arc::new(Mutex::new(Mode::Init)),
        prefs: Arc::new(Mutex::new(preferences::Preferences::default())),
        textures: Arc::new(Mutex::new(BTreeMap::new())),
        library: Arc::new(Mutex::new(library::Library::default())),
        data,
      })
    }),
  );

  Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main_js() {
  console_error_panic_hook::set_once();

  wasm_bindgen_futures::spawn_local(async {
    eframe::start_web(
      "ayase",
      eframe::WebOptions::default(),
      Box::new(|_| {
        Box::new(Ayase {
          mode: Arc::new(Mutex::new(Mode::Init)),
          // prefs: preferences::Preferences {
          //   server_url: Arc::new(std::sync::Mutex::new(String::new())),
          // },
        })
      }),
    )
    .await
    .expect("failed to start up");
  });
}

#[cfg(target_arch = "wasm32")]
fn main() {}
