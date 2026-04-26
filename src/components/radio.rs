//! Typed [`RadioGroup`] with vertical or horizontal layout, helper / error
//! footer, and disabled per-option support.

use egui::{FontId, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, vec2};

use super::{alpha, corner};
use crate::{RADIUS, SPACING, palette_of};

/// Layout direction of a [`RadioGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioLayout {
    /// One option per row (default).
    Vertical,
    /// All options on a single row.
    Horizontal,
}

/// One radio option.
#[derive(Debug, Clone)]
pub struct RadioOption<'a, T> {
    /// Underlying value bound to the group's `selected`.
    pub value: T,
    /// Label shown next to the dot.
    pub label: &'a str,
    /// Optional helper line under the label.
    pub helper: Option<&'a str>,
    /// Disable just this option.
    pub disabled: bool,
}

impl<'a, T> RadioOption<'a, T> {
    /// New simple option.
    pub fn new(value: T, label: &'a str) -> Self {
        Self {
            value,
            label,
            helper: None,
            disabled: false,
        }
    }
    /// Add a helper line.
    pub fn helper(mut self, helper: &'a str) -> Self {
        self.helper = Some(helper);
        self
    }
    /// Mark this option disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Single-choice picker over a typed enum.
pub struct RadioGroup<'a, T: PartialEq + Clone> {
    selected: &'a mut T,
    label: Option<&'a str>,
    helper: Option<&'a str>,
    error: Option<&'a str>,
    layout: RadioLayout,
    options: Vec<RadioOption<'a, T>>,
}

impl<'a, T: PartialEq + Clone> RadioGroup<'a, T> {
    /// New empty group bound to `selected`.
    pub fn new(selected: &'a mut T) -> Self {
        Self {
            selected,
            label: None,
            helper: None,
            error: None,
            layout: RadioLayout::Vertical,
            options: Vec::new(),
        }
    }
    /// Group label above the options.
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
    /// Switch to horizontal layout.
    pub fn horizontal(mut self) -> Self {
        self.layout = RadioLayout::Horizontal;
        self
    }
    /// Append an option.
    pub fn option(mut self, opt: RadioOption<'a, T>) -> Self {
        self.options.push(opt);
        self
    }
    /// Render the group. Returns the response of the last option row.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let Self {
            selected,
            label,
            helper,
            error,
            layout,
            options,
        } = self;
        let has_error = error.is_some();

        let last_resp = ui
            .vertical(|ui| {
                if let Some(label) = label {
                    ui.label(
                        egui::RichText::new(label)
                            .font(FontId::new(12.0, egui::FontFamily::Proportional))
                            .color(palette.text_secondary),
                    );
                    ui.add_space(SPACING.s1);
                }
                let mut last: Option<Response> = None;
                match layout {
                    RadioLayout::Vertical => {
                        for opt in &options {
                            last = Some(paint_option(ui, opt, selected, has_error));
                        }
                    }
                    RadioLayout::Horizontal => {
                        ui.horizontal(|ui| {
                            for opt in &options {
                                last = Some(paint_option(ui, opt, selected, has_error));
                            }
                        });
                    }
                }
                if let Some(err) = error {
                    ui.add_space(SPACING.s1);
                    ui.label(
                        egui::RichText::new(err)
                            .text_style(TextStyle::Small)
                            .color(palette.error),
                    );
                } else if let Some(help) = helper {
                    ui.add_space(SPACING.s1);
                    ui.label(
                        egui::RichText::new(help)
                            .text_style(TextStyle::Small)
                            .color(palette.text_tertiary),
                    );
                }
                last
            })
            .inner;
        last_resp.unwrap_or_else(|| ui.allocate_response(vec2(0.0, 0.0), Sense::hover()))
    }
}

fn paint_option<T: PartialEq + Clone>(
    ui: &mut Ui,
    opt: &RadioOption<'_, T>,
    selected: &mut T,
    error: bool,
) -> Response {
    let palette = palette_of(ui.ctx());
    let dot_size: f32 = 16.0;
    let gap = SPACING.s2;

    let is_selected = *selected == opt.value;

    let label_galley = ui.painter().layout_no_wrap(
        opt.label.to_string(),
        FontId::new(13.0, egui::FontFamily::Proportional),
        palette.text_primary,
    );
    let helper_galley = opt.helper.map(|h| {
        ui.painter().layout_no_wrap(
            h.to_string(),
            FontId::new(11.0, egui::FontFamily::Proportional),
            palette.text_tertiary,
        )
    });

    let text_h = label_galley.size().y + helper_galley.as_ref().map_or(0.0, |g| g.size().y + 2.0);
    let total = vec2(
        dot_size
            + gap
            + label_galley
                .size()
                .x
                .max(helper_galley.as_ref().map_or(0.0, |g| g.size().x)),
        dot_size.max(text_h),
    );
    let sense = if opt.disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, mut response) = ui.allocate_exact_size(total, sense);
    if response.clicked() && !opt.disabled {
        *selected = opt.value.clone();
        response.mark_changed();
    }

    // Outer ring.
    let center = egui::pos2(rect.left() + dot_size / 2.0, rect.top() + dot_size / 2.0);
    let stroke_color = if error {
        palette.error
    } else if is_selected || (response.hovered() && !opt.disabled) {
        palette.brand_default
    } else {
        palette.border_default
    };
    let stroke_color = if opt.disabled {
        alpha(stroke_color, 0.45)
    } else {
        stroke_color
    };
    ui.painter()
        .circle_stroke(center, dot_size / 2.0, Stroke::new(1.5, stroke_color));
    if is_selected {
        let inner = if error {
            palette.error
        } else {
            palette.brand_default
        };
        let inner = if opt.disabled {
            alpha(inner, 0.45)
        } else {
            inner
        };
        ui.painter()
            .circle_filled(center, dot_size / 2.0 - 4.0, inner);
    }

    if response.has_focus() && !opt.disabled {
        ui.painter().rect_stroke(
            egui::Rect::from_center_size(center, egui::Vec2::splat(dot_size + 4.0)),
            corner(RADIUS.full),
            Stroke::new(2.0, palette.focus_ring),
            StrokeKind::Outside,
        );
    }

    // Label + helper.
    let text_x = rect.left() + dot_size + gap;
    let label_color = if opt.disabled {
        alpha(palette.text_primary, 0.45)
    } else {
        palette.text_primary
    };
    ui.painter()
        .galley(egui::pos2(text_x, rect.top()), label_galley, label_color);
    if let Some(g) = helper_galley {
        let h_color = if opt.disabled {
            alpha(palette.text_tertiary, 0.45)
        } else {
            palette.text_tertiary
        };
        ui.painter()
            .galley(egui::pos2(text_x, rect.top() + 16.0), g, h_color);
    }

    ui.add_space(SPACING.s1);
    response
}
