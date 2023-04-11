#![allow(dead_code)]

use egui::{Image, Pos2, Rect, Response, TextureHandle, Ui, Vec2};
use terra_core::{Buff, BuffMeta, Item, ItemMeta, BUFF_SPRITE_SIZE as CORE_BUFF_SPRITE_SIZE};

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

#[derive(Debug)]
pub enum SelectedItem {
    Inventory(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ItemTab {
    Inventory,
}

#[derive(Debug)]
pub struct SelectedBuff(pub usize);

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
        _stack_size: Option<i32>,
    ) -> Response {
        let mut padding_x = slot_size;
        let mut padding_y = slot_size;

        let [width, height]: [f32; 2] = rect.size().into();
        let [x, y]: [f32; 2] = rect.left_top().into();

        let group = if selected {
            let mut frame = ClickableFrame::group(ui.style());
            frame.fill = ui.visuals().extreme_bg_color;
            frame
        } else {
            ClickableFrame::group(ui.style())
        };

        group
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

                    let min = Pos2::new(x / spritesheet_width, y / spritesheet_height);
                    let sprite_size =
                        Vec2::new(width / spritesheet_width, height / spritesheet_height);
                    let uv = Rect::from_min_size(min, sprite_size);
                    let size = Vec2::new(final_width, final_height);

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
            .response
    }

    pub fn render_item(&self, ui: &mut Ui, tab: ItemTab, index: usize, item: &Item) -> Response {
        let spritesheet = self.item_spritesheet.read();

        let meta = meta_or_default!(self.item_meta, item.id);

        let min = Pos2::new(meta.x as f32, meta.y as f32);
        let size = Vec2::new(meta.width as f32, meta.height as f32);
        let rect = Rect::from_min_size(min, size);

        let selected = match self.selected_item {
            SelectedItem::Inventory(i) => tab == ItemTab::Inventory && i == index,
        };

        if spritesheet.is_none() && !self.busy {
            self.do_update(Message::LoadItemSpritesheet);
        }

        self.render_slot(
            ui,
            ITEM_SLOT_SIZE,
            ITEM_SPRITE_SCALE,
            rect,
            selected,
            spritesheet.as_ref(),
            None,
        )
    }

    pub fn render_item_name(&self, ui: &mut Ui, item: &Item, meta: &ItemMeta) {
        let prefix_meta = meta_or_default!(self.prefix_meta, item.prefix.id);

        if prefix_meta.id != 0 {
            ui.label(format!("{} {}", &prefix_meta.name, &meta.name));
        } else {
            ui.label(&meta.name);
        };
    }

    pub fn render_selected_item(&mut self, ui: &mut Ui) {
        let mut player = self.player.write();

        let item = match self.selected_item {
            SelectedItem::Inventory(i) => &mut player.inventory[i],
        };

        let meta = meta_or_default!(self.item_meta, item.id);

        let largest_item_id = self
            .item_meta
            .last()
            .expect("We really should have at least one item")
            .id;
        let largest_prefix_id = self
            .prefix_meta
            .last()
            .expect("We really should have at least one prefix")
            .id;

        self.render_item_name(ui, item, meta);

        egui::Grid::new("selected_item")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Id: ");
                ui.drag_value_with_buttons(&mut item.id, 1., 0..=largest_item_id);
                ui.end_row();

                ui.label("Stack: ");
                ui.drag_value_with_buttons(&mut item.stack, 1., 0..=meta.max_stack);
                ui.end_row();

                ui.label("Prefix: ");
                ui.drag_value_with_buttons(&mut item.prefix.id, 1., 0..=largest_prefix_id);
                ui.end_row();
            });
    }

    pub fn render_buff(&self, ui: &mut Ui, index: usize, buff: &Buff) -> Response {
        let spritesheet = self.buff_spritesheet.read();

        let meta = meta_or_default!(self.buff_meta, buff.id);

        let min = Pos2::new(meta.x as f32, meta.y as f32);
        let size = Vec2::splat(BUFF_SPRITE_SIZE);
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

        let meta = meta_or_default!(self.buff_meta, buff.id);

        let largest_buff_id = self
            .buff_meta
            .last()
            .expect("we really should have at least one buff")
            .id;

        self.render_buff_name(ui, buff, meta);

        egui::Grid::new("selected_buff")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Id: ");
                ui.drag_value_with_buttons(&mut buff.id, 1., 0..=largest_buff_id);
                ui.end_row();

                ui.label("Duration: ");
                ui.drag_value_with_buttons(&mut buff.time, 1., 0..=i32::MAX);
                ui.end_row();
            });
    }
}
