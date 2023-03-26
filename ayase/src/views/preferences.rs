use std::sync::Arc;

use eframe::egui::{self, mutex::Mutex, TextEdit, Ui};

use crate::icon;

#[derive(Clone)]
pub struct Preferences {
  pub server_url: Arc<Mutex<String>>,
  pub items_per_row: Arc<Mutex<String>>,
}
impl Preferences {
  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.collapsing(icon!(COMPUTER, "Server"), |ui| {
      fn text_single(ui: &mut Ui, _data: &mut String) {
        ui.add_enabled(
          false,
          TextEdit::singleline(&mut String::from("http://192.168.1.123")),
        );
      }
      field(
        ui,
        "Server URL",
        "You must log out to change this",
        &mut String::from("http://192.168.1.123"),
        text_single,
      );
    });
    ui.collapsing(icon!(IMAGE, "Layout"), |ui| {
      // ComboBox::from_label("Mode").show_ui(ui, |ui| {
      //    ui.selectable_value(false, selected_value, text)
      //});

      fn text_single(ui: &mut Ui, data: &mut String) {
        ui.text_edit_singleline(data);
      }

      field(
        ui,
        "Items per row",
        "How many items to show per row in library view",
        &mut self.items_per_row.lock(),
        text_single,
      );
    });
    ui.collapsing(icon!(PLUG, "Plugins"), |_ui| {});

    ui.button("Apply");
  }
}
impl Default for Preferences {
  fn default() -> Self {
    Self {
      server_url: Arc::new(Mutex::new(String::new())),
      items_per_row: Arc::new(Mutex::new(String::new())),
    }
  }
}

fn field(
  ui: &mut Ui,
  title: &'static str,
  desc: &'static str,
  data: &mut String,
  body: fn(&mut Ui, &mut String) -> (),
) {
  ui.add_space(5.0);
  ui.scope(|ui| {
    ui.spacing_mut().item_spacing.y = 0.5;

    ui.label(title);
    ui.weak(desc);
    ui.add_space(5.0);
    body(ui, data);
  });
  ui.add_space(5.0);
}
