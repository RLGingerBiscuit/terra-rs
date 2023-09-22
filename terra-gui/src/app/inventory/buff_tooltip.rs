use egui::{Response, Ui, Vec2, Widget};
use terra_core::BuffMeta;

use super::{buff_name, buff_slot::BuffSlotOptions};

#[derive(Debug, Clone, Copy)]
pub struct BuffTooltipOptions {
    pub id: i32,
    pub time: Option<i32>,
}

#[allow(dead_code)]
impl BuffTooltipOptions {
    pub fn new() -> Self {
        Self { id: 0, time: None }
    }

    pub fn from_slot_options(options: &BuffSlotOptions) -> Self {
        Self {
            id: options.id,
            time: options.time,
        }
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn time(mut self, time: Option<i32>) -> Self {
        self.time = time;
        self
    }
}

#[derive(Debug)]
pub(super) struct BuffTooltip<'a> {
    options: BuffTooltipOptions,
    meta: &'a BuffMeta,
}

impl<'a> BuffTooltip<'a> {
    pub fn new(options: BuffTooltipOptions, meta: &'a BuffMeta) -> Self {
        Self { options, meta }
    }
}

impl<'a> Widget for BuffTooltip<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.allocate_ui(Vec2::INFINITY, |ui| {
            let buff = self.meta;
            if buff.id == 0 {
                return;
            }

            let time = self.options.time;

            ui.heading(buff_name(&buff.name, time));

            ui.small(format!("ID: {}", buff.id));

            if let Some(tooltip) = &buff.tooltip {
                for line in tooltip {
                    ui.label(line);
                }
            }
        })
        .response
    }
}
