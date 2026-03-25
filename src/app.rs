//! Top-level app coordinator.
//!
//! This struct owns cross-screen state. Each UI module renders a part of this
//! state, which is a common architecture in Rust GUI apps.

use std::collections::HashSet;

use eframe::egui::{self, Color32, FontId, RichText, SidePanel, TextStyle, TopBottomPanel};

use crate::{
    content::ordered_concepts,
    models::{Concept, ConceptId, OwnershipMode, VisualMode},
    navigation::{ConceptTab, Screen},
    theme::{apply_theme, ThemeMode},
    ui,
};

pub struct RustConceptsApp {
    pub theme_mode: ThemeMode,
    pub screen: Screen,
    pub concept_tab: ConceptTab,
    pub concepts: Vec<Concept>,
    pub opened: HashSet<ConceptId>,
    pub completed: HashSet<ConceptId>,
    pub visual_mode: VisualMode,
    pub ownership_mode: OwnershipMode,
    pub active_teaching_snippet: Option<(ConceptId, usize)>,
    pub zoom_factor: f32,
    pub target_zoom_factor: f32,
    pub text_size: f32,
    pub target_text_size: f32,
    pub controls_reveal: f32,
    pub target_controls_reveal: f32,
    pub controls_hold_until: f64,
}

impl RustConceptsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut opened = HashSet::new();
        opened.insert(ConceptId::WhatIsRust);

        let app = Self {
            theme_mode: ThemeMode::Dark,
            screen: Screen::Concept(ConceptId::WhatIsRust),
            concept_tab: ConceptTab::Explanation,
            concepts: ordered_concepts(),
            opened,
            completed: HashSet::new(),
            visual_mode: VisualMode::Idle,
            ownership_mode: OwnershipMode::Move,
            active_teaching_snippet: None,
            zoom_factor: 1.0,
            target_zoom_factor: 1.0,
            text_size: 23.0,
            target_text_size: 23.0,
            controls_reveal: 0.12,
            target_controls_reveal: 0.12,
            controls_hold_until: 0.0,
        };
        apply_theme(&cc.egui_ctx, app.theme_mode);
        apply_global_text_size(&cc.egui_ctx, app.text_size);
        cc.egui_ctx.set_zoom_factor(app.zoom_factor);
        app
    }

    pub fn open_concept(&mut self, id: ConceptId) {
        self.opened.insert(id);
        self.screen = Screen::Concept(id);
        self.concept_tab = ConceptTab::Explanation;
        self.active_teaching_snippet = None;
    }

    pub fn mark_completed(&mut self, id: ConceptId) {
        self.completed.insert(id);
    }

    pub fn concept_by_id(&self, id: ConceptId) -> Option<&Concept> {
        self.concepts.iter().find(|c| c.id == id)
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.concepts.is_empty() {
            return 0.0;
        }
        self.completed.len() as f32 / self.concepts.len() as f32
    }
}

impl eframe::App for RustConceptsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Global smooth zoom: only when user holds Ctrl and uses mouse wheel.
        // `raw_scroll_delta` maps directly to wheel movement.
        let (wheel_y, ctrl_held) = ctx.input(|i| (i.raw_scroll_delta.y, i.modifiers.ctrl));
        if ctrl_held && wheel_y.abs() > 0.0 {
            let zoom_step = 1.0 + (wheel_y * 0.0015);
            self.target_zoom_factor = (self.target_zoom_factor * zoom_step).clamp(0.75, 2.0);
        }

        let delta = self.target_zoom_factor - self.zoom_factor;
        if delta.abs() > 0.0005 {
            self.zoom_factor += delta * 0.18;
            ctx.request_repaint();
        }
        ctx.set_zoom_factor(self.zoom_factor);

        // Smoothly animate explanation font size changes.
        let text_delta = self.target_text_size - self.text_size;
        if text_delta.abs() > 0.01 {
            self.text_size += text_delta * 0.2;
            ctx.request_repaint();
        }

        // Auto-hide controls bar when not in use, but keep a subtle visible strip.
        let now = ctx.input(|i| i.time);
        let near_top = ctx
            .input(|i| i.pointer.hover_pos().map(|p| p.y <= 130.0))
            .unwrap_or(false);
        let popup_open = ctx.memory(|m| m.any_popup_open());

        if near_top || popup_open {
            // Delay hide briefly after active interaction.
            self.controls_hold_until = now + 0.9;
        }

        let keep_open = near_top || popup_open || now < self.controls_hold_until;
        self.target_controls_reveal = if keep_open { 1.0 } else { 0.12 };
        let controls_delta = self.target_controls_reveal - self.controls_reveal;
        if controls_delta.abs() > 0.001 {
            self.controls_reveal += controls_delta * 0.22;
            ctx.request_repaint();
        }

        apply_theme(ctx, self.theme_mode);
        apply_global_text_size(ctx, self.text_size);

        TopBottomPanel::top("top_bar_main")
            .show_separator_line(false)
            .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("Rust Concepts Explorer")
                        .color(self.theme_mode.accent())
                        .strong()
                        .size(self.text_size + 7.0),
                );
                ui.separator();
                ui.label(
                    RichText::new("Learning console for the Rust programming language.")
                        .color(self.theme_mode.muted_text()),
                );
                ui.separator();
            });
        });

        let controls_height = 7.0 + self.controls_reveal * 54.0;
        let controls_fill = match self.theme_mode {
            ThemeMode::Dark => {
                let alpha = (78.0 + self.controls_reveal * 72.0) as u8;
                Color32::from_rgba_unmultiplied(24, 33, 47, alpha)
            }
            ThemeMode::Light => {
                let alpha = (236.0 + self.controls_reveal * 18.0) as u8;
                Color32::from_rgba_unmultiplied(242, 248, 255, alpha)
            }
        };
        let controls_stroke = egui::Stroke::new(
            1.0,
            match self.theme_mode {
                ThemeMode::Dark => self.theme_mode.accent_soft().gamma_multiply(0.45),
                ThemeMode::Light => Color32::TRANSPARENT,
            },
        );

        TopBottomPanel::top("top_bar_controls")
            .exact_height(controls_height)
            .show_separator_line(false)
            .frame(
                egui::Frame::default()
                    .fill(controls_fill)
                    .stroke(controls_stroke),
            )
            .show(ctx, |ui| {
                if self.theme_mode == ThemeMode::Light {
                    // Always paint a light divider cue in light mode.
                    let rect = ui.max_rect();
                    let divider_rect = egui::Rect::from_min_max(
                        egui::pos2(rect.min.x, rect.max.y - 1.5),
                        egui::pos2(rect.max.x, rect.max.y),
                    );
                    ui.painter()
                        .rect_filled(divider_rect, 0.0, Color32::from_rgb(206, 226, 244));
                }

                if self.controls_reveal < 0.35 {
                    // Subtle collapsed indicator: visible, but not distracting.
                    let indicator = match self.theme_mode {
                        ThemeMode::Dark => Color32::from_rgb(41, 60, 77),
                        ThemeMode::Light => Color32::from_rgb(210, 229, 246),
                    };
                    let rect = ui.max_rect();
                    let line_rect = egui::Rect::from_min_max(
                        egui::pos2(rect.min.x + 6.0, rect.max.y - 3.5),
                        egui::pos2(rect.max.x - 6.0, rect.max.y - 1.0),
                    );
                    ui.painter().rect_filled(line_rect, 2.0, indicator);

                    ui.horizontal_centered(|ui| {
                        ui.label(
                            RichText::new("Controls")
                                .color(self.theme_mode.muted_text().gamma_multiply(0.55)),
                        );
                    });
                    return;
                }

                ui.horizontal_wrapped(|ui| {
                    let label = match self.theme_mode {
                        ThemeMode::Dark => "Switch to Light",
                        ThemeMode::Light => "Switch to Dark",
                    };
                    if ui.button(label).clicked() {
                        self.theme_mode = self.theme_mode.toggle();
                    }

                    egui::ComboBox::from_label("Zoom")
                        .selected_text(format!("{:.0}%", self.target_zoom_factor * 100.0))
                        .show_ui(ui, |ui| {
                            for zoom in zoom_options() {
                                if ui
                                    .selectable_label(
                                        (self.target_zoom_factor - zoom).abs() < 0.001,
                                        format!("{:.0}%", zoom * 100.0),
                                    )
                                    .clicked()
                                {
                                    self.target_zoom_factor = zoom;
                                }
                            }
                        });

                    egui::ComboBox::from_label("Text Size")
                        .selected_text(format!("{:.0}", self.target_text_size))
                        .show_ui(ui, |ui| {
                            for size in text_size_options() {
                                if ui
                                    .selectable_label(
                                        (self.target_text_size - size).abs() < 0.1,
                                        format!("{size:.0}"),
                                    )
                                    .clicked()
                                {
                                    self.target_text_size = size;
                                }
                            }
                        });
                });
            });

        SidePanel::left("learning_path")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Guided Path");
                ui.label("Start at the top, then continue downward.");
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!(
                        "Progress: {} / {} completed",
                        self.completed.len(),
                        self.concepts.len()
                    ))
                    .color(self.theme_mode.accent()),
                );
                ui.add(egui::ProgressBar::new(self.progress_ratio()).text("Concept completion"));
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let concepts_snapshot = self.concepts.clone();
                    for (index, concept) in concepts_snapshot.iter().enumerate() {
                        let number = index + 1;
                        let seen = self.opened.contains(&concept.id);
                        let done = self.completed.contains(&concept.id);
                        let indicator = if done {
                            "Completed"
                        } else if seen {
                            "Opened"
                        } else {
                            "New"
                        };
                        let text = format!("{number:02}. {} ({indicator})", concept.title);
                        let mut button = egui::Button::new(text);
                        if done {
                            button = button.fill(Color32::from_rgb(24, 108, 72));
                        }
                        if ui.add(button).clicked() {
                            self.open_concept(concept.id);
                        }
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.theme_mode.background()))
            .show(ctx, |ui| match self.screen {
                Screen::Concept(id) => ui::concept_view::show_concept_view(ui, self, id),
            });
    }
}

fn text_size_options() -> Vec<f32> {
    (15..=35).map(|n| n as f32).collect()
}

fn zoom_options() -> [f32; 6] {
    [0.75, 1.0, 1.25, 1.5, 1.75, 2.0]
}

fn apply_global_text_size(ctx: &egui::Context, base_size: f32) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Small, FontId::proportional((base_size - 2.0).max(8.0))),
        (TextStyle::Body, FontId::proportional(base_size)),
        (TextStyle::Button, FontId::proportional(base_size)),
        (TextStyle::Monospace, FontId::monospace(base_size)),
        (TextStyle::Heading, FontId::proportional(base_size + 6.0)),
    ]
    .into();
    ctx.set_style(style);
}
