mod app;
mod ext;

use app::App;

fn main() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native("terra-rs", options, Box::new(|cc| Box::new(App::new(cc)))).ok();
}
