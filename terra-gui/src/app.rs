use eframe::{CreationContext};

pub struct App;

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        Self
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Hello World!");
            });
        });
    }
}
