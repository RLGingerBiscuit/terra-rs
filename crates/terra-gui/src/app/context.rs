use std::{future::Future, path::PathBuf, rc::Rc, sync::Arc};

use egui::{mutex::RwLock, Key, Modifiers, TextureHandle};
use flume::{Receiver, Sender};

use terra_core::{utils::AsTicks, BuffMeta, ItemMeta, Player, PrefixMeta, ResearchItem};
use time::Duration;

use super::{
    inventory::{
        selected_buff, selected_item, ItemGroup, SelectedBuff, SelectedItem, SelectedLoadout,
    },
    meta::MetaLoader,
    visuals, AppMessage, DEFAULT_PLAYER, DEFAULT_PLAYER_DIR, SHORTCUT_EXIT, SHORTCUT_LOAD,
    SHORTCUT_SAVE,
};

#[derive(Debug)]
pub enum Message {
    Noop,
    LoadItemSpritesheet,
    LoadBuffSpritesheet,
    LoadIconSpritesheet,
    ShowAbout,
    CloseAbout,
    ShowError(anyhow::Error),
    CloseError,
    SetTheme(visuals::Theme),
    ResetPlayer,
    LoadPlayer,
    SavePlayer,
    SelectLoadout(SelectedLoadout),
    SelectItem(SelectedItem),
    SelectBuff(SelectedBuff),
    AddAllResearch,
    RemoveAllResearch,
    ToggleResearchItem(i32),
    OpenItemBrowser,
    CloseItemBrowser,
    OpenBuffBrowser,
    CloseBuffBrowser,
    OpenPrefixBrowser,
    ClosePrefixBrowser,
    OpenResearchBrowser,
    CloseResearchBrowser,
    SetCurrentItemId(i32),
    SetCurrentBuffId(i32),
    SetCurrentPrefixId(u8),
}

pub struct AppContext {
    chan: (Sender<Message>, Receiver<Message>),
    app_tx: Sender<AppMessage>,

    pub player: Arc<RwLock<Player>>,
    pub player_path: Arc<RwLock<Option<PathBuf>>>,

    pub selected_item: SelectedItem,
    pub selected_buff: SelectedBuff,
    pub selected_loadout: SelectedLoadout,

    pub prefix_meta: Arc<RwLock<Vec<PrefixMeta>>>,
    pub item_meta: Arc<RwLock<Vec<ItemMeta>>>,
    pub buff_meta: Arc<RwLock<Vec<BuffMeta>>>,

    pub item_spritesheet: Arc<RwLock<Option<TextureHandle>>>,
    pub buff_spritesheet: Arc<RwLock<Option<TextureHandle>>>,
    pub icon_spritesheet: Arc<RwLock<Option<TextureHandle>>>,

    pub search_term: String,

    pub theme: visuals::Theme,

    pub error: Option<anyhow::Error>,
    pub busy: Arc<RwLock<bool>>,

    pub show_about: bool,
    pub show_item_browser: bool,
    pub show_buff_browser: bool,
    pub show_prefix_browser: bool,
    pub show_research_browser: bool,
}

impl AppContext {
    pub fn new(
        chan: (Sender<Message>, Receiver<Message>),
        app_tx: Sender<AppMessage>,
        theme: visuals::Theme,
        meta_loader: Rc<dyn MetaLoader>,
    ) -> Self {
        let prefix_meta = meta_loader
            .load_prefixes()
            .expect("Could not load prefixes");
        let item_meta = meta_loader.load_items().expect("Could not load items");
        let buff_meta = meta_loader.load_buffs().expect("Could not load buffs");

        Self {
            chan,
            app_tx,

            player: Arc::new(RwLock::new(Player::default())),
            player_path: Arc::new(RwLock::new(None)),

            selected_item: SelectedItem(ItemGroup::Inventory, 0),
            selected_buff: SelectedBuff(0),
            selected_loadout: SelectedLoadout(0),

            prefix_meta: Arc::new(RwLock::new(prefix_meta)),
            item_meta: Arc::new(RwLock::new(item_meta)),
            buff_meta: Arc::new(RwLock::new(buff_meta)),

            item_spritesheet: Arc::new(RwLock::new(None)),
            buff_spritesheet: Arc::new(RwLock::new(None)),
            icon_spritesheet: Arc::new(RwLock::new(None)),

            theme,

            search_term: Default::default(),

            error: None,
            busy: Arc::new(RwLock::new(false)),

            show_about: false,
            show_item_browser: false,
            show_buff_browser: false,
            show_prefix_browser: false,
            show_research_browser: false,
        }
    }

    fn context_tx(&self) -> &Sender<Message> {
        &self.chan.0
    }

    fn context_rx(&self) -> &Receiver<Message> {
        &self.chan.1
    }

    fn app_tx(&self) -> &Sender<AppMessage> {
        &self.app_tx
    }

    pub fn theme(&self) -> visuals::Theme {
        self.theme
    }

    pub fn is_busy(&self) -> bool {
        *self.busy.read()
    }

    pub fn is_modal_open(&self) -> bool {
        self.is_busy()
            || self.error.is_some()
            || self.show_about
            || self.show_item_browser
            || self.show_buff_browser
            || self.show_prefix_browser
            || self.show_research_browser
    }

    pub fn do_task(
        &mut self,
        task: impl 'static + Send + Future<Output = anyhow::Result<Message>>,
    ) {
        let tx = self.context_tx().clone();
        let busy = self.busy.clone();
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

    pub fn send_context_msg(&self, msg: Message) {
        self.context_tx().send(msg).unwrap();
    }

    pub fn send_app_msg(&self, msg: AppMessage) {
        self.app_tx().send(msg).unwrap();
    }

    fn handle_update(&mut self, ctx: &egui::Context) {
        while let Ok(msg) = self.context_rx().try_recv() {
            self.handle_message(ctx, msg);
        }
    }

    fn handle_message(&mut self, ctx: &egui::Context, msg: Message) {
        match msg {
            Message::Noop => {}
            Message::LoadItemSpritesheet => {
                if self.item_spritesheet.read().is_some() {
                    return;
                }
                let spritesheet = self.item_spritesheet.clone();
                self.load_spritesheet(ctx, "items.png", spritesheet);
            }
            Message::LoadBuffSpritesheet => {
                if self.buff_spritesheet.read().is_some() {
                    return;
                }
                let spritesheet = self.buff_spritesheet.clone();
                self.load_spritesheet(ctx, "buffs.png", spritesheet);
            }
            Message::LoadIconSpritesheet => {
                if self.icon_spritesheet.read().is_some() {
                    return;
                }
                let spritesheet = self.icon_spritesheet.clone();
                self.load_spritesheet(ctx, "icons.png", spritesheet);
            }
            Message::ShowAbout => self.show_about = true,
            Message::CloseAbout => self.show_about = false,
            Message::ShowError(err) => {
                *self.busy.write() = false;
                self.error = Some(err)
            }
            Message::CloseError => self.error = None,
            Message::SetTheme(theme) => {
                theme.set_theme(ctx);
                self.theme = theme;
            }
            Message::ResetPlayer => self.player.write().clone_from(&DEFAULT_PLAYER),
            Message::LoadPlayer => {
                let player = self.player.clone();
                let item_meta = self.item_meta.clone();
                let player_path = self.player_path.clone();

                self.do_task(async move {
                    let player_dir = player_path
                        .read()
                        .clone()
                        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                        .unwrap_or_else(|| DEFAULT_PLAYER_DIR.clone());

                    let (directory, file_name) = if player_dir.is_file() {
                        let directory = player_dir
                            .parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| DEFAULT_PLAYER_DIR.clone());
                        let file_name = player_dir
                            .file_name()
                            .map(|f| f.to_string_lossy().to_string())
                            .unwrap_or_else(|| player.read().name.replace(' ', "_"));
                        (directory, file_name)
                    } else {
                        (player_dir, player.read().name.replace(' ', "_"))
                    };

                    let Some(file) = rfd::AsyncFileDialog::new()
                        .set_directory(directory)
                        .set_file_name(file_name)
                        .add_filter("Terraria Player File", &["plr"])
                        .add_filter("Decrypted Player File", &["dplr"])
                        .add_filter("All Files", &["*"])
                        .pick_file()
                        .await
                    else {
                        return Ok(Message::Noop);
                    };

                    let path = file.path().to_path_buf();
                    let data = file.read().await;

                    if path
                        .extension()
                        .is_some_and(|e| e.to_string_lossy() == "dplr")
                    {
                        player.write().load_decrypted(&item_meta.read(), &data)?;
                    } else {
                        player.write().load(&item_meta.read(), &data)?;
                    };

                    *player_path.write() = Some(path);

                    Ok(Message::Noop)
                });
            }

            Message::SavePlayer => {
                let player = self.player.clone();
                let item_meta = self.item_meta.clone();
                let player_path = self.player_path.clone();

                self.do_task(async move {
                    let player_dir = player_path
                        .read()
                        .clone()
                        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                        .unwrap_or_else(|| DEFAULT_PLAYER_DIR.clone());

                    let (directory, file_name) = if player_dir.is_file() {
                        let directory = player_dir
                            .parent()
                            .unwrap_or_else(|| DEFAULT_PLAYER_DIR.as_path())
                            .to_path_buf();
                        let file_name = player_dir
                            .file_name()
                            .map(|f| f.to_string_lossy().to_string())
                            .unwrap_or_else(|| player.read().name.replace(' ', "_"));
                        (directory, file_name)
                    } else {
                        (player_dir, player.read().name.replace(' ', "_"))
                    };

                    let Some(file) = rfd::AsyncFileDialog::new()
                        .set_directory(directory)
                        .set_file_name(file_name)
                        .add_filter("Terraria Player File", &["plr"])
                        .add_filter("Decrypted Player File", &["dplr"])
                        .add_filter("All Files", &["*"])
                        .save_file()
                        .await
                    else {
                        return Ok(Message::Noop);
                    };

                    let path = file.path().to_path_buf();

                    let data = if path
                        .extension()
                        .is_some_and(|e| e.to_string_lossy() == "dplr")
                    {
                        player.read().save_decrypted(&item_meta.read())?
                    } else {
                        player.read().save(&item_meta.read())?
                    };

                    file.write(&data).await?;

                    *player_path.write() = Some(path);

                    Ok(Message::Noop)
                });
            }
            Message::SelectLoadout(selection) => self.selected_loadout = selection,
            Message::SelectItem(selection) => self.selected_item = selection,
            Message::SelectBuff(selection) => self.selected_buff = selection,
            Message::AddAllResearch => {
                let mut player = self.player.write();
                let item_meta = self.item_meta.read();

                // TODO: Maybe remove this at some point?
                player.research.clear();
                for item in &*item_meta {
                    if item.forbidden.is_none() {
                        player.research.push(ResearchItem {
                            internal_name: item.internal_name.clone(),
                            stack: item.sacrifices,
                        });
                    }
                }
            }
            Message::RemoveAllResearch => {
                let mut player = self.player.write();
                player.research.clear();
            }
            Message::ToggleResearchItem(id) => {
                let mut player = self.player.write();

                // TODO: Maybe add `id` onto ResearchItem?
                if let Some(meta) = self.item_meta.read().iter().find(|i| i.id == id) {
                    if let Some(index) = player
                        .research
                        .iter()
                        .position(|i| i.internal_name == meta.internal_name)
                    {
                        player.research.remove(index);
                    } else {
                        player.research.push(ResearchItem {
                            internal_name: meta.internal_name.clone(),
                            stack: meta.sacrifices,
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
            Message::OpenResearchBrowser => self.show_research_browser = true,
            Message::CloseResearchBrowser => {
                self.search_term.clear();
                self.show_research_browser = false;
            }
            Message::SetCurrentItemId(id) => {
                let player = &mut *self.player.write();
                let selected_item = selected_item(self.selected_item, player);

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
                let player = &mut *self.player.write();
                let selected_buff = selected_buff(self.selected_buff, player);

                selected_buff.id = id;

                if selected_buff.time == 0 {
                    selected_buff.time = Duration::new(60 * 15, 0).as_ticks() as i32;
                }

                if self.show_buff_browser {
                    self.search_term.clear();
                    self.show_buff_browser = false;
                }
            }
            Message::SetCurrentPrefixId(id) => {
                let player = &mut *self.player.write();
                let item = selected_item(self.selected_item, player);

                item.prefix.id = id;

                if self.show_prefix_browser {
                    self.search_term.clear();
                    self.show_prefix_browser = false;
                }
            }
        }
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|input| {
            if self.is_modal_open() {
                if input.consume_key(Modifiers::NONE, Key::Escape) {
                    self.error = None;
                    self.show_about = false;
                    self.show_item_browser = false;
                    self.show_buff_browser = false;
                    self.show_prefix_browser = false;
                    self.show_research_browser = false;
                    self.search_term.clear();
                }
            } else {
                if input.consume_shortcut(&SHORTCUT_LOAD) {
                    self.send_context_msg(Message::LoadPlayer);
                }
                if input.consume_shortcut(&SHORTCUT_SAVE) {
                    self.send_context_msg(Message::SavePlayer);
                }
                if input.consume_shortcut(&SHORTCUT_EXIT) {
                    self.send_app_msg(AppMessage::Exit);
                }
            }
        });
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        self.handle_update(ctx);
        self.handle_keyboard(ctx);

        self.render_about(ctx);
        self.render_error(ctx);

        self.render_item_browser(ctx);
        self.render_buff_browser(ctx);
        self.render_prefix_browser(ctx);
        self.render_research_browser(ctx);
    }
}
