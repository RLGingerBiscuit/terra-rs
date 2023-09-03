use std::fmt::Display;

use egui::{ComboBox, Ui, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};

use terra_core::{
    meta::Meta, utils, Difficulty, Item, PrefixMeta, ARMOR_COUNT, BANK_STRIDE, BUFF_STRIDE,
    INVENTORY_STRIDE, LOADOUT_COUNT,
};

use crate::{
    app::inventory::item_slot::{self, ItemSlotIcon},
    enum_selectable_value,
    ui::UiExt,
};

use super::{
    inventory::{
        buff_slot::{self, BuffSlotOptions},
        item_slot::ItemSlotOptions,
        ItemGroup,
    },
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
        0.4,
        vec![
            Tabs::Inventory,
            Tabs::Bank,
            Tabs::Safe,
            Tabs::Forge,
            Tabs::Void,
            Tabs::Buffs,
            Tabs::Equipment,
        ],
    );
    let [load_save, stats] = tree.split_right(load_save, 0.15, vec![Tabs::Stats, Tabs::Bonuses]);
    let [_stats, _selected] = tree.split_right(stats, 0.6, vec![Tabs::Selected, Tabs::Research]);

    tree.set_focused_node(load_save);
    tree
}

#[derive(Debug)]
struct ItemTabOptions {
    id: &'static str,
    group: ItemGroup,
    columns: usize,
    rows: usize,
    stride: usize,
}

impl ItemTabOptions {
    fn new(id: &'static str, group: ItemGroup, columns: usize, rows: usize, stride: usize) -> Self {
        Self {
            id,
            group,
            columns,
            rows,
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

        egui::Grid::new("stats").num_columns(3).show(ui, |ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut player.name);
            ui.end_row();

            ui.label("Difficulty:");
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
            ui.end_row();

            ui.label("Version:");
            ui.horizontal(|ui| {
                ui.drag_value_with_buttons(&mut player.version, 1., 0..=i32::MAX);
                ui.small(utils::version_lookup(player.version));
            });
            ui.end_row();

            ui.label("Health:");
            ui.horizontal(|ui| {
                ui.drag_value_with_buttons(&mut player.life, 1., 0..=i32::MAX);
                ui.label("/");
                ui.drag_value_with_buttons(&mut player.max_life, 1., 0..=i32::MAX);
            });
            ui.end_row();

            ui.label("Mana:");
            ui.horizontal(|ui| {
                ui.drag_value_with_buttons(&mut player.mana, 1., 0..=i32::MAX);
                ui.label("/");
                ui.drag_value_with_buttons(&mut player.max_mana, 1., 0..=i32::MAX);
            });
            ui.end_row();

            ui.label("Fishing quests:");
            ui.drag_value_with_buttons(&mut player.angler_quests, 1., 0..=i32::MAX);
            ui.end_row();

            ui.label("Golfer score:");
            ui.drag_value_with_buttons(&mut player.golfer_score, 1., 0..=i32::MAX);
            ui.end_row();
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
        options: ItemTabOptions,
        prefix_meta: &[PrefixMeta],
        items: &[Item],
        extra_cols: F,
    ) where
        F: Fn(&mut Ui, usize),
    {
        let spacing = (ui.available_height()
            - item_slot::SLOT_SIZE.y * (options.rows + 1) as f32
            - ui.spacing().item_spacing.y * ((options.rows - 1) * 2 - 1) as f32)
            / 2.;
        if spacing > 0. {
            ui.add_space(spacing);
        }

        ui.horizontal_top(|ui| {
            let spacing = (ui.available_width()
                - item_slot::SLOT_SIZE.x * (options.columns + 1) as f32
                - ui.spacing().item_spacing.x * (options.columns * 2) as f32)
                / 2.;
            if spacing > 0. {
                ui.add_space(spacing);
            }

            egui::Grid::new(options.id)
                .num_columns(options.columns)
                .show(ui, |ui| {
                    for i in 0..options.stride {
                        let options = items
                            .iter()
                            .enumerate()
                            .skip(i * options.stride)
                            .take(options.stride)
                            .map(|(index, item)| {
                                (
                                    index,
                                    ItemSlotOptions::from_item(item, options.group)
                                        .prefix_meta(PrefixMeta::get(prefix_meta, item.prefix.id))
                                        .tooltip_on_hover(true),
                                )
                            });

                        self.render_item_slots(ui, options);

                        extra_cols(ui, i);

                        ui.end_row();
                    }
                });
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
                ItemGroup::Inventory,
                INVENTORY_STRIDE + EXTRA_STRIDE,
                5,
                INVENTORY_STRIDE,
            ),
            &prefix_meta,
            &player.inventory,
            |ui, row| {
                if row < 4 {
                    let coins = &player.coins[row];
                    let ammo = &player.ammo[row];

                    let options = [
                        (
                            row,
                            ItemSlotOptions::from_item(coins, ItemGroup::Coins)
                                .icon(Some(ItemSlotIcon::Coins))
                                .prefix_meta(PrefixMeta::get(&prefix_meta, coins.prefix.id)),
                        ),
                        (
                            row,
                            ItemSlotOptions::from_item(ammo, ItemGroup::Ammo)
                                .prefix_meta(PrefixMeta::get(&prefix_meta, ammo.prefix.id)),
                        ),
                    ]
                    .into_iter()
                    .map(|(i, o)| (i, o.tooltip_on_hover(true)));

                    self.render_item_slots(ui, options);
                }
            },
        );
    }

    fn render_bank_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

        self.render_item_tab(
            ui,
            ItemTabOptions::new("player_bank", ItemGroup::Bank, BANK_STRIDE, 4, BANK_STRIDE),
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
            ItemTabOptions::new("player_safe", ItemGroup::Safe, BANK_STRIDE, 4, BANK_STRIDE),
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
            ItemTabOptions::new(
                "player_forge",
                ItemGroup::Forge,
                BANK_STRIDE,
                4,
                BANK_STRIDE,
            ),
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
            ItemTabOptions::new("player_void", ItemGroup::Void, BANK_STRIDE, 4, BANK_STRIDE),
            &prefix_meta,
            &player.void_vault,
            |_, _| {},
        );
    }

    fn render_buffs_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();

        let spacing = (ui.available_height()
            - buff_slot::SLOT_SIZE.y * 4.
            - ui.spacing().item_spacing.y * 5.)
            / 2.;
        if spacing > 0. {
            ui.add_space(spacing);
        }

        ui.horizontal_top(|ui| {
            let spacing = (ui.available_width()
                - buff_slot::SLOT_SIZE.x * 11.
                - ui.spacing().item_spacing.x * 22.)
                / 2.;
            if spacing > 0. {
                ui.add_space(spacing);
            }

            egui::Grid::new("player_buffs")
                .num_columns(BUFF_STRIDE)
                .show(ui, |ui| {
                    for i in 0..BUFF_STRIDE {
                        let options = player
                            .buffs
                            .iter()
                            .enumerate()
                            .skip(i * BUFF_STRIDE)
                            .take(BUFF_STRIDE)
                            .map(|(index, buff)| {
                                (
                                    index,
                                    BuffSlotOptions::from_buff(buff).tooltip_on_hover(true),
                                )
                            });

                        self.render_buff_slots(ui, options);

                        ui.end_row();
                    }
                });
        });
    }

    fn render_equipment_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let prefix_meta = self.prefix_meta.read();

        const EQUIPMENT_ICONS: [ItemSlotIcon; 5] = [
            ItemSlotIcon::Pet,
            ItemSlotIcon::LightPet,
            ItemSlotIcon::Cart,
            ItemSlotIcon::Mount,
            ItemSlotIcon::Hook,
        ];

        const ARMOR_ICONS: [ItemSlotIcon; 3] = [
            ItemSlotIcon::HeadPiece,
            ItemSlotIcon::ArmorPiece,
            ItemSlotIcon::LegsPiece,
        ];

        const VANITY_ARMOR_ICONS: [ItemSlotIcon; 3] = [
            ItemSlotIcon::VanityHeadPiece,
            ItemSlotIcon::VanityArmorPiece,
            ItemSlotIcon::VanityLegsPiece,
        ];

        let spacing = (ui.available_height()
            - item_slot::SLOT_SIZE.y * 6.
            - ui.spacing().item_spacing.y * 7.)
            / 2.;
        if spacing > 0. {
            ui.add_space(spacing);
        }

        ui.horizontal_top(|ui| {
            let spacing = (ui.available_width()
                - item_slot::SLOT_SIZE.x * 9.
                - ui.spacing().item_spacing.x * 16.)
                / 2.;
            if spacing > 0. {
                ui.add_space(spacing);
            }

            egui::Grid::new("player_equipment")
                .num_columns(9)
                .show(ui, |ui| {
                    let current_loadout = &player.loadouts[self.selected_loadout.0];

                    for i in 0..5 {
                        let equipment_dye = &player.equipment_dyes[i];
                        let equipment = &player.equipment[i];
                        let accessory_dye = &current_loadout.accessory_dyes[i];
                        let vanity_accessory = &current_loadout.vanity_accessories[i];
                        let accessory = &current_loadout.accessories[i];

                        let options = [
                            (
                                i,
                                ItemSlotOptions::from_item(equipment_dye, ItemGroup::EquipmentDyes)
                                    .icon(Some(ItemSlotIcon::Dye))
                                    .prefix_meta(PrefixMeta::get(
                                        &prefix_meta,
                                        equipment_dye.prefix.id,
                                    )),
                            ),
                            (
                                i,
                                ItemSlotOptions::from_item(equipment, ItemGroup::Equipment)
                                    .icon(Some(EQUIPMENT_ICONS[i]))
                                    .prefix_meta(PrefixMeta::get(
                                        &prefix_meta,
                                        equipment.prefix.id,
                                    )),
                            ),
                            (
                                i,
                                ItemSlotOptions::from_item(accessory_dye, ItemGroup::AccessoryDyes)
                                    .icon(Some(ItemSlotIcon::Dye))
                                    .prefix_meta(PrefixMeta::get(
                                        &prefix_meta,
                                        accessory_dye.prefix.id,
                                    )),
                            ),
                            (
                                i,
                                ItemSlotOptions::from_item(
                                    vanity_accessory,
                                    ItemGroup::VanityAccessories,
                                )
                                .icon(Some(ItemSlotIcon::VanityAccessory))
                                .prefix_meta(PrefixMeta::get(
                                    &prefix_meta,
                                    vanity_accessory.prefix.id,
                                )),
                            ),
                            (
                                i,
                                ItemSlotOptions::from_item(accessory, ItemGroup::Accessories)
                                    .icon(Some(ItemSlotIcon::Accessory))
                                    .prefix_meta(PrefixMeta::get(
                                        &prefix_meta,
                                        accessory.prefix.id,
                                    )),
                            ),
                        ]
                        .into_iter()
                        .map(|(i, o)| (i, o.tooltip_on_hover(true)));

                        self.render_item_slots(ui, options);

                        if i < ARMOR_COUNT {
                            let armor_dye = &current_loadout.armor_dyes[i];
                            let vanity_armor = &current_loadout.vanity_armor[i];
                            let armor = &current_loadout.armor[i];

                            let slots = [
                                (
                                    i,
                                    ItemSlotOptions::from_item(armor_dye, ItemGroup::ArmorDyes)
                                        .icon(Some(ItemSlotIcon::Dye))
                                        .prefix_meta(PrefixMeta::get(
                                            &prefix_meta,
                                            armor_dye.prefix.id,
                                        )),
                                ),
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        vanity_armor,
                                        ItemGroup::VanityArmor,
                                    )
                                    .icon(Some(VANITY_ARMOR_ICONS[i]))
                                    .prefix_meta(
                                        PrefixMeta::get(&prefix_meta, vanity_armor.prefix.id),
                                    ),
                                ),
                                (
                                    i,
                                    ItemSlotOptions::from_item(armor, ItemGroup::Armor)
                                        .icon(Some(ARMOR_ICONS[i]))
                                        .prefix_meta(PrefixMeta::get(
                                            &prefix_meta,
                                            armor.prefix.id,
                                        )),
                                ),
                            ]
                            .into_iter()
                            .map(|(i, o)| (i, o.tooltip_on_hover(true)));

                            self.render_item_slots(ui, slots);
                        } else {
                            let accessory_dye =
                                &current_loadout.accessory_dyes[ARMOR_COUNT - 1 + i];
                            let vanity_accessory =
                                &current_loadout.vanity_accessories[ARMOR_COUNT - 1 + i];
                            let accessory = &current_loadout.accessories[ARMOR_COUNT - 1 + i];

                            let i = ARMOR_COUNT - 1 + i;

                            let options = [
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        accessory_dye,
                                        ItemGroup::AccessoryDyes,
                                    )
                                    .icon(Some(ItemSlotIcon::Dye))
                                    .prefix_meta(
                                        PrefixMeta::get(&prefix_meta, accessory_dye.prefix.id),
                                    ),
                                ),
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        vanity_accessory,
                                        ItemGroup::VanityAccessories,
                                    )
                                    .icon(Some(ItemSlotIcon::VanityAccessory))
                                    .prefix_meta(
                                        PrefixMeta::get(&prefix_meta, vanity_accessory.prefix.id),
                                    ),
                                ),
                                (
                                    i,
                                    ItemSlotOptions::from_item(accessory, ItemGroup::Accessories)
                                        .icon(Some(ItemSlotIcon::Accessory))
                                        .prefix_meta(PrefixMeta::get(
                                            &prefix_meta,
                                            accessory.prefix.id,
                                        )),
                                ),
                            ]
                            .into_iter()
                            .map(|(i, o)| (i, o.tooltip_on_hover(true)));

                            self.render_item_slots(ui, options);
                        }

                        if i == 0 {
                            let mut loadout = self.selected_loadout;

                            if ComboBox::from_id_source("player_loadouts")
                                .show_index(ui, &mut loadout.0, LOADOUT_COUNT, |i| {
                                    format!("Loadout {}", i + 1)
                                })
                                .changed()
                            {
                                self.do_update(Message::SelectLoadout(loadout));
                            }
                        }

                        ui.end_row();
                    }
                });
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
        ui.set_enabled(!self.is_modal_open());

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
