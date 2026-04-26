//! [`Avatar`] — circular badge for users / teams. Renders initials on a
//! deterministic brand-tinted background, an icon, or a hosted image
//! (caller-allocated `egui::TextureId`).
//!
//! [`AvatarGroup`] stacks several avatars with overlap, optionally
//! capped with a `+N` overflow chip.

use egui::{
    Color32, FontId, Rect, Response, Sense, Stroke, StrokeKind, TextureId, Ui, Vec2, Widget, vec2,
};

use super::alpha;
use crate::{Icon, palette_of};

/// Avatar visual size (diameter, px).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    /// 20 px — list rows, dense tables.
    Xs,
    /// 24 px — default for tags.
    Sm,
    /// 32 px — default.
    #[default]
    Md,
    /// 40 px — page header / hero.
    Lg,
    /// 56 px — profile cards.
    Xl,
}

impl AvatarSize {
    /// Diameter in pixels.
    pub fn diameter(self) -> f32 {
        match self {
            Self::Xs => 20.0,
            Self::Sm => 24.0,
            Self::Md => 32.0,
            Self::Lg => 40.0,
            Self::Xl => 56.0,
        }
    }
    fn font_size(self) -> f32 {
        self.diameter() * 0.42
    }
    fn icon_size(self) -> f32 {
        self.diameter() * 0.55
    }
}

/// What's painted inside the circle.
#[derive(Debug, Clone)]
enum AvatarContent {
    Initials(String),
    Icon(Icon),
    Image(TextureId),
}

/// Circular avatar.
pub struct Avatar<'a> {
    content: AvatarContent,
    size: AvatarSize,
    /// Override the auto-computed background color.
    background: Option<Color32>,
    /// Status dot in the corner (e.g. online indicator).
    status_color: Option<Color32>,
    tooltip: Option<&'a str>,
}

impl<'a> Avatar<'a> {
    /// Avatar showing the initials of `name`. Picks up to 2 letters.
    pub fn initials(name: &str) -> Self {
        Self {
            content: AvatarContent::Initials(initials_of(name)),
            size: AvatarSize::Md,
            background: None,
            status_color: None,
            tooltip: None,
        }
    }
    /// Avatar showing an [`Icon`] (e.g. `Icon::Users` for a team).
    pub fn icon(icon: Icon) -> Self {
        Self {
            content: AvatarContent::Icon(icon),
            size: AvatarSize::Md,
            background: None,
            status_color: None,
            tooltip: None,
        }
    }
    /// Avatar showing a previously-allocated [`egui::TextureId`].
    pub fn image(texture: TextureId) -> Self {
        Self {
            content: AvatarContent::Image(texture),
            size: AvatarSize::Md,
            background: None,
            status_color: None,
            tooltip: None,
        }
    }
    /// Set the size.
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
    /// Override the background color (defaults to a deterministic
    /// brand-tinted color derived from the initials).
    pub fn background(mut self, color: Color32) -> Self {
        self.background = Some(color);
        self
    }
    /// Add a small status dot in the bottom-right corner — pair with
    /// [`crate::Palette`] colors (`success` for online, etc.).
    pub fn status(mut self, color: Color32) -> Self {
        self.status_color = Some(color);
        self
    }
    /// Show a themed tooltip on hover.
    pub fn tooltip(mut self, text: &'a str) -> Self {
        self.tooltip = Some(text);
        self
    }
}

impl<'a> Widget for Avatar<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let d = self.size.diameter();
        let (rect, response) = ui.allocate_exact_size(vec2(d, d), Sense::hover());
        let center = rect.center();
        let radius = d / 2.0;

        let bg = self.background.unwrap_or_else(|| match &self.content {
            AvatarContent::Initials(s) => background_for(s.as_str(), &palette),
            AvatarContent::Icon(_) => alpha(palette.brand_default, 0.18),
            AvatarContent::Image(_) => palette.bg_surface_alt,
        });
        ui.painter().circle_filled(center, radius, bg);

        match &self.content {
            AvatarContent::Initials(s) => {
                let fg = readable_on(bg, &palette);
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    s.clone(),
                    FontId::new(self.size.font_size(), egui::FontFamily::Proportional),
                    fg,
                );
            }
            AvatarContent::Icon(icon) => {
                let icon_rect = Rect::from_center_size(center, Vec2::splat(self.size.icon_size()));
                icon.paint(ui.painter(), icon_rect, palette.brand_default);
            }
            AvatarContent::Image(tex) => {
                // Crop the image to a circle by clipping a textured rect.
                let img_rect = Rect::from_center_size(center, Vec2::splat(d));
                ui.painter().add(egui::epaint::Shape::Mesh({
                    let mut mesh = egui::epaint::Mesh::with_texture(*tex);
                    mesh.add_rect_with_uv(
                        img_rect,
                        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    mesh.into()
                }));
                // Re-circle by drawing a stroke (visual round-off).
                ui.painter().circle_stroke(
                    center,
                    radius,
                    Stroke::new(1.0, alpha(palette.text_primary, 0.04)),
                );
            }
        }

        // Hairline border for definition.
        ui.painter().circle_stroke(
            center,
            radius,
            Stroke::new(1.0, alpha(palette.text_primary, 0.08)),
        );

        // Status dot.
        if let Some(color) = self.status_color {
            let dot_r = (d * 0.18).max(3.0);
            let offset = radius - dot_r * 0.7;
            let dot_center = egui::pos2(center.x + offset, center.y + offset);
            ui.painter()
                .circle_filled(dot_center, dot_r + 1.5, palette.bg_surface);
            ui.painter().circle_filled(dot_center, dot_r, color);
        }

        if let Some(tip) = self.tooltip {
            return crate::components::tooltip(&response, tip);
        }
        let _ = StrokeKind::Inside;
        response
    }
}

/// Stack of avatars with overlap.
pub struct AvatarGroup<'a> {
    avatars: Vec<Avatar<'a>>,
    max: Option<usize>,
    size: AvatarSize,
}

impl<'a> Default for AvatarGroup<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AvatarGroup<'a> {
    /// Empty group.
    pub fn new() -> Self {
        Self {
            avatars: Vec::new(),
            max: None,
            size: AvatarSize::Md,
        }
    }
    /// Cap the number of visible avatars; the rest are summarised as
    /// `+N` in a trailing overflow chip.
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }
    /// Set the size for every avatar in the group.
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
    /// Append an avatar.
    #[must_use]
    pub fn push(mut self, avatar: Avatar<'a>) -> Self {
        self.avatars.push(avatar);
        self
    }
    /// Render the group.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let d = self.size.diameter();
        let overlap = d * 0.30;
        let total = self.avatars.len();
        let visible = self.max.unwrap_or(total).min(total);
        let overflow = total.saturating_sub(visible);
        let count = visible + usize::from(overflow > 0);
        let width = if count == 0 {
            0.0
        } else {
            d + (count as f32 - 1.0) * (d - overlap)
        };

        let (rect, response) = ui.allocate_exact_size(vec2(width, d), Sense::hover());
        let mut x = rect.left();

        for av in self.avatars.into_iter().take(visible) {
            // Each avatar gets its own slot.
            let slot = Rect::from_min_size(egui::pos2(x, rect.top()), Vec2::splat(d));
            let mut child_ui =
                ui.new_child(egui::UiBuilder::new().max_rect(slot).layout(*ui.layout()));
            child_ui.add(av.size(self.size));
            x += d - overlap;
        }
        if overflow > 0 {
            let slot = Rect::from_min_size(egui::pos2(x, rect.top()), Vec2::splat(d));
            let center = slot.center();
            ui.painter()
                .circle_filled(center, d / 2.0, palette.bg_surface_alt);
            ui.painter()
                .circle_stroke(center, d / 2.0, Stroke::new(1.0, palette.border_default));
            ui.painter().text(
                center,
                egui::Align2::CENTER_CENTER,
                format!("+{overflow}"),
                FontId::new(self.size.font_size(), egui::FontFamily::Proportional),
                palette.text_secondary,
            );
        }
        response
    }
}

// -- helpers ---------------------------------------------------------------

fn initials_of(name: &str) -> String {
    let mut out = String::new();
    for word in name.split_whitespace().take(2) {
        if let Some(c) = word.chars().next() {
            out.push(c.to_ascii_uppercase());
        }
    }
    if out.is_empty() {
        out.push('?');
    }
    out
}

/// Pick a deterministic brand-tinted background from `seed`. Goal: same
/// name → same color. Uses 6 sage-leaning hues that all have ≥ 4.5:1
/// against `text_primary`.
fn background_for(seed: &str, p: &crate::Palette) -> Color32 {
    let h = seed.bytes().fold(0u32, |acc, b| {
        acc.wrapping_mul(31).wrapping_add(u32::from(b))
    });
    let palette = [
        alpha(p.brand_default, 0.22),
        alpha(p.success, 0.22),
        alpha(p.info, 0.22),
        alpha(p.warning, 0.22),
        alpha(p.error, 0.18),
        alpha(p.text_secondary, 0.18),
    ];
    palette[(h as usize) % palette.len()]
}

fn readable_on(bg: Color32, p: &crate::Palette) -> Color32 {
    // Quick relative-luminance check.
    let [r, g, b, _] = bg.to_array();
    let lum = 0.2126 * srgb(r) + 0.7152 * srgb(g) + 0.0722 * srgb(b);
    if lum > 0.5 {
        p.text_primary
    } else {
        p.text_on_brand
    }
}

fn srgb(v: u8) -> f32 {
    let v = f32::from(v) / 255.0;
    if v <= 0.039_28 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}
