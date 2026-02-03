#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui::vec2;

use crate::app::App;

pub mod app;
pub mod ui;

fn main() {
    let viewport = egui::ViewportBuilder::default().with_inner_size(vec2(1000., 750.));

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "terra-rs",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .ok();
}
