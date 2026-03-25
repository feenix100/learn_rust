//! Program entry point.
//!
//! This file is intentionally small so beginners can see a clear startup path:
//! 1) configure native window options
//! 2) create the app state
//! 3) hand control to eframe's event loop

mod app;
mod content;
mod models;
mod navigation;
mod theme;
mod ui;

use app::RustConceptsApp;
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280.0, 840.0])
            .with_min_inner_size([980.0, 680.0])
            .with_title("Rust Concepts Explorer"),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Concepts Explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(RustConceptsApp::new(cc)))),
    )
}
