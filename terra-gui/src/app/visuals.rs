use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Egui,
    EguiLight,
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Egui => "egui Dark",
                Theme::EguiLight => "egui Light",
                Theme::Latte => "Latte",
                Theme::Frappe => "Frappe",
                Theme::Macchiato => "Macchiato",
                Theme::Mocha => "Mocha",
            }
        )
    }
}

impl Theme {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Egui,
            Self::EguiLight,
            Self::Latte,
            Self::Frappe,
            Self::Macchiato,
            Self::Mocha,
        ]
        .into_iter()
    }

    pub fn set_theme(&self, ctx: &egui::Context) {
        match self {
            Self::Egui => ctx.set_visuals(egui::style::Visuals::dark()),
            Self::EguiLight => ctx.set_visuals(egui::style::Visuals::light()),
            Self::Latte => catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE),
            Self::Frappe => catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE),
            Self::Macchiato => catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO),
            Self::Mocha => catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA),
        }
    }
}
