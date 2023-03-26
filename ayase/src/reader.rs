use eframe::egui::{Widget, KeyboardShortcut as KbdSh, Modifiers, Key};

pub struct Reader {
}
impl Widget for Reader {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.scope(|ui| {
            let l_arr = &KbdSh::new(Modifiers::NONE, Key::ArrowLeft);
            ui.input().consume_shortcut(l_arr);
        }).response
    }
}
