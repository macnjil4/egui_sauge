//! [`PageHeader`] — title, optional breadcrumb, optional subtitle, and a
//! right-aligned action slot. Use it as the first row of any non-trivial
//! page; pair with [`crate::components::Section`] for inner blocks.

use egui::{FontId, RichText, Stroke, TextStyle, Ui};

use super::nav::Breadcrumb;
use crate::{SPACING, palette_of};

/// Page-level header with title + actions row.
pub struct PageHeader<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    breadcrumb: Option<Vec<&'a str>>,
}

impl<'a> PageHeader<'a> {
    /// New header with just a title.
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            breadcrumb: None,
        }
    }
    /// Subtitle line under the title.
    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }
    /// Breadcrumb above the title.
    pub fn breadcrumb(mut self, items: &[&'a str]) -> Self {
        self.breadcrumb = Some(items.to_vec());
        self
    }
    /// Render. `actions` lays out the right-side button group.
    pub fn show(self, ui: &mut Ui, actions: impl FnOnce(&mut Ui)) -> Option<usize> {
        let palette = palette_of(ui.ctx());
        let mut clicked = None;

        if let Some(items) = &self.breadcrumb {
            let refs = items.to_vec();
            clicked = Breadcrumb::new(&refs).show(ui);
            ui.add_space(SPACING.s1);
        }

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(self.title)
                        .font(FontId::new(28.0, egui::FontFamily::Proportional))
                        .color(palette.text_primary),
                );
                if let Some(subtitle) = self.subtitle {
                    ui.label(
                        RichText::new(subtitle)
                            .text_style(TextStyle::Body)
                            .color(palette.text_secondary),
                    );
                }
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), actions);
        });

        ui.add_space(SPACING.s3);
        let y = ui.cursor().top();
        ui.painter().line_segment(
            [
                egui::pos2(ui.min_rect().left(), y),
                egui::pos2(ui.min_rect().right(), y),
            ],
            Stroke::new(1.0, palette.border_subtle),
        );
        ui.add_space(SPACING.s3);

        clicked
    }
}
