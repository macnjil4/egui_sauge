//! Applies a [`Palette`] to an [`egui::Context`] as a complete [`egui::Style`].

use egui::{CornerRadius, Stroke, Style};

use super::density::Density;
use super::elevation::Elevation;
use super::palette::Palette;
use super::state;
use super::tokens::{RADIUS, SPACING};

/// Push a theme based on `palette` onto `ctx` with the default
/// [`Density::Comfortable`]. Safe to call every frame when the palette may
/// change (e.g. light/dark toggle); otherwise call once at startup.
pub fn apply_theme(ctx: &egui::Context, palette: &Palette) {
    apply_theme_with(ctx, palette, Density::Comfortable);
}

/// Like [`apply_theme`] but lets the caller pick a [`Density`].
pub fn apply_theme_with(ctx: &egui::Context, palette: &Palette, density: Density) {
    // egui 0.34 keeps separate light/dark `Style` values. We force our look
    // on both — the active palette decides what light/dark *means* in our
    // design system, so egui's own toggle never falls back to defaults.
    ctx.all_styles_mut(|style| {
        apply_visuals(style, palette);
        apply_spacing(style, density);
    });
    state::store(ctx, palette, density);
}

fn apply_visuals(style: &mut Style, p: &Palette) {
    let v = &mut style.visuals;
    let r_md = corner(RADIUS.md);

    // Surfaces
    v.window_fill = p.bg_surface;
    v.panel_fill = p.bg_app;
    v.extreme_bg_color = p.bg_surface;
    v.faint_bg_color = p.bg_hover;
    v.window_stroke = Stroke::new(1.0, p.border_default);
    v.window_corner_radius = corner(RADIUS.lg);

    // Shadows — egui windows get Modal-level depth, popups/menus get Popover.
    v.window_shadow = Elevation::Modal.shadow(p.dark_mode);
    v.popup_shadow = Elevation::Popover.shadow(p.dark_mode);

    // Default text color
    v.override_text_color = Some(p.text_primary);

    // Non-interactive widgets (labels, separators)
    v.widgets.noninteractive.bg_fill = p.bg_surface;
    v.widgets.noninteractive.weak_bg_fill = p.bg_surface;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.border_subtle);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.text_secondary);
    v.widgets.noninteractive.corner_radius = r_md;

    // Inactive (rest) interactive widgets
    v.widgets.inactive.bg_fill = p.bg_surface;
    v.widgets.inactive.weak_bg_fill = p.bg_surface;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, p.border_default);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, p.text_primary);
    v.widgets.inactive.corner_radius = r_md;

    // Hovered
    v.widgets.hovered.bg_fill = p.bg_hover;
    v.widgets.hovered.weak_bg_fill = p.bg_hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, p.brand_default);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, p.text_primary);
    v.widgets.hovered.corner_radius = r_md;

    // Active (pressed)
    v.widgets.active.bg_fill = p.bg_pressed;
    v.widgets.active.weak_bg_fill = p.bg_pressed;
    v.widgets.active.bg_stroke = Stroke::new(1.5, p.brand_hover);
    v.widgets.active.fg_stroke = Stroke::new(1.0, p.text_primary);
    v.widgets.active.corner_radius = r_md;

    // Open (e.g. open combobox)
    v.widgets.open.bg_fill = p.bg_surface_alt;
    v.widgets.open.weak_bg_fill = p.bg_surface_alt;
    v.widgets.open.bg_stroke = Stroke::new(1.0, p.brand_default);
    v.widgets.open.fg_stroke = Stroke::new(1.0, p.text_primary);
    v.widgets.open.corner_radius = r_md;

    // Selection + focus
    v.selection.bg_fill = multiply_alpha(p.brand_default, 0.25);
    v.selection.stroke = Stroke::new(2.0, p.focus_ring);

    // Hyperlink
    v.hyperlink_color = p.brand_hover;

    // Error/warning tints used by egui internals
    v.error_fg_color = p.error;
    v.warn_fg_color = p.warning;
}

fn apply_spacing(style: &mut Style, density: Density) {
    let k = density.scale();
    let s = &mut style.spacing;
    s.item_spacing = egui::vec2(SPACING.s2 * k, 6.0 * k);
    s.button_padding = egui::vec2(SPACING.s3 * k, SPACING.s2 * k);
    s.window_margin = egui::Margin::same((SPACING.s4 * k) as i8);
    s.indent = SPACING.s4 * k;
    s.interact_size.y = density.interact_size();
    s.icon_width = 16.0 * k;
    s.icon_spacing = SPACING.s2 * k;
}

/// Convert a spec radius (pixels, f32) to [`CornerRadius`]. egui 0.34 stores
/// radii as `u8`; cast defensively. Radii in this design system stay well
/// under 255 except for `RADIUS.full` (9999), which saturates to 255 — that
/// still reads as fully round for any realistic widget size.
fn corner(px: f32) -> CornerRadius {
    CornerRadius::same(px.round().clamp(0.0, 255.0) as u8)
}

fn multiply_alpha(c: egui::Color32, factor: f32) -> egui::Color32 {
    let [r, g, b, a] = c.to_array();
    let a = (a as f32 * factor).round().clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}
