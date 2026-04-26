//! [`Drawer`] — non-blocking right-side panel. Companion to
//! [`crate::components::Dialog`]: Dialog blocks the rest of the UI with a
//! scrim; Drawer leaves the underlying surface interactive.
//!
//! Common uses: filter panels next to a table, detail view of a selected
//! row, settings that should stay visible while the user keeps clicking
//! around.

use egui::{FontId, Stroke, StrokeKind, Ui};

use super::{alpha, corner};
use crate::{Elevation, Icon, RADIUS, SPACING, palette_of};

/// Right-side drawer.
pub struct Drawer<'a> {
    title: &'a str,
    width: f32,
    closable: bool,
    id: egui::Id,
}

impl<'a> Drawer<'a> {
    /// New drawer titled `title`.
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            width: 360.0,
            closable: true,
            id: egui::Id::new(("sauge_drawer", title)),
        }
    }
    /// Override the drawer width.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    /// Hide the close × button (caller manages visibility).
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
    /// Override the internal egui id (rare — only when stacking multiple
    /// drawers with the same title).
    pub fn id(mut self, id: egui::Id) -> Self {
        self.id = id;
        self
    }

    /// Render the drawer when `open` is true. Call this from within your
    /// app's main `ui` callback — typically *before* the central content
    /// — so the drawer reserves space on the right side. Returns `true`
    /// if the user clicked the close × button; caller should flip its own
    /// visibility flag.
    pub fn show(self, ui: &mut Ui, open: bool, body: impl FnOnce(&mut Ui)) -> bool {
        if !open {
            return false;
        }
        let palette = palette_of(ui.ctx());
        let mut close_requested = false;

        egui::Panel::right(self.id)
            .resizable(false)
            .min_size(self.width)
            .max_size(self.width)
            .frame(
                egui::Frame::default()
                    .fill(palette.bg_surface)
                    .stroke(Stroke::new(1.0, palette.border_default))
                    .inner_margin(egui::Margin::same(SPACING.s4 as i8))
                    .shadow(Elevation::Popover.shadow(palette.dark_mode)),
            )
            .show_inside(ui, |ui| {
                // Header.
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(self.title)
                            .font(FontId::new(18.0, egui::FontFamily::Proportional))
                            .color(palette.text_primary),
                    );
                    if self.closable {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let (rect, resp) = ui
                                .allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                            if resp.hovered() {
                                ui.painter().rect(
                                    rect,
                                    corner(RADIUS.sm),
                                    alpha(palette.text_primary, 0.08),
                                    Stroke::NONE,
                                    StrokeKind::Inside,
                                );
                            }
                            Icon::Close.paint(
                                ui.painter(),
                                rect.shrink(5.0),
                                palette.text_secondary,
                            );
                            if resp.clicked() {
                                close_requested = true;
                            }
                        });
                    }
                });
                ui.add_space(SPACING.s3);
                let sep_y = ui.cursor().top();
                ui.painter().line_segment(
                    [
                        egui::pos2(ui.min_rect().left(), sep_y),
                        egui::pos2(ui.min_rect().right(), sep_y),
                    ],
                    Stroke::new(1.0, palette.border_subtle),
                );
                ui.add_space(SPACING.s3);

                body(ui);
            });

        close_requested
    }
}
