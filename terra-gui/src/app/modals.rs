use egui::{Align2, Frame, Grid, RichText, Vec2, Window};

use crate::ext::UiExt;

use super::{App, Message, GITHUB_REPO_NAME, GITHUB_REPO_URL};

impl App {
    pub fn render_about(&mut self, ctx: &egui::Context) {
        if self.show_about {
            Window::new("About")
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, Vec2::default())
                .fixed_size([360.0, 240.0])
                .frame(egui::Frame::window(&ctx.style()).inner_margin(8.))
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;
                    ui.vertical_centered(|ui| {
                        ui.heading("terra-rs");
                        ui.label("© 2023 RLGingerBiscuit - MIT");
                        ui.label(concat!("Version ", env!("CARGO_PKG_VERSION")));
                    });
                    Grid::new("about_box").num_columns(2).show(ui, |ui| {
                        ui.label("Github:");
                        if ui.link(GITHUB_REPO_NAME).clicked() {
                            open::that(GITHUB_REPO_URL).ok();
                        }
                        ui.end_row();
                        ui.label("GUI Library:");
                        if ui.link("emilk/egui").clicked() {
                            open::that("https://github.com/emilk/egui").ok();
                        }
                        ui.end_row();
                    });

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Ok").clicked() {
                            self.do_update(Message::CloseAbout);
                        }
                    });
                });
        }
    }

    pub fn render_error(&mut self, ctx: &egui::Context) {
        if let Some(err) = self.error.as_ref() {
            egui::Window::new("Error")
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, Vec2::default())
                .auto_sized()
                .frame(Frame::window(&ctx.style()).inner_margin(8.))
                .show(ctx, |ui| {
                    ui.label(err.to_string());

                    egui::CollapsingHeader::new("Details").show(ui, |ui| {
                        err.chain().enumerate().for_each(|(i, e)| {
                            ui.label(RichText::new(format!("{i}. {e}")).code());
                        });
                    });

                    ui.vertical_right_justified(|ui| {
                        if ui.button("Ok").clicked() {
                            self.do_update(Message::CloseAbout);
                        }
                    });
                });
        }
    }
}
