//! Shared visual theme configuration (light + dark).
//!
//! Centralizing style here demonstrates how to avoid scattered magic constants.

use eframe::egui::{self, Color32, CornerRadius, Stroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn background(self) -> Color32 {
        match self {
            Self::Dark => Color32::from_rgb(12, 16, 25),
            Self::Light => Color32::from_rgb(225, 233, 244),
        }
    }

    pub fn panel_fill(self) -> Color32 {
        match self {
            Self::Dark => Color32::from_rgb(20, 27, 42),
            Self::Light => Color32::from_rgb(240, 246, 255),
        }
    }

    pub fn accent(self) -> Color32 {
        match self {
            Self::Dark => Color32::from_rgb(0, 230, 255),
            Self::Light => Color32::from_rgb(32, 138, 214),
        }
    }

    pub fn accent_soft(self) -> Color32 {
        match self {
            Self::Dark => Color32::from_rgb(0, 109, 124),
            Self::Light => Color32::from_rgb(175, 220, 252),
        }
    }

    pub fn strong_text(self) -> Color32 {
        match self {
            Self::Dark => Color32::from_rgb(232, 245, 255),
            Self::Light => Color32::from_rgb(24, 41, 68),
        }
    }

    pub fn muted_text(self) -> Color32 {
        match self {
            Self::Dark => Color32::from_rgb(158, 184, 209),
            Self::Light => Color32::from_rgb(71, 94, 125),
        }
    }
}

pub fn apply_theme(ctx: &egui::Context, mode: ThemeMode) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.visuals = match mode {
        ThemeMode::Dark => egui::Visuals::dark(),
        ThemeMode::Light => egui::Visuals::light(),
    };
    style.visuals.window_corner_radius = CornerRadius::same(10);
    style.visuals.panel_fill = mode.panel_fill();
    style.visuals.window_fill = mode.panel_fill();
    style.visuals.faint_bg_color = match mode {
        ThemeMode::Dark => Color32::from_rgb(15, 23, 34),
        ThemeMode::Light => Color32::from_rgb(236, 244, 253),
    };
    style.visuals.extreme_bg_color = match mode {
        ThemeMode::Dark => Color32::from_rgb(8, 13, 22),
        ThemeMode::Light => Color32::from_rgb(230, 240, 251),
    };
    style.visuals.widgets.inactive.bg_fill = match mode {
        ThemeMode::Dark => mode.panel_fill(),
        ThemeMode::Light => Color32::from_rgb(244, 249, 255),
    };
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(
        1.0,
        match mode {
            ThemeMode::Dark => Color32::from_rgb(52, 88, 110),
            ThemeMode::Light => Color32::from_rgb(182, 207, 230),
        },
    );
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.2, mode.strong_text());
    style.visuals.widgets.hovered.bg_fill = match mode {
        ThemeMode::Dark => mode.accent_soft(),
        ThemeMode::Light => Color32::from_rgb(228, 242, 255),
    };
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(
        1.2,
        match mode {
            ThemeMode::Dark => mode.accent(),
            ThemeMode::Light => Color32::from_rgb(148, 196, 232),
        },
    );
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.2, mode.strong_text());
    style.visuals.widgets.active.bg_fill = match mode {
        ThemeMode::Dark => mode.accent(),
        ThemeMode::Light => Color32::from_rgb(212, 233, 250),
    };
    style.visuals.widgets.active.bg_stroke = Stroke::new(
        1.3,
        match mode {
            ThemeMode::Dark => Color32::from_rgb(0, 239, 255),
            ThemeMode::Light => Color32::from_rgb(128, 182, 224),
        },
    );
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.2, mode.strong_text());
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, mode.strong_text());
    style.visuals.override_text_color = Some(mode.strong_text());
    ctx.set_style(style);
}
