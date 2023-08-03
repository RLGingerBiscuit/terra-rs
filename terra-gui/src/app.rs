mod inventory;
mod menus;
mod modals;
mod tabs;
mod tasks;
mod visuals;

use std::{ops::DerefMut, path::PathBuf, sync::Arc, thread};

use eframe::CreationContext;
use egui::{self, Id, Key, KeyboardShortcut, LayerId, Modifiers, TextureHandle, Ui, Vec2};
use egui_dock::{DockArea, NodeIndex, Tree};
use flume::{Receiver, Sender};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;

use terra_core::{utils, BuffMeta, ItemMeta, Player, PrefixMeta, ResearchItem};

use crate::{selected_buff, selected_item};

use self::{
    inventory::{ItemTab, SelectedBuff, SelectedItem, SelectedLoadout},
    tabs::Tabs,
};

pub const GITHUB_REPO_NAME: &str = "RLGingerBiscuit/terra-rs";
pub const GITHUB_REPO_URL: &str = "https://github.com/RLGingerBiscuit/terra-rs";
pub const EGUI_GITHUB_REPO_NAME: &str = "emilk/egui";
pub const EGUI_GITHUB_REPO_URL: &str = "https://github.com/emilk/egui";

pub const THEME_KEY: &str = "theme";
pub const TREE_KEY: &str = "tree";

static SHORTCUT_LOAD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::O);
static SHORTCUT_SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
static SHORTCUT_EXIT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

static DEFAULT_PLAYER_DIR: Lazy<PathBuf> = Lazy::new(utils::get_player_dir);

static DEFAULT_PLAYER: Lazy<Player> = Lazy::new(Player::default);

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
    SetTheme(visuals::Theme),
    ResetPlayer,
    LoadPlayer,
    SavePlayer,
    SelectItem(SelectedItem),
    SelectBuff(SelectedBuff),
    RemoveAllResearch,
    AddAllResearch,
    OpenItemBrowser,
    CloseItemBrowser,
    OpenBuffBrowser,
    CloseBuffBrowser,
    OpenPrefixBrowser,
    ClosePrefixBrowser,
    SetCurrentItemId(i32),
    SetCurrentBuffId(i32),
    SetCurrentPrefixId(u8),
}

pub struct App {
    player: Arc<RwLock<Player>>,
    player_path: Option<PathBuf>,

    selected_item: SelectedItem,
    selected_buff: SelectedBuff,
    selected_loadout: SelectedLoadout,

    channel: (Sender<Message>, Receiver<Message>),

    prefix_meta: Arc<RwLock<Vec<PrefixMeta>>>,
    item_meta: Arc<RwLock<Vec<ItemMeta>>>,
    buff_meta: Arc<RwLock<Vec<BuffMeta>>>,

    item_spritesheet: Arc<RwLock<Option<TextureHandle>>>,
    buff_spritesheet: Arc<RwLock<Option<TextureHandle>>>,

    theme: visuals::Theme,
    tree: Arc<RwLock<Tree<Tabs>>>,
    closed_tabs: FxHashMap<Tabs, NodeIndex>,

    search_term: String,

    error: Option<anyhow::Error>,
    busy: bool,
    show_about: bool,
    show_item_browser: bool,
    show_buff_browser: bool,
    show_prefix_browser: bool,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let (tx, rx) = flume::unbounded();

        let prefix_meta = PrefixMeta::load().expect("Could not load prefixes");
        let item_meta = ItemMeta::load().expect("Could not load items");
        let buff_meta = BuffMeta::load().expect("Could not load buffs");

        let (theme, tree) = match cc.storage {
            Some(s) => (
                eframe::get_value::<visuals::Theme>(s, THEME_KEY).unwrap_or_default(),
                eframe::get_value(s, TREE_KEY).unwrap_or_else(tabs::default_ui),
            ),
            None => (Default::default(), Default::default()),
        };
        theme.set_theme(&cc.egui_ctx);

        Self {
            player: Arc::new(RwLock::new(Player::default())),
            player_path: None,

            selected_item: SelectedItem(ItemTab::Inventory, 0),
            selected_buff: SelectedBuff(0),
            selected_loadout: SelectedLoadout(0),

            channel: (tx, rx),

            prefix_meta: Arc::new(RwLock::new(prefix_meta)),
            item_meta: Arc::new(RwLock::new(item_meta)),
            buff_meta: Arc::new(RwLock::new(buff_meta)),

            item_spritesheet: Arc::new(RwLock::new(None)),
            buff_spritesheet: Arc::new(RwLock::new(None)),

            theme,
            tree: Arc::new(RwLock::new(tree)),
            closed_tabs: FxHashMap::default(),

            search_term: String::new(),

            error: None,
            busy: false,
            show_about: false,
            show_item_browser: false,
            show_buff_browser: false,
            show_prefix_browser: false,
        }
    }

    fn modal_open(&self) -> bool {
        self.busy
            || self.error.is_some()
            || self.show_about
            || self.show_item_browser
            || self.show_buff_browser
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
                Message::Noop => {
                    self.busy = false;
                    ctx.request_repaint();
                }
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
                Message::SetTheme(theme) => {
                    theme.set_theme(ctx);
                    self.theme = theme;
                }
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
                            player.write().load(&item_meta.read(), &path)?;
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
                        self.player_path = Some(path.clone());

                        self.do_task(move || {
                            player.read().save(&item_meta.read(), &path)?;
                            Ok(Message::Noop)
                        });
                    }
                }
                Message::SelectItem(selection) => self.selected_item = selection,
                Message::SelectBuff(selection) => self.selected_buff = selection,
                Message::RemoveAllResearch => {
                    let mut player = self.player.write();
                    player.research.clear();
                }
                Message::AddAllResearch => {
                    let mut player = self.player.write();
                    let item_meta = self.item_meta.read();

                    // TODO: Maybe remove this at some point?
                    player.research.clear();
                    for item in &*item_meta {
                        if item.forbidden.is_none() {
                            player.research.push(ResearchItem {
                                internal_name: item.internal_name.to_owned(),
                                stack: item.sacrifices,
                            });
                        }
                    }
                }
                Message::OpenItemBrowser => self.show_item_browser = true,
                Message::CloseItemBrowser => {
                    self.search_term.clear();
                    self.show_item_browser = false;
                }
                Message::OpenBuffBrowser => self.show_buff_browser = true,
                Message::CloseBuffBrowser => {
                    self.search_term.clear();
                    self.show_buff_browser = false;
                }
                Message::OpenPrefixBrowser => self.show_prefix_browser = true,
                Message::ClosePrefixBrowser => {
                    self.search_term.clear();
                    self.show_prefix_browser = false;
                }
                Message::SetCurrentItemId(id) => {
                    let mut player = self.player.write();
                    let mut selected_item =
                        selected_item!(self.selected_item, self.selected_loadout, player);

                    selected_item.id = id;

                    if selected_item.stack == 0 {
                        selected_item.stack = 1;
                    }

                    if self.show_item_browser {
                        self.search_term.clear();
                        self.show_item_browser = false;
                    }
                }
                Message::SetCurrentBuffId(id) => {
                    let mut player = self.player.write();
                    let mut selected_buff = selected_buff!(self.selected_buff, player);

                    selected_buff.id = id;

                    if selected_buff.time == 0 {
                        // TODO: utils::duration_to_ticks() ?
                        // 15 minutes
                        selected_buff.time = 54000;
                    }

                    if self.show_buff_browser {
                        self.search_term.clear();
                        self.show_buff_browser = false;
                    }
                }
                Message::SetCurrentPrefixId(id) => {
                    let mut player = self.player.write();
                    let mut selected_item =
                        selected_item!(self.selected_item, self.selected_loadout, player);

                    selected_item.prefix.id = id;

                    if self.show_prefix_browser {
                        self.search_term.clear();
                        self.show_prefix_browser = false;
                    }
                }
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
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, THEME_KEY, &self.theme);
        eframe::set_value(storage, TREE_KEY, &*self.tree.read());
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_update(ctx, frame);
        self.handle_keyboard(ctx);

        self.render_menu(ctx);

        self.render_about(ctx);
        self.render_error(ctx);

        self.render_item_browser(ctx);
        self.render_buff_browser(ctx);
        self.render_prefix_browser(ctx);

        let layer_id = LayerId::background();
        let max_rect = ctx.available_rect();
        let clip_rect = ctx.available_rect();
        let id = Id::new("dock_area");

        let mut ui = Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);

        ui.spacing_mut().item_spacing = Vec2::splat(8.);

        // TODO: This isn't working for some reason
        ui.add_enabled_ui(!self.modal_open(), |ui| {
            DockArea::new(self.tree.clone().write().deref_mut())
                .style(egui_dock::Style::from_egui(&ctx.style()))
                .show_tab_name_on_hover(true)
                .show_add_popup(true)
                .show_inside(ui, self);
        });
    }
}
