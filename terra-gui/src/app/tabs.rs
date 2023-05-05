use std::fmt::Display;

use egui::{ComboBox, Ui, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};
use terra_core::{
    utils, Difficulty, Item, ARMOR_COUNT, BANK_STRIDE, BUFF_STRIDE, INVENTORY_COUNT,
    INVENTORY_STRIDE, LOADOUT_COUNT,
};

use crate::{enum_selectable_value, ui::UiExt};

use super::{inventory::ItemTab, App, Message};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tabs {
    Stats,
    LoadSave,
    Bonuses,
    Selected,
    Inventory,
    Bank,
    Safe,
    Forge,
    Void,
    Buffs,
    Equipment,
    Research,
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
                Tabs::Selected => "Selected",
                Tabs::Inventory => "Inventory",
                Tabs::Bank => "Piggy Bank",
                Tabs::Safe => "Safe",
                Tabs::Forge => "Defender's Forge",
                Tabs::Void => "Void Vault",
                Tabs::Buffs => "Buffs",
                Tabs::Equipment => "Equipment",
                Tabs::Research => "Research",
            }
        )
    }
}

pub fn default_ui() -> Tree<Tabs> {
    let mut tree = Tree::new(vec![Tabs::LoadSave]);
    let [load_save, _inventory] = tree.split_below(
        0.into(),
        0.315,
        vec![
            Tabs::Inventory,
            Tabs::Bank,
            Tabs::Safe,
            Tabs::Forge,
            Tabs::Void,
            Tabs::Buffs,
            Tabs::Equipment,
            Tabs::Research,
        ],
    );
    let [load_save, stats] = tree.split_right(load_save, 0.15, vec![Tabs::Stats, Tabs::Bonuses]);
    let [_stats, _selected] = tree.split_right(stats, 0.6, vec![Tabs::Selected]);

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
            ComboBox::from_id_source("player_difficulty")
                .selected_text(player.difficulty.to_string())
                .show_ui(ui, |ui| {
                    enum_selectable_value!(
                        ui,
                        &mut player.difficulty,
                        Difficulty::Journey,
                        Difficulty::Classic,
                        Difficulty::Mediumcore,
                        Difficulty::Hardcore
                    );
                });
        });

        ui.labelled("Version:", |ui| {
            ui.drag_value_with_buttons(&mut player.version, 1., 0..=i32::MAX);
            ui.small(utils::version_lookup(player.version));
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

        // Add some space so bottom matches up with selected tab
        const SEPARATOR_SPACING: f32 = 6.;
        ui.add_space(SEPARATOR_SPACING);

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

    fn render_selected_tab(&mut self, ui: &mut Ui) {
        self.render_selected_item(ui);
        ui.separator();
        self.render_selected_buff(ui);
    }

    fn render_item_tab(&self, ui: &mut Ui, id: &str, tab: ItemTab, items: &[Item], stride: usize) {
        egui::Grid::new(id).num_columns(stride).show(ui, |ui| {
            for i in 0..stride {
                let slice = items
                    .iter()
                    .enumerate()
                    .skip(i * stride)
                    .take(stride)
                    .map(|(i, item)| (i, item, tab))
                    .collect::<Vec<_>>();

                self.render_item_multiple(ui, true, &slice);

                ui.end_row();
            }
        });
    }

    fn render_inventory_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        self.render_item_tab(
            ui,
            "player_inventory",
            ItemTab::Inventory,
            &player.inventory,
            INVENTORY_STRIDE,
        );
    }

    fn render_bank_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        self.render_item_tab(
            ui,
            "player_bank",
            ItemTab::Bank,
            &player.piggy_bank,
            BANK_STRIDE,
        );
    }

    fn render_safe_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        self.render_item_tab(ui, "player_safe", ItemTab::Safe, &player.safe, BANK_STRIDE);
    }

    fn render_forge_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        self.render_item_tab(
            ui,
            "player_forge",
            ItemTab::Forge,
            &player.defenders_forge,
            BANK_STRIDE,
        );
    }

    fn render_void_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        self.render_item_tab(
            ui,
            "player_void",
            ItemTab::Void,
            &player.void_vault,
            BANK_STRIDE,
        );
    }

    fn render_buffs_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();

        egui::Grid::new("player_buffs")
            .num_columns(10)
            .show(ui, |ui| {
                for i in 0..BUFF_STRIDE {
                    let slice = player
                        .buffs
                        .iter()
                        .enumerate()
                        .skip(i * BUFF_STRIDE)
                        .take(BUFF_STRIDE)
                        .collect::<Vec<_>>();

                    self.render_buff_multiple(ui, &slice);

                    ui.end_row();
                }
            });
    }

    fn render_equipment_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();

        ComboBox::from_id_source("player_loadouts").show_index(
            ui,
            // TODO: This works but I'd like to change this into a message
            &mut self.selected_loadout.0,
            LOADOUT_COUNT,
            |i| format!("Loadout {}", i + 1),
        );

        egui::Grid::new("player_equipment")
            .num_columns(8)
            .show(ui, |ui| {
                let current_loadout = &player.loadouts[self.selected_loadout.0];

                for i in 0..5 {
                    self.render_item_multiple(
                        ui,
                        false,
                        &[
                            (i, &player.equipment_dyes[i], ItemTab::EquipmentDyes),
                            (i, &player.equipment[i], ItemTab::Equipment),
                            (
                                i,
                                &current_loadout.accessory_dyes[i],
                                ItemTab::AccessoryDyes,
                            ),
                            (
                                i,
                                &current_loadout.vanity_accessories[i],
                                ItemTab::VanityAccessories,
                            ),
                            (i, &current_loadout.accessories[i], ItemTab::Accessories),
                        ],
                    );

                    if i < ARMOR_COUNT {
                        self.render_item_multiple(
                            ui,
                            false,
                            &[
                                (i, &current_loadout.armor_dyes[i], ItemTab::ArmorDyes),
                                (i, &current_loadout.vanity_armor[i], ItemTab::VanityArmor),
                                (i, &current_loadout.armor[i], ItemTab::Armor),
                            ],
                        );
                    } else {
                        self.render_item_multiple(
                            ui,
                            false,
                            &[
                                (
                                    ARMOR_COUNT - 1 + i,
                                    &current_loadout.accessory_dyes[ARMOR_COUNT - 1 + i],
                                    ItemTab::AccessoryDyes,
                                ),
                                (
                                    ARMOR_COUNT - 1 + i,
                                    &current_loadout.vanity_accessories[ARMOR_COUNT - 1 + i],
                                    ItemTab::VanityAccessories,
                                ),
                                (
                                    ARMOR_COUNT - 1 + i,
                                    &current_loadout.accessories[ARMOR_COUNT - 1 + i],
                                    ItemTab::Accessories,
                                ),
                            ],
                        );
                    }

                    ui.end_row();
                }
            });
    }

    fn render_research_tab(&mut self, ui: &mut Ui) {
        let player = self.player.write();
        let entry_text = if player.research.len() == 1 {
            "entry"
        } else {
            "entries"
        };

        ui.label(format!("{} research {}", player.research.len(), entry_text));
        ui.horizontal(|ui| {
            if ui.button("Clear all").clicked() {
                self.do_update(Message::RemoveAllResearch);
            }
            if ui.button("Unlock all").clicked() {
                self.do_update(Message::AddAllResearch);
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
            Tabs::Selected => self.render_selected_tab(ui),
            Tabs::Inventory => self.render_inventory_tab(ui),
            Tabs::Bank => self.render_bank_tab(ui),
            Tabs::Safe => self.render_safe_tab(ui),
            Tabs::Forge => self.render_forge_tab(ui),
            Tabs::Void => self.render_void_tab(ui),
            Tabs::Buffs => self.render_buffs_tab(ui),
            Tabs::Equipment => self.render_equipment_tab(ui),
            Tabs::Research => self.render_research_tab(ui),
        }
    }
}
