use std::ops::RangeInclusive;

use eframe::emath;
use egui::{
    Align, Button, DragValue, InnerResponse, KeyboardShortcut, Layout, Response, Ui, Vec2,
    WidgetText,
};

#[macro_export]
macro_rules! enum_radio_value {
    ( $ui:expr, $current:expr, $($enum_item:expr),* ) => {
        // $($ui.enum_radio_value($current, $enum_item);)*
        $(
            $ui.radio_value($current, $enum_item, $enum_item.to_string());
        )*
    };
}

pub trait UiExt {
    fn labelled<R>(
        &mut self,
        text: impl Into<WidgetText>,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R>;

    fn drag_value<Num: emath::Numeric>(
        &mut self,
        value: &mut Num,
        speed: f32,
        range: RangeInclusive<Num>,
    ) -> Response;

    fn drag_value_with_buttons<Num: emath::Numeric>(
        &mut self,
        value: &mut Num,
        speed: f32,
        range: RangeInclusive<Num>,
    ) -> Response;

    fn shortcut_button(
        &mut self,
        text: impl Into<WidgetText>,
        shortcut: &KeyboardShortcut,
    ) -> Response;

    fn vertical_right_justified<R>(
        &mut self,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R>;
}

impl UiExt for Ui {
    #[inline]
    fn labelled<R>(
        &mut self,
        text: impl Into<WidgetText>,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        self.horizontal(|ui| {
            ui.label(text);
            add_contents(ui)
        })
    }

    #[inline]
    fn drag_value<Num: emath::Numeric>(
        &mut self,
        value: &mut Num,
        speed: f32,
        range: RangeInclusive<Num>,
    ) -> Response {
        self.add(DragValue::new(value).speed(speed).clamp_range(range))
    }

    #[inline]
    fn drag_value_with_buttons<Num: emath::Numeric>(
        &mut self,
        value: &mut Num,
        speed: f32,
        range: RangeInclusive<Num>,
    ) -> Response {
        self.horizontal(|ui| {
            if ui.button("<").clicked() {
                *value = Num::from_f64(value.to_f64() - 1.);
            }

            let old_padding = ui.spacing().button_padding;

            ui.spacing_mut().button_padding = Vec2::splat(0.);
            ui.drag_value(value, speed, range);
            ui.spacing_mut().button_padding = old_padding;

            if ui.button(">").clicked() {
                *value = Num::from_f64(value.to_f64() + 1.);
            }
        })
        .response
    }

    #[inline]
    fn shortcut_button(
        &mut self,
        text: impl Into<WidgetText>,
        shortcut: &KeyboardShortcut,
    ) -> Response {
        self.add(Button::new(text).shortcut_text(self.ctx().format_shortcut(shortcut)))
    }

    #[inline]
    fn vertical_right_justified<R>(
        &mut self,
        add_contents: impl FnOnce(&mut Self) -> R,
    ) -> InnerResponse<R> {
        self.with_layout(Layout::top_down(Align::Max), add_contents)
    }
}
