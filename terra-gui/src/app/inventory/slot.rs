use egui::{Margin, Rect, Vec2, Widget};

pub(super) trait Slot<W: Widget = Self> {
    fn slot_size(&self) -> Vec2;
    fn sprite_rect(&self) -> Rect;
    fn scale(&self) -> Vec2;
    fn margin(&self) -> Margin;
    fn selected(&self) -> bool;
    fn tooltip_on_hover(&self) -> bool;
}
