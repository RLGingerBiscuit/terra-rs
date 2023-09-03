use egui::{pos2, vec2, Margin, Rect, Response, Sense, TextureHandle, Ui, Vec2, Widget, Image};
use terra_core::{Buff, BuffMeta, BUFF_SPRITE_SIZE};

use super::{calculate_uv, slot::Slot};

pub const SLOT_SIZE: Vec2 = Vec2::splat(32.);
pub const MARGIN: Margin = Margin {
    left: 0.,
    right: 0.,
    top: 0.,
    bottom: 0.,
};

pub const SPRITE_SIZE: Vec2 = Vec2::splat(BUFF_SPRITE_SIZE as f32);
pub const SPRITE_SCALE: Vec2 = Vec2::splat(2.);

#[derive(Debug, Clone, Copy)]
pub struct BuffSlotOptions {
    pub id: i32,
    pub selected: bool,
    pub tooltip_on_hover: bool,
    pub time: Option<i32>,
}

#[allow(dead_code)]
impl BuffSlotOptions {
    pub fn new() -> Self {
        Self {
            id: 0,
            selected: false,
            tooltip_on_hover: false,
            time: None,
        }
    }

    pub fn from_buff(buff: &Buff) -> Self {
        Self::new().id(buff.id).time(Some(buff.time))
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tooltip_on_hover(mut self, tooltip_on_hover: bool) -> Self {
        self.tooltip_on_hover = tooltip_on_hover;
        self
    }

    pub fn time(mut self, time: Option<i32>) -> Self {
        self.time = time;
        self
    }
}

pub(super) struct BuffSlot<'a> {
    options: BuffSlotOptions,
    meta: &'a BuffMeta,
    sheet: Option<&'a TextureHandle>,
}

#[allow(dead_code)]
impl<'a> BuffSlot<'a> {
    pub fn new(
        options: BuffSlotOptions,
        meta: &'a BuffMeta,
        spritesheet: Option<&'a TextureHandle>,
    ) -> Self {
        BuffSlot {
            options,
            meta,
            sheet: spritesheet,
        }
    }

    pub fn meta(&self) -> &BuffMeta {
        self.meta
    }

    pub fn options(&self) -> &BuffSlotOptions {
        &self.options
    }
}

impl<'a> Widget for BuffSlot<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        match self.sheet {
            None => {
                let (_, response) = ui.allocate_exact_size(self.slot_size(), Sense::hover());
                response
            }
            Some(sheet) => {
                let uv = calculate_uv(sheet, self.sprite_rect());
                let mut size = vec2(
                    self.sprite_rect().size().x * self.scale().x,
                    self.sprite_rect().size().y * self.scale().y,
                );

                if size.x > self.slot_size().x || size.y > self.slot_size().y {
                    if size.x >= size.y {
                        // Landscape
                        size *= self.slot_size().x / size.x;
                    } else {
                        // Portrait
                        size *= self.slot_size().y / size.y;
                    }
                }

                let padding = vec2(
                    if size.x < self.slot_size().x {
                        (self.slot_size().x - size.x) / 2.
                    } else {
                        0.
                    },
                    if size.y < self.slot_size().y {
                        (self.slot_size().y - size.y) / 2.
                    } else {
                        0.
                    },
                );

                ui.horizontal_top(|ui| {
                    ui.add_space(padding.x);
                    ui.vertical(|ui| {
                        ui.add_space(padding.y);
                        ui.add(Image::new(sheet, size).uv(uv));
                        ui.add_space(padding.y);
                    });
                    ui.add_space(padding.x);
                })
                .response
            }
        }
    }
}

impl<'a> Slot for BuffSlot<'a> {
    fn slot_size(&self) -> Vec2 {
        SLOT_SIZE
    }

    fn sprite_rect(&self) -> Rect {
        let min = pos2(self.meta.x as f32, self.meta.y as f32);
        Rect::from_min_size(min, SPRITE_SIZE)
    }

    fn scale(&self) -> Vec2 {
        SPRITE_SCALE
    }

    fn margin(&self) -> Margin {
        MARGIN
    }

    fn selected(&self) -> bool {
        self.options.selected
    }

    fn tooltip_on_hover(&self) -> bool {
        self.options.tooltip_on_hover
    }
}
