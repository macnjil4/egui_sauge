//! Data-display atoms: [`KeyValue`] for definition lists, [`LogLine`] for
//! console-style output, [`Skeleton`] for loading placeholders.

use egui::{Color32, FontId, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Widget, vec2};

use super::corner;
use crate::{Palette, RADIUS, SPACING, palette_of};

// -- KeyValue ---------------------------------------------------------------

/// Definition list. Renders rows of `label · value` with consistent left
/// column widths.
pub struct KeyValue<'a> {
    items: Vec<(&'a str, &'a str)>,
    label_width: f32,
}

impl<'a> Default for KeyValue<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> KeyValue<'a> {
    /// Empty list.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            label_width: 140.0,
        }
    }
    /// Append a row.
    pub fn item(mut self, label: &'a str, value: &'a str) -> Self {
        self.items.push((label, value));
        self
    }
    /// Override the label column width (default 140 px).
    pub fn label_width(mut self, w: f32) -> Self {
        self.label_width = w;
        self
    }
    /// Render the list.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        ui.vertical(|ui| {
            for (label, value) in &self.items {
                ui.horizontal(|ui| {
                    let label_widget = egui::Label::new(
                        egui::RichText::new(*label)
                            .text_style(TextStyle::Small)
                            .color(palette.text_secondary),
                    );
                    ui.add_sized(vec2(self.label_width, 20.0), label_widget);
                    ui.label(
                        egui::RichText::new(*value)
                            .text_style(TextStyle::Body)
                            .color(palette.text_primary),
                    );
                });
                ui.add_space(SPACING.s1);
            }
        })
        .response
    }
}

// -- LogLine ----------------------------------------------------------------

/// Severity of a log entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Trace — finest grain.
    Trace,
    /// Debug.
    Debug,
    /// Info — default.
    Info,
    /// Warn.
    Warn,
    /// Error.
    Error,
}

impl LogLevel {
    fn label(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
    fn color(self, p: &Palette) -> Color32 {
        match self {
            Self::Trace => p.text_tertiary,
            Self::Debug => p.text_secondary,
            Self::Info => p.info,
            Self::Warn => p.warning,
            Self::Error => p.error,
        }
    }
}

/// Single log row: timestamp · level · message, monospaced.
pub struct LogLine<'a> {
    timestamp: Option<&'a str>,
    level: LogLevel,
    message: &'a str,
}

impl<'a> LogLine<'a> {
    /// New log line.
    pub fn new(level: LogLevel, message: &'a str) -> Self {
        Self {
            timestamp: None,
            level,
            message,
        }
    }
    /// Add a timestamp prefix.
    pub fn timestamp(mut self, ts: &'a str) -> Self {
        self.timestamp = Some(ts);
        self
    }
}

impl<'a> Widget for LogLine<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let mono = FontId::new(12.0, egui::FontFamily::Monospace);
        ui.horizontal(|ui| {
            if let Some(ts) = self.timestamp {
                ui.label(
                    egui::RichText::new(ts)
                        .font(mono.clone())
                        .color(palette.text_tertiary),
                );
            }
            ui.add_sized(
                vec2(48.0, 16.0),
                egui::Label::new(
                    egui::RichText::new(self.level.label())
                        .font(mono.clone())
                        .color(self.level.color(&palette)),
                ),
            );
            ui.label(
                egui::RichText::new(self.message)
                    .font(mono)
                    .color(palette.text_primary),
            );
        })
        .response
    }
}

// -- Skeleton ---------------------------------------------------------------

/// Animated placeholder shape used while content loads.
pub struct Skeleton {
    width: f32,
    height: f32,
    circle: bool,
}

impl Skeleton {
    /// Single-line text placeholder (14 px tall, rounded).
    pub fn line(width: f32) -> Self {
        Self {
            width,
            height: 14.0,
            circle: false,
        }
    }
    /// Block placeholder (rectangle, lightly rounded).
    pub fn block(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            circle: false,
        }
    }
    /// Circular placeholder (avatar etc.).
    pub fn circle(size: f32) -> Self {
        Self {
            width: size,
            height: size,
            circle: true,
        }
    }
}

impl Widget for Skeleton {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let (rect, response) =
            ui.allocate_exact_size(vec2(self.width, self.height), Sense::hover());

        // Animated shimmer using a phase based on input time. Frozen at
        // mid-amplitude when reduce-motion is on.
        let reduced = crate::reduce_motion(ui.ctx());
        let t = if reduced {
            0.0_f32
        } else {
            ui.ctx().request_repaint();
            ui.input(|i| i.time) as f32
        };
        let phase = ((t * 1.2).sin() * 0.5 + 0.5) * 0.5 + 0.4; // 0.4..0.9
        let base = palette.bg_surface_alt;
        let alpha_strength = if palette.dark_mode { 0.06 } else { 0.04 };
        let highlight = super::alpha(
            palette.text_primary,
            alpha_strength + alpha_strength * phase,
        );
        let cr = if self.circle {
            corner(self.width / 2.0)
        } else {
            corner(RADIUS.sm)
        };
        ui.painter()
            .rect(rect, cr, base, Stroke::NONE, StrokeKind::Inside);
        ui.painter().rect_filled(rect, cr, highlight);
        response
    }
}
