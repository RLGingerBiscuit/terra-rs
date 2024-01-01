pub mod buff_slot;
pub mod buff_tooltip;
pub mod item_slot;
pub mod item_tooltip;
pub mod prefix_tooltip;
pub mod slot;

use egui::{Response, Ui, Vec2, Widget};
use terra_core::{meta::Meta, utils, Buff, BuffMeta, Item, ItemMeta, Player, PrefixMeta};

use self::{
    buff_slot::{BuffSlot, BuffSlotOptions},
    buff_tooltip::{BuffTooltip, BuffTooltipOptions},
    item_slot::{ItemSlot, ItemSlotOptions},
    item_tooltip::{ItemTooltip, ItemTooltipOptions},
    prefix_tooltip::{PrefixTooltip, PrefixTooltipOptions},
    slot::Slot,
};
use super::{App, Message};
use crate::ui::{ClickableFrame, UiExt};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ItemGroup {
    Inventory,
    Coins,
    Ammo,
    Bank,
    Safe,
    Forge,
    Void,
    Equipment,
    EquipmentDyes,
    Armor,
    VanityArmor,
    ArmorDyes,
    Accessories,
    VanityAccessories,
    AccessoryDyes,
    ItemBrowser,
    ResearchBrowser,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SelectedItem(pub ItemGroup, pub usize);

impl SelectedItem {
    pub fn equals(&self, group: ItemGroup, index: usize) -> bool {
        self.0 == group && self.1 == index
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SelectedBuff(pub usize);

impl SelectedBuff {
    pub fn equals(&self, index: usize) -> bool {
        self.0 == index
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SelectedLoadout(pub usize);

pub fn selected_item(
    item: SelectedItem,
    loadout: SelectedLoadout,
    player: &mut Player,
) -> &mut Item {
    match item.0 {
        ItemGroup::Inventory => &mut player.inventory[item.1],
        ItemGroup::Coins => &mut player.coins[item.1],
        ItemGroup::Ammo => &mut player.ammo[item.1],
        ItemGroup::Bank => &mut player.piggy_bank[item.1],
        ItemGroup::Safe => &mut player.safe[item.1],
        ItemGroup::Forge => &mut player.defenders_forge[item.1],
        ItemGroup::Void => &mut player.void_vault[item.1],
        ItemGroup::Equipment => &mut player.equipment[item.1],
        ItemGroup::EquipmentDyes => &mut player.equipment_dyes[item.1],
        ItemGroup::VanityArmor => &mut player.loadouts[loadout.0].vanity_armor[item.1],
        ItemGroup::Armor => &mut player.loadouts[loadout.0].armor[item.1],
        ItemGroup::ArmorDyes => &mut player.loadouts[loadout.0].armor_dyes[item.1],
        ItemGroup::VanityAccessories => &mut player.loadouts[loadout.0].vanity_accessories[item.1],
        ItemGroup::Accessories => &mut player.loadouts[loadout.0].accessories[item.1],
        ItemGroup::AccessoryDyes => &mut player.loadouts[loadout.0].accessory_dyes[item.1],
        ItemGroup::ResearchBrowser | ItemGroup::ItemBrowser => {
            panic!("You should never try to get the selected item of a browser")
        }
    }
}

pub fn selected_buff(buff: SelectedBuff, player: &mut Player) -> &mut Buff {
    &mut player.buffs[buff.0]
}

pub fn item_name(name: &str, prefix_meta: Option<&PrefixMeta>) -> String {
    if let Some(prefix_meta) = prefix_meta {
        if prefix_meta.id() != 0 {
            return format!("{} {}", prefix_meta.name(), name);
        }
    }

    name.to_owned()
}

pub fn buff_name(name: &str, time: Option<i32>) -> String {
    if let Some(time) = time {
        format!("{} ({})", name, utils::ticks_to_string(time))
    } else {
        name.to_owned()
    }
}

impl App {
    fn render_slot<S>(&self, ui: &mut Ui, slot: S) -> Response
    where
        S: Slot + Widget,
    {
        let group = if slot.selected() {
            ClickableFrame::group(ui.style()).fill(ui.visuals().code_bg_color)
        } else if slot.highlighted() {
            ClickableFrame::group(ui.style()).fill(ui.visuals().faint_bg_color)
        } else {
            ClickableFrame::group(ui.style())
        }
        .inner_margin(slot.margin());

        ui.style_mut().spacing.item_spacing = Vec2::ZERO;

        group.show(ui, |ui| ui.add(slot)).response
    }

    pub fn render_item_slot(&self, ui: &mut Ui, options: ItemSlotOptions) -> Response {
        let icon_spritesheet = self.icon_spritesheet.read();
        let item_spritesheet = self.item_spritesheet.read();
        let item_meta = self.item_meta.read();

        if icon_spritesheet.is_none() && !self.is_busy() {
            self.do_update(Message::LoadIconSpritesheet);
        }

        if item_spritesheet.is_none() && !self.is_busy() {
            self.do_update(Message::LoadItemSpritesheet);
        }

        // FIXME: Lifetime stuff means ::from_slot_options doesn't work
        // let tooltip_options = ItemTooltipOptions::from_slot_options(&options);
        let tooltip_options = ItemTooltipOptions::new()
            .id(options.id)
            .favourited(options.favourited)
            .prefix_meta(options.prefix_meta);
        let tooltip_on_hover = options.tooltip_on_hover;

        let meta = ItemMeta::get_or_default(&item_meta, options.id);
        let slot = ItemSlot::new(
            options,
            meta,
            item_spritesheet.as_ref(),
            icon_spritesheet.as_ref(),
        );
        let response = self.render_slot(ui, slot);

        if meta.id != 0 && tooltip_on_hover {
            response.on_hover_ui(|ui| self.render_item_tooltip(ui, tooltip_options))
        } else {
            response
        }
    }

    pub fn render_item_slots<'a, I>(&self, ui: &mut Ui, options: I)
    where
        I: Iterator<Item = (usize, ItemSlotOptions<'a>)>,
    {
        for (index, mut options) in options {
            let group = options.group;
            options.selected = self.selected_item.equals(group, index);

            if self.render_item_slot(ui, options).clicked() {
                self.do_update(Message::SelectItem(SelectedItem(group, index)));
            }
        }
    }

    pub fn render_selected_item(&mut self, ui: &mut Ui) {
        let player = &mut *self.player.write();
        let item = selected_item(self.selected_item, self.selected_loadout, player);

        let item_meta = self.item_meta.read();
        let prefix_meta = self.prefix_meta.read();

        let largest_item_id = item_meta
            .last()
            .expect("There should be at least one item")
            .id;
        let largest_prefix_id = prefix_meta
            .last()
            .expect("There should be at least one prefix")
            .id;

        let item_meta = ItemMeta::get_or_default(&item_meta, item.id);
        let prefix_meta = PrefixMeta::get(&prefix_meta, item.prefix.id);

        if item.id > 0 {
            ui.label(item_name(&item_meta.name, prefix_meta));
        } else {
            ui.label("");
        }

        egui::Grid::new("selected_item")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label("Id:");
                ui.drag_value_with_buttons(&mut item.id, 1., 0..=largest_item_id);
                if ui.button("\u{1f50e}").clicked() {
                    self.do_update(Message::OpenItemBrowser);
                }
                ui.end_row();

                ui.label("Stack:");
                ui.drag_value_with_buttons(&mut item.stack, 1., 0..=item_meta.max_stack);
                if ui.button("Max").clicked() {
                    item.stack = item_meta.max_stack;
                }
                ui.end_row();

                ui.label("Prefix:");
                ui.drag_value_with_buttons(&mut item.prefix.id, 1., 0..=largest_prefix_id);
                if ui.button("\u{1f50e}").clicked() {
                    self.do_update(Message::OpenPrefixBrowser);
                }
                ui.end_row();
            });
    }

    pub fn render_item_tooltip(&self, ui: &mut Ui, options: ItemTooltipOptions) {
        let item_meta = self.item_meta.read();
        let meta = ItemMeta::get_or_default(&item_meta, options.id);
        ui.add(ItemTooltip::new(options, meta));
    }

    pub fn render_buff_slot(&self, ui: &mut Ui, options: BuffSlotOptions) -> Response {
        let buff_spritesheet = self.buff_spritesheet.read();
        let buff_meta = self.buff_meta.read();

        if buff_spritesheet.is_none() && !self.is_busy() {
            self.do_update(Message::LoadBuffSpritesheet);
        }

        let tooltip_options = BuffTooltipOptions::from_slot_options(&options);
        let tooltip_on_hover = options.tooltip_on_hover;

        let meta = BuffMeta::get_or_default(&buff_meta, options.id);
        let slot = BuffSlot::new(options, meta, buff_spritesheet.as_ref());
        let response = self.render_slot(ui, slot);

        if meta.id != 0 && tooltip_on_hover {
            response.on_hover_ui(|ui| {
                self.render_buff_tooltip(ui, tooltip_options);
            })
        } else {
            response
        }
    }

    pub fn render_buff_slots<I>(&self, ui: &mut Ui, options: I)
    where
        I: Iterator<Item = (usize, BuffSlotOptions)>,
    {
        for (index, mut options) in options {
            options.selected = self.selected_buff.equals(index);

            if self.render_buff_slot(ui, options).clicked() {
                self.do_update(Message::SelectBuff(SelectedBuff(index)));
            }
        }
    }

    pub fn render_selected_buff(&mut self, ui: &mut Ui) {
        let player = &mut *self.player.write();
        let buff = selected_buff(self.selected_buff, player);

        let buff_meta = self.buff_meta.read();

        let largest_buff_id = buff_meta
            .last()
            .expect("We really should have at least one buff")
            .id;

        let buff_meta = BuffMeta::get_or_default(&buff_meta, buff.id);

        if buff.id > 0 {
            ui.label(buff_name(&buff_meta.name, Some(buff.time)));
        } else {
            ui.label("");
        }

        egui::Grid::new("selected_buff")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label("Id:");
                ui.drag_value_with_buttons(&mut buff.id, 1., 0..=largest_buff_id);
                if ui.button("\u{1f50e}").clicked() {
                    self.do_update(Message::OpenBuffBrowser);
                }
                ui.end_row();

                ui.label("Duration:");
                ui.drag_value_with_buttons(&mut buff.time, 1., 0..=i32::MAX);
                if ui.button("Max").clicked() {
                    buff.time = i32::MAX;
                }
                ui.end_row();
            });
    }

    pub fn render_buff_tooltip(&self, ui: &mut Ui, options: BuffTooltipOptions) {
        let buff_meta = self.buff_meta.read();
        let meta = BuffMeta::get_or_default(&buff_meta, options.id);
        ui.add(BuffTooltip::new(options, meta));
    }

    pub fn render_prefix_tooltip(&self, ui: &mut Ui, options: PrefixTooltipOptions) {
        let prefix_meta = self.prefix_meta.read();
        let meta = PrefixMeta::get_or_default(&prefix_meta, options.id);
        ui.add(PrefixTooltip::new(options, meta));
    }
}
