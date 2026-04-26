//! [`DiffView`] — render a unified-style code diff with line numbers and
//! sage-tinted backgrounds for added / removed lines.

use egui::{Color32, FontId, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, vec2};

use super::{alpha, corner};
use crate::{RADIUS, SPACING, palette_of};

/// Per-line kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffKind {
    /// Unchanged context line.
    Context,
    /// Line added (`+`).
    Added,
    /// Line removed (`-`).
    Removed,
    /// Hunk header (`@@ … @@`).
    Hunk,
}

/// One diff line.
#[derive(Debug, Clone)]
pub struct DiffLine<'a> {
    /// Kind: context / added / removed / hunk header.
    pub kind: DiffKind,
    /// Line number on the original file (None for `Added`).
    pub old: Option<u32>,
    /// Line number on the new file (None for `Removed`).
    pub new: Option<u32>,
    /// Line content. Don't include the leading `+`/`-` — the component
    /// renders that as part of the gutter.
    pub text: &'a str,
}

impl<'a> DiffLine<'a> {
    /// Convenience: a context line with both old and new line numbers.
    pub fn context(old: u32, new: u32, text: &'a str) -> Self {
        Self {
            kind: DiffKind::Context,
            old: Some(old),
            new: Some(new),
            text,
        }
    }
    /// Convenience: an added line (only `new` line number).
    pub fn added(new: u32, text: &'a str) -> Self {
        Self {
            kind: DiffKind::Added,
            old: None,
            new: Some(new),
            text,
        }
    }
    /// Convenience: a removed line (only `old` line number).
    pub fn removed(old: u32, text: &'a str) -> Self {
        Self {
            kind: DiffKind::Removed,
            old: Some(old),
            new: None,
            text,
        }
    }
    /// Convenience: a hunk header line.
    pub fn hunk(text: &'a str) -> Self {
        Self {
            kind: DiffKind::Hunk,
            old: None,
            new: None,
            text,
        }
    }
}

/// Render a sequence of [`DiffLine`]s.
pub struct DiffView<'a> {
    lines: Vec<DiffLine<'a>>,
    header: Option<&'a str>,
    line_numbers: bool,
}

impl<'a> Default for DiffView<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> DiffView<'a> {
    /// Empty view.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            header: None,
            line_numbers: true,
        }
    }
    /// Add a header strip above the diff (typically the file path).
    pub fn header(mut self, header: &'a str) -> Self {
        self.header = Some(header);
        self
    }
    /// Hide the line-number gutter.
    pub fn no_line_numbers(mut self) -> Self {
        self.line_numbers = false;
        self
    }
    /// Append one line.
    pub fn line(mut self, line: DiffLine<'a>) -> Self {
        self.lines.push(line);
        self
    }
    /// Append all lines from an iterator.
    pub fn lines(mut self, lines: impl IntoIterator<Item = DiffLine<'a>>) -> Self {
        self.lines.extend(lines);
        self
    }
    /// Render the diff.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let mono = FontId::new(12.0, egui::FontFamily::Monospace);

        egui::Frame::default()
            .fill(palette.bg_surface)
            .stroke(Stroke::new(1.0, palette.border_subtle))
            .corner_radius(corner(RADIUS.md))
            .inner_margin(egui::Margin::same(0))
            .show(ui, |ui| {
                if let Some(header) = self.header {
                    let header_rect = ui
                        .horizontal(|ui| {
                            ui.add_space(SPACING.s3);
                            ui.label(
                                egui::RichText::new(header)
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

                ui.add_space(SPACING.s2);
                for line in &self.lines {
                    paint_line(ui, line, &mono, &palette, self.line_numbers);
                }
                ui.add_space(SPACING.s2);
            })
            .response
    }
}

fn paint_line(
    ui: &mut Ui,
    line: &DiffLine<'_>,
    mono: &FontId,
    palette: &crate::Palette,
    line_numbers: bool,
) {
    let row_height = 18.0;
    let total_w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(total_w, row_height), Sense::hover());

    // Background per kind.
    let (bg, marker, text_color) = match line.kind {
        DiffKind::Context => (Color32::TRANSPARENT, " ", palette.text_secondary),
        DiffKind::Added => (alpha(palette.success, 0.12), "+", palette.text_primary),
        DiffKind::Removed => (alpha(palette.error, 0.12), "-", palette.text_primary),
        DiffKind::Hunk => (alpha(palette.info, 0.10), " ", palette.info),
    };
    if bg != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, corner(0.0), bg);
    }

    let mut x = rect.left();

    // Line numbers.
    if line_numbers {
        let gutter_w = 56.0;
        let cy = rect.center().y;
        let old_str = line.old.map(|n| n.to_string()).unwrap_or_default();
        let new_str = line.new.map(|n| n.to_string()).unwrap_or_default();
        ui.painter().text(
            egui::pos2(x + 28.0, cy),
            egui::Align2::RIGHT_CENTER,
            old_str,
            mono.clone(),
            palette.text_tertiary,
        );
        ui.painter().text(
            egui::pos2(x + 56.0, cy),
            egui::Align2::RIGHT_CENTER,
            new_str,
            mono.clone(),
            palette.text_tertiary,
        );
        x += gutter_w + SPACING.s2;
    }

    // +/- marker.
    let marker_color = match line.kind {
        DiffKind::Added => palette.success,
        DiffKind::Removed => palette.error,
        _ => palette.text_tertiary,
    };
    ui.painter().text(
        egui::pos2(x + 8.0, rect.center().y),
        egui::Align2::CENTER_CENTER,
        marker,
        mono.clone(),
        marker_color,
    );
    x += 16.0;

    // Content.
    ui.painter().text(
        egui::pos2(x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        line.text,
        mono.clone(),
        text_color,
    );
    let _ = StrokeKind::Inside;
    let _ = TextStyle::Body;
}
