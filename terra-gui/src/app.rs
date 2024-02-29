mod context;
mod inventory;
mod menus;
mod modals;
mod tabs;
mod tasks;
mod visuals;

use std::path::PathBuf;

use eframe::CreationContext;
use egui::{self, Key, KeyboardShortcut, Modifiers, Vec2, ViewportCommand};
use egui_dock::{DockArea, DockState, NodeIndex};
use flume::Receiver;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use terra_core::{utils, Player};

use self::{
    context::{AppContext, Message},
    tabs::{default_ui, Tabs},
};

pub const GITHUB_REPO_NAME: &str = "RLGingerBiscuit/terra-rs";
pub const GITHUB_REPO_URL: &str = "https://github.com/RLGingerBiscuit/terra-rs";
pub const EGUI_GITHUB_REPO_NAME: &str = "emilk/egui";
pub const EGUI_GITHUB_REPO_URL: &str = "https://github.com/emilk/egui";

pub const THEME_KEY: &str = "theme";
pub const TREE_KEY: &str = "tree";
pub const CLOSED_TABS_KEY: &str = "closed_tabs";

static SHORTCUT_LOAD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::O);
static SHORTCUT_SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
static SHORTCUT_EXIT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

static DEFAULT_PLAYER_DIR: Lazy<PathBuf> = Lazy::new(utils::get_player_dir);

static DEFAULT_PLAYER: Lazy<Player> = Lazy::new(Player::default);

#[derive(Debug)]
pub enum AppMessage {
    ResetTabs,
    Exit,
}

pub struct App {
    crx: Receiver<AppMessage>,
    context: AppContext,

    dock_state: DockState<Tabs>,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let (tx, rx) = flume::unbounded();
        let (ctx, crx) = flume::unbounded();

        let (theme, dock_state, closed_tabs) = match cc.storage {
            Some(s) => (
                eframe::get_value::<visuals::Theme>(s, THEME_KEY).unwrap_or_default(),
                eframe::get_value::<DockState<Tabs>>(s, TREE_KEY).unwrap_or_else(tabs::default_ui),
                eframe::get_value::<FxHashMap<Tabs, NodeIndex>>(s, CLOSED_TABS_KEY)
                    .unwrap_or_default(),
            ),
            None => (Default::default(), default_ui(), Default::default()),
        };
        theme.set_theme(&cc.egui_ctx);

        let context = AppContext::new(tx, rx, ctx, theme, closed_tabs);

        Self {
            crx,
            context,
            dock_state,
        }
    }

    fn handle_update(&mut self, ctx: &egui::Context) {
        if let Ok(msg) = self.crx.try_recv() {
            self.handle_message(ctx, msg);
        }
    }

    fn handle_message(&mut self, ctx: &egui::Context, msg: AppMessage) {
        match msg {
            AppMessage::ResetTabs => {
                self.dock_state = default_ui();
            }
            AppMessage::Exit => ctx.send_viewport_cmd(ViewportCommand::Close),
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, THEME_KEY, &self.context.theme());
        eframe::set_value(storage, TREE_KEY, &self.dock_state);
        eframe::set_value(storage, CLOSED_TABS_KEY, self.context.closed_tabs());
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_update(ctx);
        self.context.update(ctx);

        let layer_id = egui::LayerId::background();
        let max_rect = ctx.available_rect();
        let clip_rect = ctx.available_rect();
        let id = egui::Id::new("dock_area");

        let mut ui = egui::Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);

        ui.spacing_mut().item_spacing = Vec2::splat(8.);
        ui.set_enabled(!self.context.is_modal_open());

        DockArea::new(&mut self.dock_state)
            .style(egui_dock::Style::from_egui(&ctx.style()))
            .show_tab_name_on_hover(true)
            .show_add_popup(true)
            .show_inside(&mut ui, &mut self.context);
    }
}
