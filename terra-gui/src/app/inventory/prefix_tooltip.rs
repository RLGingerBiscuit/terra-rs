use egui::{Response, Ui, Vec2, Widget};
use terra_core::PrefixMeta;

#[derive(Debug, Clone, Copy)]
pub struct PrefixTooltipOptions {
    pub id: u8,
}

impl PrefixTooltipOptions {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn id(mut self, id: u8) -> Self {
        self.id = id;
        self
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct PrefixTooltip<'a> {
    options: PrefixTooltipOptions,
    meta: &'a PrefixMeta,
}

impl<'a> PrefixTooltip<'a> {
    pub fn new(options: PrefixTooltipOptions, meta: &'a PrefixMeta) -> Self {
        Self { options, meta }
    }
}

impl<'a> Widget for PrefixTooltip<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.allocate_ui(Vec2::INFINITY, |ui| {
            let prefix = self.meta;
            if prefix.id == 0 {
                return;
            }

            ui.heading(prefix.name.as_ref());
            ui.small(format!("Id: {}", prefix.id));
        })
        .response
    }
}
