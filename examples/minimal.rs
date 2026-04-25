//! Minimal demo of the theme applied to a handful of widgets.
//!
//! Run with: `cargo run --example minimal`.

use eframe::egui;
use egui_sauge::{Palette, apply_theme, install_fonts};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_sauge — minimal",
        options,
        Box::new(|cc| {
            install_fonts(&cc.egui_ctx);
            apply_theme(&cc.egui_ctx, &Palette::light());
            Ok(Box::new(App::default()) as Box<dyn eframe::App>)
        }),
    )
}

#[derive(Default)]
struct App {
    name: String,
    dark: bool,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("egui_sauge");
        ui.label("A fresh, natural design system for egui.");
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
        });

        ui.add_space(8.0);
        if ui.checkbox(&mut self.dark, "Dark mode").changed() {
            let p = if self.dark {
                Palette::dark()
            } else {
                Palette::light()
            };
            apply_theme(ui.ctx(), &p);
        }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            let _ = ui.button("Primary");
            let _ = ui.button("Secondary");
        });
    }
}
