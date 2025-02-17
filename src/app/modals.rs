use egui::{
    vec2, Align2, CollapsingHeader, Grid, RichText, ScrollArea, TextStyle, Ui, Vec2, WidgetText,
};

use super::{
    context::AppContext,
    inventory::{
        buff_slot::{self, BuffSlotOptions},
        item_slot::{self, ItemSlotOptions},
        prefix_tooltip::PrefixTooltipOptions,
        ItemGroup,
    },
    Message, EGUI_GITHUB_REPO_NAME, EGUI_GITHUB_REPO_URL, GITHUB_REPO_NAME, GITHUB_REPO_URL,
};
use crate::ui::UiExt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Sizing {
    Auto,
    Fixed(Vec2),
}

const DEFAULT_MODAL_WIDTH: f32 = 360.;
const DEFAULT_MODAL_HEIGHT: f32 = 240.;

const ABOUT_MODAL_WIDTH: f32 = 300.;
const ABOUT_MODAL_HEIGHT: f32 = DEFAULT_MODAL_HEIGHT;

const ERROR_MODAL_WIDTH: f32 = DEFAULT_MODAL_WIDTH * 2.;
const ERROR_MODAL_HEIGHT: f32 = DEFAULT_MODAL_HEIGHT * 2.;

const ITEM_BROWSER_COLS: usize = 6;
const BUFF_BROWSER_COLS: usize = 8;
const PREFIX_BROWSER_COLS: usize = 4;

#[inline]
fn item_browser_sizing(ctx: &egui::Context) -> Sizing {
    Sizing::Fixed(vec2(
        item_slot::SLOT_SIZE.x * (ITEM_BROWSER_COLS + 1) as f32
            + ctx.style().spacing.item_spacing.x * (ITEM_BROWSER_COLS * 2) as f32
            - ctx.style().spacing.scroll.bar_width,
        DEFAULT_MODAL_HEIGHT * 2.,
    ))
}

#[inline]
fn buff_browser_sizing(ctx: &egui::Context) -> Sizing {
    Sizing::Fixed(vec2(
        buff_slot::SLOT_SIZE.x * BUFF_BROWSER_COLS as f32
            + ctx.style().spacing.item_spacing.x * (BUFF_BROWSER_COLS * 2) as f32,
        DEFAULT_MODAL_HEIGHT * 2.,
    ))
}

#[inline]
fn prefix_browser_sizing(ctx: &egui::Context) -> Sizing {
    Sizing::Fixed(vec2(
        DEFAULT_MODAL_WIDTH + ctx.style().spacing.item_spacing.x,
        DEFAULT_MODAL_HEIGHT * 2.,
    ))
}

impl AppContext {
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
                Sizing::Fixed(vec2(ABOUT_MODAL_WIDTH, ABOUT_MODAL_HEIGHT)),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.vertical_centered(|ui| {
                        ui.heading("terra-rs");
                        ui.label(format!(
                            "\u{a9} 2022-{} RLGingerBiscuit - MIT",
                            time::OffsetDateTime::now_utc().year()
                        ));
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
                            self.send_context_msg(Message::CloseAbout);
                        }
                    });
                },
            );
        }
    }

    pub fn render_error(&self, ctx: &egui::Context) {
        let Some(err) = self.error.as_ref() else {
            return;
        };

        self.render_modal(
            ctx,
            "Error",
            false,
            Sizing::Fixed(vec2(ERROR_MODAL_WIDTH, ERROR_MODAL_HEIGHT)),
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
                        self.send_context_msg(Message::CloseError);
                    }
                });
            },
        );
    }

    pub fn render_item_browser(&mut self, ctx: &egui::Context) {
        if self.show_item_browser {
            let mut search_term = self.search_term.clone();
            let mut term_changed = false;

            self.render_modal(ctx, "Item Browser", false, item_browser_sizing(ctx), |ui| {
                ui.spacing_mut().item_spacing.y = 8.;

                ui.vertical_centered_justified(|ui| {
                    term_changed = ui.text_edit_singleline(&mut search_term).changed();
                });

                if !search_term.is_empty() {
                    let search_term_lower = search_term.to_lowercase();
                    let meta = &self.item_meta.read();
                    let filtered = meta
                        .iter()
                        .filter(|meta| meta.name.to_lowercase().contains(&search_term_lower))
                        .filter(|meta| {
                            meta.forbidden.is_none() || meta.forbidden.is_some_and(|f| !f)
                        });

                    let total_rows = ((filtered.clone().count() as f32)
                        / (ITEM_BROWSER_COLS as f32))
                        .ceil() as usize;

                    ScrollArea::new([false, true])
                        .id_source("item_browser_scrollarea")
                        .show_rows(ui, item_slot::SLOT_SIZE.y, total_rows, |ui, row_range| {
                            Grid::new("item_browser_grid")
                                .num_columns(ITEM_BROWSER_COLS)
                                .show(ui, |ui| {
                                    let mut filtered =
                                        filtered.skip(row_range.start * ITEM_BROWSER_COLS);

                                    for i in (row_range.start * ITEM_BROWSER_COLS)
                                        ..(row_range.end * ITEM_BROWSER_COLS)
                                    {
                                        let Some(meta) = filtered.next() else {
                                            continue;
                                        };

                                        let options = ItemSlotOptions::from_meta(
                                            meta,
                                            ItemGroup::ItemBrowser,
                                        )
                                        .tooltip_on_hover(true);

                                        let response = self.render_item_slot(ui, options);
                                        if response.clicked() {
                                            self.send_context_msg(Message::SetCurrentItemId(
                                                meta.id,
                                            ));
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
                        self.send_context_msg(Message::CloseItemBrowser);
                    }
                });
            });

            if term_changed {
                self.search_term = search_term;
            }
        }
    }

    pub fn render_buff_browser(&mut self, ctx: &egui::Context) {
        if self.show_buff_browser {
            let mut search_term = self.search_term.clone();
            let mut term_changed = false;

            self.render_modal(ctx, "Buff Browser", false, buff_browser_sizing(ctx), |ui| {
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
                        .show_rows(ui, buff_slot::SLOT_SIZE.y, total_rows, |ui, row_range| {
                            Grid::new("buff_browser_grid")
                                .num_columns(BUFF_BROWSER_COLS)
                                .show(ui, |ui| {
                                    let mut filtered =
                                        filtered.skip(row_range.start * BUFF_BROWSER_COLS);

                                    for i in (row_range.start * BUFF_BROWSER_COLS)
                                        ..(row_range.end * BUFF_BROWSER_COLS)
                                    {
                                        let Some(meta) = filtered.next() else {
                                            continue;
                                        };

                                        let options =
                                            BuffSlotOptions::from_meta(meta).tooltip_on_hover(true);

                                        let response = self.render_buff_slot(ui, options);
                                        if response.clicked() {
                                            self.send_context_msg(Message::SetCurrentBuffId(
                                                meta.id,
                                            ));
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
                        self.send_context_msg(Message::CloseBuffBrowser);
                    }
                });
            });

            if term_changed {
                self.search_term = search_term;
            }
        }
    }

    // TODO: Add a toggle to show only class-specific prefixes
    //       (and the Terrarian variant of Legendary)
    pub fn render_prefix_browser(&mut self, ctx: &egui::Context) {
        if self.show_prefix_browser {
            let mut search_term = self.search_term.clone();
            let mut term_changed = false;

            self.render_modal(
                ctx,
                "Prefix Browser",
                false,
                prefix_browser_sizing(ctx),
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
                                        .show(ui, |ui| {
                                            let mut filtered = filtered
                                                .skip(row_range.start * PREFIX_BROWSER_COLS);

                                            for i in (row_range.start * PREFIX_BROWSER_COLS)
                                                ..(row_range.end * PREFIX_BROWSER_COLS)
                                            {
                                                let Some(meta) = filtered.next() else {
                                                    continue;
                                                };

                                                let response = ui.button(meta.name.as_ref());

                                                if response.clicked() {
                                                    self.send_context_msg(
                                                        Message::SetCurrentPrefixId(meta.id),
                                                    );
                                                }

                                                response.on_hover_ui(|ui| {
                                                    let options =
                                                        PrefixTooltipOptions::new().id(meta.id);
                                                    self.render_prefix_tooltip(ui, options);
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
                            self.send_context_msg(Message::ClosePrefixBrowser);
                        }
                    });
                },
            );

            if term_changed {
                self.search_term = search_term;
            }
        }
    }

    pub fn render_research_browser(&mut self, ctx: &egui::Context) {
        if self.show_research_browser {
            let mut search_term = self.search_term.clone();
            let mut term_changed = false;

            self.render_modal(
                ctx,
                "Research Browser",
                false,
                item_browser_sizing(ctx),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 8.;

                    ui.vertical_centered_justified(|ui| {
                        term_changed = ui.text_edit_singleline(&mut search_term).changed();
                    });

                    let search_term_lower = search_term.to_lowercase();
                    let meta = &self.item_meta.read();
                    let filtered = meta
                        .iter()
                        .filter(|meta| {
                            search_term_lower.is_empty()
                                || meta.name.to_lowercase().contains(&search_term_lower)
                        })
                        .filter(|meta| {
                            meta.forbidden.is_none() || meta.forbidden.is_some_and(|f| !f)
                        });

                    let total_rows = ((filtered.clone().count() as f32)
                        / (ITEM_BROWSER_COLS as f32))
                        .ceil() as usize;

                    ScrollArea::new([false, true])
                        .id_source("research_browser_scrollarea")
                        .show_rows(ui, item_slot::SLOT_SIZE.y, total_rows, |ui, row_range| {
                            Grid::new("research_browser_grid")
                                .num_columns(ITEM_BROWSER_COLS)
                                .show(ui, |ui| {
                                    let mut filtered =
                                        filtered.skip(row_range.start * ITEM_BROWSER_COLS);

                                    let player = self.player.read();

                                    for i in (row_range.start * ITEM_BROWSER_COLS)
                                        ..(row_range.end * ITEM_BROWSER_COLS)
                                    {
                                        let Some(meta) = filtered.next() else {
                                            continue;
                                        };

                                        let options = ItemSlotOptions::from_meta(
                                            meta,
                                            ItemGroup::ResearchBrowser,
                                        )
                                        .highlighted(
                                            player
                                                .research
                                                .iter()
                                                .any(|i| i.internal_name == meta.internal_name),
                                        )
                                        .tooltip_on_hover(true);

                                        let response = self.render_item_slot(ui, options);
                                        if response.clicked() {
                                            self.send_context_msg(Message::ToggleResearchItem(
                                                meta.id,
                                            ));
                                        }

                                        if i % ITEM_BROWSER_COLS == ITEM_BROWSER_COLS - 1 {
                                            ui.end_row();
                                        }
                                    }
                                });
                        });

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Close").clicked() {
                            self.send_context_msg(Message::CloseResearchBrowser);
                        }
                    });
                },
            );

            if term_changed {
                self.search_term = search_term;
            }
        }
    }
}
