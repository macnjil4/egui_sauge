//! [`Gauge`] — donut or linear bar showing a 0..1 ratio with optional
//! semantic thresholds (success/warning/error). Use for resource meters
//! (CPU, memory, disk, queue depth) in IT dashboards.

use egui::{Color32, FontId, Response, Sense, Stroke, StrokeKind, Ui, Widget, vec2};

use super::{alpha, corner};
use crate::{RADIUS, palette_of};

/// Visual flavour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GaugeShape {
    /// Donut / ring (270° arc).
    Ring,
    /// Linear horizontal bar.
    Bar,
}

/// Color thresholds. The fill turns warning above `warn_at` and error
/// above `error_at`. Defaults to `(0.75, 0.9)` — sane for resource use.
#[derive(Debug, Clone, Copy)]
pub struct Thresholds {
    /// Above this ratio (0..1) the fill uses [`crate::Palette::warning`].
    pub warn_at: f32,
    /// Above this ratio (0..1) the fill uses [`crate::Palette::error`].
    pub error_at: f32,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            warn_at: 0.75,
            error_at: 0.9,
        }
    }
}

/// Resource gauge.
pub struct Gauge<'a> {
    /// Ratio in `[0.0, 1.0]`.
    value: f32,
    shape: GaugeShape,
    /// Override the fill color (defaults to threshold-aware brand/warning/error).
    color: Option<Color32>,
    label: Option<&'a str>,
    show_percent: bool,
    thresholds: Thresholds,
    size: f32,
}

impl<'a> Gauge<'a> {
    /// Donut / ring gauge for `value` in `[0.0, 1.0]`.
    pub fn ring(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            shape: GaugeShape::Ring,
            color: None,
            label: None,
            show_percent: true,
            thresholds: Thresholds::default(),
            size: 64.0,
        }
    }
    /// Linear horizontal-bar gauge for `value` in `[0.0, 1.0]`.
    pub fn bar(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            shape: GaugeShape::Bar,
            color: None,
            label: None,
            show_percent: true,
            thresholds: Thresholds::default(),
            size: 8.0, // bar height
        }
    }
    /// Caption above (bar) or center (ring).
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    /// Hide the inline percentage readout.
    pub fn no_percent(mut self) -> Self {
        self.show_percent = false;
        self
    }
    /// Override the fill color, bypassing threshold-based selection.
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
    /// Override the warn / error thresholds (defaults `(0.75, 0.9)`).
    pub fn thresholds(mut self, t: Thresholds) -> Self {
        self.thresholds = t;
        self
    }
    /// Size: ring diameter (Ring) or bar height (Bar).
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    fn auto_color(&self, p: &crate::Palette) -> Color32 {
        if let Some(c) = self.color {
            return c;
        }
        if self.value >= self.thresholds.error_at {
            p.error
        } else if self.value >= self.thresholds.warn_at {
            p.warning
        } else {
            p.brand_default
        }
    }
}

impl<'a> Widget for Gauge<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let fill = self.auto_color(&palette);
        match self.shape {
            GaugeShape::Ring => paint_ring(ui, &self, &palette, fill),
            GaugeShape::Bar => paint_bar(ui, &self, &palette, fill),
        }
    }
}

fn paint_ring(ui: &mut Ui, g: &Gauge<'_>, p: &crate::Palette, fill: Color32) -> Response {
    let d = g.size;
    let (rect, response) = ui.allocate_exact_size(vec2(d, d), Sense::hover());
    let center = rect.center();
    let radius = d / 2.0 - 4.0;
    let stroke_w = (d * 0.10).max(3.0);

    // Track (full ring at low alpha).
    let segments = 64;
    let start = std::f32::consts::PI * 0.75; // 135°
    let end_full = std::f32::consts::PI * 2.25; // 405°
    let span = end_full - start;
    let mut track = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let a = start + (i as f32 / segments as f32) * span;
        track.push(egui::pos2(
            center.x + a.cos() * radius,
            center.y + a.sin() * radius,
        ));
    }
    ui.painter().add(egui::Shape::line(
        track,
        Stroke::new(stroke_w, alpha(p.text_primary, 0.06)),
    ));

    // Fill arc.
    let fill_segments = (segments as f32 * g.value).round() as usize;
    if fill_segments > 0 {
        let mut arc = Vec::with_capacity(fill_segments + 1);
        for i in 0..=fill_segments {
            let a = start + (i as f32 / segments as f32) * span;
            arc.push(egui::pos2(
                center.x + a.cos() * radius,
                center.y + a.sin() * radius,
            ));
        }
        ui.painter()
            .add(egui::Shape::line(arc, Stroke::new(stroke_w, fill)));
    }

    // Center text.
    if g.show_percent {
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            format!("{:.0}%", g.value * 100.0),
            FontId::new(d * 0.28, egui::FontFamily::Proportional),
            p.text_primary,
        );
    }
    if let Some(label) = g.label {
        ui.painter().text(
            egui::pos2(center.x, rect.bottom() + 4.0),
            egui::Align2::CENTER_TOP,
            label,
            FontId::new(11.0, egui::FontFamily::Proportional),
            p.text_secondary,
        );
    }
    response
}

fn paint_bar(ui: &mut Ui, g: &Gauge<'_>, p: &crate::Palette, fill: Color32) -> Response {
    let width = ui.available_width().max(80.0);
    let row_height = if g.label.is_some() || g.show_percent {
        g.size + 18.0
    } else {
        g.size
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, row_height), Sense::hover());

    if g.label.is_some() || g.show_percent {
        // Header row: label left, percent right.
        let header_y = rect.top();
        if let Some(label) = g.label {
            ui.painter().text(
                egui::pos2(rect.left(), header_y),
                egui::Align2::LEFT_TOP,
                label,
                FontId::new(12.0, egui::FontFamily::Proportional),
                p.text_secondary,
            );
        }
        if g.show_percent {
            ui.painter().text(
                egui::pos2(rect.right(), header_y),
                egui::Align2::RIGHT_TOP,
                format!("{:.0}%", g.value * 100.0),
                FontId::new(12.0, egui::FontFamily::Monospace),
                p.text_secondary,
            );
        }
    }
    let bar_y = rect.bottom() - g.size;
    let track = egui::Rect::from_min_size(egui::pos2(rect.left(), bar_y), vec2(width, g.size));
    ui.painter().rect(
        track,
        corner(RADIUS.full),
        p.bg_surface_alt,
        Stroke::new(1.0, p.border_subtle),
        StrokeKind::Inside,
    );
    let fill_w = width * g.value;
    if fill_w > 0.0 {
        let fill_rect = egui::Rect::from_min_size(track.min, vec2(fill_w, g.size));
        ui.painter()
            .rect_filled(fill_rect, corner(RADIUS.full), fill);
    }
    response
}
