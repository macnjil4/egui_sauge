//! Progress / loading indicators: [`Spinner`], [`ProgressBar`].

use egui::{Color32, FontId, Response, Sense, Stroke, StrokeKind, Ui, Widget, pos2, vec2};

use super::corner;
use crate::{RADIUS, SPACING, palette_of};

/// Rotating circular spinner.
pub struct Spinner {
    size: f32,
    color: Option<Color32>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Spinner {
    /// 20 px spinner using `brand_default`.
    pub fn new() -> Self {
        Self {
            size: 20.0,
            color: None,
        }
    }
    /// Override the diameter.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    /// Override the stroke color (default: brand).
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let color = self.color.unwrap_or(palette.brand_default);

        let (rect, response) = ui.allocate_exact_size(vec2(self.size, self.size), Sense::hover());

        // Continuous repaint so the arc animates.
        ui.ctx().request_repaint();
        let t = ui.input(|i| i.time) as f32;
        let start = t * std::f32::consts::TAU;
        let radius = self.size * 0.42;
        let stroke_w = (self.size * 0.11).max(1.5);

        // Draw an arc as a polyline of 24 short segments spanning 0.7 turns.
        let span = std::f32::consts::TAU * 0.7;
        let segments = 24;
        let mut pts = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let a = start + (i as f32 / segments as f32) * span;
            let (sin, cos) = a.sin_cos();
            pts.push(pos2(
                rect.center().x + cos * radius,
                rect.center().y + sin * radius,
            ));
        }
        ui.painter()
            .add(egui::Shape::line(pts, Stroke::new(stroke_w, color)));
        response
    }
}

/// Linear progress indicator. `value` is clamped to `[0.0, 1.0]`.
pub struct ProgressBar<'a> {
    value: f32,
    label: Option<&'a str>,
    show_percent: bool,
    color: Option<Color32>,
    height: f32,
}

impl<'a> ProgressBar<'a> {
    /// New progress bar at `value` (clamped to 0..=1).
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            label: None,
            show_percent: false,
            color: None,
            height: 8.0,
        }
    }
    /// Add a label above the bar.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    /// Display the percentage on the right of the label row.
    pub fn show_percent(mut self) -> Self {
        self.show_percent = true;
        self
    }
    /// Override the fill color.
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
    /// Override the bar height (default 8 px).
    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }
}

impl<'a> Widget for ProgressBar<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let color = self.color.unwrap_or(palette.brand_default);

        ui.vertical(|ui| {
            if self.label.is_some() || self.show_percent {
                ui.horizontal(|ui| {
                    if let Some(label) = self.label {
                        ui.label(
                            egui::RichText::new(label)
                                .font(FontId::new(12.0, egui::FontFamily::Proportional))
                                .color(palette.text_secondary),
                        );
                    }
                    if self.show_percent {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:.0}%", self.value * 100.0))
                                    .font(FontId::new(12.0, egui::FontFamily::Monospace))
                                    .color(palette.text_secondary),
                            );
                        });
                    }
                });
                ui.add_space(SPACING.s1);
            }
            let (rect, _) =
                ui.allocate_exact_size(vec2(ui.available_width(), self.height), Sense::hover());
            ui.painter().rect(
                rect,
                corner(RADIUS.full),
                palette.bg_surface_alt,
                Stroke::new(1.0, palette.border_subtle),
                StrokeKind::Inside,
            );
            let fill_w = rect.width() * self.value;
            if fill_w > 0.0 {
                let fill_rect = egui::Rect::from_min_size(rect.min, vec2(fill_w, rect.height()));
                ui.painter()
                    .rect_filled(fill_rect, corner(RADIUS.full), color);
            }
        })
        .response
    }
}
