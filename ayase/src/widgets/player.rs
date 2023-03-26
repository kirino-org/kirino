use eframe::{
  egui::{self, style::Margin, Button, Frame, Layout, RichText, Widget},
  emath::Align,
  epaint::Vec2,
};

use crate::{icon, PURPLE};

pub struct Player {}
impl Widget for Player {
  fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
    ui.spacing_mut().item_spacing = Vec2::splat(15.0);
    egui::TopBottomPanel::bottom("player")
      .frame(
        Frame::default()
          .inner_margin(Margin::symmetric(10.0, 15.0))
          .fill(ui.visuals().faint_bg_color.to_opaque()),
      )
      .show(ui.ctx(), |ui| {
        //ui.add(Slider::new(value, range))
        ui.horizontal(|ui| {
          ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.image(
              ui.ctx()
                .load_texture(
                  "example",
                  egui::ColorImage::example(),
                  egui::TextureOptions::default(),
                )
                .id(),
              Vec2::splat(40.0),
            );
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
              ui.add_space(5.0);
              ui.strong("Hello, world!");
              ui.label("The Examples");
              ui.shrink_height_to_current();
            });
          });
          ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.button(RichText::new(icon::NEXT).size(18.0));
            ui.add(Button::new(RichText::new(icon::PLAY).size(24.0)).fill(PURPLE));
            ui.button(RichText::new(icon::PREV).size(18.0));
          });
        })
      })
      .response
  }
}
