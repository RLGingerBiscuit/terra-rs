use std::fmt::Display;

use egui::{ComboBox, Grid, Ui, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};

use terra_core::{
    utils, Difficulty, Item, PrefixMeta, ARMOR_COUNT, BANK_STRIDE, BUFF_STRIDE, INVENTORY_STRIDE,
    LOADOUT_COUNT,
};

use crate::{enum_selectable_value, meta_or_default, ui::UiExt};

use super::{
    inventory::{BuffSlot, ItemSlot, ItemTab},
    App, Message,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl Tabs {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Tabs::LoadSave,
            Tabs::Stats,
            Tabs::Bonuses,
            Tabs::Selected,
            Tabs::Inventory,
            Tabs::Bank,
            Tabs::Safe,
            Tabs::Forge,
            Tabs::Void,
            Tabs::Buffs,
            Tabs::Equipment,
            Tabs::Research,
        ]
        .into_iter()
    }
}

pub fn default_ui() -> Tree<Tabs> {
    let mut tree = Tree::new(vec![Tabs::LoadSave]);
    let [load_save, _inventory] = tree.split_below(
        0.into(),
        0.415,
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

#[derive(Debug)]
struct ItemTabOptions {
    id: &'static str,
    tab: ItemTab,
    columns: usize,
    stride: usize,
}

impl ItemTabOptions {
    fn new(id: &'static str, tab: ItemTab, columns: usize, stride: usize) -> Self {
        Self {
            id,
            tab,
            columns,
            stride,
        }
    }
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

    fn render_item_tab<F>(
        &self,
        ui: &mut Ui,
        opts: ItemTabOptions,
        prefix_meta: &[PrefixMeta],
        items: &[Item],
        extra_cols: F,
    ) where
        F: Fn(&mut Ui, usize),
    {
        egui::Grid::new(opts.id)
            .num_columns(opts.columns)
            .show(ui, |ui| {
                for i in 0..opts.stride {
                    let slots = items
                        .iter()
                        .enumerate()
                        .skip(i * opts.stride)
                        .take(opts.stride)
                        .map(|(index, item)| {
                            (
                                ItemSlot::new(
                                    item.id,
                                    Some(meta_or_default!(prefix_meta, item.prefix.id)),
                                    item.favourited,
                                    Some(item.stack),
                                ),
                                opts.tab,
                                index,
                            )
                        });

                    self.render_item_slots(ui, true, slots);

                    extra_cols(ui, i);

                    ui.end_row();
                }
            });
    }

    fn render_inventory_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

        const EXTRA_STRIDE: usize = 2;

        self.render_item_tab(
            ui,
            ItemTabOptions::new(
                "player_inventory",
                ItemTab::Inventory,
                INVENTORY_STRIDE + EXTRA_STRIDE,
                INVENTORY_STRIDE,
            ),
            &prefix_meta,
            &player.inventory,
            |ui, row| {
                if row < 4 {
                    let coins_item = &player.coins[row];
                    let ammo_item = &player.ammo[row];

                    let slots = [
                        (
                            ItemSlot::new(
                                coins_item.id,
                                Some(meta_or_default!(prefix_meta, coins_item.prefix.id)),
                                coins_item.favourited,
                                Some(coins_item.stack),
                            ),
                            ItemTab::Coins,
                            row,
                        ),
                        (
                            ItemSlot::new(
                                ammo_item.id,
                                Some(meta_or_default!(prefix_meta, ammo_item.prefix.id)),
                                ammo_item.favourited,
                                Some(ammo_item.stack),
                            ),
                            ItemTab::Ammo,
                            row,
                        ),
                    ]
                    .into_iter();

                    self.render_item_slots(ui, true, slots);
                }
            },
        );
    }

    fn render_bank_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

        self.render_item_tab(
            ui,
            ItemTabOptions::new("player_bank", ItemTab::Bank, BANK_STRIDE, BANK_STRIDE),
            &prefix_meta,
            &player.piggy_bank,
            |_, _| {},
        );
    }

    fn render_safe_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

        self.render_item_tab(
            ui,
            ItemTabOptions::new("player_safe", ItemTab::Safe, BANK_STRIDE, BANK_STRIDE),
            &prefix_meta,
            &player.safe,
            |_, _| {},
        );
    }

    fn render_forge_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

        self.render_item_tab(
            ui,
            ItemTabOptions::new("player_forge", ItemTab::Forge, BANK_STRIDE, BANK_STRIDE),
            &prefix_meta,
            &player.defenders_forge,
            |_, _| {},
        );
    }

    fn render_void_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

        self.render_item_tab(
            ui,
            ItemTabOptions::new("player_void", ItemTab::Void, BANK_STRIDE, BANK_STRIDE),
            &prefix_meta,
            &player.void_vault,
            |_, _| {},
        );
    }

    fn render_buffs_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();

        Grid::new("player_buffs").num_columns(10).show(ui, |ui| {
            for i in 0..BUFF_STRIDE {
                let slots = player
                    .buffs
                    .iter()
                    .enumerate()
                    .skip(i * BUFF_STRIDE)
                    .take(BUFF_STRIDE)
                    .map(|(index, buff)| (BuffSlot::new(buff.id, Some(buff.time)), index));

                self.render_buff_slots(ui, true, slots);

                ui.end_row();
            }
        });
    }

    fn render_equipment_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

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
                    let equipment_dye = &player.equipment_dyes[i];
                    let equipment_item = &player.equipment[i];
                    let accessory_dye = &current_loadout.accessory_dyes[i];
                    let vanity_accessory_item = &current_loadout.vanity_accessories[i];
                    let accessory_item = &current_loadout.accessories[i];

                    let slots = [
                        (
                            ItemSlot::new(
                                equipment_dye.id,
                                Some(meta_or_default!(prefix_meta, equipment_dye.id)),
                                equipment_dye.favourited,
                                None,
                            ),
                            ItemTab::EquipmentDyes,
                            i,
                        ),
                        (
                            ItemSlot::new(
                                equipment_item.id,
                                Some(meta_or_default!(prefix_meta, equipment_item.id)),
                                equipment_item.favourited,
                                None,
                            ),
                            ItemTab::Equipment,
                            i,
                        ),
                        (
                            ItemSlot::new(
                                accessory_dye.id,
                                Some(meta_or_default!(prefix_meta, accessory_dye.id)),
                                accessory_dye.favourited,
                                None,
                            ),
                            ItemTab::AccessoryDyes,
                            i,
                        ),
                        (
                            ItemSlot::new(
                                vanity_accessory_item.id,
                                Some(meta_or_default!(prefix_meta, vanity_accessory_item.id)),
                                vanity_accessory_item.favourited,
                                None,
                            ),
                            ItemTab::VanityAccessories,
                            i,
                        ),
                        (
                            ItemSlot::new(
                                accessory_item.id,
                                Some(meta_or_default!(prefix_meta, accessory_item.id)),
                                accessory_item.favourited,
                                None,
                            ),
                            ItemTab::Accessories,
                            i,
                        ),
                    ]
                    .into_iter();

                    self.render_item_slots(ui, true, slots);

                    if i < ARMOR_COUNT {
                        let armor_dye = &current_loadout.armor_dyes[i];
                        let vanity_armor = &current_loadout.vanity_armor[i];
                        let armor = &current_loadout.armor[i];

                        let slots = [
                            (
                                ItemSlot::new(
                                    armor_dye.id,
                                    Some(meta_or_default!(prefix_meta, armor_dye.id)),
                                    armor_dye.favourited,
                                    None,
                                ),
                                ItemTab::ArmorDyes,
                                i,
                            ),
                            (
                                ItemSlot::new(
                                    vanity_armor.id,
                                    Some(meta_or_default!(prefix_meta, vanity_armor.id)),
                                    vanity_armor.favourited,
                                    None,
                                ),
                                ItemTab::VanityArmor,
                                i,
                            ),
                            (
                                ItemSlot::new(
                                    armor.id,
                                    Some(meta_or_default!(prefix_meta, armor.id)),
                                    armor.favourited,
                                    None,
                                ),
                                ItemTab::Armor,
                                i,
                            ),
                        ]
                        .into_iter();

                        self.render_item_slots(ui, true, slots);
                    } else {
                        let accessory_dye = &current_loadout.accessory_dyes[ARMOR_COUNT - 1 + i];
                        let vanity_accessory =
                            &current_loadout.vanity_accessories[ARMOR_COUNT - 1 + i];
                        let accessory = &current_loadout.accessories[ARMOR_COUNT - 1 + i];

                        let slots = [
                            (
                                ItemSlot::new(
                                    accessory_dye.id,
                                    Some(meta_or_default!(prefix_meta, accessory_dye.id)),
                                    accessory_dye.favourited,
                                    None,
                                ),
                                ItemTab::AccessoryDyes,
                                ARMOR_COUNT - 1 + i,
                            ),
                            (
                                ItemSlot::new(
                                    vanity_accessory.id,
                                    Some(meta_or_default!(prefix_meta, vanity_accessory.id)),
                                    vanity_accessory.favourited,
                                    None,
                                ),
                                ItemTab::VanityAccessories,
                                ARMOR_COUNT - 1 + i,
                            ),
                            (
                                ItemSlot::new(
                                    accessory.id,
                                    Some(meta_or_default!(prefix_meta, accessory.id)),
                                    accessory.favourited,
                                    None,
                                ),
                                ItemTab::Accessories,
                                ARMOR_COUNT - 1 + i,
                            ),
                        ]
                        .into_iter();

                        self.render_item_slots(ui, true, slots);
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
