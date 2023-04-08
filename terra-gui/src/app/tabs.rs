use std::fmt::Display;

use egui::{Ui, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};
use terra_core::{utils, Difficulty};

use crate::{enum_radio_value, ext::UiExt};

use super::{App, Message};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tabs {
    Stats,
    LoadSave,
    Bonuses,
    Inventory,
    Buffs,
}

impl Display for Tabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tabs::LoadSave => "Load & Save",
                Tabs::Stats => "Stats",
                Tabs::Bonuses => "Permanent Bonuses",
                Tabs::Inventory => "Inventory",
                Tabs::Buffs => "Buffs",
            }
        )
    }
}

pub fn default_ui() -> Tree<Tabs> {
    let mut tree = Tree::new(vec![Tabs::LoadSave]);
    let [load_save, _inventory] =
        tree.split_below(0.into(), 0.315, vec![Tabs::Inventory, Tabs::Buffs]);
    let [load_save, _stats] = tree.split_right(load_save, 0.15, vec![Tabs::Stats, Tabs::Bonuses]);

    tree.set_focused_node(load_save);
    tree
}

impl App {
    fn render_load_save_tab(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.spacing_mut().item_spacing = [16., 16.].into();
            if ui.button("Load Player").clicked() {
                self.do_update(Message::LoadPlayer);
            }
            if ui.button("Save Player").clicked() {
                self.do_update(Message::SavePlayer);
            }
            if ui.button("Reset Player").clicked() {
                self.do_update(Message::ResetPlayer);
            }
        });
    }

    fn render_stats_tab(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();

        ui.labelled("Name: ", |ui| {
            ui.text_edit_singleline(&mut player.name);
        });

        ui.labelled("Difficulty: ", |ui| {
            enum_radio_value!(
                ui,
                &mut player.difficulty,
                Difficulty::Journey,
                Difficulty::Classic,
                Difficulty::Mediumcore,
                Difficulty::Hardcore
            );
        });

        ui.labelled("Version:", |ui| {
            ui.drag_value_with_buttons(&mut player.version, 1., 0..=i32::MAX);
            ui.label("/");
            ui.label(utils::version_lookup(player.version));
        });

        ui.labelled("Health: ", |ui| {
            ui.drag_value_with_buttons(&mut player.life, 1., 0..=i32::MAX);
            ui.label("/");
            ui.drag_value_with_buttons(&mut player.max_life, 1., 0..=i32::MAX);
        });

        ui.labelled("Mana: ", |ui| {
            ui.drag_value_with_buttons(&mut player.mana, 1., 0..=i32::MAX);
            ui.label("/");
            ui.drag_value_with_buttons(&mut player.max_mana, 1., 0..=i32::MAX);
        });

        ui.labelled("Fishing quests: ", |ui| {
            ui.drag_value_with_buttons(&mut player.angler_quests, 1., 0..=i32::MAX);
        });

        ui.labelled("Golfer score: ", |ui| {
            ui.drag_value_with_buttons(&mut player.golfer_score, 1., 0..=i32::MAX);
        });
    }

    fn render_bonuses_tab(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();
        // TODO: Display icons
        // const HEART_ID: i32 = 3335;
        // const FAVOR_ID: i32 = 5043;
        // const LOAF_ID: i32 = 5326;
        // const VITAL_ID: i32 = 5337;
        // const FRUIT_ID: i32 = 5338;
        // const ARCANE_ID: i32 = 5339;
        // const PEARL_ID: i32 = 5340;
        // const WORM_ID: i32 = 5341;
        // const AMBROSIA_ID: i32 = 5342;
        // const ETERNIA_ID: i32 = 3828;
        // const CART_ID: i32 = 5289;

        egui::Grid::new("player_bonuses")
            .num_columns(2)
            .show(ui, |ui| {
                ui.checkbox(&mut player.demon_heart, "Demon Heart");
                ui.end_row();
                ui.checkbox(&mut player.biome_torches, "Torch God's Favor");
                ui.checkbox(
                    &mut player.biome_torches_enabled,
                    "Biome torch swap enabled",
                );
                ui.end_row();
                ui.checkbox(&mut player.artisan_loaf, "Artisan Loaf");
                ui.checkbox(&mut player.vital_crystal, "Vital Crystal");
                ui.end_row();
                ui.checkbox(&mut player.aegis_fruit, "Aegis Fruit");
                ui.checkbox(&mut player.arcane_crystal, "Arcane Crystal");
                ui.end_row();
                ui.checkbox(&mut player.galaxy_pearl, "Galaxy Pearl");
                ui.checkbox(&mut player.gummy_worm, "Gummy Worm");
                ui.end_row();
                ui.checkbox(&mut player.ambrosia, "Ambrosia");
                ui.end_row();
                ui.checkbox(&mut player.defeated_ooa, "Killed Old One's Army");
                ui.end_row();
                ui.checkbox(&mut player.super_cart, "Minecart Upgrade Kit");
                ui.checkbox(&mut player.super_cart_enabled, "Boosted minecart enabled");
            });
    }

    fn render_inventory_tab(&mut self, ui: &mut Ui) {
        let player = self.player.write();

        egui::Grid::new("player_inventory")
            .num_columns(10)
            .show(ui, |ui| {
                for i in 0..player.inventory.len() {
                    let item = &player.inventory[i];
                    self.render_item(ui, &item);
                    if (i + 1) % 10 == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn render_buffs_tab(&mut self, ui: &mut Ui) {
        let player = self.player.write();

        egui::Grid::new("player_buffs")
            .num_columns(10)
            .show(ui, |ui| {
                for i in 0..player.buffs.len() {
                    let buff = &player.buffs[i];
                    self.render_buff(ui, &buff);
                    if (i + 1) % 11 == 0 {
                        ui.end_row();
                    }
                }
            });
    }
}

impl TabViewer for App {
    type Tab = Tabs;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.closed_tabs.insert(*tab, NodeIndex::root());
        true
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            Tabs::LoadSave => self.render_load_save_tab(ui),
            Tabs::Stats => self.render_stats_tab(ui),
            Tabs::Bonuses => self.render_bonuses_tab(ui),
            Tabs::Inventory => self.render_inventory_tab(ui),
            Tabs::Buffs => self.render_buffs_tab(ui),
        }
    }
}
