//! Themed [`Checkbox`] with optional indeterminate / error states.

use egui::{
    Color32, FontId, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, Widget, vec2,
};

use super::{alpha, corner};
use crate::{Icon, RADIUS, SPACING, palette_of};

/// A two-or-three-state check input. The third state is *indeterminate*
/// (set via [`Self::indeterminate`]) — typically a "select all" master
/// when only some children are selected.
pub struct Checkbox<'a> {
    value: &'a mut bool,
    label: Option<&'a str>,
    indeterminate: bool,
    disabled: bool,
    error: bool,
}

impl<'a> Checkbox<'a> {
    /// New checkbox bound to `value`, no label.
    pub fn new(value: &'a mut bool) -> Self {
        Self {
            value,
            label: None,
            indeterminate: false,
            disabled: false,
            error: false,
        }
    }
    /// Convenience: `value` + label.
    pub fn with_label(value: &'a mut bool, label: &'a str) -> Self {
        Self::new(value).label(label)
    }
    /// Add a label to the right of the box.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    /// Render the indeterminate state. Independent from `value` — the box
    /// shows a horizontal bar instead of a check until the user clicks
    /// (which forces `value = true` and clears the indeterminate hint).
    pub fn indeterminate(mut self, on: bool) -> Self {
        self.indeterminate = on;
        self
    }
    /// Disable interaction (no hover, 0.45 opacity).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Style the box border with the error color.
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
}

impl<'a> Widget for Checkbox<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let box_size: f32 = 16.0;
        let gap = SPACING.s2;

        let label_w = self.label.map_or(0.0, |label| {
            let g = ui.painter().layout_no_wrap(
                label.to_string(),
                FontId::new(13.0, egui::FontFamily::Proportional),
                palette.text_primary,
            );
            g.size().x + gap
        });
        let label_h = if self.label.is_some() { 16.0 } else { 0.0 };
        let total = vec2(box_size + label_w, box_size.max(label_h));

        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, mut response) = ui.allocate_exact_size(total, sense);
        if response.clicked() && !self.disabled {
            *self.value = !*self.value;
            response.mark_changed();
        }

        let on = *self.value;
        let box_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left(), rect.center().y - box_size / 2.0),
            Vec2::splat(box_size),
        );

        let (fill, stroke_color) = if self.error {
            (
                if on || self.indeterminate {
                    palette.error
                } else {
                    palette.bg_surface
                },
                palette.error,
            )
        } else if on || self.indeterminate {
            (palette.brand_default, palette.brand_default)
        } else if response.hovered() {
            (palette.bg_surface, palette.brand_default)
        } else {
            (palette.bg_surface, palette.border_default)
        };
        let (fill, stroke_color) = if self.disabled {
            (alpha(fill, 0.45), alpha(stroke_color, 0.45))
        } else {
            (fill, stroke_color)
        };

        ui.painter().rect(
            box_rect,
            corner(RADIUS.sm),
            fill,
            Stroke::new(1.5, stroke_color),
            StrokeKind::Inside,
        );

        // Glyph inside the box.
        let glyph_color = if on || self.indeterminate {
            palette.text_on_brand
        } else {
            Color32::TRANSPARENT
        };
        if self.indeterminate && !on {
            // Horizontal bar.
            let bar = egui::Rect::from_center_size(box_rect.center(), vec2(8.0, 2.0));
            ui.painter().rect_filled(bar, corner(1.0), glyph_color);
        } else if on {
            Icon::Check.paint(ui.painter(), box_rect.shrink(2.0), glyph_color);
        }

        if response.has_focus() && !self.disabled {
            ui.painter().rect_stroke(
                box_rect.expand(2.0),
                corner(RADIUS.sm + 2.0),
                Stroke::new(2.0, palette.focus_ring),
                StrokeKind::Outside,
            );
        }

        // Label.
        if let Some(label) = self.label {
            let color = if self.disabled {
                alpha(palette.text_primary, 0.45)
            } else {
                palette.text_primary
            };
            let pos = egui::pos2(box_rect.right() + gap, rect.center().y);
            ui.painter().text(
                pos,
                egui::Align2::LEFT_CENTER,
                label,
                FontId::new(13.0, egui::FontFamily::Proportional),
                color,
            );
        }
        let _ = TextStyle::Body;
        response
    }
}
