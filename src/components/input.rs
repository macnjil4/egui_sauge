//! [`InputField`] — a labelled text input with helper/error states, prefix
//! and suffix icons.

use egui::{FontId, Rect, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, vec2};

use super::corner;
use crate::{Icon, RADIUS, SPACING, palette_of};

/// [`egui::TextEdit`] wrapper with label, helper text, error text, and slot icons.
pub struct InputField<'a> {
    value: &'a mut String,
    label: Option<&'a str>,
    placeholder: Option<&'a str>,
    helper: Option<&'a str>,
    error: Option<&'a str>,
    leading: Option<Icon>,
    trailing: Option<Icon>,
    password: bool,
    disabled: bool,
    desired_width: Option<f32>,
}

impl<'a> InputField<'a> {
    /// New input bound to `value`.
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            label: None,
            placeholder: None,
            helper: None,
            error: None,
            leading: None,
            trailing: None,
            password: false,
            disabled: false,
            desired_width: None,
        }
    }
    /// Add a label above the input.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    /// Placeholder shown when the input is empty.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }
    /// Helper text shown below (overridden by [`Self::error`]).
    pub fn helper(mut self, helper: &'a str) -> Self {
        self.helper = Some(helper);
        self
    }
    /// Error text shown below in red. Overrides helper.
    pub fn error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }
    /// Leading (left-aligned) icon inside the input.
    pub fn leading(mut self, icon: Icon) -> Self {
        self.leading = Some(icon);
        self
    }
    /// Trailing (right-aligned) icon inside the input.
    pub fn trailing(mut self, icon: Icon) -> Self {
        self.trailing = Some(icon);
        self
    }
    /// Mask the text (password entry).
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }
    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Fixed width for the input.
    pub fn desired_width(mut self, w: f32) -> Self {
        self.desired_width = Some(w);
        self
    }

    /// Render the input. Returns the inner [`egui::TextEdit`]'s [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let width = self.desired_width.unwrap_or(260.0);
        let has_error = self.error.is_some();

        ui.allocate_ui_with_layout(
            vec2(width, 0.0),
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

                let height = 32.0;
                let (outer, _outer_resp) = ui.allocate_exact_size(
                    vec2(width, height),
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

                let icon_size = 14.0;
                let pad = SPACING.s2;
                let mut left = outer.left() + pad;
                let mut right = outer.right() - pad;

                if let Some(icon) = self.leading {
                    let r = Rect::from_min_size(
                        egui::pos2(left, outer.center().y - icon_size / 2.0),
                        Vec2::splat(icon_size),
                    );
                    icon.paint(ui.painter(), r, palette.text_tertiary);
                    left += icon_size + pad;
                }
                if let Some(icon) = self.trailing {
                    let r = Rect::from_min_size(
                        egui::pos2(right - icon_size, outer.center().y - icon_size / 2.0),
                        Vec2::splat(icon_size),
                    );
                    icon.paint(ui.painter(), r, palette.text_tertiary);
                    right -= icon_size + pad;
                }

                let inner = Rect::from_min_max(
                    egui::pos2(left, outer.top() + 4.0),
                    egui::pos2(right, outer.bottom() - 4.0),
                );

                let mut edit = egui::TextEdit::singleline(self.value)
                    .frame(egui::Frame::NONE)
                    .margin(egui::Margin::ZERO)
                    .desired_width(inner.width())
                    .password(self.password)
                    .text_color(palette.text_primary);
                if let Some(ph) = self.placeholder {
                    edit = edit.hint_text(ph);
                }
                if self.disabled {
                    edit = edit.interactive(false);
                }
                let resp = ui.put(inner, edit);

                if resp.has_focus() && !self.disabled {
                    let ring = outer.expand(1.0);
                    ui.painter().rect_stroke(
                        ring,
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
