//! # egui_sauge
//!
//! A fresh, natural design system for [egui] — sage palette, warm
//! neutrals, WCAG AA contrast, and a ready-to-use component library
//! aimed at IT applications.
//!
//! ## Quickstart
//!
//! ```no_run
//! use eframe::egui;
//! use egui_sauge::{Palette, Density, apply_theme_with, install_fonts, set_locale, Locale};
//!
//! # fn run() -> eframe::Result<()> {
//! eframe::run_native(
//!     "My app",
//!     eframe::NativeOptions::default(),
//!     Box::new(|cc| {
//!         install_fonts(&cc.egui_ctx);
//!         apply_theme_with(&cc.egui_ctx, &Palette::light(), Density::Comfortable);
//!         set_locale(&cc.egui_ctx, Locale::En); // or Locale::Fr
//!         Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)
//!     }),
//! )
//! # }
//! # #[derive(Default)]
//! # struct MyApp;
//! # impl eframe::App for MyApp {
//! #     fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}
//! # }
//! ```
//!
//! ## What you get
//!
//! - [`Palette`] — 20 semantic color roles, light + dark modes, WCAG AA
//!   validated by `tests/contrast.rs`.
//! - [`SPACING`] / [`RADIUS`] / [`Elevation`] — numerical token tables.
//! - [`Icon`] — ~80 named icons backed by [Phosphor](https://phosphoricons.com/),
//!   plus [`Icon::Glyph`] and [`Icon::Custom`] escape hatches.
//! - [`Density`] preset (`Comfortable` / `Compact`) and [`Locale`] (`En` / `Fr`)
//!   read from [`egui::Context`] by every component.
//! - A [`components`] module with buttons, cards, alerts, inputs, dialogs,
//!   toasts, navigation primitives, menus + submenus, etc.
//!
//! ## Modules
//!
//! - [`components`] — opinionated, ready-to-use UI atoms and containers.
//!
//! ## See also
//!
//! - `GUIDE.md` — UX/UI playbook (composition, navigation, modal vs side
//!   panel, button order, i18n…).
//! - `cargo run --example showcase` — every component in one window.
//!
//! [egui]: https://github.com/emilk/egui

#![doc(html_root_url = "https://docs.rs/egui_sauge/1.1.0")]
// Pedantic lints we deliberately silence:
// - `must_use_candidate` / `return_self_not_must_use`: every builder here is
//   chain-style; #[must_use] would be noise on ~180 setters.
// - `wildcard_imports` inside small private match blocks (locale tr) is
//   intentional and local.
#![allow(
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::enum_glob_use,
    clippy::elidable_lifetime_names,
    // Multiple variants intentionally produce the same color (Primary &
    // Danger → text_on_brand; Secondary & Ghost → bg_pressed). Collapsing
    // to `_ =>` would hide intent.
    clippy::match_same_arms
)]

mod icons;
mod text;
mod theme;

pub mod components;

pub use icons::{Icon, IconPainter};
pub use text::{install_fonts, install_phosphor_variant};
pub use theme::{
    apply::{apply_theme, apply_theme_with},
    density::Density,
    elevation::Elevation,
    locale::Locale,
    palette::Palette,
    state::{density_of, locale_of, palette_of, reduce_motion, set_locale, set_reduce_motion},
    tokens::{RADIUS, Radius, SPACING, Spacing},
};
