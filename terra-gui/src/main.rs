mod app;
mod ui;

use egui::Vec2;

use app::App;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::splat(800.)),
        ..Default::default()
    };

    eframe::run_native("terra-rs", options, Box::new(|cc| Box::new(App::new(cc)))).ok();
}
