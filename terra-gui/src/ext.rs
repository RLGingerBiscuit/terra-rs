use egui::{Align, Button, InnerResponse, KeyboardShortcut, Layout, Response, Ui, WidgetText};

pub trait UiExt {
    /// Usage: `if ui.shortcut_button("Click me", &KeyboardShortcut::new(Modifiers::COMMAND, Key::C)).clicked() { … }`
    ///
    /// Shortcut for `add(Button::new(text).shortcut_text(self.ctx().format_shortcut(shortcut)))`
    ///
    /// Clickable button with text and keyboard shortcut (e.g. `Ctrl+5`).
    ///
    /// See also [`Button`].
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// # fn do_stuff() {}
    ///
    /// if ui.shortcut_button("Click me", &KeyboardShortcut::new(Modifiers::COMMAND, Key::C)).clicked() {
    ///     do_stuff();
    /// }
    /// # });
    /// ```
    fn shortcut_button(
        &mut self,
        text: impl Into<WidgetText>,
        shortcut: &KeyboardShortcut,
    ) -> Response;

    /// Start a ui with vertical layout.
    /// Widgets will be right-justified.
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// ui.vertical_right_justified(|ui| {
    ///     ui.label("over");
    ///     ui.label("under");
    /// });
    /// # });
    /// ```
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
