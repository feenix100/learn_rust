//! Reusable UI pieces.
//!
//! Reuse keeps screen modules short and shows how shared helpers can live in
//! their own file under `ui/`.

use eframe::egui::{self, CornerRadius, RichText, Stroke, Ui};

use crate::theme::ThemeMode;

pub fn panel_card(ui: &mut Ui, theme: ThemeMode, title: &str, add_body: impl FnOnce(&mut Ui)) {
    let frame = egui::Frame::group(ui.style())
        .fill(theme.panel_fill())
        .stroke(Stroke::new(1.0, theme.accent_soft()))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(12.0);
    frame.show(ui, |ui| {
        ui.label(
            RichText::new(title)
                .color(theme.accent())
                .strong()
                .heading(),
        );
        ui.add_space(6.0);
        add_body(ui);
    });
}

pub fn keyword_chip(ui: &mut Ui, theme: ThemeMode, text: &str) {
    let frame = egui::Frame::new()
        .fill(theme.accent_soft())
        .corner_radius(CornerRadius::same(6))
        .inner_margin(egui::Margin::symmetric(8, 4));
    frame.show(ui, |ui| {
        ui.label(RichText::new(text).color(theme.strong_text()).strong());
    });
}
