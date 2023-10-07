use egui::{vec2, Image, Margin, Rect, Response, TextureHandle, Ui, Vec2, Widget};

pub(super) trait Slot<W: Widget = Self> {
    fn slot_size(&self) -> Vec2;
    fn sprite_rect(&self) -> Rect;
    fn scale(&self) -> Vec2;
    fn margin(&self) -> Margin;
    fn selected(&self) -> bool;
    fn tooltip_on_hover(&self) -> bool;
}

pub(super) fn calc_uv_size_padding(
    sheet: &TextureHandle,
    sprite_rect: Rect,
    scale: Vec2,
    slot_size: Vec2,
) -> (Rect, Vec2, Vec2) {
    let sheet_size = sheet.size_vec2();
    let min = (sprite_rect.min.to_vec2() / sheet_size).to_pos2();
    let size = sprite_rect.size() / sheet_size;
    let uv = Rect::from_min_size(min, size);

    let mut size = sprite_rect.size() * scale;

    if size.x > slot_size.x || size.y > slot_size.y {
        if size.x >= size.y {
            // Landscape
            size *= slot_size.x / size.x;
        } else {
            // Portrait
            size *= slot_size.y / size.y;
        }
    }

    let padding = vec2(
        if size.x < slot_size.x {
            (slot_size.x - size.x) / 2.
        } else {
            0.
        },
        if size.y < slot_size.y {
            (slot_size.y - size.y) / 2.
        } else {
            0.
        },
    );

    (uv, size, padding)
}

pub(super) fn render_padded_sprite(
    ui: &mut Ui,
    sheet: &TextureHandle,
    uv: Rect,
    slot_size: Vec2,
    size: Vec2,
    padding: Vec2,
) -> Response {
    ui.allocate_ui(slot_size, |ui| {
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
