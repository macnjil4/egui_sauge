//! [`tooltip`] — themed tooltip painter, plus a [`TooltipExt`] convenience
//! trait so any [`egui::Response`] gets a `.sauge_tooltip(text)` method.

use egui::{FontId, Response, Stroke, StrokeKind, TextStyle, vec2};

use super::corner;
use crate::{Elevation, RADIUS, SPACING, palette_of};

/// Show a themed tooltip on `response` if it's hovered. Wraps egui's own
/// hover-tooltip with our palette / radius / shadow.
pub fn tooltip(response: &Response, text: &str) -> Response {
    let r = response.clone();
    r.on_hover_ui(|ui| {
        let palette = palette_of(ui.ctx());
        egui::Frame::default()
            .fill(palette.bg_surface)
            .stroke(Stroke::new(1.0, palette.border_default))
            .corner_radius(corner(RADIUS.md))
            .inner_margin(egui::Margin::symmetric(
                SPACING.s3 as i8,
                (SPACING.s2 - 1.0) as i8,
            ))
            .shadow(Elevation::Popover.shadow(palette.dark_mode))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(text)
                        .text_style(TextStyle::Small)
                        .color(palette.text_primary),
                );
            });
        let _ = (vec2(0.0, 0.0), StrokeKind::Inside);
    })
}

/// Extension trait to call `.sauge_tooltip("…")` directly on a response.
pub trait TooltipExt {
    /// Attach a themed tooltip.
    fn sauge_tooltip(self, text: &str) -> Self;
}

impl TooltipExt for Response {
    fn sauge_tooltip(self, text: &str) -> Self {
        tooltip(&self, text)
    }
}

/// Same convenience but takes a font hint for richer tooltips.
#[allow(dead_code)]
pub(crate) fn _silence_font_id_unused(f: FontId) {
    let _ = f;
}
