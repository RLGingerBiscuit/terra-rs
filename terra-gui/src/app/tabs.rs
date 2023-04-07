use std::fmt::Display;

use egui::{Ui, WidgetText};
use egui_dock::{NodeIndex, TabViewer, Tree};
use terra_core::{Buff, Item};

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

        ui.vertical(|ui| {
            ui.label("Item Sprites");

            let item_426 = Item {
                id: 426,
                ..Default::default()
            };
            let item_2 = Item {
                id: 2,
                ..Default::default()
            };
            let item_69 = Item {
                id: 69,
                ..Default::default()
            };
            let item_5455 = Item {
                id: 5455,
                ..Default::default()
            };
            let item_7500 = Item {
                id: 7500,
                ..Default::default()
            };

            ui.horizontal(|ui| {
                self.render_item(ui, &item_426);
                self.render_item(ui, &item_2);
                self.render_item(ui, &item_69);
                self.render_item(ui, &item_5455);
                self.render_item(ui, &item_7500);
            });
        });

        ui.vertical(|ui| {
            ui.label("Buff Sprites");

            let buff_0 = Buff {
                id: 0,
                ..Default::default()
            };
            let buff_1 = Buff {
                id: 1,
                ..Default::default()
            };
            let buff_69 = Buff {
                id: 69,
                ..Default::default()
            };
            let buff_27 = Buff {
                id: 27,
                ..Default::default()
            };
            let buff_353 = Buff {
                id: 353,
                ..Default::default()
            };
            let buff_354 = Buff {
                id: 354,
                ..Default::default()
            };
            let buff_999 = Buff {
                id: 999,
                ..Default::default()
            };

            ui.horizontal(|ui| {
                self.render_buff(ui, &buff_0);
                self.render_buff(ui, &buff_1);
                self.render_buff(ui, &buff_69);
                self.render_buff(ui, &buff_27);
                self.render_buff(ui, &buff_353);
                self.render_buff(ui, &buff_354);
                self.render_buff(ui, &buff_999);
            });
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
