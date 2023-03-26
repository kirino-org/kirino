use crate::{
  egui::{self, CentralPanel, RichText},
  icon,
};

pub fn sharing(ctx: &egui::Context) {
  CentralPanel::default().show(ctx, |ui| {
    ui.heading(icon!(SMILE, "Sharing"));
    ui.label("Connect with friends' servers and share libraries");
    ui.add_space(5.0);

    ui.group(|ui| {
      ui.vertical_centered(|ui| {
        ui.set_height(ui.available_height());
        ui.label(RichText::new("No shared servers :(").heading().weak());
      })
    });
  });
}
