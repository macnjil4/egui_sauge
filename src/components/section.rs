//! [`Section`] — a heading + body pair for long scrolling pages.
//! [`CodeBlock`] — a monospace container for command output, config snippets.

use egui::{FontId, RichText, Stroke, StrokeKind, TextStyle, Ui, vec2};

use super::corner;
use crate::{RADIUS, SPACING, palette_of};

/// Titled region used to group related content on long pages.
pub struct Section<'a> {
    title: &'a str,
    description: Option<&'a str>,
}

impl<'a> Section<'a> {
    /// New section with just a title.
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
        }
    }
    /// Add a descriptive subtitle under the title.
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
    /// Render the heading and body.
    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> R {
        let palette = palette_of(ui.ctx());
        ui.add_space(SPACING.s4);
        ui.label(
            RichText::new(self.title)
                .text_style(TextStyle::Heading)
                .color(palette.text_primary),
        );
        if let Some(desc) = self.description {
            ui.label(
                RichText::new(desc)
                    .text_style(TextStyle::Body)
                    .color(palette.text_secondary),
            );
        }
        ui.add_space(SPACING.s2);
        body(ui)
    }
}

/// Monospace container with optional header (e.g. a file name or language).
pub struct CodeBlock<'a> {
    code: &'a str,
    header: Option<&'a str>,
    max_height: Option<f32>,
}

impl<'a> CodeBlock<'a> {
    /// New block wrapping `code`.
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            header: None,
            max_height: None,
        }
    }
    /// Add a header label (filename, language).
    pub fn header(mut self, header: &'a str) -> Self {
        self.header = Some(header);
        self
    }
    /// Cap the block's height; content scrolls vertically beyond it.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }
    /// Render the block.
    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let palette = palette_of(ui.ctx());
        egui::Frame::default()
            .fill(palette.bg_surface_alt)
            .stroke(Stroke::new(1.0, palette.border_subtle))
            .corner_radius(corner(RADIUS.md))
            .inner_margin(egui::Margin::same(0))
            .show(ui, |ui| {
                if let Some(header) = self.header {
                    let header_rect = ui
                        .horizontal(|ui| {
                            ui.add_space(SPACING.s3);
                            ui.label(
                                RichText::new(header)
                                    .font(FontId::new(11.0, egui::FontFamily::Monospace))
                                    .color(palette.text_tertiary),
                            );
                            ui.add_space(SPACING.s3);
                        })
                        .response
                        .rect;
                    ui.painter().line_segment(
                        [
                            egui::pos2(header_rect.left(), header_rect.bottom()),
                            egui::pos2(header_rect.right(), header_rect.bottom()),
                        ],
                        Stroke::new(1.0, palette.border_subtle),
                    );
                }
                let inner = |ui: &mut Ui| {
                    ui.add_space(SPACING.s3);
                    ui.horizontal(|ui| {
                        ui.add_space(SPACING.s3);
                        ui.vertical(|ui| {
                            for line in self.code.lines() {
                                ui.label(
                                    RichText::new(line)
                                        .text_style(TextStyle::Monospace)
                                        .color(palette.text_primary),
                                );
                            }
                        });
                    });
                    ui.add_space(SPACING.s3);
                };
                if let Some(h) = self.max_height {
                    egui::ScrollArea::vertical().max_height(h).show(ui, inner);
                } else {
                    inner(ui);
                }
            })
            .response
    }
}

// Unused import guards that rustfmt sometimes removes: keep Vec2/StrokeKind
// referenced if needed downstream.
#[allow(dead_code)]
fn _keep_imports_alive(rect: egui::Rect) {
    let _ = (vec2(0.0, 0.0), StrokeKind::Inside, rect);
}
