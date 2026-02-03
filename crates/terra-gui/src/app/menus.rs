use egui::{RichText, Ui, UiKind};

use super::{visuals, App, AppMessage, Message, SHORTCUT_EXIT, SHORTCUT_LOAD, SHORTCUT_SAVE};
use crate::ui::UiExt;

impl App {
    pub fn render_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = false;
            if self.context.is_modal_open() {
                ui.disable();
            }

            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.render_file_menu(ui));
                ui.menu_button("Window", |ui| self.render_window_menu(ui));
                ui.menu_button("Help", |ui| self.render_help_menu(ui));
            });
        });
    }

    fn render_file_menu(&mut self, ui: &mut Ui) {
        if ui.shortcut_button("Load", &SHORTCUT_LOAD).clicked() {
            ui.close_kind(UiKind::Menu);
            self.send_context_msg(Message::LoadPlayer);
        }
        if ui.shortcut_button("Save", &SHORTCUT_SAVE).clicked() {
            ui.close_kind(UiKind::Menu);
            self.send_context_msg(Message::SavePlayer);
        }
        if ui.shortcut_button("Exit", &SHORTCUT_EXIT).clicked() {
            ui.close_kind(UiKind::Menu);
            self.send_app_msg(AppMessage::Exit);
        }
    }

    fn render_window_menu(&mut self, ui: &mut Ui) {
        let mut theme_change = None;

        ui.label(RichText::new("Theme").strong());
        visuals::Theme::iter().for_each(|theme| {
            if ui
                .radio(theme == self.context.theme, theme.to_string())
                .clicked()
            {
                theme_change = Some(theme);
            }
        });

        if let Some(theme) = theme_change {
            self.send_context_msg(Message::SetTheme(theme));
            ui.close_kind(UiKind::Menu);
        }

        ui.separator();

        if ui.button("Reset Tabs").clicked() {
            ui.close_kind(UiKind::Menu);
            self.send_app_msg(AppMessage::ResetTabs);
        }
    }

    fn render_help_menu(&mut self, ui: &mut Ui) {
        if ui.button("About").clicked() {
            ui.close_kind(UiKind::Menu);
            self.send_context_msg(Message::ShowAbout);
        };
    }
}
