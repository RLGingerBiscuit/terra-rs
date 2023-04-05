use egui::Ui;

use crate::ext::UiExt;

use super::{App, Message, SHORTCUT_EXIT, SHORTCUT_LOAD, SHORTCUT_SAVE};

impl App {
    pub fn render_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = false;
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.render_file_menu(ui));
                ui.menu_button("Help", |ui| self.render_help_menu(ui));
            });
        });
    }

    fn render_file_menu(&mut self, ui: &mut Ui) {
        if ui.shortcut_button("Load", &SHORTCUT_LOAD).clicked() {
            ui.close_menu();
            self.do_update(Message::LoadPlayer);
        }
        if ui.shortcut_button("Save", &SHORTCUT_SAVE).clicked() {
            ui.close_menu();
            self.do_update(Message::SavePlayer);
        }
        if ui.shortcut_button("Exit", &SHORTCUT_EXIT).clicked() {
            ui.close_menu();
            self.do_update(Message::Exit);
        }
    }

    fn render_help_menu(&mut self, ui: &mut Ui) {
        if ui.button("About").clicked() {
            self.do_update(Message::ShowAbout);
        };
    }
}
