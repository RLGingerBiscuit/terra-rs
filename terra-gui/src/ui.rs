mod clickable_frame;

use std::ops::RangeInclusive;

use eframe::emath;
use egui::{
    vec2, Align, Button, DragValue, InnerResponse, KeyboardShortcut, Layout, Response, Ui,
    WidgetText,
};

pub use clickable_frame::ClickableFrame;

#[macro_export]
macro_rules! enum_radio_value {
    ( $ui:expr,$current:expr,$($enum_item:expr),* ) => {
        $(
            $ui.radio_value($current, $enum_item, $enum_item.to_string());
        )*
    };
}

#[macro_export]
macro_rules! enum_selectable_value {
    ( $ui:expr,$current:expr,$($enum_item:expr),* ) => {
        $(
            $ui.selectable_value($current, $enum_item, $enum_item.to_string());
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

    fn clickable_group<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R>;

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
                let new_value = Num::from_f64(value.to_f64() - 1.);
                if range.contains(&new_value) {
                    *value = new_value;
                }
            }

            let old_padding = ui.spacing().button_padding;

            ui.spacing_mut().button_padding = vec2(0., 0.);
            ui.drag_value(value, speed, range.clone());
            ui.spacing_mut().button_padding = old_padding;

            if ui.button(">").clicked() {
                let new_value = Num::from_f64(value.to_f64() + 1.);
                if range.contains(&new_value) {
                    *value = new_value;
                }
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
    fn clickable_group<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ClickableFrame::group(self.style()).show(self, add_contents)
    }

    #[inline]
    fn vertical_right_justified<R>(
        &mut self,
        add_contents: impl FnOnce(&mut Self) -> R,
    ) -> InnerResponse<R> {
        self.with_layout(Layout::top_down(Align::Max), add_contents)
    }
}
