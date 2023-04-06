use std::fmt::Display;

use egui::{Color32, Image, Pos2, Rect, Ui, Vec2, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};
use terra_core::BUFF_SPRITE_SIZE;

use super::{App, Message};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tabs {
    Main,
    Inventory,
}

impl Display for Tabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tabs::Main => "Info",
                Tabs::Inventory => "Inventory",
            }
        )
    }
}

pub fn default_ui() -> Tree<Tabs> {
    let mut tree = Tree::new(vec![Tabs::Main]);
    let [main, _side] = tree.split_below(0.into(), 0.5, vec![Tabs::Inventory]);

    tree.set_focused_node(main);
    tree
}

impl App {
    fn render_main_tab(&mut self, ui: &mut Ui) {
        ui.heading("Main");
        let player = self.player.read();

        ui.label(format!("Name: {}", player.name));

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
                    let sprite_size =
                        Vec2::new(width / spritesheet_size[0], height / spritesheet_size[1]);
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
                    let sprite_size =
                        Vec2::new(width / spritesheet_size[0], height / spritesheet_size[1]);
                    let uv = Rect::from_min_size(min, sprite_size);

                    let size = Vec2::new(width, height) * 4.;

                    ui.vertical(|ui| {
                        ui.label("Buff Sprite");
                        ui.add(Image::new(spritesheet, size).uv(uv));
                    });
                }
            }
        });
    }

    fn render_inventory_tab(&mut self, ui: &mut Ui) {
        ui.heading("Inventory");
    }
}

impl TabViewer for App {
    type Tab = Tabs;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.to_string().into()
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.closed_tabs.insert(*tab, NodeIndex::root());
        true
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        ui.set_enabled(!self.modal_open());

        match tab {
            Tabs::Main => self.render_main_tab(ui),
            Tabs::Inventory => self.render_inventory_tab(ui),
        }
    }
}
