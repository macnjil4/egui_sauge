//! [`StatusBar`] — bottom strip with three slots (left / center / right)
//! and helpers for individual segments.
//!
//! Use it for branch info, build status, line:col, encoding, locale,
//! anything you'd find in an IDE's footer.

use egui::{FontId, Response, Sense, Stroke, Ui, vec2};

use crate::{Icon, SPACING, palette_of};

/// Bottom workbench bar with three slots.
///
/// ```ignore
/// StatusBar::show(
///     ui,
///     |ui| {
///         StatusBar::segment(ui, Some(Icon::GitBranch), "main");
///         StatusBar::segment(ui, None, "8 changed files");
///     },
///     |_| {},
///     |ui| {
///         StatusBar::segment(ui, None, "Cargo Check");
///         StatusBar::segment(ui, None, "Ln 42, Col 8");
///         StatusBar::segment(ui, None, "UTF-8");
///     },
/// );
/// ```
pub struct StatusBar;

impl StatusBar {
    /// Render the bar. Each slot lays out left-to-right inside its
    /// region. The bar height is 28 px.
    pub fn show(
        ui: &mut Ui,
        left: impl FnOnce(&mut Ui),
        center: impl FnOnce(&mut Ui),
        right: impl FnOnce(&mut Ui),
    ) -> Response {
        let palette = palette_of(ui.ctx());
        let height = 28.0;
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
        ui.painter()
            .rect_filled(rect, super::corner(0.0), palette.bg_surface_alt);
        // Top hairline.
        ui.painter().line_segment(
            [
                egui::pos2(rect.left(), rect.top()),
                egui::pos2(rect.right(), rect.top()),
            ],
            Stroke::new(1.0, palette.border_subtle),
        );

        // Three columns: 1/3 each (caller can adjust by laying out
        // their own widgets within the slot ui).
        let col_w = rect.width() / 3.0;
        let left_rect = egui::Rect::from_min_size(rect.min, vec2(col_w, height));
        let center_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + col_w, rect.top()),
            vec2(col_w, height),
        );
        let right_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + col_w * 2.0, rect.top()),
            vec2(col_w, height),
        );

        // Left.
        let mut left_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(left_rect.shrink2(vec2(SPACING.s3, 0.0)))
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        left(&mut left_ui);

        // Center.
        let mut center_ui = ui.new_child(egui::UiBuilder::new().max_rect(center_rect).layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
        ));
        center(&mut center_ui);

        // Right.
        let mut right_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(right_rect.shrink2(vec2(SPACING.s3, 0.0)))
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        right(&mut right_ui);

        response
    }

    /// Tiny segment helper: optional leading icon + small label, with
    /// hover background. Use inside one of the [`Self::show`] slots.
    pub fn segment(ui: &mut Ui, icon: Option<Icon>, text: &str) -> Response {
        let palette = palette_of(ui.ctx());
        let font = FontId::new(11.0, egui::FontFamily::Proportional);
        let galley = ui
            .painter()
            .layout_no_wrap(text.into(), font, palette.text_secondary);
        let icon_w = if icon.is_some() { 14.0 + 4.0 } else { 0.0 };
        let pad = vec2(SPACING.s2, 2.0);
        let size = vec2(galley.size().x + icon_w + pad.x * 2.0, 22.0);
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if response.hovered() {
            ui.painter()
                .rect_filled(rect, super::corner(crate::RADIUS.sm), palette.bg_hover);
        }
        let mut x = rect.left() + pad.x;
        let cy = rect.center().y;
        if let Some(icon) = icon {
            let r = egui::Rect::from_min_size(egui::pos2(x, cy - 7.0), egui::Vec2::splat(14.0));
            icon.paint(ui.painter(), r, palette.text_secondary);
            x += 14.0 + 4.0;
        }
        ui.painter().galley(
            egui::pos2(x, cy - galley.size().y / 2.0),
            galley,
            palette.text_secondary,
        );
        response
    }
}
