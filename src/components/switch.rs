//! A horizontal toggle ([`Switch`]) — a nicer presentation than a checkbox
//! for on/off feature flags.

use egui::{Color32, Response, Sense, Stroke, StrokeKind, Ui, Widget, pos2, vec2};

use super::{alpha, corner};
use crate::{RADIUS, palette_of};

/// On/off toggle.
pub struct Switch<'a> {
    value: &'a mut bool,
    disabled: bool,
}

impl<'a> Switch<'a> {
    /// New switch bound to `value`.
    pub fn new(value: &'a mut bool) -> Self {
        Self {
            value,
            disabled: false,
        }
    }
    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for Switch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let (w, h) = (40.0, 22.0);
        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, mut response) = ui.allocate_exact_size(vec2(w, h), sense);
        if response.clicked() && !self.disabled {
            *self.value = !*self.value;
            response.mark_changed();
        }

        let on = *self.value;
        let track_fill: Color32 = if on {
            palette.brand_default
        } else {
            palette.bg_surface_alt
        };
        let track_stroke = if on {
            palette.brand_default
        } else {
            palette.border_default
        };
        let track_fill = if self.disabled {
            alpha(track_fill, 0.45)
        } else {
            track_fill
        };
        let track_stroke = if self.disabled {
            alpha(track_stroke, 0.45)
        } else {
            track_stroke
        };

        ui.painter().rect(
            rect,
            corner(RADIUS.full),
            track_fill,
            Stroke::new(1.0, track_stroke),
            StrokeKind::Inside,
        );

        let knob_r = 8.0;
        let pad = 3.0;
        let knob_x = if on {
            rect.right() - knob_r - pad
        } else {
            rect.left() + knob_r + pad
        };
        let knob_c = pos2(knob_x, rect.center().y);
        let knob_color = if self.disabled {
            alpha(palette.bg_surface, 0.8)
        } else {
            palette.bg_surface
        };
        ui.painter().circle_filled(knob_c, knob_r, knob_color);
        ui.painter().circle_stroke(
            knob_c,
            knob_r,
            Stroke::new(1.0, alpha(palette.border_default, 0.6)),
        );

        if response.has_focus() {
            let ring = rect.expand(2.0);
            ui.painter().rect_stroke(
                ring,
                corner(RADIUS.full),
                Stroke::new(2.0, palette.focus_ring),
                StrokeKind::Outside,
            );
        }
        response
    }
}
