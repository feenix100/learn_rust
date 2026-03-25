//! Guided beginner code walkthrough.
//!
//! Users click syntax tokens and read targeted explanations.

use eframe::egui::{self, RichText, Ui};

use crate::{
    app::RustConceptsApp,
    models::WalkthroughStep,
    ui::components::{keyword_chip, panel_card},
};

pub fn show_code_walkthrough(ui: &mut Ui, app: &mut RustConceptsApp) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        panel_card(ui, app.theme_mode, "Guided Code Walkthrough", |ui| {
            ui.label(
                "Read this file top-to-bottom: module declaration -> public function -> variable binding -> mutation -> output -> import path.",
            );
            ui.code(
                "mod lessons;\n\n\
                 pub fn run() {\n\
                     let mut score = 0;\n\
                     score += 1;\n\
                     println!(\"{score}\");\n\
                 }\n\n\
                 use crate::lessons::intro;",
            );
            ui.add_space(6.0);
            ui.label(
                RichText::new(
                    "Tip: Rust code becomes easier when you ask two questions: \
                     \"Who can access this?\" and \"Who owns this value?\"",
                )
                .italics(),
            );
        });

        ui.add_space(8.0);
        panel_card(ui, app.theme_mode, "Click-To-Explain Keywords", |ui| {
            ui.label("Select a keyword to see a deeper explanation and a tiny practical example.");
            ui.horizontal_wrapped(|ui| {
                if ui.button("let").clicked() {
                    app.walkthrough_step = WalkthroughStep::Let;
                }
                if ui.button("mut").clicked() {
                    app.walkthrough_step = WalkthroughStep::Mut;
                }
                if ui.button("fn").clicked() {
                    app.walkthrough_step = WalkthroughStep::Fn;
                }
                if ui.button("mod").clicked() {
                    app.walkthrough_step = WalkthroughStep::Mod;
                }
                if ui.button("pub").clicked() {
                    app.walkthrough_step = WalkthroughStep::Pub;
                }
                if ui.button("use").clicked() {
                    app.walkthrough_step = WalkthroughStep::Use;
                }
            });
        });

        ui.add_space(8.0);
        panel_card(ui, app.theme_mode, "Current Explanation", |ui| {
            let (token, line_focus, purpose, plain_english, common_mistake, mini_example) =
                match app.walkthrough_step {
                    WalkthroughStep::Let => (
                        "let",
                        "Line focus: `let mut score = 0;`",
                        "Purpose: Creates a new binding (a named value).",
                        "Plain English: \"Create a variable called score and store 0 in it.\"",
                        "Common mistake: Expecting `let` bindings to be mutable without `mut`.",
                        "Mini example:\nlet name = \"Rust\";",
                    ),
                    WalkthroughStep::Mut => (
                        "mut",
                        "Line focus: `let mut score = 0;` and `score += 1;`",
                        "Purpose: Marks a binding as changeable.",
                        "Plain English: \"This value is allowed to change after creation.\"",
                        "Common mistake: Adding `mut` everywhere instead of only where needed.",
                        "Mini example:\nlet mut count = 1;\ncount += 1;",
                    ),
                    WalkthroughStep::Fn => (
                        "fn",
                        "Line focus: `pub fn run() { ... }`",
                        "Purpose: Declares a function (reusable behavior block).",
                        "Plain English: \"Define a callable action named run.\"",
                        "Common mistake: Forgetting return types on functions that should return data.",
                        "Mini example:\nfn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
                    ),
                    WalkthroughStep::Mod => (
                        "mod",
                        "Line focus: `mod lessons;`",
                        "Purpose: Declares a module boundary and includes code from another file/module.",
                        "Plain English: \"This project has a module named lessons.\"",
                        "Common mistake: Declaring `mod` without matching file/module structure.",
                        "Mini example:\nmod ui;\nmod content;",
                    ),
                    WalkthroughStep::Pub => (
                        "pub",
                        "Line focus: `pub fn run() { ... }`",
                        "Purpose: Makes an item accessible from outside its current module.",
                        "Plain English: \"Other modules are allowed to call this function.\"",
                        "Common mistake: Assuming items are public by default (they are private by default).",
                        "Mini example:\npub struct User {\n    pub name: String,\n}",
                    ),
                    WalkthroughStep::Use => (
                        "use",
                        "Line focus: `use crate::lessons::intro;`",
                        "Purpose: Brings a long path into local scope for cleaner code.",
                        "Plain English: \"Import this path so we can refer to it more easily.\"",
                        "Common mistake: Using `use` but still writing full paths everywhere.",
                        "Mini example:\nuse crate::models::Concept;",
                    ),
                };

            keyword_chip(ui, app.theme_mode, token);
            ui.label(RichText::new(line_focus).strong());
            ui.label(purpose);
            ui.label(plain_english);
            ui.add_space(6.0);
            ui.label(RichText::new("Common Beginner Mistake").strong());
            ui.label(common_mistake);
            ui.add_space(6.0);
            ui.label(RichText::new("Mini Example").strong());
            ui.code(mini_example);
            ui.add_space(6.0);
            ui.label(
                RichText::new(
                    "Next step: click another keyword and compare how it changes ownership, visibility, or structure.",
                )
                .italics(),
            );
        });

        ui.add_space(8.0);
        panel_card(ui, app.theme_mode, "Line-by-Line Mental Model", |ui| {
            ui.label("1. `mod lessons;` => project structure: where code lives.");
            ui.label("2. `pub fn run()` => visibility + behavior entry point.");
            ui.label("3. `let mut score = 0;` => binding with optional mutability.");
            ui.label("4. `score += 1;` => state change (only legal because of `mut`).");
            ui.label("5. `println!(...)` => observable output.");
            ui.label("6. `use crate::...` => path management for readability.");
        });

        ui.add_space(8.0);
        panel_card(ui, app.theme_mode, "Teaching Note", |ui| {
            ui.label(
                RichText::new(
                    "This walkthrough explains syntax meaning and structure. \
                     It is intentionally not a live compiler or code editor.",
                )
                .italics(),
            );
        });
    });
}
