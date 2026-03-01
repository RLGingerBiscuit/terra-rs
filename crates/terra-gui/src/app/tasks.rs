use std::{future::Future, sync::Arc};

use egui::{ColorImage, TextureHandle, TextureOptions};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use crate::app::context::Message;

use super::context::AppContext;

pub(crate) fn do_task(
    tx: flume::Sender<Message>,
    busy: &Arc<RwLock<bool>>,
    task: impl 'static + Send + Future<Output = anyhow::Result<Message>>,
) {
    let busy = busy.clone();
    *busy.write() = true;

    #[cfg(not(target_arch = "wasm32"))]
    tokio::spawn(async move {
        match task.await {
            Ok(msg) => tx.send(msg).unwrap(),
            Err(err) => tx.send(Message::ShowError(err)).unwrap(),
        }
        *busy.write() = false;
    });
}

impl AppContext {
    pub fn load_spritesheet(
        &mut self,
        ctx: &egui::Context,
        file_name: &str,
        spritesheet: Arc<OnceCell<TextureHandle>>,
    ) {
        if self.is_busy() || spritesheet.get().is_some() {
            return;
        }

        let ctx = ctx.clone();
        let debug_name = format!("{}_spritesheet", file_name);
        let path = std::env::current_exe()
            .expect("No current exe?")
            .parent()
            .expect("No parent?")
            .join("resources")
            .join(file_name);

        self.do_task(async move {
            let image = image::open(&path)?;
            let rgba = image.as_rgba8().unwrap();

            let image =
                ColorImage::from_rgba_unmultiplied([image.width() as _, image.height() as _], rgba);

            println!("{}: {}x{}", path.display(), image.width(), image.height());

            let handle = ctx.load_texture(debug_name, image, TextureOptions::NEAREST);

            spritesheet.set(handle).ok();

            ctx.request_repaint();
            Ok(Message::Noop)
        });
    }
}
