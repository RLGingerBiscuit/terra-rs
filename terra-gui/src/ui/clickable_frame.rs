#![allow(unused)]

// This entire thing is copied from the original implementation.
// The only change is on line 242, to allow the frame to sense clicks.

use egui::epaint::*;
use egui::{layers::ShapeIdx, *};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct ClickableFrame {
    pub inner_margin: Margin,

    pub outer_margin: Margin,

    pub rounding: Rounding,

    pub shadow: Shadow,

    pub fill: Color32,

    pub stroke: Stroke,
}

impl ClickableFrame {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn frame(&self) -> Frame {
        Frame {
            inner_margin: self.inner_margin,
            outer_margin: self.outer_margin,
            rounding: self.rounding,
            shadow: self.shadow,
            fill: self.fill,
            stroke: self.stroke,
        }
    }

    pub fn group(style: &Style) -> Self {
        Self {
            rounding: style.visuals.widgets.noninteractive.rounding,
            stroke: style.visuals.widgets.noninteractive.bg_stroke,
            ..Default::default()
        }
    }

    pub fn side_top_panel(style: &Style) -> Self {
        Self {
            inner_margin: Margin::symmetric(8.0, 2.0),
            fill: style.visuals.panel_fill,
            ..Default::default()
        }
    }

    pub fn central_panel(style: &Style) -> Self {
        Self {
            inner_margin: Margin::same(8.0),
            fill: style.visuals.panel_fill,
            ..Default::default()
        }
    }

    pub fn window(style: &Style) -> Self {
        Self {
            inner_margin: style.spacing.window_margin,
            rounding: style.visuals.window_rounding,
            shadow: style.visuals.window_shadow,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    pub fn menu(style: &Style) -> Self {
        Self {
            inner_margin: style.spacing.menu_margin,
            rounding: style.visuals.menu_rounding,
            shadow: style.visuals.popup_shadow,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    pub fn popup(style: &Style) -> Self {
        Self {
            inner_margin: style.spacing.menu_margin,
            rounding: style.visuals.menu_rounding,
            shadow: style.visuals.popup_shadow,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    pub fn canvas(style: &Style) -> Self {
        Self {
            inner_margin: Margin::same(2.0),
            rounding: style.visuals.widgets.noninteractive.rounding,
            fill: style.visuals.extreme_bg_color,
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        }
    }

    pub fn dark_canvas(style: &Style) -> Self {
        Self {
            fill: Color32::from_black_alpha(250),
            ..Self::canvas(style)
        }
    }
}

impl ClickableFrame {
    #[inline]
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn rounding(mut self, rounding: impl Into<Rounding>) -> Self {
        self.rounding = rounding.into();
        self
    }

    #[inline]
    pub fn inner_margin(mut self, inner_margin: impl Into<Margin>) -> Self {
        self.inner_margin = inner_margin.into();
        self
    }

    #[inline]
    pub fn outer_margin(mut self, outer_margin: impl Into<Margin>) -> Self {
        self.outer_margin = outer_margin.into();
        self
    }

    #[inline]
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }

    #[inline]
    pub fn multiply_with_opacity(mut self, opacity: f32) -> Self {
        self.fill = self.fill.gamma_multiply(opacity);
        self.stroke.color = self.stroke.color.gamma_multiply(opacity);
        self.shadow.color = self.shadow.color.gamma_multiply(opacity);
        self
    }
}

impl ClickableFrame {
    #[inline]
    pub fn total_margin(&self) -> Margin {
        self.inner_margin + self.outer_margin
    }
}

pub struct Prepared {
    pub frame: ClickableFrame,

    where_to_put_background: ShapeIdx,

    pub content_ui: Ui,
}

impl ClickableFrame {
    pub fn begin(self, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let outer_rect_bounds = ui.available_rect_before_wrap();

        let mut inner_rect = outer_rect_bounds - self.outer_margin - self.inner_margin;

        inner_rect.max.x = inner_rect.max.x.max(inner_rect.min.x);
        inner_rect.max.y = inner_rect.max.y.max(inner_rect.min.y);

        let content_ui = ui.child_ui(
            inner_rect,
            *ui.layout(),
            Some(UiStackInfo::new(UiKind::Frame).with_frame(self.frame())),
        );

        Prepared {
            frame: self,
            where_to_put_background,
            content_ui,
        }
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        self.show_dyn(ui, Box::new(add_contents))
    }

    pub fn show_dyn<'c, R>(
        self,
        ui: &mut Ui,
        add_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    ) -> InnerResponse<R> {
        let mut prepared = self.begin(ui);
        let ret = add_contents(&mut prepared.content_ui);
        let response = prepared.end(ui);
        InnerResponse::new(ret, response)
    }

    pub fn paint(&self, outer_rect: Rect) -> Shape {
        let Self {
            inner_margin: _,
            outer_margin: _,
            rounding,
            shadow,
            fill,
            stroke,
        } = *self;

        let frame_shape = Shape::Rect(epaint::RectShape::new(outer_rect, rounding, fill, stroke));

        if shadow == Default::default() {
            frame_shape
        } else {
            let shadow = shadow.as_shape(outer_rect, rounding);
            Shape::Vec(vec![Shape::from(shadow), frame_shape])
        }
    }
}

impl Prepared {
    fn content_with_margin(&self) -> Rect {
        self.content_ui.min_rect() + self.frame.inner_margin + self.frame.outer_margin
    }

    pub fn allocate_space(&self, ui: &mut Ui) -> Response {
        ui.allocate_rect(self.content_with_margin(), Sense::click())
    }

    pub fn paint(&self, ui: &Ui) {
        let paint_rect = self.content_ui.min_rect() + self.frame.inner_margin;

        if ui.is_rect_visible(paint_rect) {
            let shape = self.frame.paint(paint_rect);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }

    pub fn end(self, ui: &mut Ui) -> Response {
        self.paint(ui);
        self.allocate_space(ui)
    }
}
