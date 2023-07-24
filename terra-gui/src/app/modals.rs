use egui::{
    vec2, Align2, CollapsingHeader, Grid, RichText, ScrollArea, TextStyle, Ui, Vec2, WidgetText,
};

use crate::{app::inventory::ITEM_SLOT_SIZE, ui::UiExt};

use super::{
    inventory::BUFF_SLOT_SIZE, App, Message, EGUI_GITHUB_REPO_NAME, EGUI_GITHUB_REPO_URL,
    GITHUB_REPO_NAME, GITHUB_REPO_URL,
};

#[allow(unused)]
#[derive(Debug)]
enum Sizing {
    Auto,
    Fixed(Vec2),
}

const DEFAULT_MODAL_WIDTH: f32 = 360.;
const DEFAULT_MODAL_HEIGHT: f32 = 240.;

const ITEM_BROWSER_COLS: usize = 6;
const BUFF_BROWSER_COLS: usize = 6;
const PREFIX_BROWSER_COLS: usize = 4;

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

    pub fn render_item_browser(&mut self, ctx: &egui::Context) {
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

                    if !search_term.is_empty() {
                        let search_term_lower = search_term.to_lowercase();
                        let meta = &self.item_meta.read();
                        let filtered = meta
                            .iter()
                            .filter(|meta| meta.name.to_lowercase().contains(&search_term_lower));

                        let total_rows = ((filtered.clone().count() as f32)
                            / (ITEM_BROWSER_COLS as f32))
                            .ceil() as usize;

                        ScrollArea::new([false, true])
                            .id_source("item_browser_scrollarea")
                            .show_rows(ui, ITEM_SLOT_SIZE, total_rows, |ui, row_range| {
                                Grid::new("item_browser_grid")
                                    .num_columns(ITEM_BROWSER_COLS)
                                    .striped(true)
                                    .show(ui, |ui| {
                                        let mut filtered =
                                            filtered.skip(row_range.start * ITEM_BROWSER_COLS);

                                        for i in (row_range.start * ITEM_BROWSER_COLS)
                                            ..(row_range.end * ITEM_BROWSER_COLS)
                                        {
                                            let meta = filtered.next();
                                            if meta.is_none() {
                                                break;
                                            }
                                            let meta = unsafe { meta.unwrap_unchecked() };

                                            let response = self
                                                .render_item_slot(ui, meta.id,None, false, None, true);

                                            if response.clicked() {
                                                self.do_update(Message::SetCurrentItemId(meta.id));
                                            }

                                            if i % ITEM_BROWSER_COLS == ITEM_BROWSER_COLS - 1 {
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

    pub fn render_buff_browser(&mut self, ctx: &egui::Context) {
        let mut search_term = self.search_term.clone();
        let mut term_changed = false;

        if self.show_buff_browser {
            self.render_modal(
                ctx,
                "Buff Browser",
                false,
                Sizing::Fixed(vec2(
                    DEFAULT_MODAL_WIDTH + ctx.style().spacing.item_spacing.x,
                    DEFAULT_MODAL_HEIGHT * 2.,
                )),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.vertical_centered_justified(|ui| {
                        term_changed = ui.text_edit_singleline(&mut search_term).changed();
                    });

                    if !search_term.is_empty() {
                        let search_term_lower = search_term.to_lowercase();
                        let meta = &self.buff_meta.read();
                        let filtered = meta
                            .iter()
                            .filter(|meta| meta.name.to_lowercase().contains(&search_term_lower));

                        let total_rows = ((filtered.clone().count() as f32)
                            / (BUFF_BROWSER_COLS as f32))
                            .ceil() as usize;

                        ScrollArea::new([false, true])
                            .id_source("buff_browser_scrollarea")
                            .show_rows(ui, BUFF_SLOT_SIZE, total_rows, |ui, row_range| {
                                Grid::new("buff_browser_grid")
                                    .num_columns(BUFF_BROWSER_COLS)
                                    .striped(true)
                                    .show(ui, |ui| {
                                        let mut filtered =
                                            filtered.skip(row_range.start * BUFF_BROWSER_COLS);

                                        for i in (row_range.start * BUFF_BROWSER_COLS)
                                            ..(row_range.end * BUFF_BROWSER_COLS)
                                        {
                                            let meta = filtered.next();
                                            if meta.is_none() {
                                                break;
                                            }
                                            let meta = unsafe { meta.unwrap_unchecked() };

                                            let response = self
                                                .render_buff_slot(ui, meta.id, None, false, true);

                                            if response.clicked() {
                                                self.do_update(Message::SetCurrentBuffId(meta.id));
                                            }

                                            if i % BUFF_BROWSER_COLS == BUFF_BROWSER_COLS - 1 {
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

    // TODO: Add a toggle to show only class-specific prefixes
    //       (and the Terrarian variant of Legendary)
    pub fn render_prefix_browser(&mut self, ctx: &egui::Context) {
        let mut search_term = self.search_term.clone();
        let mut term_changed = false;

        if self.show_prefix_browser {
            self.render_modal(
                ctx,
                "Prefix Browser",
                false,
                Sizing::Fixed(vec2(
                    DEFAULT_MODAL_WIDTH + ctx.style().spacing.item_spacing.x,
                    DEFAULT_MODAL_HEIGHT * 2.,
                )),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.vertical_centered_justified(|ui| {
                        term_changed = ui.text_edit_singleline(&mut search_term).changed();
                    });

                    if !search_term.is_empty() {
                        let search_term_lower = search_term.to_lowercase();
                        let meta = &self.prefix_meta.read();
                        let filtered = meta
                            .iter()
                            .filter(|meta| meta.name.to_lowercase().contains(&search_term_lower));

                        let total_rows = ((filtered.clone().count() as f32)
                            / (PREFIX_BROWSER_COLS as f32))
                            .ceil() as usize;

                        ScrollArea::new([false, true])
                            .id_source("prefix_browser_scrollarea")
                            .show_rows(
                                ui,
                                ui.text_style_height(&TextStyle::Body),
                                total_rows,
                                |ui, row_range| {
                                    Grid::new("prefix_browser_grid")
                                        .num_columns(PREFIX_BROWSER_COLS)
                                        .striped(true)
                                        .show(ui, |ui| {
                                            let mut filtered = filtered
                                                .skip(row_range.start * PREFIX_BROWSER_COLS);

                                            for i in (row_range.start * PREFIX_BROWSER_COLS)
                                                ..(row_range.end * PREFIX_BROWSER_COLS)
                                            {
                                                let meta = filtered.next();
                                                if meta.is_none() {
                                                    break;
                                                }
                                                let meta = unsafe { meta.unwrap_unchecked() };

                                                let response = ui.button(&meta.name);

                                                if response.clicked() {
                                                    self.do_update(Message::SetCurrentPrefixId(
                                                        meta.id,
                                                    ));
                                                }

                                                response.on_hover_ui(|ui| {
                                                    self.render_prefix_tooltip(ui, meta);
                                                });

                                                if i % PREFIX_BROWSER_COLS
                                                    == PREFIX_BROWSER_COLS - 1
                                                {
                                                    ui.end_row();
                                                }
                                            }
                                        });
                                },
                            );
                    }

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Close").clicked() {
                            self.do_update(Message::ClosePrefixBrowser);
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
