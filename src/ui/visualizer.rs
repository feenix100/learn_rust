//! Ownership and borrowing visualizer.
//!
//! This widget uses simple boxes/arrows text to reinforce Rust's borrow rules.

use eframe::egui::{Color32, RichText, Ui};

use crate::{
    app::RustConceptsApp,
    models::OwnershipMode,
    ui::components::{keyword_chip, panel_card},
};

pub fn show_ownership_visualizer_content(ui: &mut Ui, app: &mut RustConceptsApp) {
    panel_card(ui, app.theme_mode, "Ownership and Borrowing Visualizer", |ui| {
        ui.label(
            "Use this panel to see how Rust tracks who can read or write a value at any moment.",
        );
        ui.label("Read each mode in order: Move -> Immutable Borrow -> Mutable Borrow.");
        ui.horizontal_wrapped(|ui| {
            if ui.button("Move Value").clicked() {
                app.ownership_mode = OwnershipMode::Move;
            }
            if ui.button("Immutable Borrow").clicked() {
                app.ownership_mode = OwnershipMode::ImmutableBorrow;
            }
            if ui.button("Mutable Borrow").clicked() {
                app.ownership_mode = OwnershipMode::MutableBorrow;
            }
        });
    });

    ui.add_space(10.0);
    panel_card(ui, app.theme_mode, "State Preview", |ui| {
        match app.ownership_mode {
            OwnershipMode::Move => {
                keyword_chip(ui, app.theme_mode, "Move");
                ui.label(RichText::new("What changed").strong());
                ui.label("owner_a [String]  --->  owner_b [String]");
                ui.label(
                    RichText::new(
                        "The value's ownership was transferred. `owner_a` is now invalid for that value.",
                    )
                    .size(app.text_size),
                );
                ui.add_space(4.0);
                ui.label(RichText::new("Compiler protection").strong());
                ui.colored_label(
                    Color32::from_rgb(255, 190, 150),
                    "Use-after-move is blocked at compile time.",
                );
                ui.code("let a = String::from(\"rust\");\nlet b = a;\n// println!(\"{a}\"); // error");
            }
            OwnershipMode::ImmutableBorrow => {
                keyword_chip(ui, app.theme_mode, "&T");
                ui.label(RichText::new("What changed").strong());
                ui.label("owner [String]");
                ui.label("  |--> read_ref_1");
                ui.label("  |--> read_ref_2");
                ui.label(
                    RichText::new(
                        "Many read-only references can coexist because none can mutate the data.",
                    )
                    .size(app.text_size),
                );
                ui.add_space(4.0);
                ui.label(RichText::new("Compiler protection").strong());
                ui.colored_label(
                    Color32::from_rgb(150, 220, 255),
                    "Blocks mutation while immutable borrows are active.",
                );
                ui.code("let name = String::from(\"Rust\");\nlet r1 = &name;\nlet r2 = &name;\n// name.push_str(\"!\"); // error");
            }
            OwnershipMode::MutableBorrow => {
                keyword_chip(ui, app.theme_mode, "&mut T");
                ui.label(RichText::new("What changed").strong());
                ui.label("owner [String]");
                ui.label("  |--> write_ref (exclusive)");
                ui.label(
                    RichText::new(
                        "A mutable reference gets exclusive access, so no other reads/writes can overlap.",
                    )
                    .size(app.text_size),
                );
                ui.add_space(4.0);
                ui.label(RichText::new("Compiler protection").strong());
                ui.colored_label(
                    Color32::from_rgb(255, 160, 160),
                    "Prevents aliasing bugs and data races by enforcing single-writer access.",
                );
                ui.code("let mut name = String::from(\"Rust\");\nlet r = &mut name;\nr.push_str(\"acean\");\n// let r2 = &name; // error while `r` is active");
            }
        }
    });

    ui.add_space(10.0);
    panel_card(ui, app.theme_mode, "How To Reason About It", |ui| {
        ui.label("1. Who owns this value right now?");
        ui.label("2. Are we sharing read access (`&T`) or exclusive write access (`&mut T`)?");
        ui.label("3. Are any references still alive in this scope?");
        ui.label(
            RichText::new(
                "If Rust rejects a borrow, it is usually preventing overlapping access that could be unsafe.",
            )
            .strong(),
        );
    });

    ui.add_space(10.0);
    panel_card(ui, app.theme_mode, "Why Rust Enforces This", |ui| {
        ui.label(
            RichText::new(
                "Ownership rules move memory safety checks to compile time, which removes entire classes of runtime bugs.",
            )
            .strong(),
        );
        ui.label("You get predictable cleanup, no null ownership states, and safer concurrency by default.");
    });
}
