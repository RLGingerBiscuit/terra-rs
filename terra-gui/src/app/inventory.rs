#![allow(dead_code)]

use egui::{Image, Pos2, Rect, TextureHandle, Ui, Vec2};
use terra_core::{Buff, Item, BUFF_SPRITE_SIZE as CORE_BUFF_SPRITE_SIZE};

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

impl App {
    // TODO: split sprite into render_icon (or something)
    // TODO: render slot icons
    // TODO: render coloured slots
    pub fn render_slot(
        &self,
        ui: &mut Ui,
        slot_size: f32,
        scale: f32,
        width: f32,
        height: f32,
        x: f32,
        y: f32,
        spritesheet: &TextureHandle,
        _stack_size: Option<i32>,
    ) {
        let [spritesheet_width, spritesheet_height] = spritesheet.size().map(|s| s as f32);

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

        let padding_x = if final_width < slot_size {
            (slot_size - final_width) / 2.
        } else {
            0.
        };
        let padding_y = if final_height < slot_size {
            (slot_size - final_height) / 2.
        } else {
            0.
        };

        let min = Pos2::new(x / spritesheet_width, y / spritesheet_height);
        let sprite_size = Vec2::new(width / spritesheet_width, height / spritesheet_height);
        let uv = Rect::from_min_size(min, sprite_size);

        let size = Vec2::new(final_width, final_height);

        // TODO: Display stack size if not None
        ui.group(|ui| {
            ui.spacing_mut().item_spacing = [0., 0.].into();
            ui.add_space(padding_x);
            ui.vertical(|ui| {
                ui.add_space(padding_y);
                ui.add(Image::new(spritesheet, size).uv(uv));
                ui.add_space(padding_y);
            });
            ui.add_space(padding_x);
        });
    }

    pub fn render_item(&self, ui: &mut Ui, item: &Item) {
        let spritesheet = self.item_spritesheet.read();

        if let Some(spritesheet) = &*spritesheet {
            let meta = meta_or_default!(self.item_meta, item.id);

            let width = meta.width as f32;
            let height = meta.height as f32;
            let x = meta.x as f32;
            let y = meta.y as f32;

            self.render_slot(
                ui,
                ITEM_SLOT_SIZE,
                ITEM_SPRITE_SCALE,
                width,
                height,
                x,
                y,
                spritesheet,
                None,
            );
        } else if !self.busy {
            self.do_update(Message::LoadItemSpritesheet);
        }
    }

    pub fn render_buff(&self, ui: &mut Ui, buff: &Buff) {
        let spritesheet = self.buff_spritesheet.read();

        if let Some(spritesheet) = &*spritesheet {
            let meta = meta_or_default!(self.buff_meta, buff.id);

            let width = BUFF_SPRITE_SIZE;
            let height = BUFF_SPRITE_SIZE;
            let x = meta.x as f32;
            let y = meta.y as f32;

            self.render_slot(
                ui,
                BUFF_SLOT_SIZE,
                BUFF_SPRITE_SCALE,
                width,
                height,
                x,
                y,
                spritesheet,
                None,
            );
        } else if !self.busy {
            self.do_update(Message::LoadBuffSpritesheet);
        }
    }
}
