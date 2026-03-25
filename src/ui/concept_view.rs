//! Concept detail screens.
//!
//! This file renders both:
//! - generic concept pages
//! - dedicated module-system teaching screen

use eframe::egui::{self, Color32, RichText, Ui};

use crate::{
    app::RustConceptsApp,
    models::{ConceptId, VisualMode},
    navigation::ConceptTab,
    ui::{
        components::{keyword_chip, panel_card},
        visualizer,
    },
};

pub fn show_concept_view(ui: &mut Ui, app: &mut RustConceptsApp, id: ConceptId) {
    let Some(concept) = app.concept_by_id(id).cloned() else {
        ui.label("Concept not found.");
        return;
    };

    egui::ScrollArea::vertical().show(ui, |ui| {
        panel_card(ui, app.theme_mode, concept.title, |ui| {
            ui.label(
                RichText::new(concept.why_it_matters)
                    .strong()
                    .size(app.text_size + 0.5),
            );
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(app.concept_tab == ConceptTab::Explanation, "Explanation")
                    .clicked()
                {
                    app.concept_tab = ConceptTab::Explanation;
                }
                if ui
                    .selectable_label(app.concept_tab == ConceptTab::Code, "Code")
                    .clicked()
                {
                    app.concept_tab = ConceptTab::Code;
                }
            });
        });

        ui.add_space(10.0);
        match app.concept_tab {
            ConceptTab::Explanation => show_explanation(ui, app, id, &concept),
            ConceptTab::Code => show_code(ui, app, &concept),
        }

        if id == ConceptId::ModulesOrganization {
            ui.add_space(10.0);
            show_module_system_deep_dive_content(ui, app);
        }
        if id == ConceptId::Ownership {
            ui.add_space(10.0);
            panel_card(ui, app.theme_mode, "Ownership Visualizer (Embedded)", |ui| {
                ui.label(
                    RichText::new(
                        "This concept includes the full ownership visualizer so you can learn the rule and immediately inspect the behavior.",
                    )
                    .size(app.text_size),
                );
            });
            ui.add_space(8.0);
            visualizer::show_ownership_visualizer_content(ui, app);
        }

        ui.add_space(10.0);
        panel_card(ui, app.theme_mode, "Quick Recap", |ui| {
            ui.label(RichText::new(concept.recap).size(app.text_size));
            if ui.button("Mark Concept Complete").clicked() {
                app.mark_completed(id);
            }
        });
    });
}

fn show_explanation(
    ui: &mut Ui,
    app: &RustConceptsApp,
    id: ConceptId,
    concept: &crate::models::Concept,
) {
    panel_card(ui, app.theme_mode, "Beginner-Friendly Explanation", |ui| {
        ui.label(RichText::new(concept.explanation).size(app.text_size));
    });

    if id == ConceptId::WhatIsRust {
        ui.add_space(8.0);
        panel_card(ui, app.theme_mode, "Core Rust Keywords Guide", |ui| {
            ui.label(RichText::new("These keywords appear throughout beginner Rust code:").size(app.text_size));
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
                ui.label("makes an item visible outside its module");
                keyword_chip(ui, app.theme_mode, "use");
                ui.label("imports module paths into local scope");
            });
        });
    }

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "Common Beginner Mistake", |ui| {
        ui.label(
            RichText::new(concept.beginner_mistake)
                .size(app.text_size)
                .color(Color32::from_rgb(255, 140, 130)),
        );
    });
}

fn show_code(ui: &mut Ui, app: &mut RustConceptsApp, concept: &crate::models::Concept) {
    let snippets = concept_snippets(concept.id);
    let active_index = app
        .active_teaching_snippet
        .and_then(|(concept_id, index)| (concept_id == concept.id).then_some(index))
        .filter(|index| *index < snippets.len());

    panel_card(ui, app.theme_mode, "Sample Rust Code", |ui| {
        if let Some(index) = active_index {
            let snippet = snippets[index];
            ui.label(
                RichText::new(format!("Interactive Snippet: {}", snippet.title))
                    .strong()
                    .color(app.theme_mode.accent()),
            );
            ui.add_space(4.0);
            ui.label("Base snippet:");
            render_code_block(ui, snippet.base);
            ui.add_space(8.0);
            ui.label("Updated snippet (changed lines are highlighted):");
            render_code_with_highlight(ui, snippet.updated, snippet.changed_lines);
            ui.add_space(6.0);
            ui.label(RichText::new(snippet.note).size(app.text_size));
        } else {
            render_code_block(ui, concept.code);
            ui.separator();
            ui.label(RichText::new(concept.code_explanation).size(app.text_size));
        }
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "Clickable Teaching Snippets", |ui| {
        ui.label("These snippets are tailored to this concept. Click one to view the code delta and concept reinforcement.");
        ui.horizontal_wrapped(|ui| {
            for (index, snippet) in snippets.iter().enumerate() {
                let is_active = active_index == Some(index);
                if snippet_button(ui, app, is_active, snippet.button_label).clicked() {
                    app.visual_mode = snippet.visual_mode;
                    app.active_teaching_snippet = Some((concept.id, index));
                }
            }
            if ui.button("Reset to concept code").clicked() {
                app.active_teaching_snippet = None;
                app.visual_mode = VisualMode::Idle;
            }
        });
    });
}

fn snippet_button(
    ui: &mut Ui,
    app: &RustConceptsApp,
    is_active: bool,
    label: &str,
) -> egui::Response {
    let mut button = egui::Button::new(label);
    if is_active {
        button = button.fill(app.theme_mode.accent_soft()).stroke(egui::Stroke::new(1.4, app.theme_mode.accent()));
    }
    ui.add(button)
}

fn render_code_block(ui: &mut Ui, code: &str) {
    for line in code.lines() {
        ui.label(RichText::new(line).monospace());
    }
}

fn render_code_with_highlight(ui: &mut Ui, code: &str, changed_lines: &[usize]) {
    for (idx, line) in code.lines().enumerate() {
        let line_number = idx + 1;
        let highlight = changed_lines.contains(&line_number);
        let mut text = RichText::new(format!("{line_number:>2}  {line}")).monospace();
        if highlight {
            text = text
                .background_color(Color32::from_rgb(64, 120, 86))
                .color(Color32::from_rgb(230, 255, 236))
                .strong();
        }
        ui.label(text);
    }
}

#[derive(Clone, Copy)]
struct ConceptSnippet {
    button_label: &'static str,
    title: &'static str,
    base: &'static str,
    updated: &'static str,
    changed_lines: &'static [usize],
    note: &'static str,
    visual_mode: VisualMode,
}

fn concept_snippets(id: ConceptId) -> &'static [ConceptSnippet] {
    match id {
        ConceptId::WhatIsRust => &[ConceptSnippet {
            button_label: "println! macro",
            title: "From Program Entry to Output",
            base: "fn main() {\n}",
            updated: "fn main() {\n    println!(\"Hello, Rust learner!\");\n}",
            changed_lines: &[2],
            note: "Rust starts at `main`; `println!` is a macro for formatted console output.",
            visual_mode: VisualMode::Idle,
        }],
        ConceptId::VariablesMutability => &[ConceptSnippet {
            button_label: "Add mut + update",
            title: "Mutability in Practice",
            base: "let score = 10;",
            updated: "let mut score = 10;\nscore += 1;",
            changed_lines: &[1, 2],
            note: "Adding `mut` enables controlled state updates.",
            visual_mode: VisualMode::MutabilityEdit,
        }],
        ConceptId::Functions => &[ConceptSnippet {
            button_label: "Return expression",
            title: "Function Return Value",
            base: "fn add(a: i32, b: i32) -> i32 {\n    return a + b;\n}",
            updated: "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
            changed_lines: &[2],
            note: "Idiomatic Rust returns the last expression without `return` or a semicolon.",
            visual_mode: VisualMode::Idle,
        }],
        ConceptId::Types => &[ConceptSnippet {
            button_label: "Annotate explicit types",
            title: "Type Clarity",
            base: "let count = 3;\nlet ratio = 0.5;",
            updated: "let count: i32 = 3;\nlet ratio: f64 = 0.5;",
            changed_lines: &[1, 2],
            note: "Type annotations make intent explicit and reduce ambiguity.",
            visual_mode: VisualMode::Idle,
        }],
        ConceptId::ControlFlow => &[
            ConceptSnippet {
                button_label: "match success arm",
                title: "Control Flow with match",
                base: "let mood = \"focused\";\nlet message = \"\";",
                updated: "let mood = \"focused\";\nlet message = match mood {\n    \"focused\" => \"Keep building\",\n    _ => \"Take a break\",\n};",
                changed_lines: &[2, 3, 4, 5],
                note: "`match` enforces handling all relevant branches.",
                visual_mode: VisualMode::MatchHappyPath,
            },
            ConceptSnippet {
                button_label: "match fallback arm",
                title: "Fallback Branch",
                base: "let mood = \"tired\";\nlet message = \"\";",
                updated: "let mood = \"tired\";\nlet message = match mood {\n    \"focused\" => \"Keep building\",\n    _ => \"Take a break\",\n};",
                changed_lines: &[2, 3, 4, 5],
                note: "The `_` arm catches unmatched cases safely.",
                visual_mode: VisualMode::MatchFallback,
            },
        ],
        ConceptId::Ownership => &[ConceptSnippet {
            button_label: "Move ownership",
            title: "Move Semantics",
            base: "let a = String::from(\"rust\");",
            updated: "let a = String::from(\"rust\");\nlet b = a;",
            changed_lines: &[2],
            note: "Assignment moves heap-allocated values unless explicitly cloned.",
            visual_mode: VisualMode::BorrowExclusive,
        }],
        ConceptId::BorrowingReferences => &[
            ConceptSnippet {
                button_label: "Shared borrows",
                title: "Immutable References",
                base: "let name = String::from(\"Rust\");",
                updated: "let name = String::from(\"Rust\");\nlet r1 = &name;\nlet r2 = &name;",
                changed_lines: &[2, 3],
                note: "Multiple immutable references are allowed at the same time.",
                visual_mode: VisualMode::BorrowShared,
            },
            ConceptSnippet {
                button_label: "Exclusive mutable borrow",
                title: "Mutable Reference Rules",
                base: "let mut name = String::from(\"Rust\");",
                updated: "let mut name = String::from(\"Rust\");\nlet r = &mut name;\nr.push_str(\"acean\");",
                changed_lines: &[2, 3],
                note: "Only one mutable reference can exist at a time.",
                visual_mode: VisualMode::BorrowExclusive,
            },
        ],
        ConceptId::Slices => &[ConceptSnippet {
            button_label: "Create a slice",
            title: "Borrow Part of a Value",
            base: "let text = String::from(\"Rustacean\");",
            updated: "let text = String::from(\"Rustacean\");\nlet part: &str = &text[0..4];",
            changed_lines: &[2],
            note: "Slices borrow part of data without allocating a copy.",
            visual_mode: VisualMode::BorrowShared,
        }],
        ConceptId::Structs => &[ConceptSnippet {
            button_label: "Add field to struct",
            title: "Struct Field Grouping",
            base: "struct User {\n    name: String,\n    level: u32,\n}",
            updated: "struct User {\n    name: String,\n    level: u32,\n    online: bool,\n}",
            changed_lines: &[4],
            note: "Structs model related attributes under one named type.",
            visual_mode: VisualMode::StructFocus,
        }],
        ConceptId::Enums => &[
            ConceptSnippet {
                button_label: "Switch to Running",
                title: "Enum Variant Change",
                base: "enum State { Idle, Running }\nlet state = State::Idle;",
                updated: "enum State { Idle, Running }\nlet state = State::Running;",
                changed_lines: &[2],
                note: "Enums encode explicit states instead of loose string flags.",
                visual_mode: VisualMode::EnumStateRunning,
            },
            ConceptSnippet {
                button_label: "Switch back to Idle",
                title: "Enum Variant Reset",
                base: "enum State { Idle, Running }\nlet state = State::Running;",
                updated: "enum State { Idle, Running }\nlet state = State::Idle;",
                changed_lines: &[2],
                note: "Changing variant changes legal control-flow paths.",
                visual_mode: VisualMode::EnumStateIdle,
            },
        ],
        ConceptId::PatternMatching => &[ConceptSnippet {
            button_label: "Pattern guard arm",
            title: "Destructure + Guard",
            base: "let status = Some(1);",
            updated: "let status = Some(3);\nmatch status {\n    Some(v) if v > 2 => println!(\"high\"),\n    Some(_) => println!(\"low\"),\n    None => println!(\"empty\"),\n}",
            changed_lines: &[1, 2, 3, 4, 5, 6],
            note: "Pattern guards (`if`) add conditions to specific match arms.",
            visual_mode: VisualMode::MatchHappyPath,
        }],
        ConceptId::OptionResult => &[
            ConceptSnippet {
                button_label: "Handle Ok value",
                title: "Result Success Path",
                base: "let result: Result<i32, &str> = Ok(2);",
                updated: "let result: Result<i32, &str> = Ok(2);\nlet text = match result {\n    Ok(v) => format!(\"ok: {v}\"),\n    Err(e) => format!(\"err: {e}\"),\n};",
                changed_lines: &[2, 3, 4, 5],
                note: "Use `match` to safely unwrap success and error branches.",
                visual_mode: VisualMode::MatchHappyPath,
            },
            ConceptSnippet {
                button_label: "Handle Err value",
                title: "Result Error Path",
                base: "let result: Result<i32, &str> = Err(\"offline\");",
                updated: "let result: Result<i32, &str> = Err(\"offline\");\nlet text = match result {\n    Ok(v) => format!(\"ok: {v}\"),\n    Err(e) => format!(\"err: {e}\"),\n};",
                changed_lines: &[2, 3, 4, 5],
                note: "Explicit error handling avoids hidden exceptions.",
                visual_mode: VisualMode::MatchFallback,
            },
        ],
        ConceptId::VectorsStrings => &[ConceptSnippet {
            button_label: "Grow Vec and String",
            title: "Owned Collection Mutation",
            base: "let nums = vec![1, 2, 3];\nlet text = String::from(\"Hi\");",
            updated: "let mut nums = vec![1, 2, 3];\nnums.push(4);\nlet mut text = String::from(\"Hi\");\ntext.push_str(\" Rust\");",
            changed_lines: &[1, 2, 3, 4],
            note: "Collections on the heap require `mut` for in-place growth.",
            visual_mode: VisualMode::MutabilityEdit,
        }],
        ConceptId::ErrorHandling => &[
            ConceptSnippet {
                button_label: "Use ? propagation",
                title: "Early Return on Error",
                base: "fn load(path: &str) -> Result<String, std::io::Error> {\n    let data = std::fs::read_to_string(path);\n    Ok(data.unwrap())\n}",
                updated: "fn load(path: &str) -> Result<String, std::io::Error> {\n    let data = std::fs::read_to_string(path)?;\n    Ok(data)\n}",
                changed_lines: &[2, 3],
                note: "`?` propagates errors cleanly and keeps success flow readable.",
                visual_mode: VisualMode::MatchHappyPath,
            },
            ConceptSnippet {
                button_label: "Match explicit error",
                title: "Branch on Result",
                base: "let file = std::fs::read_to_string(\"config.txt\");",
                updated: "let file = std::fs::read_to_string(\"config.txt\");\nmatch file {\n    Ok(data) => println!(\"{data}\"),\n    Err(e) => println!(\"failed: {e}\"),\n}",
                changed_lines: &[2, 3, 4, 5],
                note: "Match makes the error path explicit to readers and users.",
                visual_mode: VisualMode::MatchFallback,
            },
        ],
        ConceptId::ModulesOrganization => &[
            ConceptSnippet {
                button_label: "Declare modules",
                title: "Add mod declarations",
                base: "fn main() {\n    // app entry\n}",
                updated: "mod ui;\nmod content;\n\nfn main() {\n    // app entry\n}",
                changed_lines: &[1, 2],
                note: "`mod` organizes source into separate files/modules.",
                visual_mode: VisualMode::MatchHappyPath,
            },
            ConceptSnippet {
                button_label: "Expose with pub",
                title: "Public Visibility",
                base: "struct Concept {\n    title: &'static str,\n}",
                updated: "pub struct Concept {\n    pub title: &'static str,\n}",
                changed_lines: &[1, 2],
                note: "`pub` exposes selected items outside their module.",
                visual_mode: VisualMode::MatchHappyPath,
            },
        ],
        ConceptId::TraitsGenerics => &[ConceptSnippet {
            button_label: "Add trait bound",
            title: "Generic Function Contract",
            base: "fn show<T>(item: &T) {\n    // ...\n}",
            updated: "trait Summary { fn summary(&self) -> String; }\nfn show<T: Summary>(item: &T) {\n    println!(\"{}\", item.summary());\n}",
            changed_lines: &[1, 2, 3],
            note: "Trait bounds describe behavior required by generic types.",
            visual_mode: VisualMode::StructFocus,
        }],
        ConceptId::LifetimesIntro => &[ConceptSnippet {
            button_label: "Annotate lifetime",
            title: "Reference Relationship",
            base: "fn longest(a: &str, b: &str) -> &str {\n    if a.len() > b.len() { a } else { b }\n}",
            updated: "fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {\n    if a.len() > b.len() { a } else { b }\n}",
            changed_lines: &[1],
            note: "Lifetime parameters express how returned references relate to inputs.",
            visual_mode: VisualMode::BorrowShared,
        }],
        ConceptId::CargoBasics => &[ConceptSnippet {
            button_label: "Add dependency",
            title: "Cargo.toml Dependency",
            base: "[dependencies]\n",
            updated: "[dependencies]\nserde = \"1\"\n",
            changed_lines: &[2],
            note: "Cargo manages project dependencies from `Cargo.toml`.",
            visual_mode: VisualMode::Idle,
        }],
    }
}

fn show_module_system_deep_dive_content(ui: &mut Ui, app: &mut RustConceptsApp) {
    panel_card(ui, app.theme_mode, "Module System Deep Dive (Section 15)", |ui| {
        ui.label(
            RichText::new(
                "Modules are Rust's way of organizing code by responsibility. \
                 Think of this as a map from files -> modules -> visibility rules.",
            )
            .size(app.text_size),
        );
        ui.horizontal_wrapped(|ui| {
            keyword_chip(ui, app.theme_mode, "crate");
            keyword_chip(ui, app.theme_mode, "mod");
            keyword_chip(ui, app.theme_mode, "pub");
            keyword_chip(ui, app.theme_mode, "use");
        });
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "What Is a Crate?", |ui| {
        ui.label(
            RichText::new(
                "A crate is one Rust compilation unit. \
                 `main.rs` defines a binary crate, and `lib.rs` defines a library crate.",
            )
            .size(app.text_size),
        );
        ui.code("use crate::models::Concept;");
        ui.label(
            RichText::new(
                "`crate::` means: start from the root of the current crate, then walk the module path.",
            )
            .size(app.text_size),
        );
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "mod Declares Modules", |ui| {
        ui.label(
            RichText::new(
                "Use `mod` to tell Rust a module exists. Rust then loads code from \
                 a matching file/folder.",
            )
            .size(app.text_size),
        );
        ui.code("mod ui;\nmod content;\nmod navigation;\n");
        ui.code("src/\n  main.rs\n  content.rs\n  navigation.rs\n  ui/\n    mod.rs\n    home.rs");
        ui.label(
            RichText::new(
                "If `main.rs` says `mod ui;`, Rust looks for `src/ui.rs` or `src/ui/mod.rs`.",
            )
            .size(app.text_size),
        );
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "pub and Privacy Rules", |ui| {
        ui.label(
            RichText::new(
                "Everything is private by default. Add `pub` only when another module needs access.",
            )
            .size(app.text_size),
        );
        ui.code("mod models {\n    pub struct Concept {\n        pub title: &'static str,\n        id: u32,\n    }\n}");
        ui.label(
            RichText::new("Here, `title` is public, but `id` is private to the `models` module.")
                .size(app.text_size),
        );
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "use Imports Paths", |ui| {
        ui.label(
            RichText::new(
                "`use` shortens long paths so code stays readable and repetitive paths disappear.",
            )
            .size(app.text_size),
        );
        ui.code("use crate::ui::home::show_home;");
        ui.label(
            RichText::new(
                "After this import, call `show_home(ui, app)` directly instead of the full path.",
            )
            .size(app.text_size),
        );
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "Path Forms At a Glance", |ui| {
        ui.code("crate::ui::home::show_home   // absolute path from crate root");
        ui.code("self::helpers::format_title   // relative path from current module");
        ui.code("super::models::Concept        // go to parent module first");
        ui.label(
            RichText::new(
                "Use `crate::` when you want explicit, project-wide paths. \
                 Use `self::` and `super::` for local module relationships.",
            )
            .size(app.text_size),
        );
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "Interactive Path Visual", |ui| {
        ui.label(RichText::new("Click to simulate access scope:").size(app.text_size));
        ui.horizontal_wrapped(|ui| {
            if ui.button("Private Field Access").clicked() {
                app.visual_mode = VisualMode::MatchFallback;
            }
            if ui.button("Public Field Access").clicked() {
                app.visual_mode = VisualMode::MatchHappyPath;
            }
        });
        match app.visual_mode {
            VisualMode::MatchHappyPath => {
                ui.label(
                    RichText::new("Path resolved: public item visible via module path.")
                        .size(app.text_size)
                        .color(Color32::from_rgb(120, 240, 170)),
                );
            }
            VisualMode::MatchFallback => {
                ui.label(
                    RichText::new("Compiler-style block: private item not visible from this module.")
                        .size(app.text_size)
                        .color(Color32::from_rgb(255, 145, 145)),
                );
            }
            _ => {
                ui.label(RichText::new("No path state selected yet.").size(app.text_size));
            }
        }
    });

    ui.add_space(8.0);
    panel_card(ui, app.theme_mode, "Quick Mental Model", |ui| {
        ui.label(RichText::new("1. `mod` creates structure.").size(app.text_size));
        ui.label(RichText::new("2. `pub` opens selected doors.").size(app.text_size));
        ui.label(RichText::new("3. `use` keeps call sites clean.").size(app.text_size));
        ui.label(
            RichText::new(
                "If something is inaccessible, first check module path, then check `pub` visibility.",
            )
            .size(app.text_size),
        );
    });
}

