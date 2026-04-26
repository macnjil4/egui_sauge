//! [`Sparkline`] — tiny inline chart (line or bars), no external deps.
//! Pair with [`crate::components::Stat`] cards in dashboards to show a
//! 24h trend next to the current value.

use egui::{Color32, Response, Sense, Stroke, Ui, Widget, vec2};

use super::{alpha, corner};
use crate::{RADIUS, palette_of};

/// Visual flavour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SparklineKind {
    /// Polyline through the points (default).
    #[default]
    Line,
    /// Vertical bars from the bottom.
    Bars,
    /// Polyline + filled area below.
    Area,
}

/// Inline mini-chart.
pub struct Sparkline<'a> {
    data: &'a [f32],
    kind: SparklineKind,
    width: f32,
    height: f32,
    color: Option<Color32>,
    /// Optional explicit (min, max) to normalise the curve. When `None`,
    /// derive from the data.
    range: Option<(f32, f32)>,
    /// Show a dot on the last point.
    show_last: bool,
}

impl<'a> Sparkline<'a> {
    /// New line sparkline from `data`. Defaults: 120 × 24 px,
    /// `palette.brand_default` color.
    pub fn new(data: &'a [f32]) -> Self {
        Self {
            data,
            kind: SparklineKind::Line,
            width: 120.0,
            height: 24.0,
            color: None,
            range: None,
            show_last: true,
        }
    }
    /// Switch to vertical bars.
    pub fn bars(mut self) -> Self {
        self.kind = SparklineKind::Bars;
        self
    }
    /// Polyline + filled area.
    pub fn area(mut self) -> Self {
        self.kind = SparklineKind::Area;
        self
    }
    /// Override size.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    /// Override the stroke / fill color (default: brand).
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
    /// Pin the y-axis range. Useful when you want comparable sparklines
    /// across rows (e.g. always 0..100 for percentages).
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.range = Some((min, max));
        self
    }
    /// Hide the trailing dot.
    pub fn no_marker(mut self) -> Self {
        self.show_last = false;
        self
    }
}

impl<'a> Widget for Sparkline<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let color = self.color.unwrap_or(palette.brand_default);
        let (rect, response) =
            ui.allocate_exact_size(vec2(self.width, self.height), Sense::hover());

        if self.data.len() < 2 {
            return response;
        }

        // Compute range.
        let (min, max) = self.range.unwrap_or_else(|| {
            let mut lo = f32::INFINITY;
            let mut hi = f32::NEG_INFINITY;
            for v in self.data {
                lo = lo.min(*v);
                hi = hi.max(*v);
            }
            (lo, hi)
        });
        let span = (max - min).abs().max(f32::EPSILON);
        let to_y = |v: f32| -> f32 {
            let n = ((v - min) / span).clamp(0.0, 1.0);
            rect.bottom() - n * rect.height()
        };
        let n = self.data.len();
        let to_x = |i: usize| -> f32 { rect.left() + (i as f32 / (n - 1) as f32) * rect.width() };

        match self.kind {
            SparklineKind::Bars => {
                let bar_w = (rect.width() / n as f32).clamp(1.0, 8.0);
                for (i, v) in self.data.iter().enumerate() {
                    let x = to_x(i) - bar_w / 2.0;
                    let y = to_y(*v);
                    let bar = egui::Rect::from_min_max(
                        egui::pos2(x, y),
                        egui::pos2(x + bar_w * 0.85, rect.bottom()),
                    );
                    ui.painter()
                        .rect_filled(bar, corner(RADIUS.sm * 0.5), color);
                }
            }
            SparklineKind::Line | SparklineKind::Area => {
                let pts: Vec<egui::Pos2> = self
                    .data
                    .iter()
                    .enumerate()
                    .map(|(i, v)| egui::pos2(to_x(i), to_y(*v)))
                    .collect();
                if matches!(self.kind, SparklineKind::Area) {
                    let mut closed = pts.clone();
                    closed.push(egui::pos2(rect.right(), rect.bottom()));
                    closed.push(egui::pos2(rect.left(), rect.bottom()));
                    ui.painter().add(egui::Shape::convex_polygon(
                        closed,
                        alpha(color, 0.18),
                        Stroke::NONE,
                    ));
                }
                ui.painter()
                    .add(egui::Shape::line(pts.clone(), Stroke::new(1.5, color)));
                if self.show_last
                    && let Some(last) = pts.last()
                {
                    ui.painter().circle_filled(*last, 2.5, color);
                }
            }
        }
        response
    }
}
