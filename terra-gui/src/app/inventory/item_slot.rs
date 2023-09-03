use egui::{
    pos2, vec2, Align2, Image, Margin, Rect, Response, Sense, TextStyle, TextureHandle, Ui, Vec2,
    Widget,
};
use terra_core::{meta::Meta, Item, ItemMeta, PrefixMeta};

use super::{calculate_uv, slot::Slot, ItemGroup};

pub const SLOT_SIZE: Vec2 = Vec2::splat(40.);
pub const MARGIN: Margin = Margin {
    left: 6.,
    right: 6.,
    top: 6.,
    bottom: 6.,
};

pub const SPRITE_SCALE: Vec2 = Vec2::splat(2.);

#[derive(Debug, Copy, Clone)]
pub struct ItemSlotOptions<'a> {
    pub id: i32,
    pub group: ItemGroup,
    pub prefix_meta: Option<&'a PrefixMeta>,
    pub selected: bool,
    pub favourited: bool,
    pub tooltip_on_hover: bool,
    pub stack: Option<i32>,
}

#[allow(dead_code)]
impl<'a> ItemSlotOptions<'a> {
    pub fn new(group: ItemGroup) -> Self {
        Self {
            id: 0,
            group,
            prefix_meta: None,
            selected: false,
            favourited: false,
            tooltip_on_hover: false,
            stack: None,
        }
    }

    pub fn from_item(item: &Item, tab: ItemGroup, prefix_meta: &'a [PrefixMeta]) -> Self {
        Self::new(tab)
            .id(item.id)
            .prefix_meta(PrefixMeta::get(prefix_meta, item.prefix.id))
            .favourited(item.favourited)
            .stack(Some(item.stack))
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn group(mut self, group: ItemGroup) -> Self {
        self.group = group;
        self
    }

    pub fn prefix_meta(mut self, meta: Option<&'a PrefixMeta>) -> Self {
        self.prefix_meta = meta;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn favourited(mut self, favourited: bool) -> Self {
        self.favourited = favourited;
        self
    }

    pub fn tooltip_on_hover(mut self, tooltip_on_hover: bool) -> Self {
        self.tooltip_on_hover = tooltip_on_hover;
        self
    }

    pub fn stack(mut self, stack: Option<i32>) -> Self {
        self.stack = stack;
        self
    }
}

pub(super) struct ItemSlot<'a> {
    options: ItemSlotOptions<'a>,
    meta: &'a ItemMeta,
    sheet: Option<&'a TextureHandle>,
}

#[allow(dead_code)]
impl<'a> ItemSlot<'a> {
    pub fn new(
        options: ItemSlotOptions<'a>,
        meta: &'a ItemMeta,
        spritesheet: Option<&'a TextureHandle>,
    ) -> Self {
        Self {
            options,
            meta,
            sheet: spritesheet,
        }
    }

    pub fn meta(&self) -> &ItemMeta {
        self.meta
    }

    pub fn options(&self) -> &ItemSlotOptions {
        &self.options
    }

    pub fn prefix_meta(&self) -> Option<&PrefixMeta> {
        self.options.prefix_meta
    }
}

impl<'a> Widget for ItemSlot<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = match self.sheet {
            None => {
                let (_, response) = ui.allocate_exact_size(self.slot_size(), Sense::hover());
                response
            }
            Some(sheet) => {
                let uv = calculate_uv(sheet, self.sprite_rect());
                let mut size = self.sprite_rect().size() * self.scale();

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
        };

        if let Some(stack) = self.options.stack {
            ui.painter().text(
                response.rect.max,
                Align2::RIGHT_BOTTOM,
                stack.to_string(),
                TextStyle::Body.resolve(ui.style()),
                ui.style().visuals.text_color(),
            );
        }

        response
    }
}

impl<'a> Slot for ItemSlot<'a> {
    fn slot_size(&self) -> Vec2 {
        SLOT_SIZE
    }

    fn sprite_rect(&self) -> Rect {
        let min = pos2(self.meta.x as f32, self.meta.y as f32);
        let size = vec2(self.meta.width as f32, self.meta.height as f32);
        Rect::from_min_size(min, size)
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
