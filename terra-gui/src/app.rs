mod menus;
mod modals;

use eframe::CreationContext;
use egui;
use flume::{Receiver, Sender};

pub const GITHUB_REPO_NAME: &str = "Hub-of-Cringe-Nerds/RLGingerBiscuit-terra-rs";
pub const GITHUB_REPO_URL: &str = "https://github.com/Hub-of-Cringe-Nerds/RLGingerBiscuit-terra-rs";

#[allow(dead_code)]
#[derive(Debug)]
pub enum Message {
    Noop,
    Exit,
    ShowAbout,
    CloseAbout,
    ShowError(anyhow::Error),
    CloseError,
}

pub struct App {
    channel: (Sender<Message>, Receiver<Message>),

    error: Option<anyhow::Error>,
    show_about: bool,
}

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        let (tx, rx) = flume::unbounded();

        Self {
            channel: (tx, rx),

            error: None,
            show_about: false,
        }
    }

    fn do_update(&self, msg: Message) {
        self.channel.0.send(msg).unwrap();
    }

    fn handle_update(&mut self, _ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(msg) = self.channel.1.try_recv() {
            match msg {
                Message::Noop => {}
                Message::Exit => frame.close(),
                Message::ShowAbout => self.show_about = true,
                Message::CloseAbout => self.show_about = false,
                Message::ShowError(err) => self.error = Some(err),
                Message::CloseError => self.error = None,
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_update(ctx, frame);

        self.render_menu(ctx);
        self.render_about(ctx);
        self.render_error(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Hello World!");
            });
        });
    }
}
