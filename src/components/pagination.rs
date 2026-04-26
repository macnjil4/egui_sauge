//! [`Pagination`] — page navigation typically paired with [`super::Table`].

use egui::{FontId, Response, Sense, Stroke, StrokeKind, Ui, Vec2};

use super::{alpha, corner};
use crate::{Icon, RADIUS, SPACING, palette_of};

/// Page navigation controls.
///
/// Caller passes the current `page` (0-based), `total_items`, and
/// `page_size` mutably; the component renders prev/next buttons, the
/// page number range, and a "Showing N–M of T" status. When the user
/// clicks, `page` is updated and the response is marked changed.
pub struct Pagination<'a> {
    page: &'a mut usize,
    total_items: usize,
    page_size: &'a mut usize,
    page_sizes: Vec<usize>,
}

impl<'a> Pagination<'a> {
    /// New pagination bound to `page` (current 0-based page),
    /// `total_items` (rows in the source), and `page_size` (rows per
    /// page).
    pub fn new(page: &'a mut usize, total_items: usize, page_size: &'a mut usize) -> Self {
        Self {
            page,
            total_items,
            page_size,
            page_sizes: vec![10, 25, 50, 100],
        }
    }

    /// Override the page-size choices in the dropdown
    /// (default `[10, 25, 50, 100]`).
    pub fn page_sizes(mut self, sizes: impl IntoIterator<Item = usize>) -> Self {
        self.page_sizes = sizes.into_iter().collect();
        self
    }

    /// Render. Returns the inner row [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let total_pages = if *self.page_size == 0 {
            1
        } else {
            self.total_items.div_ceil(*self.page_size).max(1)
        };
        if *self.page >= total_pages {
            *self.page = total_pages - 1;
        }

        let from = if self.total_items == 0 {
            0
        } else {
            *self.page * *self.page_size + 1
        };
        let to = ((*self.page + 1) * *self.page_size).min(self.total_items);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!(
                    "Showing {from}–{to} of {total}",
                    total = self.total_items
                ))
                .text_style(egui::TextStyle::Small)
                .color(palette.text_secondary),
            );
            ui.add_space(SPACING.s4);

            // Page-size selector.
            let prev_size = *self.page_size;
            egui::ComboBox::from_id_salt("page_size")
                .selected_text(format!("{} / page", *self.page_size))
                .width(110.0)
                .show_ui(ui, |ui| {
                    for size in &self.page_sizes {
                        ui.selectable_value(self.page_size, *size, format!("{size} / page"));
                    }
                });
            if *self.page_size != prev_size {
                // Stay roughly in the same item range when size changes.
                let first_item = (*self.page) * prev_size;
                *self.page = first_item / (*self.page_size).max(1);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let next_disabled = *self.page + 1 >= total_pages;
                let prev_disabled = *self.page == 0;
                let next_resp = paint_arrow(ui, Icon::ChevronRight, next_disabled);
                if next_resp.clicked() && !next_disabled {
                    *self.page += 1;
                }
                ui.add_space(SPACING.s2);
                ui.label(
                    egui::RichText::new(format!("{}/{}", *self.page + 1, total_pages))
                        .font(FontId::new(12.0, egui::FontFamily::Monospace))
                        .color(palette.text_primary),
                );
                ui.add_space(SPACING.s2);
                let prev_resp = paint_arrow(ui, Icon::ChevronLeft, prev_disabled);
                if prev_resp.clicked() && !prev_disabled {
                    *self.page -= 1;
                }
            });
        })
        .response
    }
}

fn paint_arrow(ui: &mut Ui, icon: Icon, disabled: bool) -> Response {
    let palette = palette_of(ui.ctx());
    let size = 26.0;
    let sense = if disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), sense);
    let bg = if !disabled && resp.hovered() {
        palette.bg_hover
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect(
        rect,
        corner(RADIUS.sm),
        bg,
        Stroke::new(1.0, palette.border_default),
        StrokeKind::Inside,
    );
    let color = if disabled {
        alpha(palette.text_secondary, 0.45)
    } else {
        palette.text_primary
    };
    icon.paint(ui.painter(), rect.shrink(6.0), color);
    resp
}
