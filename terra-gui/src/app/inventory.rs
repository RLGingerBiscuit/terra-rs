#![allow(dead_code)]

use egui::{pos2, vec2, Align2, Image, Rect, Response, TextStyle, TextureHandle, Ui};
use terra_core::{
    Buff, BuffMeta, Item, ItemMeta, PrefixMeta, BUFF_SPRITE_SIZE as CORE_BUFF_SPRITE_SIZE,
};

use crate::ui::{ClickableFrame, UiExt};

use super::{App, Message};

pub const ITEM_SLOT_SIZE: f32 = 40.;
pub const ITEM_SPRITE_SCALE: f32 = 2.;

pub const BUFF_SLOT_SIZE: f32 = 32.;
pub const BUFF_SPRITE_SIZE: f32 = CORE_BUFF_SPRITE_SIZE as f32;
pub const BUFF_SPRITE_SCALE: f32 = 2.;

macro_rules! meta_or_default {
    ($meta:expr, $id:expr) => {
        $meta
            .get($id as usize)
            // .iter().filter(|m| m.id == id).next()
            .unwrap_or($meta.get(0).expect("We really should have a zeroth meta"))
    };
}

#[macro_export]
macro_rules! selected_item {
    ($selected_item:expr,$selected_loadout:expr, $player:expr) => {
        match $selected_item.0 {
            ItemTab::Inventory => &mut $player.inventory[$selected_item.1],
            ItemTab::Bank => &mut $player.piggy_bank[$selected_item.1],
            ItemTab::Safe => &mut $player.safe[$selected_item.1],
            ItemTab::Forge => &mut $player.defenders_forge[$selected_item.1],
            ItemTab::Void => &mut $player.void_vault[$selected_item.1],
            ItemTab::Equipment => &mut $player.equipment[$selected_item.1],
            ItemTab::EquipmentDyes => &mut $player.equipment_dyes[$selected_item.1],
            // TODO: The only case where a change will happen immediately without clicking on another item is changing loadouts, do I want to keep this?
            ItemTab::VanityArmor => {
                &mut $player.loadouts[$selected_loadout.0].vanity_armor[$selected_item.1]
            }
            ItemTab::Armor => {
                &mut $player.loadouts[$selected_loadout.0].armor[$selected_item.1]
            }
            ItemTab::ArmorDyes => {
                &mut $player.loadouts[$selected_loadout.0].armor_dyes[$selected_item.1]
            }
            ItemTab::VanityAccessories => {
                &mut $player.loadouts[$selected_loadout.0].vanity_accessories
                    [$selected_item.1]
            }
            ItemTab::Accessories => {
                &mut $player.loadouts[$selected_loadout.0].accessories[$selected_item.1]
            }
            ItemTab::AccessoryDyes => {
                &mut $player.loadouts[$selected_loadout.0].accessory_dyes[$selected_item.1]
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemTab {
    Inventory,
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
}

#[derive(Debug)]
pub struct SelectedItem(pub ItemTab, pub usize);

#[derive(Debug)]
pub struct SelectedBuff(pub usize);

#[derive(Debug)]
pub struct SelectedLoadout(pub usize);

impl App {
    // TODO: split sprite into render_icon (or something)
    // TODO: render slot icons
    // TODO: render coloured slots
    pub fn render_slot(
        &self,
        ui: &mut Ui,
        slot_size: f32,
        scale: f32,
        rect: Rect,
        selected: bool,
        spritesheet: Option<&TextureHandle>,
        stack_size: Option<i32>,
    ) -> Response {
        let mut padding_x = slot_size;
        let mut padding_y = slot_size;

        let [width, height]: [f32; 2] = rect.size().into();
        let [x, y]: [f32; 2] = rect.left_top().into();

        let group = if selected {
            let mut frame = ClickableFrame::group(ui.style());
            frame.fill = ui.visuals().code_bg_color;
            frame
        } else {
            ClickableFrame::group(ui.style())
        };

        let response = group
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = [0., 0.].into();

                if let Some(spritesheet) = spritesheet {
                    let [spritesheet_width, spritesheet_height] =
                        spritesheet.size().map(|s| s as f32);

                    let mut final_width = width * scale;
                    let mut final_height = height * scale;

                    let scale = if final_width > slot_size || final_height > slot_size {
                        if final_width <= final_height {
                            slot_size / final_height
                        } else {
                            slot_size / final_width
                        }
                    } else {
                        1.
                    };

                    final_width *= scale;
                    final_height *= scale;

                    padding_x = if final_width < slot_size {
                        (slot_size - final_width) / 2.
                    } else {
                        0.
                    };
                    padding_y = if final_height < slot_size {
                        (slot_size - final_height) / 2.
                    } else {
                        0.
                    };

                    let min = pos2(x / spritesheet_width, y / spritesheet_height);
                    let sprite_size = vec2(width / spritesheet_width, height / spritesheet_height);
                    let uv = Rect::from_min_size(min, sprite_size);
                    let size = vec2(final_width, final_height);

                    // TODO: Display stack size if not None
                    ui.add_space(padding_x);
                    ui.vertical(|ui| {
                        ui.add_space(padding_y);
                        ui.add(Image::new(spritesheet, size).uv(uv));
                        ui.add_space(padding_y);
                    });
                    ui.add_space(padding_x);
                } else {
                    ui.add_space(padding_x);
                    ui.vertical(|ui| {
                        ui.add_space(padding_y);
                    });
                }
            })
            .response;

        if let Some(stack) = stack_size {
            let max = response.rect.max;
            let icon_spacing = ui.style().spacing.icon_spacing;

            ui.painter().text(
                pos2(max.x - icon_spacing, max.y - icon_spacing),
                Align2::RIGHT_BOTTOM,
                stack.to_string(),
                TextStyle::Body.resolve(ui.style()),
                ui.style().visuals.text_color(),
            );
        }

        response
    }

    pub fn render_item(
        &self,
        ui: &mut Ui,
        render_stack: bool,
        tab: ItemTab,
        index: usize,
        item: &Item,
    ) -> Response {
        let spritesheet = self.item_spritesheet.read();

        let item_meta = &*self.item_meta.read();
        let meta = meta_or_default!(item_meta, item.id);

        let min = pos2(meta.x as f32, meta.y as f32);
        let size = vec2(meta.width as f32, meta.height as f32);
        let rect = Rect::from_min_size(min, size);

        let selected = self.selected_item.0 == tab && self.selected_item.1 == index;

        if spritesheet.is_none() && !self.busy {
            self.do_update(Message::LoadItemSpritesheet);
        }

        let stack = if render_stack { Some(item.stack) } else { None };

        self.render_slot(
            ui,
            ITEM_SLOT_SIZE,
            ITEM_SPRITE_SCALE,
            rect,
            selected,
            spritesheet.as_ref(),
            stack,
        )
    }

    pub fn render_item_multiple(
        &self,
        ui: &mut Ui,
        render_stack: bool,
        items: &[(&Item, usize, ItemTab)],
    ) {
        for (item, index, tab) in items {
            // TODO: Is there a way to avoid cloning these?
            let index = index.to_owned();
            let tab = tab.to_owned();
            if self
                .render_item(ui, render_stack, tab, index, item)
                .clicked()
            {
                self.do_update(Message::SelectItem(SelectedItem(tab, index)));
            }
        }
    }

    pub fn render_item_name(&self, ui: &mut Ui, item_meta: &ItemMeta, prefix_meta: &PrefixMeta) {
        if prefix_meta.id != 0 {
            ui.label(format!("{} {}", &prefix_meta.name, &item_meta.name));
        } else {
            ui.label(&item_meta.name);
        };
    }

    pub fn render_selected_item(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();

        let item = selected_item!(self.selected_item, self.selected_loadout, player);

        let item_meta = &*self.item_meta.read();
        let prefix_meta = &*self.prefix_meta.read();

        let current_item_meta = meta_or_default!(item_meta, item.id);
        let current_prefix_meta = meta_or_default!(prefix_meta, item.prefix.id);

        let largest_item_id = item_meta
            .last()
            .expect("We really should have at least one item")
            .id;
        let largest_prefix_id = prefix_meta
            .last()
            .expect("We really should have at least one prefix")
            .id;

        self.render_item_name(ui, current_item_meta, current_prefix_meta);

        egui::Grid::new("selected_item")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label("Id: ");
                ui.drag_value_with_buttons(&mut item.id, 1., 0..=largest_item_id);
                ui.end_row();

                ui.label("Stack: ");
                ui.drag_value_with_buttons(&mut item.stack, 1., 0..=current_item_meta.max_stack);
                if ui.button("Max").clicked() {
                    item.stack = current_item_meta.max_stack;
                }
                ui.end_row();

                ui.label("Prefix: ");
                ui.drag_value_with_buttons(&mut item.prefix.id, 1., 0..=largest_prefix_id);
                ui.end_row();
            });
    }

    pub fn render_buff(&self, ui: &mut Ui, index: usize, buff: &Buff) -> Response {
        let spritesheet = self.buff_spritesheet.read();

        let buff_meta = &*self.buff_meta.read();
        let meta = meta_or_default!(buff_meta, buff.id);

        let min = pos2(meta.x as f32, meta.y as f32);
        let size = vec2(BUFF_SPRITE_SIZE, BUFF_SPRITE_SIZE);
        let rect = Rect::from_min_size(min, size);

        let selected = self.selected_buff.0 == index;

        if spritesheet.is_none() && !self.busy {
            self.do_update(Message::LoadBuffSpritesheet);
        }

        self.render_slot(
            ui,
            BUFF_SLOT_SIZE,
            BUFF_SPRITE_SCALE,
            rect,
            selected,
            spritesheet.as_ref(),
            None,
        )
    }

    pub fn render_buff_name(&self, ui: &mut Ui, buff: &Buff, meta: &BuffMeta) {
        const FRAMES_PER_SECOND: i32 = 60;
        const FRAMES_PER_MINUTE: i32 = FRAMES_PER_SECOND * 60;
        const FRAMES_PER_HOUR: i32 = FRAMES_PER_MINUTE * 60;
        const FRAMES_PER_THOUSAND_HOURS: i32 = FRAMES_PER_HOUR * 1000;

        let time = buff.time;
        let time = if time < FRAMES_PER_SECOND {
            format!("({}f)", buff.time)
        } else if time < FRAMES_PER_MINUTE {
            format!("({}s)", buff.time / FRAMES_PER_SECOND)
        } else if time < FRAMES_PER_HOUR {
            format!("({}m)", buff.time / FRAMES_PER_MINUTE)
        } else if time < FRAMES_PER_THOUSAND_HOURS {
            format!("({}h)", buff.time / FRAMES_PER_HOUR)
        } else {
            "(∞)".to_owned()
        };

        ui.label(format!("{} {}", &meta.name, time));
    }

    pub fn render_selected_buff(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();

        let buff = &mut player.buffs[self.selected_buff.0];

        let buff_meta = &*self.buff_meta.read();
        let meta = meta_or_default!(buff_meta, buff.id);

        let largest_buff_id = buff_meta
            .last()
            .expect("we really should have at least one buff")
            .id;

        self.render_buff_name(ui, buff, meta);

        egui::Grid::new("selected_buff")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label("Id: ");
                ui.drag_value_with_buttons(&mut buff.id, 1., 0..=largest_buff_id);
                ui.end_row();

                ui.label("Duration: ");
                ui.drag_value_with_buttons(&mut buff.time, 1., 0..=i32::MAX);
                if ui.button("Max").clicked() {
                    buff.time = i32::MAX;
                }
                ui.end_row();
            });
    }
}
