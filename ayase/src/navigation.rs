use crate::{
  egui::{self, Layout},
  icon,
  icon::*,
  Ayase, Mode,
};

pub fn nav(h: &mut Ayase, ctx: &egui::Context) {
  egui::TopBottomPanel::top("nav")
    .show_separator_line(false)
    .exact_height(10.0)
    .show(ctx, |ui| {
      ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
          if ui.button(icon!(BOOKS, "Library")).clicked() {
            *h.mode.lock() = Mode::Library;
          }
          if ui.button(icon!(SMILE, "Sharing")).clicked() {
            *h.mode.lock() = Mode::Sharing;
          }
          //});
          ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(COG).clicked() {
              *h.mode.lock() = Mode::Preferences;
            }
          });
        });
      });
    });
}
