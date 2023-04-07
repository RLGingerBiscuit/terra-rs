use std::sync::Arc;

use egui::{ColorImage, TextureHandle, TextureOptions};
use parking_lot::RwLock;

use super::{App, Message};

impl App {
    pub fn load_spritesheet(
        &mut self,
        ctx: &egui::Context,
        filename: &str,
        spritesheet: Arc<RwLock<Option<TextureHandle>>>,
    ) {
        if self.busy {
            return;
        }
        let ctx = ctx.clone();
        let debug_name = format!("{}_spritesheet", filename);
        let path = std::env::current_exe()
            .expect("No current exe?")
            .parent()
            .expect("No parent?")
            .join("resources")
            .join(filename);

        self.do_task(move || {
            let image = image::open(&path)?;
            let rgba = image.as_rgba8().unwrap();

            let image =
                ColorImage::from_rgba_unmultiplied([image.width() as _, image.height() as _], rgba);

            println!("{}: {}x{}", path.display(), image.width(), image.height());

            let handle = ctx.load_texture(debug_name, image, TextureOptions::NEAREST);

            let mut spritesheet = spritesheet.write();
            *spritesheet = Some(handle);

            ctx.request_repaint();
            Ok(Message::Noop)
        });
    }
}
