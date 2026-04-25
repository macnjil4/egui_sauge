//! Small status atoms: [`Badge`], [`Tag`], [`StatusDot`], [`Kbd`].

use egui::{Color32, FontId, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget, vec2};

use super::{alpha, corner};
use crate::{Icon, Palette, RADIUS, SPACING, palette_of};

// -- Badge -------------------------------------------------------------------

/// Visual tone of a [`Badge`] or [`Tag`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeTone {
    /// Neutral (`border_default` on `bg_surface_alt`).
    Neutral,
    /// Brand-tinted.
    Brand,
    /// Success semantic.
    Success,
    /// Warning semantic.
    Warning,
    /// Error semantic.
    Error,
    /// Info semantic.
    Info,
}

/// Whether a [`Badge`] renders as solid or soft-tinted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeStyle {
    /// Solid fill, light text.
    Solid,
    /// Soft tinted fill (≈12% alpha), colored text.
    Soft,
}

/// Tiny pill used for status, counts, categorisation.
pub struct Badge<'a> {
    label: &'a str,
    tone: BadgeTone,
    style: BadgeStyle,
    leading: Option<Icon>,
}

impl<'a> Badge<'a> {
    /// Neutral soft badge.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            tone: BadgeTone::Neutral,
            style: BadgeStyle::Soft,
            leading: None,
        }
    }
    /// Change the tone.
    pub fn tone(mut self, tone: BadgeTone) -> Self {
        self.tone = tone;
        self
    }
    /// Pick solid vs soft style.
    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }
    /// Add a small leading icon.
    pub fn leading(mut self, icon: Icon) -> Self {
        self.leading = Some(icon);
        self
    }
}

impl<'a> Widget for Badge<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let (fill, fg) = tone_colors(&palette, self.tone, self.style);
        paint_pill(ui, self.label, self.leading, fill, fg, 11.0, 3.0)
    }
}

// -- Tag ---------------------------------------------------------------------

/// Larger, closable chip. Contains a label and an optional "close" affordance.
pub struct Tag<'a, 'b> {
    label: &'a str,
    tone: BadgeTone,
    on_close: Option<&'b mut bool>,
}

impl<'a, 'b> Tag<'a, 'b> {
    /// New neutral tag.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            tone: BadgeTone::Neutral,
            on_close: None,
        }
    }
    /// Change the tone.
    pub fn tone(mut self, tone: BadgeTone) -> Self {
        self.tone = tone;
        self
    }
    /// Attach a closable flag — flipped to `true` if the user clicks the × icon.
    pub fn closable(mut self, flag: &'b mut bool) -> Self {
        self.on_close = Some(flag);
        self
    }
}

impl<'a, 'b> Widget for Tag<'a, 'b> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let (fill, fg) = tone_colors(&palette, self.tone, BadgeStyle::Soft);
        let font = FontId::new(12.0, egui::FontFamily::Proportional);
        let galley = ui.painter().layout_no_wrap(self.label.into(), font, fg);

        let pad = vec2(SPACING.s2, 4.0);
        let close_w = if self.on_close.is_some() {
            14.0 + SPACING.s1
        } else {
            0.0
        };
        let size = vec2(
            galley.size().x + pad.x * 2.0 + close_w,
            galley.size().y + pad.y * 2.0,
        );
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        ui.painter().rect_filled(rect, corner(RADIUS.full), fill);

        let text_pos = egui::pos2(rect.left() + pad.x, rect.center().y - galley.size().y / 2.0);
        ui.painter().galley(text_pos, galley, fg);

        if let Some(flag) = self.on_close {
            let icon_size = 12.0;
            let icon_rect = Rect::from_min_size(
                egui::pos2(
                    rect.right() - pad.x - icon_size,
                    rect.center().y - icon_size / 2.0,
                ),
                Vec2::splat(icon_size),
            );
            let click_r = icon_rect.expand(3.0);
            let clicked = ui
                .interact(click_r, response.id.with("close"), Sense::click())
                .clicked();
            let c = if clicked { fg } else { alpha(fg, 0.7) };
            Icon::Close.paint(ui.painter(), icon_rect, c);
            if clicked {
                *flag = true;
            }
        }

        response
    }
}

// -- StatusDot --------------------------------------------------------------

/// Operational state signalled by a colored dot + label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    /// Healthy / up / active.
    Online,
    /// Degraded / warning state.
    Degraded,
    /// Down / incident.
    Offline,
    /// Idle / neutral.
    Idle,
}

impl StatusLevel {
    fn color(self, p: &Palette) -> Color32 {
        match self {
            Self::Online => p.success,
            Self::Degraded => p.warning,
            Self::Offline => p.error,
            Self::Idle => p.text_tertiary,
        }
    }
    fn label(self, locale: crate::Locale) -> &'static str {
        use crate::theme::locale::{Key, tr};
        let key = match self {
            Self::Online => Key::StatusOnline,
            Self::Degraded => Key::StatusDegraded,
            Self::Offline => Key::StatusOffline,
            Self::Idle => Key::StatusIdle,
        };
        tr(locale, key)
    }
}

/// Dot + optional label. Designed for dashboards, server lists, service
/// health tiles.
pub struct StatusDot<'a> {
    level: StatusLevel,
    label: Option<&'a str>,
    pulse: bool,
}

impl<'a> StatusDot<'a> {
    /// New dot with the default label (Online/Degraded/Offline/Idle).
    pub fn new(level: StatusLevel) -> Self {
        Self {
            level,
            label: None,
            pulse: false,
        }
    }
    /// Override the label text.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    /// Hide the label, just render the dot.
    pub fn dot_only(mut self) -> Self {
        self.label = Some("");
        self
    }
    /// Draw a subtle outer halo to suggest activity. Pure presentation; no
    /// animation to avoid pulling in time sources.
    pub fn pulse(mut self) -> Self {
        self.pulse = true;
        self
    }
}

impl<'a> Widget for StatusDot<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let locale = crate::locale_of(ui.ctx());
        let c = self.level.color(&palette);
        let dot_r: f32 = 5.0;
        let halo_r: f32 = if self.pulse { 9.0 } else { 0.0 };
        let text = self.label.unwrap_or_else(|| self.level.label(locale));
        let font = FontId::new(12.0, egui::FontFamily::Proportional);
        let galley = ui
            .painter()
            .layout_no_wrap(text.into(), font, palette.text_secondary);

        let dot_space = halo_r.max(dot_r) * 2.0;
        let gap = if text.is_empty() { 0.0 } else { SPACING.s2 };
        let size = vec2(
            dot_space + gap + galley.size().x,
            dot_space.max(galley.size().y),
        );
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

        let dot_center = egui::pos2(rect.left() + dot_space / 2.0, rect.center().y);
        if self.pulse {
            ui.painter()
                .circle_filled(dot_center, halo_r, alpha(c, 0.25));
        }
        ui.painter().circle_filled(dot_center, dot_r, c);

        if !text.is_empty() {
            let text_pos = egui::pos2(
                dot_center.x + dot_r + gap,
                rect.center().y - galley.size().y / 2.0,
            );
            ui.painter()
                .galley(text_pos, galley, palette.text_secondary);
        }
        response
    }
}

// -- Kbd --------------------------------------------------------------------

/// Keyboard shortcut chip (e.g. `⌘K`, `Ctrl+Enter`).
pub struct Kbd<'a> {
    keys: &'a str,
}

impl<'a> Kbd<'a> {
    /// New kbd chip.
    pub fn new(keys: &'a str) -> Self {
        Self { keys }
    }
}

impl<'a> Widget for Kbd<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let font = FontId::new(11.0, egui::FontFamily::Monospace);
        let galley = ui
            .painter()
            .layout_no_wrap(self.keys.into(), font, palette.text_secondary);

        let pad = vec2(SPACING.s2, 2.0);
        let size = galley.size() + pad * 2.0;
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        ui.painter().rect(
            rect,
            corner(RADIUS.sm),
            palette.bg_surface_alt,
            Stroke::new(1.0, palette.border_default),
            StrokeKind::Inside,
        );
        let text_pos = egui::pos2(rect.left() + pad.x, rect.center().y - galley.size().y / 2.0);
        ui.painter()
            .galley(text_pos, galley, palette.text_secondary);
        response
    }
}

// -- shared helpers ---------------------------------------------------------

fn tone_colors(p: &Palette, tone: BadgeTone, style: BadgeStyle) -> (Color32, Color32) {
    let base = match tone {
        BadgeTone::Neutral => p.text_secondary,
        BadgeTone::Brand => p.brand_default,
        BadgeTone::Success => p.success,
        BadgeTone::Warning => p.warning,
        BadgeTone::Error => p.error,
        BadgeTone::Info => p.info,
    };
    match style {
        BadgeStyle::Solid => (base, p.text_on_brand),
        BadgeStyle::Soft => {
            let fill = if matches!(tone, BadgeTone::Neutral) {
                p.bg_surface_alt
            } else {
                alpha(base, 0.18)
            };
            let fg = if matches!(tone, BadgeTone::Neutral) {
                p.text_secondary
            } else {
                base
            };
            (fill, fg)
        }
    }
}

fn paint_pill(
    ui: &mut Ui,
    label: &str,
    leading: Option<Icon>,
    fill: Color32,
    fg: Color32,
    font_size: f32,
    pad_y: f32,
) -> Response {
    let font = FontId::new(font_size, egui::FontFamily::Proportional);
    let galley = ui.painter().layout_no_wrap(label.into(), font, fg);
    let icon_size = font_size;
    let gap = 4.0;
    let pad_x = SPACING.s2;
    let mut content_w = galley.size().x;
    if leading.is_some() {
        content_w += icon_size + gap;
    }
    let size = vec2(content_w + pad_x * 2.0, galley.size().y + pad_y * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(rect, corner(RADIUS.full), fill);

    let mut cursor_x = rect.left() + pad_x;
    let cy = rect.center().y;
    if let Some(icon) = leading {
        let r = Rect::from_min_size(
            egui::pos2(cursor_x, cy - icon_size / 2.0),
            Vec2::splat(icon_size),
        );
        icon.paint(ui.painter(), r, fg);
        cursor_x += icon_size + gap;
    }
    let text_pos = egui::pos2(cursor_x, cy - galley.size().y / 2.0);
    ui.painter().galley(text_pos, galley, fg);
    response
}
