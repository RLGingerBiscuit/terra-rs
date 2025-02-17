use std::fmt::Display;

use egui::{Align2, ComboBox, TextStyle, Ui, WidgetText};
use egui_dock::{DockState, TabViewer};

use terra_core::{
    meta::Meta, utils, Difficulty, Item, PrefixMeta, ARMOR_COUNT, BANK_STRIDE, BUFF_STRIDE,
    HAIR_DYE_COUNT, HAIR_STYLE_COUNT, INVENTORY_STRIDE, LOADOUT_COUNT, SKIN_VARIANT_COUNT,
};

use super::{
    context::AppContext,
    inventory::{
        buff_slot::{self, BuffSlotOptions},
        item_slot::ItemSlotOptions,
        ItemGroup,
    },
    Message,
};
use crate::{
    app::{
        inventory::{
            item_slot::{self, ItemSlotIcon},
            slot::SlotText,
        },
        AppMessage,
    },
    enum_selectable_value,
    ui::UiExt,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Tab {
    Stats,
    LoadSave,
    Appearance,
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

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tab::LoadSave => "Load & Save",
                Tab::Stats => "Stats",
                Tab::Appearance => "Appearance",
                Tab::Bonuses => "Permanent Bonuses",
                Tab::Selected => "Selected",
                Tab::Inventory => "Inventory",
                Tab::Bank => "Piggy Bank",
                Tab::Safe => "Safe",
                Tab::Forge => "Defender's Forge",
                Tab::Void => "Void Vault",
                Tab::Buffs => "Buffs",
                Tab::Equipment => "Equipment",
                Tab::Research => "Research",
            }
        )
    }
}

impl Tab {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Tab::LoadSave,
            Tab::Stats,
            Tab::Appearance,
            Tab::Bonuses,
            Tab::Selected,
            Tab::Inventory,
            Tab::Bank,
            Tab::Safe,
            Tab::Forge,
            Tab::Void,
            Tab::Buffs,
            Tab::Equipment,
            Tab::Research,
        ]
        .into_iter()
    }
}

pub fn default_ui() -> DockState<Tab> {
    let mut state = DockState::new(vec![Tab::LoadSave]);
    let main_surface = state.main_surface_mut();
    let [load_save, _] = main_surface.split_below(
        0.into(),
        0.425,
        vec![
            Tab::Inventory,
            Tab::Bank,
            Tab::Safe,
            Tab::Forge,
            Tab::Void,
            Tab::Buffs,
            Tab::Equipment,
        ],
    );
    let [load_save, stats] = main_surface.split_right(
        load_save,
        0.22,
        vec![Tab::Stats, Tab::Appearance, Tab::Bonuses],
    );
    let [_stats, _selected] =
        main_surface.split_right(stats, 0.6, vec![Tab::Selected, Tab::Research]);

    main_surface.set_focused_node(load_save);
    state
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

impl AppContext {
    fn render_load_save_tab(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.spacing_mut().item_spacing = [8.; 2].into();
            if ui.button("Load Player").clicked() {
                self.send_context_msg(Message::LoadPlayer);
            }
            if ui.button("Save Player").clicked() {
                self.send_context_msg(Message::SavePlayer);
            }
            if ui.button("Reset Player").clicked() {
                self.send_context_msg(Message::ResetPlayer);
            }
        });
    }

    fn render_appearance_tab(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();

        egui::Grid::new("appearanec").num_columns(6).show(ui, |ui| {
            ui.label("Hair style:");
            ui.drag_value_with_buttons(&mut player.hair_style, 1., 0..=HAIR_STYLE_COUNT);
            ui.end_row();

            ui.label("Hair dye:");
            ui.drag_value_with_buttons(&mut player.hair_dye, 1., 0..=HAIR_DYE_COUNT);
            ui.end_row();

            ui.label("Skin variant:");
            ui.drag_value_with_buttons(&mut player.skin_variant, 1., 0..=SKIN_VARIANT_COUNT);
            ui.end_row();

            ui.label("Hair color");
            ui.color_edit_button_srgb(&mut player.hair_color);

            ui.label("Skin color");
            ui.color_edit_button_srgb(&mut player.skin_color);
            ui.end_row();

            ui.label("Eye color");
            ui.color_edit_button_srgb(&mut player.eye_color);

            ui.label("Shirt color");
            ui.color_edit_button_srgb(&mut player.shirt_color);
            ui.end_row();

            ui.label("Undershirt color");
            ui.color_edit_button_srgb(&mut player.undershirt_color);

            ui.label("Pants color");
            ui.color_edit_button_srgb(&mut player.pants_color);
            ui.end_row();

            ui.label("Shoe color");
            ui.color_edit_button_srgb(&mut player.shoe_color);
            ui.end_row();
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
                    let font_id = TextStyle::Body.resolve(ui.style());
                    let text_color = ui.visuals().text_color();

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
                                ItemSlotOptions::from_item(
                                    accessory_dye,
                                    ItemGroup::AccessoryDyes(self.selected_loadout),
                                )
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
                                    ItemGroup::VanityAccessories(self.selected_loadout),
                                )
                                .icon(Some(ItemSlotIcon::VanityAccessory))
                                .prefix_meta(PrefixMeta::get(
                                    &prefix_meta,
                                    vanity_accessory.prefix.id,
                                )),
                            ),
                            (
                                i,
                                ItemSlotOptions::from_item(
                                    accessory,
                                    ItemGroup::Accessories(self.selected_loadout),
                                )
                                .icon(Some(ItemSlotIcon::Accessory))
                                .prefix_meta(PrefixMeta::get(&prefix_meta, accessory.prefix.id)),
                            ),
                        ]
                        .into_iter()
                        .map(|(i, o)| (i, o.tooltip_on_hover(true)));

                        self.render_item_slots(ui, options);

                        if i < ARMOR_COUNT {
                            let armor_dye = &current_loadout.armor_dyes[i];
                            let vanity_armor = &current_loadout.vanity_armor[i];
                            let armor = &current_loadout.armor[i];

                            let options = [
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        armor_dye,
                                        ItemGroup::ArmorDyes(self.selected_loadout),
                                    )
                                    .icon(Some(ItemSlotIcon::Dye))
                                    .prefix_meta(
                                        PrefixMeta::get(&prefix_meta, armor_dye.prefix.id),
                                    ),
                                ),
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        vanity_armor,
                                        ItemGroup::VanityArmor(self.selected_loadout),
                                    )
                                    .icon(Some(VANITY_ARMOR_ICONS[i]))
                                    .prefix_meta(
                                        PrefixMeta::get(&prefix_meta, vanity_armor.prefix.id),
                                    ),
                                ),
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        armor,
                                        ItemGroup::Armor(self.selected_loadout),
                                    )
                                    .icon(Some(ARMOR_ICONS[i]))
                                    .prefix_meta(PrefixMeta::get(&prefix_meta, armor.prefix.id)),
                                ),
                            ]
                            .into_iter()
                            .map(|(i, o)| (i, o.tooltip_on_hover(true)));

                            self.render_item_slots(ui, options);
                        } else {
                            let i = ARMOR_COUNT - 1 + i;

                            let accessory_dye = &current_loadout.accessory_dyes[i];
                            let vanity_accessory = &current_loadout.vanity_accessories[i];
                            let accessory = &current_loadout.accessories[i];

                            let options = [
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        accessory_dye,
                                        ItemGroup::AccessoryDyes(self.selected_loadout),
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
                                        ItemGroup::VanityAccessories(self.selected_loadout),
                                    )
                                    .icon(Some(ItemSlotIcon::VanityAccessory))
                                    .prefix_meta(
                                        PrefixMeta::get(&prefix_meta, vanity_accessory.prefix.id),
                                    ),
                                ),
                                (
                                    i,
                                    ItemSlotOptions::from_item(
                                        accessory,
                                        ItemGroup::Accessories(self.selected_loadout),
                                    )
                                    .icon(Some(ItemSlotIcon::Accessory))
                                    .prefix_meta(
                                        PrefixMeta::get(&prefix_meta, accessory.prefix.id),
                                    ),
                                ),
                            ]
                            .into_iter()
                            .map(|(i, o)| (i, o.tooltip_on_hover(true)))
                            .map(|(i, o)| {
                                (
                                    i,
                                    o.add_text(SlotText::new(
                                        Align2::LEFT_TOP,
                                        match i {
                                            5 => "E".to_string(), // Export
                                            6 => "M".to_string(), // Master
                                            _ => panic!("What?"),
                                        },
                                        font_id.clone(),
                                        text_color,
                                    )),
                                )
                            });

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
                                self.send_context_msg(Message::SelectLoadout(loadout));
                            }
                        }

                        ui.end_row();
                    }
                });
        });
    }

    fn render_research_tab(&mut self, ui: &mut Ui) {
        let player = self.player.read();
        let entry_text = if player.research.len() == 1 {
            "item"
        } else {
            "items"
        };

        ui.label(format!(
            "{} researched {}",
            player.research.len(),
            entry_text
        ));

        ui.horizontal(|ui| {
            if ui.button("Clear all").clicked() {
                self.send_context_msg(Message::RemoveAllResearch);
            }
            if ui.button("Unlock all").clicked() {
                self.send_context_msg(Message::AddAllResearch);
            }
            if ui.button("\u{1f50e}").clicked() {
                self.send_context_msg(Message::OpenResearchBrowser);
            }
        });
    }
}

impl TabViewer for AppContext {
    type Tab = Tab;

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        if self.is_modal_open() {
            ui.disable();
        }

        match tab {
            Tab::LoadSave => self.render_load_save_tab(ui),
            Tab::Appearance => self.render_appearance_tab(ui),
            Tab::Stats => self.render_stats_tab(ui),
            Tab::Bonuses => self.render_bonuses_tab(ui),
            Tab::Selected => self.render_selected_tab(ui),
            Tab::Inventory => self.render_inventory_tab(ui),
            Tab::Bank => self.render_bank_tab(ui),
            Tab::Safe => self.render_safe_tab(ui),
            Tab::Forge => self.render_forge_tab(ui),
            Tab::Void => self.render_void_tab(ui),
            Tab::Buffs => self.render_buffs_tab(ui),
            Tab::Equipment => self.render_equipment_tab(ui),
            Tab::Research => self.render_research_tab(ui),
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        !matches!(*tab, Tab::LoadSave)
    }

    fn add_popup(
        &mut self,
        ui: &mut Ui,
        surface: egui_dock::SurfaceIndex,
        node: egui_dock::NodeIndex,
    ) {
        ui.set_min_width(120.);
        ui.style_mut().visuals.button_frame = false;

        for tab in Tab::iter() {
            if ui.button(tab.to_string()).clicked() {
                self.send_app_msg(AppMessage::AddTab(tab, surface, node));
            }
        }
    }
}
