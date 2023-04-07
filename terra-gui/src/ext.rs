use egui::{Align, Button, InnerResponse, KeyboardShortcut, Layout, Response, Ui, WidgetText};

pub trait UiExt {
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
