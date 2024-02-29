#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
mod ui;

use egui::vec2;

use app::App;

fn main() {
    let viewport = egui::ViewportBuilder::default().with_inner_size(vec2(800., 600.));

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    eframe::run_native("terra-rs", options, Box::new(|cc| Box::new(App::new(cc)))).ok();
}
