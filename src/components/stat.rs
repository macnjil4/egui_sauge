//! KPI tile: big number + label + optional delta indicator.

use egui::{FontId, Response, Sense, Stroke, TextStyle, Ui, Widget, vec2};

use super::corner;
use crate::{Icon, RADIUS, SPACING, palette_of};

/// Direction of a [`Stat`] delta, used to pick color and glyph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    /// Positive change (success color, up chevron).
    Up,
    /// Negative change (error color, down chevron).
    Down,
    /// Neutral (`text_secondary`, right chevron).
    Flat,
}

/// Single-number KPI tile. Typical use: "Active sessions · 1,284 · ↑ 2.3%".
pub struct Stat<'a> {
    label: &'a str,
    value: &'a str,
    delta: Option<(&'a str, Trend)>,
}

impl<'a> Stat<'a> {
    /// New stat with a label.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            value: "",
            delta: None,
        }
    }
    /// Big-number value to display under the label.
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }
    /// Delta text (e.g. `"2.3%"`, `"+12"`) and trend direction.
    pub fn delta(mut self, text: &'a str, trend: Trend) -> Self {
        self.delta = Some((text, trend));
        self
    }
}

impl<'a> Widget for Stat<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let frame = egui::Frame::default()
            .fill(palette.bg_surface)
            .stroke(Stroke::new(1.0, palette.border_subtle))
            .corner_radius(corner(RADIUS.md))
            .inner_margin(egui::Margin::same(SPACING.s4 as i8));
        frame
            .show(ui, |ui| {
                ui.set_min_width(140.0);
                ui.label(
                    egui::RichText::new(self.label)
                        .text_style(TextStyle::Small)
                        .color(palette.text_secondary),
                );
                ui.add_space(SPACING.s1);
                ui.label(
                    egui::RichText::new(self.value)
                        .font(FontId::new(28.0, egui::FontFamily::Proportional))
                        .color(palette.text_primary),
                );
                if let Some((text, trend)) = self.delta {
                    ui.add_space(SPACING.s1);
                    ui.horizontal(|ui| {
                        let (icon, color) = match trend {
                            Trend::Up => (Icon::ChevronUp, palette.success),
                            Trend::Down => (Icon::ChevronDown, palette.error),
                            Trend::Flat => (Icon::ChevronRight, palette.text_secondary),
                        };
                        let (rect, _) = ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
                        icon.paint(ui.painter(), rect, color);
                        ui.label(
                            egui::RichText::new(text)
                                .font(FontId::new(12.0, egui::FontFamily::Proportional))
                                .color(color),
                        );
                    });
                }
            })
            .response
    }
}
