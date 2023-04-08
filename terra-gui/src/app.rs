mod inventory;
mod menus;
mod modals;
mod tabs;
mod tasks;

use std::{ops::DerefMut, path::PathBuf, sync::Arc, thread};

use eframe::CreationContext;
use egui::{self, Id, Key, KeyboardShortcut, LayerId, Modifiers, TextureHandle, Ui};
use egui_dock::{DockArea, NodeIndex, StyleBuilder, Tree};
use flume::{Receiver, Sender};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;

use terra_core::{utils, BuffMeta, ItemMeta, Player, PrefixMeta};

use self::{
    inventory::{SelectedBuff, SelectedItem},
    tabs::{default_ui, Tabs},
};

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
    SelectItem(SelectedItem),
    SelectBuff(SelectedBuff),
}

#[allow(dead_code)]
pub struct App {
    player: Arc<RwLock<Player>>,
    player_path: Option<PathBuf>,

    selected_item: SelectedItem,
    selected_buff: SelectedBuff,

    channel: (Sender<Message>, Receiver<Message>),

    prefix_meta: Vec<PrefixMeta>,
    item_meta: Vec<ItemMeta>,
    buff_meta: Vec<BuffMeta>,

    item_spritesheet: Arc<RwLock<Option<TextureHandle>>>,
    buff_spritesheet: Arc<RwLock<Option<TextureHandle>>>,

    tree: Arc<RwLock<Tree<Tabs>>>,
    closed_tabs: FxHashMap<Tabs, NodeIndex>,

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

            selected_item: SelectedItem::Inventory(0),
            selected_buff: SelectedBuff(0),

            channel: (tx, rx),

            prefix_meta,
            item_meta,
            buff_meta,

            item_spritesheet: Arc::new(RwLock::new(None)),
            buff_spritesheet: Arc::new(RwLock::new(None)),

            tree: Arc::new(RwLock::new(default_ui())),
            closed_tabs: FxHashMap::default(),

            error: None,
            busy: false,
            show_about: false,
        }
    }

    fn modal_open(&self) -> bool {
        self.busy || self.error.is_some() || self.show_about
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
                        let spritesheet = self.item_spritesheet.read();
                        if self.busy || (*spritesheet).is_some() {
                            return;
                        }
                    }
                    let spritesheet = self.item_spritesheet.clone();
                    self.load_spritesheet(ctx, "items.png", spritesheet);
                }
                Message::LoadBuffSpritesheet => {
                    {
                        let spritesheet = self.buff_spritesheet.read();
                        if self.busy || (*spritesheet).is_some() {
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
                Message::SelectItem(selection) => self.selected_item = selection,
                Message::SelectBuff(selection) => self.selected_buff = selection,
            }
        }
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        if !self.modal_open() {
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_update(ctx, frame);
        self.handle_keyboard(ctx);

        self.render_menu(ctx);
        self.render_about(ctx);
        self.render_error(ctx);

        let layer_id = LayerId::background();
        let max_rect = ctx.available_rect();
        let clip_rect = ctx.available_rect();
        let id = Id::new("dock_area");
        let mut ui = Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);

        ui.spacing_mut().item_spacing = [8.0, 8.0].into();
        ui.set_enabled(!self.modal_open());

        DockArea::new(self.tree.clone().write().deref_mut())
            .style(
                StyleBuilder::from_egui(&ctx.style())
                    .show_close_buttons(false)
                    .build(),
            )
            .show_inside(&mut ui, self);
    }
}
