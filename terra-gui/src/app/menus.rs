use egui::Ui;

use crate::ext::UiExt;

use super::{
    tabs::{self, Tabs},
    App, Message, SHORTCUT_EXIT, SHORTCUT_LOAD, SHORTCUT_SAVE,
};

impl App {
    pub fn render_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = false;
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.render_file_menu(ui));
                ui.menu_button("Window", |ui| self.render_window_menu(ui));
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

    fn render_window_menu(&mut self, ui: &mut Ui) {
        if ui.button("Reset").clicked() {
            ui.close_menu();
            *self.tree.write() = tabs::default_ui();
        }

        ui.separator();

        for tab in [Tabs::Stats, Tabs::Bonuses, Tabs::Inventory, Tabs::Buffs] {
            let mut disabled = !self.closed_tabs.contains_key(&tab);

            if ui.checkbox(&mut disabled, format!(" {tab}")).changed() {
                ui.close_menu();

                let mut tree = self.tree.write();

                if let Some(_) = self.closed_tabs.remove(&tab) {
                    tree.push_to_focused_leaf(tab);
                } else if let Some((parent_index, node_index)) = tree.find_tab(&tab) {
                    let parent = tree.iter_mut().nth(parent_index.0).unwrap();
                    parent.remove_tab(node_index);
                    self.closed_tabs.insert(tab, parent_index);
                    tree.remove_empty_leaf();
                }
            }
        }
    }

    fn render_help_menu(&mut self, ui: &mut Ui) {
        if ui.button("About").clicked() {
            self.do_update(Message::ShowAbout);
        };
    }
}
