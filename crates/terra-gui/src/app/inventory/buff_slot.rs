use egui::{pos2, Margin, Rect, Response, Sense, TextureHandle, Ui, Vec2, Widget};
use terra_core::{Buff, BuffMeta, BUFF_SPRITE_SIZE};

use super::slot::{calc_uv_size_padding, render_padded_sprite, Slot, SlotText};

pub const SLOT_SIZE: Vec2 = Vec2::splat(32.);
pub const MARGIN: Margin = Margin {
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
};

pub const SPRITE_SIZE: Vec2 = Vec2::splat(BUFF_SPRITE_SIZE as f32);
pub const SPRITE_SCALE: Vec2 = Vec2::splat(2.);

#[derive(Debug, Clone)]
pub struct BuffSlotOptions {
    pub id: i32,
    pub selected: bool,
    pub highlighted: bool,
    pub tooltip_on_hover: bool,
    pub time: Option<i32>,
    pub texts: Vec<SlotText>,
}

#[allow(dead_code)]
impl BuffSlotOptions {
    pub fn new() -> Self {
        Self {
            id: 0,
            selected: false,
            highlighted: false,
            tooltip_on_hover: false,
            time: None,
            texts: Vec::new(),
        }
    }

    pub fn from_buff(buff: &Buff) -> Self {
        let options = Self::new().id(buff.id);

        if buff.time > 0 {
            options.time(Some(buff.time))
        } else {
            options
        }
    }

    pub fn from_meta(meta: &BuffMeta) -> Self {
        Self::new().id(meta.id)
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
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

    pub fn texts(mut self, texts: &[SlotText]) -> Self {
        self.texts = texts.to_owned();
        self
    }
}

pub(super) struct BuffSlot<'a> {
    options: BuffSlotOptions,
    meta: &'a BuffMeta,
    buff_sheet: Option<&'a TextureHandle>,
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
            buff_sheet: spritesheet,
        }
    }

    pub fn meta(&self) -> &BuffMeta {
        self.meta
    }

    pub fn options(&self) -> &BuffSlotOptions {
        &self.options
    }
}

impl Widget for BuffSlot<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(SLOT_SIZE, Sense::hover());

        {
            let mut ui = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(*ui.layout()));

            if let Some(sheet) = self.buff_sheet {
                let (uv, size, padding) =
                    calc_uv_size_padding(sheet, self.sprite_rect(), self.scale(), self.slot_size());

                render_padded_sprite(&mut ui, sheet, uv, size, padding, None);
            }
        }

        response
    }
}

impl Slot for BuffSlot<'_> {
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

    fn highlighted(&self) -> bool {
        self.options.highlighted
    }
}
