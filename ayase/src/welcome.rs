use std::sync::Arc;

use eframe::egui::{self, mutex::Mutex, Direction, Layout, Widget};

use crate::{icon, widgets};

#[derive(Clone, Copy)]
pub enum Mode {
  First,
  Keybinds,
  Project,
}

#[derive(Clone)]
pub struct Welcome {
  mode: Arc<Mutex<Mode>>,
}
impl Widget for Welcome {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    use Mode::*;

    use crate::icon::*;
    ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| ui.group(|ui| ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            ui.set_height(ui.available_height());

            let mode = *Arc::clone(&self.mode).lock();
            match mode {
                First => {
                    ui.heading(format!("{MUSIC_NOTE} {BOOK} {TELEVISION} {TAPE_CARTRIDGE} {MOVIE_CAMERA}"));
                    ui.heading("Welcome!");
                    {
                        ui.label("This is Kirino, the lightweight, modular media server.");
                        ui.label("Thank you for trying it out! It helps us out a lot. Feel free to give feedback or contribute on Github.");
                    }
                    ui.add_space(5.0);
                    if ui.button("Get Started").clicked() {
                        *self.mode.lock() = Project;
                    }
                    if ui.button("Keybinds").clicked() {
                        *self.mode.lock() = Keybinds;
                    }
                }
                Keybinds => {
                    ui.heading("Keybinds");
                    ui.group(|ui| {
                        ui.horizontal(|ui| ui.columns(2, |col| {
                            col[0].strong("/");
                            col[1].label("Search");
                        }));
                    });
                }
                _Plugins => {
                  ui.heading(icon!(PLUG, "Plugins"));
                  ui.label("Plugins power Kirino. Let's get a few installed to get you started.");
                  egui::ScrollArea::new([false, true]).show(ui, |ui| {
                    ui.add(widgets::Plugin {
                      name: "MusicBrainz".to_string(),
                      description: "THE music database".to_string(),
                      version: "v0.1.0".to_string(),
                    });
                    ui.add(widgets::Plugin {
                      name: "YouTube".to_string(),
                      description: "Stream videos and music from YouTube & YouTube Music".to_string(),
                      version: "v0.1.0".to_string(),
                    });
                    ui.add(widgets::Plugin {
                      name: "Bandcamp".to_string(),
                      description: "Stream purchased music from Bandcamp".to_string(),
                      version: "v0.1.0".to_string(),
                    });
                    ui.add(widgets::Plugin {
                      name: "Mora".to_string(),
                      description: "Stream purchased music from Mora, Sony's Hi-Res music service".to_string(),
                      version: "v0.1.0".to_string(),
                    });
                    ui.add(widgets::Plugin {
                      name: "OTOTOY".to_string(),
                      description: "Stream purchased music from OTOTOY, an independent Hi-Res music service".to_string(),
                      version: "v0.1.0".to_string(),
                    });
                  });
                }
                Project => {
                    ui.heading(icon!(HEART, "Support"));
                    {
                      ui.label("Thank you for trying Kirino!");
                      ui.label("If you find this project to be of any value, please consider supporting its development.");
                    }
                    {
                      ui.hyperlink_to(icon!(GITHUB, "Sauce"), "https://github.com/kirino-org/kirino");
                      ui.hyperlink_to(icon!(BUG, "Bugs"), "https://github.com/kirino-org/kirino/issues");
                    }
                    if ui.button("Ok").clicked() {
                      // get rid of the sad puppy dog garbage
                    }
                }
            }})));
    ui.scope(|_| {}).response
  }
}
impl Default for Welcome {
  fn default() -> Self {
    Self {
      mode: Arc::new(Mutex::new(Mode::First)),
    }
  }
}
