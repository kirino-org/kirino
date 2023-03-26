use eframe::egui::{self, Widget};

pub struct Plugin {
  pub name: String,
  pub description: String,
  pub version: String,
}
impl Widget for Plugin {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    ui.scope(|ui| {
      ui.group(|ui| {
        ui.horizontal(|ui| {
          ui.label(self.name);
          ui.weak(self.version);
        });
        ui.label(self.description);
        ui.horizontal(|ui| {
          ui.button("Enable");
        });
      })
    })
    .response
  }
}
