use egui::{
    load::SizedTexture, vec2, Align, Align2, Color32, FontId, Image, ImageSource, Margin, Rect,
    TextureHandle, Ui, Vec2, Widget,
};

#[derive(Debug, Clone)]
pub struct SlotText {
    pub alignment: Align2,
    pub text: String,
    pub font_id: FontId,
    pub text_color: Color32,
}

impl SlotText {
    pub fn new(alignment: Align2, text: String, font_id: FontId, text_color: Color32) -> Self {
        Self {
            alignment,
            text,
            font_id,
            text_color,
        }
    }
}

pub(super) trait Slot<W: Widget = Self> {
    fn slot_size(&self) -> Vec2;
    fn sprite_rect(&self) -> Rect;
    fn scale(&self) -> Vec2;
    fn margin(&self) -> Margin;
    fn selected(&self) -> bool;
    fn highlighted(&self) -> bool;
}

pub(super) fn paint_texts(ui: &mut Ui, rect: Rect, texts: &[SlotText]) {
    let painter = ui.painter();

    for text in texts.iter() {
        let pos = match text.alignment.0 {
            [Align::Min, Align::Max] => rect.left_bottom(),
            [Align::Min, Align::Center] => rect.left_center(),
            [Align::Min, Align::Min] => rect.left_top(),

            [Align::Center, Align::Max] => rect.center_bottom(),
            [Align::Center, Align::Center] => rect.center(),
            [Align::Center, Align::Min] => rect.center_top(),

            [Align::Max, Align::Max] => rect.right_bottom(),
            [Align::Max, Align::Center] => rect.right_center(),
            [Align::Max, Align::Min] => rect.right_top(),
        };
        let anchor = text.alignment;
        let font_id = text.font_id.clone();
        let text_color = text.text_color;
        let text = text.text.as_str();

        painter.text(pos, anchor, text, font_id, text_color);
    }
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
    size: Vec2,
    padding: Vec2,
    tint: Option<Color32>,
) {
    ui.add_space(padding.x);
    ui.vertical(|ui| {
        ui.add_space(padding.y);
        let source = ImageSource::Texture(SizedTexture::new(sheet, size));
        let mut img = Image::new(source).uv(uv);
        if let Some(tint) = tint {
            img = img.tint(tint);
        }
        ui.add(img);
        ui.add_space(padding.y);
    });
    ui.add_space(padding.x);
}
