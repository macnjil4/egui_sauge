//! [`Workbench`] — IDE-style top-level layout. Procedural builder that
//! orchestrates the standard six zones (topbar, two activity rails,
//! left / right / bottom tool windows, status bar, central) in the
//! right order without locking your `&mut self` for the whole frame.
//!
//! Each method renders its slot immediately, so closures don't outlive
//! each other — you can freely call `&mut self` methods from inside
//! every slot.
//!
//! ```ignore
//! Workbench::begin(ui)
//!     .top(|ui| { self.render_topbar(ui); })
//!     .left_activity(|ui| { ActivityBar::new(...).show(ui); })
//!     .left("Project", self.left.is_some(), |ui| { /* file tree */ })
//!     .bottom("Terminal", self.terminal_open, |ui| { /* terminal */ })
//!     .status(|ui| StatusBar::show(ui, |_|{}, |_|{}, |_|{}))
//!     .central(|ui| { /* editor */ });
//! ```

use egui::Ui;

use super::splitter::{Splitter, SplitterSide};
use crate::SPACING;

/// IDE-style workbench shell. Construct with [`Self::begin`] then
/// chain slot methods in any order. Finalise with [`Self::central`].
///
/// Order doesn't matter for correctness — the builder is just sugar
/// over `egui::Panel` calls — but conventional order is top, status,
/// activity rails, tool windows, central.
pub struct Workbench<'ui> {
    ui: &'ui mut Ui,
}

impl<'ui> Workbench<'ui> {
    /// Begin a workbench layout on `ui`.
    pub fn begin(ui: &'ui mut Ui) -> Self {
        Self { ui }
    }

    /// Top strip — title bar, project tabs, run config…
    pub fn top(self, body: impl FnOnce(&mut Ui)) -> Self {
        let palette = crate::palette_of(self.ui.ctx());
        egui::Panel::top("sauge_workbench_top")
            .resizable(false)
            .frame(
                egui::Frame::default()
                    .fill(palette.bg_surface_alt)
                    .stroke(egui::Stroke::new(1.0, palette.border_subtle))
                    .inner_margin(egui::Margin::symmetric(
                        SPACING.s2 as i8,
                        (SPACING.s2 - 1.0) as i8,
                    )),
            )
            .show_inside(self.ui, |ui| {
                ui.set_height(36.0);
                ui.horizontal_centered(body);
            });
        self
    }

    /// Status bar (28 px, always visible).
    pub fn status(self, body: impl FnOnce(&mut Ui)) -> Self {
        let palette = crate::palette_of(self.ui.ctx());
        egui::Panel::bottom("sauge_workbench_status")
            .resizable(false)
            .frame(egui::Frame::default().fill(palette.bg_surface_alt))
            .show_inside(self.ui, body);
        self
    }

    /// Vertical left activity rail (icon-only, 44 px wide).
    pub fn left_activity(self, body: impl FnOnce(&mut Ui)) -> Self {
        let palette = crate::palette_of(self.ui.ctx());
        egui::Panel::left("sauge_workbench_left_activity")
            .resizable(false)
            .min_size(44.0)
            .max_size(44.0)
            .frame(
                egui::Frame::default()
                    .fill(palette.bg_surface_alt)
                    .stroke(egui::Stroke::new(1.0, palette.border_subtle))
                    .inner_margin(egui::Margin::ZERO),
            )
            .show_inside(self.ui, body);
        self
    }

    /// Vertical right activity rail (icon-only, 44 px wide).
    pub fn right_activity(self, body: impl FnOnce(&mut Ui)) -> Self {
        let palette = crate::palette_of(self.ui.ctx());
        egui::Panel::right("sauge_workbench_right_activity")
            .resizable(false)
            .min_size(44.0)
            .max_size(44.0)
            .frame(
                egui::Frame::default()
                    .fill(palette.bg_surface_alt)
                    .stroke(egui::Stroke::new(1.0, palette.border_subtle))
                    .inner_margin(egui::Margin::ZERO),
            )
            .show_inside(self.ui, body);
        self
    }

    /// Resizable left tool window (file tree, commit, etc.). When
    /// `open` is `false` the slot is skipped.
    pub fn left(self, title: &str, open: bool, body: impl FnOnce(&mut Ui)) -> Self {
        if open {
            Splitter::new("sauge_workbench_left", SplitterSide::Left)
                .title(title)
                .default_size(240.0)
                .show(self.ui, body);
        }
        self
    }

    /// Resizable right tool window. Skipped when `open=false`.
    pub fn right(self, title: &str, open: bool, body: impl FnOnce(&mut Ui)) -> Self {
        if open {
            Splitter::new("sauge_workbench_right", SplitterSide::Right)
                .title(title)
                .default_size(240.0)
                .show(self.ui, body);
        }
        self
    }

    /// Resizable bottom tool window (terminal, problems, debug…).
    /// Skipped when `open=false`.
    pub fn bottom(self, title: &str, open: bool, body: impl FnOnce(&mut Ui)) -> Self {
        if open {
            Splitter::new("sauge_workbench_bottom", SplitterSide::Bottom)
                .title(title)
                .default_size(200.0)
                .show(self.ui, body);
        }
        self
    }

    /// Central editor area — call this last. The closure receives the
    /// remaining space after every other slot has been carved out.
    pub fn central(self, body: impl FnOnce(&mut Ui)) {
        let palette = crate::palette_of(self.ui.ctx());
        egui::Frame::default()
            .fill(palette.bg_app)
            .show(self.ui, body);
    }
}
