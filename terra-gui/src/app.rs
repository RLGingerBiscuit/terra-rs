mod menus;
mod modals;
mod tasks;

use std::{path::PathBuf, sync::Arc, thread};

use anyhow::anyhow;
use eframe::CreationContext;
use egui::{
    self, Color32, Image, Key, KeyboardShortcut, Modifiers, Pos2, Rect, TextureHandle, Vec2,
};
use flume::{Receiver, Sender};
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};

use terra_core::{utils, BuffMeta, ItemMeta, Player, PrefixMeta, BUFF_SPRITE_SIZE};

pub const GITHUB_REPO_NAME: &str = "Hub-of-Cringe-Nerds/RLGingerBiscuit-terra-rs";
pub const GITHUB_REPO_URL: &str = "https://github.com/Hub-of-Cringe-Nerds/RLGingerBiscuit-terra-rs";
pub const EGUI_GITHUB_REPO_NAME: &str = "emilk/egui";
pub const EGUI_GITHUB_REPO_URL: &str = "https://github.com/emilk/egui";

static SHORTCUT_LOAD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::O);
static SHORTCUT_SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
static SHORTCUT_EXIT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

static DEFAULT_PLAYER_DIR: Lazy<PathBuf> = Lazy::new(|| utils::get_player_dir());

static DEFAULT_PLAYER: Lazy<Player> = Lazy::new(|| Player::default());

#[allow(dead_code)]
#[derive(Debug)]
pub enum Message {
    Noop,
    Exit,
    LoadItemSpritesheet,
    LoadBuffSpritesheet,
    ShowAbout,
    CloseAbout,
    ShowError(anyhow::Error),
    CloseError,
    ResetPlayer,
    LoadPlayer,
    SavePlayer,
}

#[allow(dead_code)]
pub struct App {
    player: Arc<RwLock<Player>>,
    player_path: Option<PathBuf>,

    channel: (Sender<Message>, Receiver<Message>),

    prefix_meta: Arc<Vec<PrefixMeta>>,
    item_meta: Arc<Vec<ItemMeta>>,
    buff_meta: Arc<Vec<BuffMeta>>,

    item_spritesheet: Arc<Mutex<Option<TextureHandle>>>,
    buff_spritesheet: Arc<Mutex<Option<TextureHandle>>>,

    error: Option<anyhow::Error>,
    busy: bool,
    show_about: bool,
}

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        let (tx, rx) = flume::unbounded();

        let prefix_meta = PrefixMeta::load().expect("Could not load prefixes");
        let item_meta = ItemMeta::load().expect("Could not load items");
        let buff_meta = BuffMeta::load().expect("Could not load buffs");

        Self {
            player: Arc::new(RwLock::new(Player::default())),
            player_path: None,

            channel: (tx, rx),

            prefix_meta: Arc::new(prefix_meta),
            item_meta: Arc::new(item_meta),
            buff_meta: Arc::new(buff_meta),

            item_spritesheet: Arc::new(Mutex::new(None)),
            buff_spritesheet: Arc::new(Mutex::new(None)),

            error: None,
            busy: false,
            show_about: false,
        }
    }

    fn modal_open(&self) -> bool {
        self.busy || self.error.is_some()
    }

    fn do_task(&mut self, task: impl 'static + Send + Sync + FnOnce() -> anyhow::Result<Message>) {
        let tx = self.channel.0.clone();
        let task = Box::new(task);
        self.busy = true;
        thread::spawn(move || {
            tx.send(match task() {
                Ok(msg) => msg,
                Err(err) => Message::ShowError(err),
            })
            .unwrap();
        });
    }

    fn do_update(&self, msg: Message) {
        self.channel.0.send(msg).unwrap();
    }

    fn handle_update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(msg) = self.channel.1.try_recv() {
            match msg {
                Message::Noop => self.busy = false,
                Message::Exit => frame.close(),
                Message::LoadItemSpritesheet => {
                    {
                        let spritesheet = self.item_spritesheet.lock();
                        if (*spritesheet).is_some() {
                            self.do_update(Message::ShowError(anyhow!(
                                "Item sprites should only be loaded once."
                            )));
                            return;
                        }
                    }
                    let spritesheet = self.item_spritesheet.clone();
                    self.load_spritesheet(ctx, "items.png", spritesheet);
                }
                Message::LoadBuffSpritesheet => {
                    {
                        let spritesheet = self.buff_spritesheet.lock();
                        if (*spritesheet).is_some() {
                            self.do_update(Message::ShowError(anyhow!(
                                "Buff sprites should only be loaded once."
                            )));
                            return;
                        }
                    }
                    let spritesheet = self.buff_spritesheet.clone();
                    self.load_spritesheet(ctx, "buffs.png", spritesheet);
                }
                Message::ShowAbout => self.show_about = true,
                Message::CloseAbout => self.show_about = false,
                Message::ShowError(err) => {
                    self.busy = false;
                    self.error = Some(err)
                }
                Message::CloseError => self.error = None,
                Message::ResetPlayer => self.player.write().clone_from(&DEFAULT_PLAYER),
                Message::LoadPlayer => {
                    let player_path = self
                        .player_path
                        .get_or_insert_with(|| DEFAULT_PLAYER_DIR.clone());

                    let player_path = if player_path.is_dir() {
                        player_path.clone()
                    } else {
                        match player_path.parent() {
                            Some(directory) => directory.to_path_buf(),
                            None => DEFAULT_PLAYER_DIR.clone(),
                        }
                    };

                    if let Some(path) = rfd::FileDialog::new()
                        .set_directory(player_path)
                        .add_filter("Terraria Player File", &["plr"])
                        .add_filter("All Files", &["*"])
                        .pick_file()
                    {
                        self.player_path = Some(path.clone());

                        let player = self.player.clone();
                        let item_meta = self.item_meta.clone();

                        self.do_task(move || {
                            player.write().load(&item_meta, &path)?;
                            Ok(Message::Noop)
                        });
                    }
                }
                Message::SavePlayer => {
                    let player_path = self
                        .player_path
                        .get_or_insert_with(|| DEFAULT_PLAYER_DIR.clone());

                    let (directory, filename) = if player_path.is_dir() {
                        (player_path.clone(), self.player.read().name.clone())
                    } else {
                        let directory = match player_path.parent() {
                            Some(directory) => directory.to_path_buf(),
                            None => DEFAULT_PLAYER_DIR.clone(),
                        };
                        let filename = match player_path.file_name() {
                            Some(file_name) => file_name.to_string_lossy().to_string(),
                            None => self.player.read().name.clone(),
                        };

                        (directory, filename)
                    };

                    let player = self.player.clone();
                    let item_meta = self.item_meta.clone();

                    if let Some(path) = rfd::FileDialog::new()
                        .set_directory(directory)
                        .set_file_name(&filename)
                        .add_filter("Terraria Player File", &["plr"])
                        .add_filter("All Files", &["*"])
                        .save_file()
                    {
                        self.do_task(move || {
                            player.read().save(&item_meta, &path)?;
                            Ok(Message::Noop)
                        });
                    }
                }
            }
        }
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|input| {
            if input.consume_shortcut(&SHORTCUT_LOAD) {
                self.do_update(Message::LoadPlayer);
            }
            if input.consume_shortcut(&SHORTCUT_SAVE) {
                self.do_update(Message::SavePlayer);
            }
            if input.consume_shortcut(&SHORTCUT_EXIT) {
                self.do_update(Message::Exit);
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_update(ctx, frame);
        self.handle_keyboard(ctx);

        self.render_menu(ctx);
        self.render_about(ctx);
        self.render_error(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 8.;

            ui.add_enabled_ui(!self.modal_open(), |ui| {
                let player = self.player.read();

                ui.heading(format!("Name: {}", player.name));

                ui.horizontal(|ui| {
                    if ui.button("Load Player").clicked() {
                        self.do_update(Message::LoadPlayer);
                    }
                    if ui.button("Save Player").clicked() {
                        self.do_update(Message::SavePlayer);
                    }
                    if ui.button("Reset Player").clicked() {
                        self.do_update(Message::ResetPlayer);
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Load Item Spritesheet").clicked() {
                        self.do_update(Message::LoadItemSpritesheet);
                    }
                    if ui.button("Load Buff Spritesheet").clicked() {
                        self.do_update(Message::LoadBuffSpritesheet);
                    }
                });

                ui.horizontal(|ui| {
                    {
                        let spritesheet = self.item_spritesheet.lock();

                        if let Some(spritesheet) = &*spritesheet {
                            let spritesheet_size = spritesheet.size().map(|s| s as f32);

                            let sprite = self.item_meta.get(426).unwrap();
                            let width = sprite.width as f32;
                            let height = sprite.height as f32;
                            let x = sprite.x as f32;
                            let y = sprite.y as f32;

                            let min = Pos2::new(x / spritesheet_size[0], y / spritesheet_size[1]);
                            let sprite_size = Vec2::new(
                                width / spritesheet_size[0],
                                height / spritesheet_size[1],
                            );
                            let uv = Rect::from_min_size(min, sprite_size);

                            let size = Vec2::new(width, height) * 4.;

                            ui.vertical(|ui| {
                                ui.label("Item Sprite");
                                ui.add(
                                    Image::new(spritesheet, size)
                                        .uv(uv)
                                        .bg_fill(Color32::LIGHT_GRAY),
                                );
                            });
                        }
                    }

                    {
                        let spritesheet = self.buff_spritesheet.lock();

                        if let Some(spritesheet) = &*spritesheet {
                            let spritesheet_size = spritesheet.size().map(|s| s as f32);

                            let sprite = self.buff_meta.get(1).unwrap();
                            let width = BUFF_SPRITE_SIZE as f32;
                            let height = BUFF_SPRITE_SIZE as f32;
                            let x = sprite.x as f32;
                            let y = sprite.y as f32;

                            let min = Pos2::new(x / spritesheet_size[0], y / spritesheet_size[1]);
                            let sprite_size = Vec2::new(
                                width / spritesheet_size[0],
                                height / spritesheet_size[1],
                            );
                            let uv = Rect::from_min_size(min, sprite_size);

                            let size = Vec2::new(width, height) * 4.;

                            ui.vertical(|ui| {
                                ui.label("Buff Sprite");
                                ui.add(Image::new(spritesheet, size).uv(uv));
                            });
                        }
                    }
                });
            });
        });
    }
}
