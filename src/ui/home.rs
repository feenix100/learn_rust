//! Home screen UI.
//!
//! This page gives a strong "Start Here" flow so beginners know what to click.

use eframe::egui::{self, RichText, Ui};

use crate::{
    app::RustConceptsApp,
    models::ConceptId,
    ui::components::{keyword_chip, panel_card},
};

pub fn show_home(ui: &mut Ui, app: &mut RustConceptsApp) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        panel_card(ui, app.theme_mode, "Begin Your Rust Journey", |ui| {
            ui.label(
                "Rust is a modern systems language focused on safety, speed, and fearless refactoring. \
                 This app guides you from fundamentals to deeper concepts with interactive visual cues.",
            );
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                keyword_chip(ui, app.theme_mode, "Start Here");
                keyword_chip(ui, app.theme_mode, "Follow Guided Order");
                keyword_chip(ui, app.theme_mode, "Open Any Concept Anytime");
            });
            ui.add_space(8.0);
            if ui.button("Start With Variables and Mutability").clicked() {
                app.open_concept(ConceptId::VariablesMutability);
            }
        });

        ui.add_space(12.0);
        panel_card(ui, app.theme_mode, "How To Use This App", |ui| {
            ui.label("1. Use the left sidebar as your learning path.");
            ui.label("2. Each concept has Explanation and Code tabs.");
            ui.label("3. Click teaching snippets to trigger visual state changes.");
            ui.label("4. Mark concepts complete as you finish each recap.");
        });

        ui.add_space(12.0);
        panel_card(ui, app.theme_mode, "Concept Relationship Map", |ui| {
            ui.label(
                RichText::new(
                    "Variables -> Functions -> Types -> Control Flow -> Ownership \
                     -> Borrowing -> Structs/Enums -> Pattern Matching -> Error Handling \
                     -> Modules -> Traits/Generics -> Lifetimes",
                )
                .strong(),
            );
            ui.label("Tip: Ownership and borrowing unlock most of Rust's design decisions.");
        });

        ui.add_space(12.0);
        panel_card(ui, app.theme_mode, "Special Learning Sections", |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Open Modules Concept (Section 15)").clicked() {
                    app.open_concept(ConceptId::ModulesOrganization);
                }
            });
        });

        ui.add_space(12.0);
        panel_card(ui, app.theme_mode, "Rust Keywords Quick Guide", |ui| {
            ui.label("This section summarizes common beginner keywords and what they do:");
            ui.horizontal_wrapped(|ui| {
                keyword_chip(ui, app.theme_mode, "let");
                ui.label("creates a variable binding");
                keyword_chip(ui, app.theme_mode, "mut");
                ui.label("allows that binding to change");
                keyword_chip(ui, app.theme_mode, "fn");
                ui.label("declares a function");
            });
            ui.horizontal_wrapped(|ui| {
                keyword_chip(ui, app.theme_mode, "mod");
                ui.label("declares a module");
                keyword_chip(ui, app.theme_mode, "pub");
                ui.label("makes items visible outside the module");
                keyword_chip(ui, app.theme_mode, "use");
                ui.label("imports paths into local scope");
            });
            ui.add_space(6.0);
            ui.label("For deeper module keyword practice, open section 15.");
        });
    });
}
