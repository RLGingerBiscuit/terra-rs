#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::vec2;

mod app;
mod ui;

use app::App;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(800., 600.)),
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    eframe::run_native("terra-rs", options, Box::new(|cc| Box::new(App::new(cc)))).ok();
}
