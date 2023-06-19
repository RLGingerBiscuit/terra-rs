use egui::{vec2, Align2, CollapsingHeader, Grid, RichText, ScrollArea, Ui, Vec2, WidgetText};

use crate::ui::UiExt;

use super::{
    App, Message, EGUI_GITHUB_REPO_NAME, EGUI_GITHUB_REPO_URL, GITHUB_REPO_NAME, GITHUB_REPO_URL,
};

#[allow(unused)]
#[derive(Debug)]
enum Sizing {
    Auto,
    Fixed(Vec2),
}

const DEFAULT_MODAL_WIDTH: f32 = 360.;
const DEFAULT_MODAL_HEIGHT: f32 = 240.;

impl App {
    fn render_modal<R>(
        &self,
        ctx: &egui::Context,
        title: impl Into<WidgetText>,
        collapsible: bool,
        sizing: Sizing,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) {
        let window = egui::Window::new(title)
            .collapsible(collapsible)
            .anchor(Align2::CENTER_CENTER, vec2(0., 0.));

        let window = match sizing {
            Sizing::Auto => window.auto_sized(),
            Sizing::Fixed(size) => window.fixed_size(size),
        };

        window.show(ctx, add_contents);
    }

    pub fn render_about(&self, ctx: &egui::Context) {
        if self.show_about {
            self.render_modal(
                ctx,
                "About",
                false,
                Sizing::Fixed(vec2(DEFAULT_MODAL_WIDTH, DEFAULT_MODAL_HEIGHT)),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.vertical_centered(|ui| {
                        ui.heading("terra-rs");
                        ui.label("\u{a9} 2023 RLGingerBiscuit - MIT");
                        ui.label(concat!("Version ", env!("CARGO_PKG_VERSION")));
                    });

                    Grid::new("about_grid").num_columns(2).show(ui, |ui| {
                        ui.label("Github:");
                        if ui.link(GITHUB_REPO_NAME).clicked() {
                            open::that(GITHUB_REPO_URL).ok();
                        }
                        ui.end_row();

                        ui.label("GUI Library Github:");
                        if ui.link(EGUI_GITHUB_REPO_NAME).clicked() {
                            open::that(EGUI_GITHUB_REPO_URL).ok();
                        }
                        ui.end_row();
                    });

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Ok").clicked() {
                            self.do_update(Message::CloseAbout);
                        }
                    });
                },
            );
        }
    }

    pub fn render_error(&self, ctx: &egui::Context) {
        if let Some(err) = self.error.as_ref() {
            self.render_modal(
                ctx,
                "Error",
                false,
                Sizing::Fixed(vec2(DEFAULT_MODAL_WIDTH * 2., DEFAULT_MODAL_WIDTH * 2.)),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.label(err.to_string());

                    CollapsingHeader::new("Details").show(ui, |ui| {
                        err.chain().enumerate().for_each(|(i, e)| {
                            ui.label(RichText::new(format!("{i}. {e}")).code());
                        });
                    });

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Ok").clicked() {
                            self.do_update(Message::CloseError);
                        }
                    });
                },
            );
        }
    }

    // TODO: Item tooltip once I've gotten that working
    pub fn render_item_browser(&mut self, ctx: &egui::Context) {
        // TODO: This seems kinda wasteful (clone *every frame*)
        // Can't do directly cause borrow checker says no
        let mut search_term = self.search_term.clone();
        let mut term_changed = false;

        if self.show_item_browser {
            let style = &*ctx.style();

            self.render_modal(
                ctx,
                "Item Browser",
                false,
                Sizing::Fixed(vec2(
                    DEFAULT_MODAL_WIDTH + style.spacing.item_spacing.x,
                    DEFAULT_MODAL_HEIGHT * 2.,
                )),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.vertical_centered_justified(|ui| {
                        term_changed = ui.text_edit_singleline(&mut search_term).changed();
                    });

                    // TODO: Maybe make search not case-sensitive?
                    if search_term != "" {
                        let meta: &Vec<terra_core::ItemMeta> = &*self.item_meta.read();
                        let filtered = meta
                            .as_slice()
                            .iter()
                            .filter(|meta| meta.name.contains(&search_term))
                            .enumerate();

                        const BROWSER_COLS: usize = 6;

                        ScrollArea::new([false, true])
                            .id_source("item_browser_scrollarea")
                            .show(ui, |ui| {
                                Grid::new("item_browser_grid")
                                    .num_columns(BROWSER_COLS)
                                    .striped(true)
                                    .show(ui, |ui| {
                                        for (i, meta) in filtered {
                                            if self
                                                .render_item_slot(ui, meta.id, false, None)
                                                .clicked()
                                            {
                                                self.do_update(Message::SetCurrentItemId(meta.id));
                                                self.do_update(Message::CloseItemBrowser);
                                            }

                                            if i % BROWSER_COLS == BROWSER_COLS - 1 {
                                                ui.end_row();
                                            }
                                        }
                                    });
                            });
                    }

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Close").clicked() {
                            self.do_update(Message::CloseItemBrowser);
                        }
                    });
                },
            );
        }

        if term_changed {
            self.search_term = search_term;
        }
    }

    // TODO: Buff tooltip once I've gotten that working
    pub fn render_buff_browser(&mut self, ctx: &egui::Context) {
        // TODO: This seems kinda wasteful (clone *every frame*)
        // Can't do directly cause borrow checker says no
        let mut search_term = self.search_term.clone();
        let mut term_changed = false;

        if self.show_buff_browser {
            let style = &*ctx.style();

            self.render_modal(
                ctx,
                "Buff Browser",
                false,
                Sizing::Fixed(vec2(
                    DEFAULT_MODAL_WIDTH + style.spacing.item_spacing.x,
                    DEFAULT_MODAL_HEIGHT * 2.,
                )),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.vertical_centered_justified(|ui| {
                        term_changed = ui.text_edit_singleline(&mut search_term).changed();
                    });

                    // TODO: Maybe make search case-sensitive?
                    if search_term != "" {
                        let meta: &Vec<terra_core::BuffMeta> = &*self.buff_meta.read();
                        let filtered = meta
                            .as_slice()
                            .iter()
                            .filter(|meta| meta.name.contains(&search_term))
                            .enumerate();

                        const BROWSER_COLS: usize = 6;

                        ScrollArea::new([false, true])
                            .id_source("buff_browser_scrollarea")
                            .show(ui, |ui| {
                                Grid::new("buff_browser_grid")
                                    .num_columns(BROWSER_COLS)
                                    .striped(true)
                                    .show(ui, |ui| {
                                        for (i, meta) in filtered {
                                            if self.render_buff_slot(ui, meta.id, false).clicked() {
                                                self.do_update(Message::SetCurrentBuffId(meta.id));
                                                self.do_update(Message::CloseBuffBrowser);
                                            }

                                            if i % BROWSER_COLS == BROWSER_COLS - 1 {
                                                ui.end_row();
                                            }
                                        }
                                    });
                            });
                    }

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Close").clicked() {
                            self.do_update(Message::CloseBuffBrowser);
                        }
                    });
                },
            );
        }

        if term_changed {
            self.search_term = search_term;
        }
    }
}
