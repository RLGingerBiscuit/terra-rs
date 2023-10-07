use egui::{
    pos2, vec2, Align2, Color32, Image, Margin, Rect, Response, Sense, TextStyle, TextureHandle,
    Ui, Vec2, Widget,
};
use terra_core::{Item, ItemMeta, PrefixMeta};

use super::{
    slot::{calc_uv_size_padding, render_padded_sprite, Slot},
    ItemGroup,
};

pub const ICON_SIZE: Vec2 = Vec2::splat(17.);
pub const ICON_DISPLAYED_SIZE: Vec2 = Vec2::splat(32.);

pub const SLOT_SIZE: Vec2 = Vec2::splat(40.);
pub const MARGIN: Margin = Margin {
    left: 6.,
    right: 6.,
    top: 6.,
    bottom: 6.,
};

pub const SPRITE_SCALE: Vec2 = Vec2::splat(2.);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemSlotIcon {
    HeadPiece,
    VanityHeadPiece,
    ArmorPiece,
    VanityArmorPiece,
    LegsPiece,
    VanityLegsPiece,
    Dye,
    Hook,
    Cart,
    Pet,
    Mount,
    // ...
    VanityAccessory,
    // ...
    // ...
    Accessory,
    // ...
    LightPet,
    //
    // TODO: Coins icon
    Coins,
}

#[derive(Debug, Copy, Clone)]
pub struct ItemSlotOptions<'a> {
    pub id: i32,
    pub group: ItemGroup,
    pub icon: Option<ItemSlotIcon>,
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
            icon: None,
            prefix_meta: None,
            selected: false,
            favourited: false,
            tooltip_on_hover: false,
            stack: None,
        }
    }

    pub fn from_item(item: &Item, group: ItemGroup) -> Self {
        let options = Self::new(group).id(item.id).favourited(item.favourited);

        if item.stack > 0 {
            options.stack(Some(item.stack))
        } else {
            options
        }
    }

    pub fn from_meta(meta: &ItemMeta, group: ItemGroup) -> Self {
        Self::new(group).id(meta.id)
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn group(mut self, group: ItemGroup) -> Self {
        self.group = group;
        self
    }

    pub fn icon(mut self, icon: Option<ItemSlotIcon>) -> Self {
        self.icon = icon;
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
    item_sheet: Option<&'a TextureHandle>,
    icon_sheet: Option<&'a TextureHandle>,
}

#[allow(dead_code)]
impl<'a> ItemSlot<'a> {
    pub fn new(
        options: ItemSlotOptions<'a>,
        meta: &'a ItemMeta,
        item_spritesheet: Option<&'a TextureHandle>,
        icon_spritesheet: Option<&'a TextureHandle>,
    ) -> Self {
        Self {
            options,
            meta,
            item_sheet: item_spritesheet,
            icon_sheet: icon_spritesheet,
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

    fn render_item_slot_icon(
        &self,
        ui: &mut Ui,
        icon: ItemSlotIcon,
        sheet: &TextureHandle,
    ) -> Response {
        let index = match icon {
            ItemSlotIcon::HeadPiece => vec2(0., 0.),
            ItemSlotIcon::VanityHeadPiece => vec2(0., 1.),
            ItemSlotIcon::ArmorPiece => vec2(0., 2.),
            ItemSlotIcon::VanityArmorPiece => vec2(0., 3.),
            ItemSlotIcon::LegsPiece => vec2(0., 4.),
            ItemSlotIcon::VanityLegsPiece => vec2(0., 5.),
            ItemSlotIcon::Dye => vec2(1., 0.),
            ItemSlotIcon::Hook => vec2(1., 1.),
            ItemSlotIcon::Cart => vec2(1., 2.),
            ItemSlotIcon::Pet => vec2(1., 3.),
            ItemSlotIcon::Mount => vec2(1., 4.),
            ItemSlotIcon::VanityAccessory => vec2(2., 0.),
            ItemSlotIcon::Accessory => vec2(2., 3.),
            ItemSlotIcon::LightPet => vec2(2., 5.),
            ItemSlotIcon::Coins => todo!(),
        };

        let sheet_size = sheet.size_vec2();
        let min = ((index * ICON_SIZE) / sheet_size).to_pos2();
        let uv = Rect::from_min_size(min, ICON_SIZE / sheet_size);
        let padding = (SLOT_SIZE - ICON_DISPLAYED_SIZE) / 2.;

        // TODO: Make translucent, but IDK how to do that atm
        let tint = ui.style().visuals.weak_text_color();
        let tint = Color32::from_rgba_premultiplied(
            tint.r(),
            tint.g(),
            tint.b(),
            (u8::MAX as f32 / 8.) as u8,
        );

        ui.horizontal_top(|ui| {
            ui.add_space(padding.x);
            ui.vertical(|ui| {
                ui.add_space(padding.y);
                ui.add(Image::new(sheet, ICON_DISPLAYED_SIZE).uv(uv).tint(tint));
                ui.add_space(padding.y);
            });
            ui.add_space(padding.x);
        })
        .response
    }
}

impl<'a> Widget for ItemSlot<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let allocate_size = |ui: &mut Ui| {
            let (_, response) = ui.allocate_exact_size(self.slot_size(), Sense::hover());
            response
        };

        let try_render_icon = |ui: &mut Ui| match self.options.icon {
            None => allocate_size(ui),
            Some(icon) => {
                // TODO: Coins icon
                if icon == ItemSlotIcon::Coins {
                    allocate_size(ui)
                } else {
                    match self.icon_sheet {
                        None => allocate_size(ui),
                        Some(sheet) => self.render_item_slot_icon(ui, icon, sheet),
                    }
                }
            }
        };

        let response = if self.meta.id == 0 {
            try_render_icon(ui)
        } else {
            match self.item_sheet {
                None => try_render_icon(ui),
                Some(sheet) => {
                    let (uv, size, padding) = calc_uv_size_padding(
                        sheet,
                        self.sprite_rect(),
                        self.scale(),
                        self.slot_size(),
                    );

                    render_padded_sprite(ui, sheet, uv, self.slot_size(), size, padding)
                }
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
