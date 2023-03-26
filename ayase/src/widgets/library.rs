use std::sync::Arc;

use eframe::{
  egui::{self, mutex::Mutex, Frame, Sense, Style, Widget},
  epaint::Shadow,
};

use crate::icon::*;

#[derive(Clone)]
pub struct Library {
  pub name: String,
  hov: Arc<Mutex<bool>>,
}
impl Library {
  pub fn new(name: &str) -> Self {
    Self {
      name: String::from(name),
      hov: Arc::new(Mutex::new(false)),
    }
  }
}
impl Widget for Library {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    let is_hov = *self.hov.lock();

    let mut res = ui
      .scope(|ui| {
        let mut grp = Frame::group(&Style::default());
        if is_hov {
          grp = grp.shadow(Shadow::small_light());
        }
        grp.show(ui, |ui| {
          ui.horizontal(|ui| {
            ui.group(|ui| ui.heading(BOOKS));
            ui.label(self.name);
          })
        })
      })
      .response;
    res = res.interact(Sense::click().union(Sense::hover()));
    if res.clicked() {
      // TODO
    }
    *self.hov.lock() = res.hovered();
    res
  }
}
