extern crate eframe;

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

extern crate kirino_proto as proto;
use proto::kirino::{Archive, Item};

pub mod views;
pub mod navigation;
pub mod welcome;
// mod reader;
// mod player;
pub mod widgets;
#[macro_use]
pub mod icon;
//mod player;

pub use views::*;
pub use widgets::*;

pub(crate) const PURPLE: Color32 = Color32::from_rgb(0, 85, 119);

use std::{collections::BTreeMap, sync::Arc};

use views::*;
use eframe::{
  egui::{
    self, mutex::Mutex, ColorImage, FontData, FontDefinitions, FontFamily, Style,
    TextureOptions, Vec2,
  },
  emath::Align2,
  epaint::{Color32, TextureId},
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const BUILD_INFO: &[(&str, &str)] = &[
  ("Version", env!("CARGO_PKG_VERSION")),
  ("License", env!("CARGO_PKG_LICENSE")),
];

#[derive(Clone)]
pub enum Mode {
  Init,
  Home,
  Details(Item),
  Library,
  Sharing,
  Preferences,
  Welcome(welcome::Welcome),
}
#[derive(Clone)]
pub struct Ayase {
  pub mode: Arc<Mutex<Mode>>,
  pub prefs: Arc<Mutex<preferences::Preferences>>,
  pub textures: Arc<Mutex<BTreeMap<String, ColorImage>>>,
  pub library: Arc<Mutex<library::Library>>,
  pub data: Archive,
}

fn style() -> Style {
  Style::default()
}
fn fonts() -> FontDefinitions {
  let mut fn_defs = FontDefinitions::default();
  fn_defs.font_data.insert(
    "Falling Sky".to_string(),
    FontData::from_static(include_bytes!("../assets/fallingsky.otf")),
  );
  fn_defs.families.insert(
    FontFamily::Name("Falling Sky".into()),
    vec!["Falling Sky".to_string()],
  );
  fn_defs
}

impl eframe::App for Ayase {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    use Mode::*;
    let mode = (*self.mode.lock()).clone();

    ctx.set_fonts(fonts());

    egui::gui_zoom::zoom_with_keyboard_shortcuts(
      ctx,
      frame.info().native_pixels_per_point,
    );

    // #[cfg(feature = "desktop")]
    // egui::Window::new("desktop")
    //   .title_bar(false)
    //   .resizable(false)
    //   .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
    //   .show(ctx, |ui| {
    //     ui.spacing_mut().item_spacing = Vec2::splat(5.0);

    //     ui.label("Welcome!");

    //     ui.label("What's your server URL?");
    //     ui.text_edit_singleline(&mut String::default());
    //   });

    if let Welcome(_) = mode {
    } else {
      navigation::nav(self, ctx);
    }
    egui::CentralPanel::default().show(ctx, |ui| {
      match mode {
        Init => {
          let data = self.data.clone();
          {
            let item = &"test";
            if let Ok(img) = image::io::Reader::open(format!("./covers/{}.jpg", item)) {
              if let Ok(img) = img.decode() {
                let size = [img.width() as _, img.height() as _];
                let img = ColorImage::from_rgba_unmultiplied(
                  size,
                  img.to_rgba8().as_flat_samples().as_slice(),
                );
                (*self.textures.lock()).insert(item.to_string(), img);
              }
            };
          }
          for item in data.items {
            //
            if let Ok(img) = image::io::Reader::open(format!("./covers/{}.jpg", item.id)) {
              if let Ok(img) = img.decode() {
                let size = [img.width() as _, img.height() as _];
                let img = ColorImage::from_rgba_unmultiplied(
                  size,
                  img.to_rgba8().as_flat_samples().as_slice(),
                );
                (*self.textures.lock()).insert(item.id.to_string(), img);
              }
            };
          }

          ctx.set_pixels_per_point(1.5);
          *self.mode.lock() = Mode::Preferences;
        }
        /*
        Welcome(u, p) => {
          // if !*self.running.lock() {
          //    ctx.set_pixels_per_point(1.5);
          //}

          // ui.label(RichText::new("Recently added").heading());
          // egui::ScrollArea::new([true, true]).show(ui, |ui| {
          // ui.horizontal_wrapped(|ui| {
          // for i in IMGS.iter() {
          // ui.group(|ui| {
          // ui.vertical(|ui| {
          // i.show_max_size(ui, Vec2 { x: 200.0, y: 200.0 });
          // ui.label("Onii-chan ha Oshimai!");
          // })
          // });
          // }
          // });
          // });
          // ui.separator();
          // ui.label(RichText::new("Continue watching").heading());

          ui.add_space(30.0);

          ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Ayase").size(32.0).heading());
            ui.add_space(50.0);
            ui.vertical(|ui| {
              ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 5.0;
                ui.add(TextEdit::singleline(&mut *u.lock()).hint_text("Username"));
                ui.add(
                  TextEdit::singleline(&mut *p.lock())
                    .password(true)
                    .hint_text("Password"),
                );
                ui.button("Login");
              })
            });
          });

          ui.group(|ui| {
            ui.horizontal(|ui| {
              ui.spacing_mut().item_spacing.x = 0.0;

              ui.label("4.78TB");
              ui.weak(" / 26TB");
            })
          });
        }
                */
        Home => {
          egui::CentralPanel::default().show(ctx, |_ui| {
            // ui.add(library::Library {});
          });
        }
        Details(_data) => {
          /*
                  let tex_id = ctx
          .load_texture(
            data.id.clone(),
            (*(*self.textures.lock()).get(&data.id.clone()).unwrap()).clone(),
            Default::default(),
          )
          .id();
                  */
          ui.add(details::Details(
            // ctx
            //   .load_texture(
            //     "test",
            //     (*(*self.textures.lock()).get("test").unwrap()).clone(),
            //     TextureOptions::NEAREST,
            //   )
            //   .id(),
            TextureId::default(),
          ));
        }
        Preferences => {
          (*self.prefs.lock()).ui(ui);
          ui.collapsing("Build info", |ui| {
            for (k, v) in BUILD_INFO {
              ui.horizontal(|ui| {
                ui.strong(*k);
                ui.label(*v);
              });
            }
          });
        }
        Welcome(wel) => {
          ui.add(wel);
        }
        Library => {
          let data = self.data.clone();
          let lib = self.library.clone();
          (*lib.lock()).ui(self, ui, frame, data);
        }
        Sharing => {
          sharing::sharing(ctx);
        }
      }
      ui.add(widgets::Player {});
    });
  }
}
