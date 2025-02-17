mod context;
mod inventory;
mod menus;
mod meta;
mod modals;
mod tabs;
mod tasks;
mod visuals;

use std::path::PathBuf;

use eframe::CreationContext;
use egui::{self, Key, KeyboardShortcut, Modifiers, UiStackInfo, Vec2, ViewportCommand};
use egui_dock::{DockArea, DockState};
use flume::{Receiver, Sender};
use once_cell::sync::Lazy;

use terra_core::{utils, Player};

use self::{
    context::{AppContext, Message},
    meta::platform_meta_loader,
    tabs::{default_ui, Tab},
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
pub enum AppMessage {
    Exit,
    ResetTabs,
    AddTab(Tab, egui_dock::SurfaceIndex, egui_dock::NodeIndex),
}

pub struct App {
    ctx: Sender<Message>,
    app_chan: (Sender<AppMessage>, Receiver<AppMessage>),
    context: AppContext,

    dock_state: DockState<Tab>,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let (atx, arx) = flume::unbounded();
        let (ctx, crx) = flume::unbounded();

        let (theme, dock_state) = match cc.storage {
            Some(s) => (
                eframe::get_value::<visuals::Theme>(s, THEME_KEY).unwrap_or_default(),
                eframe::get_value::<DockState<Tab>>(s, TREE_KEY).unwrap_or_else(tabs::default_ui),
            ),
            None => (Default::default(), default_ui()),
        };
        theme.set_theme(&cc.egui_ctx);

        let context = AppContext::new(
            ctx.clone(),
            crx.clone(),
            atx.clone(),
            theme,
            platform_meta_loader(),
        );

        Self {
            app_chan: (atx, arx),
            ctx,
            context,
            dock_state,
        }
    }

    fn ctx(&self) -> &Sender<Message> {
        &self.ctx
    }

    fn atx(&self) -> &Sender<AppMessage> {
        &self.app_chan.0
    }

    fn arx(&self) -> &Receiver<AppMessage> {
        &self.app_chan.1
    }

    fn send_app_msg(&self, msg: AppMessage) {
        self.atx().send(msg).unwrap();
    }

    fn send_context_msg(&self, msg: Message) {
        self.ctx().send(msg).unwrap();
    }

    fn handle_update(&mut self, ctx: &egui::Context) {
        while let Ok(msg) = self.arx().try_recv() {
            self.handle_message(ctx, msg);
        }
    }

    fn handle_message(&mut self, ctx: &egui::Context, msg: AppMessage) {
        match msg {
            AppMessage::Exit => ctx.send_viewport_cmd(ViewportCommand::Close),
            AppMessage::ResetTabs => {
                self.dock_state = default_ui();
            }
            AppMessage::AddTab(tab, surface, node) => {
                if let Some((surface, node, tab)) = self.dock_state.find_tab(&tab) {
                    self.dock_state
                        .set_focused_node_and_surface((surface, node));
                    self.dock_state.set_active_tab((surface, node, tab));
                } else {
                    self.dock_state
                        .set_focused_node_and_surface((surface, node));
                    self.dock_state.push_to_focused_leaf(tab);
                }
            }
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, THEME_KEY, &self.context.theme());
        eframe::set_value(storage, TREE_KEY, &self.dock_state);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_update(ctx);
        self.render_menu(ctx);
        self.context.update(ctx);

        let layer_id = egui::LayerId::background();
        let max_rect = ctx.available_rect();
        let clip_rect = ctx.available_rect();
        let id = egui::Id::new("dock_area");

        let mut ui = egui::Ui::new(
            ctx.clone(),
            layer_id,
            id,
            max_rect,
            clip_rect,
            UiStackInfo::default(),
        );

        // let mut ui = egui::Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);

        ui.spacing_mut().item_spacing = Vec2::splat(8.);
        if self.context.is_modal_open() {
            ui.disable();
        }

        let dock_style = egui_dock::Style::from_egui(&ctx.style());

        DockArea::new(&mut self.dock_state)
            .style(dock_style)
            .show_tab_name_on_hover(true)
            .show_add_buttons(true)
            .show_add_popup(true)
            .show_inside(&mut ui, &mut self.context);
    }
}
