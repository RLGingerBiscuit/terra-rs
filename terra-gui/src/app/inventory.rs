use egui::{Image, Pos2, Rect, TextureHandle, Ui, Vec2};
use terra_core::{Buff, BuffMeta, Item, ItemMeta, BUFF_SPRITE_SIZE as CORE_BUFF_SPRITE_SIZE, PrefixMeta};

use super::{App, Message};

pub const ITEM_SIZE: f32 = 40.;
pub const ITEM_SCALE: f32 = 2.;

pub const BUFF_SPRITE_SIZE: f32 = CORE_BUFF_SPRITE_SIZE as f32;
pub const BUFF_SIZE: f32 = 32.;
pub const BUFF_SCALE: f32 = 2.;

// TODO: Get correct meta
impl App {
    fn get_item_meta_or_default<'a>(&'a self, id: i32) -> &'a ItemMeta {
        self.item_meta
            .iter()
            .filter(|m| m.id == id)
            .next()
            .unwrap_or(
                self.item_meta
                    .iter()
                    // .filter(|m| m.id == 0)
                    .next()
                    .expect("We really should have a zeroth item"),
            )
    }

    fn get_buff_meta_or_default<'a>(&'a self, id: i32) -> &'a BuffMeta {
        self.buff_meta
            .iter()
            .filter(|m| m.id == id)
            .next()
            .unwrap_or(
                self.buff_meta
                    .iter()
                    // .filter(|m| m.id == 0)
                    .next()
                    .expect("We really should have a zeroth buff"),
            )
    }

    fn get_prefix_meta_or_default<'a>(&'a self, id: u8) -> &'a PrefixMeta {
        self.prefix_meta
            .iter()
            .filter(|m| m.id == id)
            .next()
            .unwrap_or(
                self.prefix_meta
                    .iter()
                    // .filter(|m| m.id == 0)
                    .next()
                    .expect("We really should have a zeroth prefix"),
            )
    }

    fn render_sprite(
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
        // TODO: Should this be locked & unlocked every time?
        let spritesheet = self.item_spritesheet.lock();

        if let Some(spritesheet) = &*spritesheet {
            let meta = self.get_item_meta_or_default(item.id);

            let width = meta.width as f32;
            let height = meta.height as f32;
            let x = meta.x as f32;
            let y = meta.y as f32;

            self.render_sprite(
                ui,
                ITEM_SIZE,
                ITEM_SCALE,
                width,
                height,
                x,
                y,
                spritesheet,
                None,
            );
        }
    }

    pub fn render_buff(&self, ui: &mut Ui, buff: &Buff) {
        // TODO: Should this be locked & unlocked every time?
        let spritesheet = self.buff_spritesheet.lock();

        if let Some(spritesheet) = &*spritesheet {
            let meta = self.get_buff_meta_or_default(buff.id);

            let x = meta.x as f32;
            let y = meta.y as f32;

            self.render_sprite(
                ui,
                BUFF_SIZE,
                BUFF_SCALE,
                BUFF_SPRITE_SIZE,
                BUFF_SPRITE_SIZE,
                x,
                y,
                spritesheet,
                None,
            );
        }
    }
}
