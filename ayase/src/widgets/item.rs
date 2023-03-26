use std::sync::Arc;

use eframe::{
  egui::{self, emath::Vec2, mutex::Mutex, Frame, Sense, Style, TextureId},
  epaint::Shadow,
};

use crate::{Ayase, Mode};

#[derive(Clone)]
pub struct Item {
  hov: Arc<Mutex<bool>>,
}
impl Item {
  pub fn ui(self, h: &mut Ayase, txtr: TextureId, ui: &mut egui::Ui) -> egui::Response {
    let is_hov = *self.hov.lock();

    let mut res = ui
      .scope(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(25.0, 25.0);
        ui.vertical_centered(|ui| {
          let mut grp = Frame::group(&Style::default());
          if is_hov {
            grp = grp.shadow(Shadow::small_light());
          }
          grp.show(ui, |ui| {
            ui.set_width(240.0);
            ui.spacing_mut().item_spacing = Vec2::new(5.0, 5.0);

            // ui.image(
            //  ui.ctx()
            //    .load_texture(, img, Default::default())
            //    .id(),
            //    Vec2::new(240.0, 135.0),
            //);
            ui.image(txtr, Vec2::new(240.0, 135.0));

            ui.scope(|ui| {
              ui.spacing_mut().item_spacing.y = 0.5;
              ui.label("Why do hurricane lanterns look like that?");
            });
          })
        })
      })
      .response;

    res = res.interact(Sense::click().union(Sense::hover()));
    if res.clicked() {
      *h.mode.lock() = Mode::Details(proto::kirino::Item::default());
    }

    *self.hov.lock() = res.hovered();
    res
  }
}
impl Default for Item {
  fn default() -> Self {
    Self {
      hov: Arc::new(Mutex::new(false)),
    }
  }
}
