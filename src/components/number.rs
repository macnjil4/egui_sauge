//! Numeric input with stepper buttons. [`NumberField`] wraps an `f64`
//! value and exposes min/max clamping plus an optional unit suffix
//! (e.g. `"MB"`, `"%"`).

use egui::{FontId, Rect, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, vec2};

use super::{alpha, corner};
use crate::{Icon, RADIUS, SPACING, palette_of};

/// Numeric field with `+` / `−` stepper.
pub struct NumberField<'a> {
    value: &'a mut f64,
    label: Option<&'a str>,
    helper: Option<&'a str>,
    error: Option<&'a str>,
    suffix: Option<&'a str>,
    min: f64,
    max: f64,
    step: f64,
    decimals: usize,
    disabled: bool,
    width: f32,
}

impl<'a> NumberField<'a> {
    /// New field bound to `value`. Defaults: no clamp, step 1, 0 decimals.
    pub fn new(value: &'a mut f64) -> Self {
        Self {
            value,
            label: None,
            helper: None,
            error: None,
            suffix: None,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            step: 1.0,
            decimals: 0,
            disabled: false,
            width: 200.0,
        }
    }
    /// Add a label above.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    /// Helper line below.
    pub fn helper(mut self, helper: &'a str) -> Self {
        self.helper = Some(helper);
        self
    }
    /// Error line below (overrides helper).
    pub fn error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }
    /// Inline unit suffix shown right of the number, before the stepper.
    pub fn suffix(mut self, suffix: &'a str) -> Self {
        self.suffix = Some(suffix);
        self
    }
    /// Inclusive minimum.
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }
    /// Inclusive maximum.
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }
    /// Increment used by the +/- buttons (default 1).
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
    /// Decimal places shown (default 0).
    pub fn decimals(mut self, n: usize) -> Self {
        self.decimals = n;
        self
    }
    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Field width in pixels.
    pub fn desired_width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Render the field. Returns the inner [`egui::TextEdit`]'s response.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let has_error = self.error.is_some();
        let height = 32.0;

        ui.allocate_ui_with_layout(
            vec2(self.width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                if let Some(label) = self.label {
                    ui.label(
                        egui::RichText::new(label)
                            .font(FontId::new(12.0, egui::FontFamily::Proportional))
                            .color(palette.text_secondary),
                    );
                    ui.add_space(4.0);
                }

                let (outer, _) = ui.allocate_exact_size(
                    vec2(self.width, height),
                    if self.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );
                let border_color = if has_error {
                    palette.error
                } else {
                    palette.border_default
                };
                ui.painter().rect(
                    outer,
                    corner(RADIUS.md),
                    palette.bg_surface,
                    Stroke::new(1.0, border_color),
                    StrokeKind::Inside,
                );

                let stepper_w = 24.0;
                let pad = SPACING.s2;
                let inner_left = outer.left() + pad;
                let inner_right = outer.right() - stepper_w - pad;

                // Stepper buttons (right side).
                let plus_rect = Rect::from_min_size(
                    egui::pos2(outer.right() - stepper_w, outer.top()),
                    vec2(stepper_w, height / 2.0),
                );
                let minus_rect = Rect::from_min_size(
                    egui::pos2(outer.right() - stepper_w, outer.top() + height / 2.0),
                    vec2(stepper_w, height / 2.0),
                );
                let plus_resp = paint_stepper(
                    ui,
                    plus_rect,
                    Icon::Plus,
                    self.disabled || *self.value >= self.max,
                );
                let minus_resp = paint_stepper(
                    ui,
                    minus_rect,
                    Icon::Minus,
                    self.disabled || *self.value <= self.min,
                );
                if plus_resp.clicked() {
                    *self.value = (*self.value + self.step).clamp(self.min, self.max);
                }
                if minus_resp.clicked() {
                    *self.value = (*self.value - self.step).clamp(self.min, self.max);
                }
                // Vertical separator before the stepper column.
                ui.painter().line_segment(
                    [
                        egui::pos2(outer.right() - stepper_w, outer.top() + 4.0),
                        egui::pos2(outer.right() - stepper_w, outer.bottom() - 4.0),
                    ],
                    Stroke::new(1.0, palette.border_subtle),
                );

                // Suffix.
                let suffix_w = if let Some(suffix) = self.suffix {
                    let g = ui.painter().layout_no_wrap(
                        suffix.to_string(),
                        FontId::new(12.0, egui::FontFamily::Monospace),
                        palette.text_tertiary,
                    );
                    let suf_x = inner_right - g.size().x;
                    ui.painter().galley(
                        egui::pos2(suf_x, outer.center().y - g.size().y / 2.0),
                        g.clone(),
                        palette.text_tertiary,
                    );
                    g.size().x + 4.0
                } else {
                    0.0
                };

                let edit_rect = Rect::from_min_max(
                    egui::pos2(inner_left, outer.top() + 4.0),
                    egui::pos2(inner_right - suffix_w, outer.bottom() - 4.0),
                );

                let mut buf = format_number(*self.value, self.decimals);
                let mut edit = egui::TextEdit::singleline(&mut buf)
                    .frame(egui::Frame::NONE)
                    .margin(egui::Margin::ZERO)
                    .desired_width(edit_rect.width())
                    .text_color(palette.text_primary)
                    .font(FontId::new(13.0, egui::FontFamily::Monospace));
                if self.disabled {
                    edit = edit.interactive(false);
                }
                let resp = ui.put(edit_rect, edit);
                if resp.changed()
                    && let Ok(parsed) = buf.trim().parse::<f64>()
                {
                    *self.value = parsed.clamp(self.min, self.max);
                }

                if resp.has_focus() && !self.disabled {
                    ui.painter().rect_stroke(
                        outer.expand(1.0),
                        corner(RADIUS.md),
                        Stroke::new(2.0, palette.focus_ring),
                        StrokeKind::Outside,
                    );
                }

                if let Some(err) = self.error {
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(err)
                            .text_style(TextStyle::Small)
                            .color(palette.error),
                    );
                } else if let Some(help) = self.helper {
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(help)
                            .text_style(TextStyle::Small)
                            .color(palette.text_tertiary),
                    );
                }

                resp
            },
        )
        .inner
    }
}

fn paint_stepper(ui: &mut Ui, rect: Rect, icon: Icon, disabled: bool) -> Response {
    let palette = palette_of(ui.ctx());
    let sense = if disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let resp = ui.interact(rect, ui.next_auto_id().with(icon_key(icon)), sense);
    if !disabled && resp.hovered() {
        ui.painter()
            .rect_filled(rect.shrink(2.0), corner(RADIUS.sm), palette.bg_hover);
    }
    let color = if disabled {
        alpha(palette.text_secondary, 0.45)
    } else {
        palette.text_secondary
    };
    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(12.0));
    icon.paint(ui.painter(), icon_rect, color);
    resp
}

fn icon_key(icon: Icon) -> &'static str {
    match icon {
        Icon::Plus => "plus_step",
        Icon::Minus => "minus_step",
        _ => "step",
    }
}

fn format_number(v: f64, decimals: usize) -> String {
    if decimals == 0 {
        format!("{}", v.round() as i64)
    } else {
        format!("{v:.decimals$}")
    }
}
