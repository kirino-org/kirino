use std::{fmt::Display, sync::Arc};

use eframe::{egui::{
  self, emath::Vec2, mutex::Mutex, ComboBox, Grid, Label, Layout, RichText, Sense,
  TextEdit, TextureOptions, Widget,
}, epaint::TextureId};
use kirino_proto::kirino::Archive;

use crate::{
  icon::{self, *},
  widgets, Ayase,
};

pub struct Library {
  search: Arc<Mutex<String>>,
  layout: Arc<Mutex<LibLayout>>,
  togis: Arc<Mutex<Vec<ToggleIcon>>>,
  lib: Arc<Mutex<widgets::Library>>,
  item: Arc<Mutex<widgets::Item>>,
}
impl Library {
  pub fn ui(
    &self,
    h: &mut Ayase,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    data: Archive,
  ) -> egui::Response {
    // Screen width
    let scr_width =
      unsafe { frame.info().window_info.size.x.to_int_unchecked::<usize>() };
    ui.scope(|ui| {
      ui.spacing_mut().item_spacing.x = 5.0;
      ui.spacing_mut().item_spacing.y = 10.0;
      ui.group(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.horizontal(|ui| {
          ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add(
              TextEdit::singleline(&mut *self.search.lock())
                .hint_text("")
                .margin(Vec2::new(4.0, 2.5))
                .desired_width(scr_width as f32 / 5.0),
            );
            ui.button(SEARCH);
          });
          ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            // Minimal- Cover art
            // Simple- Cover art and title
            // Extended- Cover art, title, author (if applicable),
            // volume/track/disc/episode/season count
            //
            let curr = *self.layout.lock();
            ComboBox::from_label(icon::IMAGE)
              .selected_text(format!("{curr}"))
              .show_ui(ui, |ui| {
                use LibLayout::*;
                ui.selectable_value(&mut *self.layout.lock(), Minimal, "Minimal");
                ui.selectable_value(&mut *self.layout.lock(), Simple, "Simple");
                ui.selectable_value(&mut *self.layout.lock(), Extended, "Extended");
              });
            ui.add_space(10.0);
            for t in (*self.togis.lock()).iter().clone() {
              t.clone().ui(ui);
            }
          });
        });
      });

      {
        (*self.lib.lock()).clone().ui(ui);
      }

      egui::ScrollArea::new([false, true]).show_viewport(ui, |ui, _| {
        let _prefs = Arc::clone(&h.prefs);

        // Calculate *i*tems *p*er *r*ow
        let _ipr: usize = scr_width / 240;
        //(*(*prefs.lock()).items_per_row.lock()).clone().parse().unwrap();
        ui.set_width(ui.available_width());
        Grid::new("library")
          .min_col_width(240.0)
          .spacing(Vec2::new(10.0, 10.0))
          .num_columns(
            unsafe { ui.available_width().round().to_int_unchecked::<usize>() } / 250,
          )
          .show(ui, |ui| {
            let item = self.item.clone();
            (*item.lock()).clone().ui(
              h,
              // ui.ctx()
              //   .load_texture(
              //     "test",
              //     (*(*h.textures.clone().lock()).get("test").unwrap()).clone(),
              //     TextureOptions::NEAREST,
              //   )
              //   .id(),
              TextureId::default(),
              ui,
            );
            /*
            for (i, item) in data
              .items
              .iter()
              .filter(|x| {
                x.name
                  .to_lowercase()
                  .as_str()
                  .contains((*self.search.lock()).to_lowercase().as_str())
              })
              .enumerate()
            {
              ui.scope(|ui| widgets::Item::default().ui(ui));
              if (i + 1) % ipr == 0 {
                ui.end_row()
              }
            }
                        */
        });
        for item in data.items {
          println!("[{}][{:?}] {}", item.id, item.r#type(), item.name);
        }
      });
    })
    .response
  }
}
impl Default for Library {
  fn default() -> Self {
    Self {
      search: Arc::new(Mutex::new(String::new())),
      layout: Arc::new(Mutex::new(LibLayout::Simple)),
      togis: Arc::new(Mutex::new(vec![
        ToggleIcon::new(icon::MUSIC_NOTE),
        ToggleIcon::new(icon::TELEVISION),
        ToggleIcon::new(icon::BOOK),
        ToggleIcon::new(icon::MOVIE_CAMERA),
        ToggleIcon::new(icon::MOVIE_TAPE),
      ])),
      lib: Arc::new(Mutex::new(widgets::Library::new("Example Library"))),
      item: Arc::new(Mutex::new(widgets::Item::default())),
    }
  }
}

#[derive(Clone)]
struct ToggleIcon(&'static str, Arc<Mutex<bool>>, Arc<Mutex<bool>>);
impl ToggleIcon {
  pub fn new(icon: &'static str) -> Self {
    Self(
      icon,
      Arc::new(Mutex::new(false)),
      Arc::new(Mutex::new(false)),
    )
  }
}
impl Widget for ToggleIcon {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    ui.scope(|ui| {
      let Self(i, hov, sel) = self;
      {
        let is_hov = *hov.lock();
        let is_sel = *sel.lock();

        let lbl = ui.add(
          Label::new(if is_hov | is_sel {
            // Regular font if hovered or selected
            RichText::new(i)
          } else {
            // Weaker font if not
            RichText::new(i).weak()
          })
            // Enable sensitivity to clicks and hovering
          .sense(Sense::click().union(Sense::hover())),
        );

        *hov.lock() = lbl.hovered();

        // If clicked, toggle
        //
        // So cool how simple this can be!
        if lbl.clicked() {
          *sel.lock() = !is_sel;
        }
      }
    })
    .response
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LibLayout {
  Minimal,
  Simple,
  Extended,
}
impl Display for LibLayout {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use LibLayout::*;
    write!(
      f,
      "{}",
      match self {
        Minimal => "Minimal",
        Simple => "Simple",
        Extended => "Extended",
      }
    )
  }
}
