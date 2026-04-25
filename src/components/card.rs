//! [`Card`] — a themed, shadowed surface. [`EmptyState`] — a centered
//! illustration + text block for zero-state views.

use egui::{FontId, Stroke, TextStyle, Ui, Widget, vec2};

use super::corner;
use crate::{Elevation, Icon, RADIUS, SPACING, palette_of};

/// A titled, bordered, shadowed surface.
pub struct Card<'a> {
    title: Option<&'a str>,
    subtitle: Option<&'a str>,
    elevation: Elevation,
}

impl<'a> Default for Card<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Card<'a> {
    /// New untitled card at [`Elevation::Card`].
    pub fn new() -> Self {
        Self {
            title: None,
            subtitle: None,
            elevation: Elevation::Card,
        }
    }
    /// Set a header title (Heading style).
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
    /// Set a secondary subtitle under the title.
    pub fn subtitle(mut self, sub: &'a str) -> Self {
        self.subtitle = Some(sub);
        self
    }
    /// Override the elevation (default [`Elevation::Card`]).
    pub fn elevation(mut self, elevation: Elevation) -> Self {
        self.elevation = elevation;
        self
    }

    /// Render the card, calling `body` with a child `Ui` for the content.
    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> R {
        let palette = palette_of(ui.ctx());
        let frame = egui::Frame::default()
            .fill(palette.bg_surface)
            .stroke(Stroke::new(1.0, palette.border_subtle))
            .corner_radius(corner(RADIUS.md))
            .inner_margin(egui::Margin::same(SPACING.s4 as i8))
            .shadow(self.elevation.shadow(palette.dark_mode));
        frame
            .show(ui, |ui| {
                if let Some(title) = self.title {
                    ui.label(
                        egui::RichText::new(title)
                            .text_style(TextStyle::Name("h3".into()))
                            .color(palette.text_primary),
                    );
                }
                if let Some(sub) = self.subtitle {
                    ui.label(
                        egui::RichText::new(sub)
                            .text_style(TextStyle::Small)
                            .color(palette.text_secondary),
                    );
                }
                if self.title.is_some() || self.subtitle.is_some() {
                    ui.add_space(SPACING.s3);
                }
                body(ui)
            })
            .inner
    }
}

/// Centered illustration + title + body + optional action. For "no data",
/// "nothing selected", permission-denied, etc.
pub struct EmptyState<'a> {
    icon: Icon,
    title: &'a str,
    body: Option<&'a str>,
}

impl<'a> EmptyState<'a> {
    /// New empty state with a signature icon and title.
    pub fn new(icon: Icon, title: &'a str) -> Self {
        Self {
            icon,
            title,
            body: None,
        }
    }
    /// Add a body paragraph.
    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }
}

impl<'a> Widget for EmptyState<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let palette = palette_of(ui.ctx());
        ui.vertical_centered(|ui| {
            ui.add_space(SPACING.s5);
            let rect = ui
                .allocate_exact_size(vec2(48.0, 48.0), egui::Sense::hover())
                .0;
            self.icon.paint(ui.painter(), rect, palette.text_tertiary);
            ui.add_space(SPACING.s3);
            ui.label(
                egui::RichText::new(self.title)
                    .font(FontId::new(16.0, egui::FontFamily::Proportional))
                    .color(palette.text_primary),
            );
            if let Some(body) = self.body {
                ui.add_space(SPACING.s1);
                ui.label(
                    egui::RichText::new(body)
                        .text_style(TextStyle::Body)
                        .color(palette.text_secondary),
                );
            }
            ui.add_space(SPACING.s5);
        })
        .response
    }
}
