use egui::Ui;
use egui_dock::{Node, NodeIndex};

use crate::ui::UiExt;

use super::{
    tabs::{self, Tabs},
    visuals, App, Message, SHORTCUT_EXIT, SHORTCUT_LOAD, SHORTCUT_SAVE,
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
        let mut theme_change = None;

        ui.labelled("Theme: ", |ui| {
            egui::ComboBox::new("window_theme", "")
                .selected_text(self.theme.to_string())
                .show_ui(ui, |ui| {
                    let mut current_theme = self.theme;

                    for theme in visuals::Theme::iter() {
                        if ui
                            .selectable_value(&mut current_theme, theme, theme.to_string())
                            .clicked()
                        {
                            theme_change = Some(theme);
                        }
                    }
                });
        });

        if let Some(theme) = theme_change {
            self.do_update(Message::SetTheme(theme));
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Reset Tabs").clicked() {
            ui.close_menu();
            *self.tree.write() = tabs::default_ui();
        }

        ui.separator();

        for tab in Tabs::iter() {
            let mut disabled = !self.closed_tabs.contains_key(&tab);

            if ui.checkbox(&mut disabled, format!(" {tab}")).changed() {
                let mut tree = self.tree.write();

                if self.closed_tabs.remove(&tab).is_some() {
                    tree.push_to_focused_leaf(tab);
                } else if let Some((parent_index, node_index)) = tree.find_tab(&tab) {
                    let parent = tree.iter_mut().nth(parent_index.0).unwrap();
                    parent.remove_tab(node_index);
                    self.closed_tabs.insert(tab, parent_index);

                    // NOTE: Below is just inlined remove_empty_leaf (which was removed in egui_dock v0.5.0)
                    let mut nodes = tree.iter().enumerate();
                    let node = nodes.find_map(|(index, node)| match node {
                        Node::Leaf { tabs, .. } if tabs.is_empty() => Some(index),
                        _ => None,
                    });

                    if let Some(node) = node {
                        let node = NodeIndex(node);
                        (*tree).remove_leaf(node);
                    }
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
