use egui::{Align, InnerResponse, Layout, Ui};

pub trait UiExt {
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
    fn vertical_right_justified<R>(
        &mut self,
        add_contents: impl FnOnce(&mut Self) -> R,
    ) -> InnerResponse<R> {
        self.with_layout(Layout::top_down(Align::Max), add_contents)
    }
}
