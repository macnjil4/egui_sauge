//! Inline [`Alert`] banner for feedback in place (not floating — that's
//! [`crate::components::Toast`]).

use egui::{FontId, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Widget, vec2};

use super::{alpha, corner};
use crate::{Icon, Palette, RADIUS, SPACING, palette_of};

/// Semantic level of an [`Alert`] or [`crate::components::Toast`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Informational (info color).
    Info,
    /// Success (success color).
    Success,
    /// Attention (warning color).
    Warning,
    /// Failure (error color).
    Error,
}

impl Level {
    /// Default icon for this level.
    pub fn icon(self) -> Icon {
        match self {
            Self::Info => Icon::Info,
            Self::Success => Icon::Success,
            Self::Warning => Icon::Warning,
            Self::Error => Icon::Error,
        }
    }
    /// Accent color (sourced from the palette).
    pub fn color(self, p: &Palette) -> egui::Color32 {
        match self {
            Self::Info => p.info,
            Self::Success => p.success,
            Self::Warning => p.warning,
            Self::Error => p.error,
        }
    }
}

/// Inline alert with icon, title, body and optional dismiss affordance.
pub struct Alert<'a, 'b> {
    level: Level,
    title: Option<&'a str>,
    body: &'a str,
    dismiss: Option<&'b mut bool>,
}

impl<'a, 'b> Alert<'a, 'b> {
    /// New alert with a body text.
    pub fn new(level: Level, body: &'a str) -> Self {
        Self {
            level,
            title: None,
            body,
            dismiss: None,
        }
    }
    /// Add a bold title above the body.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
    /// Attach a flag set to `true` when the user clicks the × dismiss icon.
    pub fn dismiss(mut self, flag: &'b mut bool) -> Self {
        self.dismiss = Some(flag);
        self
    }
}

impl<'a, 'b> Widget for Alert<'a, 'b> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let accent = self.level.color(&palette);
        let fill = alpha(accent, 0.12);
        let border = alpha(accent, 0.45);

        egui::Frame::default()
            .fill(fill)
            .stroke(Stroke::new(1.0, border))
            .corner_radius(corner(RADIUS.md))
            .inner_margin(egui::Margin::same(SPACING.s3 as i8))
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(20.0, 20.0), Sense::hover());
                    self.level.icon().paint(ui.painter(), rect, accent);
                    ui.add_space(SPACING.s2);
                    ui.vertical(|ui| {
                        if let Some(title) = self.title {
                            ui.label(
                                egui::RichText::new(title)
                                    .font(FontId::new(14.0, egui::FontFamily::Proportional))
                                    .color(palette.text_primary),
                            );
                            ui.add_space(2.0);
                        }
                        ui.label(
                            egui::RichText::new(self.body)
                                .text_style(TextStyle::Body)
                                .color(palette.text_secondary),
                        );
                    });

                    if let Some(flag) = self.dismiss {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            let (rect, resp) =
                                ui.allocate_exact_size(vec2(18.0, 18.0), Sense::click());
                            let c = if resp.hovered() {
                                palette.text_primary
                            } else {
                                alpha(palette.text_secondary, 0.8)
                            };
                            // Small hover background.
                            if resp.hovered() {
                                ui.painter().rect(
                                    rect,
                                    corner(RADIUS.sm),
                                    alpha(palette.text_primary, 0.08),
                                    Stroke::NONE,
                                    StrokeKind::Inside,
                                );
                            }
                            Icon::Close.paint(ui.painter(), rect.shrink(3.0), c);
                            if resp.clicked() {
                                *flag = true;
                            }
                        });
                    }
                });
            })
            .response
    }
}
