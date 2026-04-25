//! Buttons with variants, sizes, optional leading/trailing icons.

use egui::{Color32, FontId, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget, vec2};

use super::{alpha, corner};
use crate::{Icon, Palette, RADIUS, SPACING, palette_of};

/// Visual role of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Solid brand fill. Use for the single primary action per screen.
    Primary,
    /// Bordered neutral. Use for secondary actions.
    Secondary,
    /// No background until hover. Use for low-emphasis actions (toolbars).
    Ghost,
    /// Solid error fill. Use for destructive actions (delete, reset…).
    Danger,
}

/// Button size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Small — 26 px tall.
    Sm,
    /// Medium — 32 px tall. Default.
    #[default]
    Md,
    /// Large — 40 px tall.
    Lg,
}

impl ButtonSize {
    fn height(self) -> f32 {
        match self {
            Self::Sm => 26.0,
            Self::Md => 32.0,
            Self::Lg => 40.0,
        }
    }
    fn font(self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Md => 14.0,
            Self::Lg => 15.0,
        }
    }
    fn pad_x(self) -> f32 {
        match self {
            Self::Sm => SPACING.s2,
            Self::Md => SPACING.s3,
            Self::Lg => SPACING.s4,
        }
    }
    fn icon(self) -> f32 {
        match self {
            Self::Sm => 14.0,
            Self::Md => 16.0,
            Self::Lg => 18.0,
        }
    }
}

/// A labelled button with an optional leading and/or trailing icon.
pub struct Button<'a> {
    label: &'a str,
    variant: ButtonVariant,
    size: ButtonSize,
    leading: Option<Icon>,
    trailing: Option<Icon>,
    disabled: bool,
    full_width: bool,
}

impl<'a> Button<'a> {
    /// New button with [`ButtonVariant::Primary`] and [`ButtonSize::Md`].
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            leading: None,
            trailing: None,
            disabled: false,
            full_width: false,
        }
    }
    /// Shortcut for [`ButtonVariant::Primary`].
    pub fn primary(label: &'a str) -> Self {
        Self::new(label).variant(ButtonVariant::Primary)
    }
    /// Shortcut for [`ButtonVariant::Secondary`].
    pub fn secondary(label: &'a str) -> Self {
        Self::new(label).variant(ButtonVariant::Secondary)
    }
    /// Shortcut for [`ButtonVariant::Ghost`].
    pub fn ghost(label: &'a str) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }
    /// Shortcut for [`ButtonVariant::Danger`].
    pub fn danger(label: &'a str) -> Self {
        Self::new(label).variant(ButtonVariant::Danger)
    }
    /// Override the variant.
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    /// Override the size.
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }
    /// Icon painted before the label.
    pub fn leading(mut self, icon: Icon) -> Self {
        self.leading = Some(icon);
        self
    }
    /// Icon painted after the label.
    pub fn trailing(mut self, icon: Icon) -> Self {
        self.trailing = Some(icon);
        self
    }
    /// Disable interaction (no hover, 0.45 opacity).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Stretch to the full available width.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let font = FontId::new(self.size.font(), egui::FontFamily::Proportional);
        let text_color = text_color(&palette, self.variant);

        let galley = ui
            .painter()
            .layout_no_wrap(self.label.into(), font.clone(), text_color);

        let icon_size = self.size.icon();
        let gap = SPACING.s2;
        let pad_x = self.size.pad_x();

        let mut content_w = galley.size().x;
        if self.leading.is_some() {
            content_w += icon_size + gap;
        }
        if self.trailing.is_some() {
            content_w += icon_size + gap;
        }
        let min_w = content_w + pad_x * 2.0;
        let height = self.size.height();
        let desired = if self.full_width {
            vec2(ui.available_width(), height)
        } else {
            vec2(min_w, height)
        };

        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = ui.allocate_exact_size(desired, sense);

        let (fill, stroke_color) = colors(&palette, self.variant, &response, self.disabled);
        let stroke = match self.variant {
            ButtonVariant::Secondary => Stroke::new(1.0, stroke_color),
            ButtonVariant::Ghost if response.hovered() => Stroke::new(1.0, stroke_color),
            _ => Stroke::NONE,
        };

        ui.painter()
            .rect(rect, corner(RADIUS.md), fill, stroke, StrokeKind::Inside);

        // Focus ring.
        if response.has_focus() {
            let ring = rect.expand(2.0);
            ui.painter().rect_stroke(
                ring,
                corner(RADIUS.md + 2.0),
                Stroke::new(2.0, palette.focus_ring),
                StrokeKind::Outside,
            );
        }

        // Layout content.
        let text_w = galley.size().x;
        let mut total_w = text_w;
        if self.leading.is_some() {
            total_w += icon_size + gap;
        }
        if self.trailing.is_some() {
            total_w += icon_size + gap;
        }
        let mut cursor_x = rect.center().x - total_w / 2.0;
        let cy = rect.center().y;

        if let Some(icon) = self.leading {
            let r = Rect::from_min_size(
                egui::pos2(cursor_x, cy - icon_size / 2.0),
                Vec2::splat(icon_size),
            );
            icon.paint(ui.painter(), r, text_color);
            cursor_x += icon_size + gap;
        }

        let text_pos = egui::pos2(cursor_x, cy - galley.size().y / 2.0);
        ui.painter().galley(text_pos, galley, text_color);
        cursor_x += text_w;

        if let Some(icon) = self.trailing {
            cursor_x += gap;
            let r = Rect::from_min_size(
                egui::pos2(cursor_x, cy - icon_size / 2.0),
                Vec2::splat(icon_size),
            );
            icon.paint(ui.painter(), r, text_color);
        }

        response
    }
}

/// Square icon-only button.
pub struct IconButton {
    icon: Icon,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    tooltip: Option<String>,
}

impl IconButton {
    /// A ghost-variant icon button by default.
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Md,
            disabled: false,
            tooltip: None,
        }
    }
    /// Override the variant.
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    /// Override the size.
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }
    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Attach a tooltip shown on hover.
    pub fn tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip = Some(text.into());
        self
    }
}

impl Widget for IconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let h = self.size.height();
        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(h, h), sense);

        let (fill, stroke_color) = colors(&palette, self.variant, &response, self.disabled);
        let stroke = match self.variant {
            ButtonVariant::Secondary => Stroke::new(1.0, stroke_color),
            ButtonVariant::Ghost if response.hovered() => Stroke::new(1.0, stroke_color),
            _ => Stroke::NONE,
        };
        ui.painter()
            .rect(rect, corner(RADIUS.md), fill, stroke, StrokeKind::Inside);

        if response.has_focus() {
            let ring = rect.expand(2.0);
            ui.painter().rect_stroke(
                ring,
                corner(RADIUS.md + 2.0),
                Stroke::new(2.0, palette.focus_ring),
                StrokeKind::Outside,
            );
        }

        let icon_size = self.size.icon();
        let r = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
        self.icon
            .paint(ui.painter(), r, text_color(&palette, self.variant));

        if let Some(tip) = &self.tooltip {
            return response.on_hover_text(tip);
        }
        response
    }
}

// --- helpers -------------------------------------------------------------

fn text_color(p: &Palette, v: ButtonVariant) -> Color32 {
    match v {
        ButtonVariant::Primary => p.text_on_brand,
        ButtonVariant::Secondary => p.text_primary,
        ButtonVariant::Ghost => p.text_primary,
        ButtonVariant::Danger => p.text_on_brand,
    }
}

fn colors(
    p: &Palette,
    v: ButtonVariant,
    response: &Response,
    disabled: bool,
) -> (Color32, Color32) {
    let (mut fill, mut stroke) = match v {
        ButtonVariant::Primary => (p.brand_default, p.brand_default),
        ButtonVariant::Secondary => (p.bg_surface, p.border_default),
        ButtonVariant::Ghost => (Color32::TRANSPARENT, p.border_default),
        ButtonVariant::Danger => (p.error, p.error),
    };

    if !disabled {
        if response.is_pointer_button_down_on() {
            fill = match v {
                ButtonVariant::Primary => p.brand_pressed,
                ButtonVariant::Secondary => p.bg_pressed,
                ButtonVariant::Ghost => p.bg_pressed,
                ButtonVariant::Danger => alpha(p.error, 0.85),
            };
            stroke = fill;
        } else if response.hovered() {
            fill = match v {
                ButtonVariant::Primary => p.brand_hover,
                ButtonVariant::Secondary => p.bg_hover,
                ButtonVariant::Ghost => p.bg_hover,
                ButtonVariant::Danger => alpha(p.error, 0.92),
            };
            stroke = match v {
                ButtonVariant::Secondary => p.brand_default,
                _ => fill,
            };
        }
    }

    if disabled {
        fill = alpha(fill, 0.45);
        stroke = alpha(stroke, 0.45);
    }

    (fill, stroke)
}
