//! Semantic color palette. Each field maps a *role* (not a hue) to a
//! concrete [`egui::Color32`]. Components should consume roles, never
//! raw hues.

use egui::Color32;

/// A full set of semantic colors for one theme mode (light or dark).
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    /// `true` if this palette represents the dark mode. Used to pick the
    /// right shadow tuning in [`crate::Elevation::shadow`] and anywhere else
    /// behaviour differs between modes.
    pub dark_mode: bool,

    /// App-level background (behind panels).
    pub bg_app: Color32,
    /// Elevated surface (cards, default panel).
    pub bg_surface: Color32,
    /// Secondary surface (subtler elevated area).
    pub bg_surface_alt: Color32,
    /// Hover background for interactive surfaces.
    pub bg_hover: Color32,
    /// Pressed/active background for interactive surfaces.
    pub bg_pressed: Color32,

    /// Very light border, separators.
    pub border_subtle: Color32,
    /// Default border for inputs and cards.
    pub border_default: Color32,
    /// Emphasised border (dividers, focus-adjacent).
    pub border_strong: Color32,

    /// Primary text.
    pub text_primary: Color32,
    /// Secondary text (sub-titles, captions).
    pub text_secondary: Color32,
    /// Tertiary text (placeholders, metadata).
    pub text_tertiary: Color32,
    /// Text that sits on top of a `brand_*` fill.
    pub text_on_brand: Color32,

    /// Brand fill at rest.
    pub brand_default: Color32,
    /// Brand fill on hover.
    pub brand_hover: Color32,
    /// Brand fill when pressed/active.
    pub brand_pressed: Color32,

    /// Focus ring color.
    pub focus_ring: Color32,

    /// Success (positive) semantic.
    pub success: Color32,
    /// Warning (attention) semantic.
    pub warning: Color32,
    /// Error (destructive / failure) semantic.
    pub error: Color32,
    /// Info (neutral highlight) semantic.
    pub info: Color32,
}

impl Palette {
    /// The default light palette.
    pub const fn light() -> Self {
        Self {
            dark_mode: false,
            bg_app: Color32::from_rgb(0xFA, 0xFA, 0xF7),
            bg_surface: Color32::WHITE,
            bg_surface_alt: Color32::from_rgb(0xF1, 0xEF, 0xE8),
            bg_hover: Color32::from_rgb(0xF1, 0xEF, 0xE8),
            bg_pressed: Color32::from_rgb(0xE3, 0xDF, 0xD4),

            border_subtle: Color32::from_rgb(0xE3, 0xDF, 0xD4),
            border_default: Color32::from_rgb(0xCF, 0xC9, 0xBA),
            border_strong: Color32::from_rgb(0x8A, 0x84, 0x73),

            text_primary: Color32::from_rgb(0x1C, 0x1B, 0x16),
            text_secondary: Color32::from_rgb(0x4A, 0x46, 0x38),
            text_tertiary: Color32::from_rgb(0x6E, 0x68, 0x5A),
            text_on_brand: Color32::WHITE,

            // Brand triplet (light): the spec table §4 listed sage-500
            // (#4A8B6B) but white-on-sage-500 lands at 4.04:1 — below AA.
            // Shifted one tonal step darker so `text_on_brand` passes 4.5:1
            // with a margin on every state.
            brand_default: Color32::from_rgb(0x3F, 0x7A, 0x5D),
            brand_hover: Color32::from_rgb(0x2B, 0x5E, 0x45),
            brand_pressed: Color32::from_rgb(0x1C, 0x42, 0x30),
            focus_ring: Color32::from_rgb(0x2B, 0x5E, 0x45),

            // Semantic colors (light): spec §4 values failed 4.5:1 against
            // the very-light app background for `success` (#3F8A5C → 4.02)
            // and `warning` (#B8822A → 3.21). Darkened so every semantic
            // token used as text on `bg_app` clears AA.
            success: Color32::from_rgb(0x2E, 0x70, 0x48),
            warning: Color32::from_rgb(0x8A, 0x63, 0x17),
            error: Color32::from_rgb(0xB2, 0x4A, 0x3E),
            info: Color32::from_rgb(0x3A, 0x7A, 0x8C),
        }
    }

    /// The default dark palette.
    pub const fn dark() -> Self {
        Self {
            dark_mode: true,
            bg_app: Color32::from_rgb(0x14, 0x29, 0x1F),
            bg_surface: Color32::from_rgb(0x1F, 0x2E, 0x25),
            bg_surface_alt: Color32::from_rgb(0x26, 0x36, 0x2C),
            bg_hover: Color32::from_rgb(0x2E, 0x3F, 0x35),
            bg_pressed: Color32::from_rgb(0x38, 0x49, 0x3F),

            border_subtle: Color32::from_rgb(0x2E, 0x3F, 0x35),
            border_default: Color32::from_rgb(0x3E, 0x52, 0x47),
            border_strong: Color32::from_rgb(0x6F, 0xA9, 0x8A),

            text_primary: Color32::from_rgb(0xEC, 0xEA, 0xE1),
            text_secondary: Color32::from_rgb(0xC3, 0xBF, 0xB1),
            text_tertiary: Color32::from_rgb(0x9A, 0x94, 0x85),
            text_on_brand: Color32::from_rgb(0x14, 0x29, 0x1F),

            brand_default: Color32::from_rgb(0x6F, 0xA9, 0x8A),
            brand_hover: Color32::from_rgb(0x85, 0xBF, 0xA1),
            brand_pressed: Color32::from_rgb(0x5A, 0x92, 0x79),
            focus_ring: Color32::from_rgb(0x6F, 0xA9, 0x8A),

            success: Color32::from_rgb(0x6F, 0xB9, 0x8A),
            warning: Color32::from_rgb(0xE0, 0xB0, 0x56),
            error: Color32::from_rgb(0xE5, 0x85, 0x78),
            info: Color32::from_rgb(0x7F, 0xB8, 0xC7),
        }
    }
}

impl Palette {
    /// Build a palette from a base ([`Self::light`] or [`Self::dark`]) and a
    /// closure that overrides individual roles. Useful when you want the
    /// egui_sauge typography / spacing / icons but a different brand color.
    ///
    /// All fields are `pub`, so functional update syntax works too:
    ///
    /// ```no_run
    /// use egui_sauge::Palette;
    /// let p = Palette::custom(Palette::light(), |p| {
    ///     p.brand_default = egui::Color32::from_rgb(0x1F, 0x4D, 0xC2);
    ///     p.brand_hover   = egui::Color32::from_rgb(0x18, 0x3A, 0x96);
    ///     p.brand_pressed = egui::Color32::from_rgb(0x10, 0x27, 0x66);
    /// });
    /// ```
    ///
    /// Customising colors bypasses the WCAG AA invariant enforced for the
    /// stock light/dark palettes by `tests/contrast.rs`. Verify your
    /// custom values yourself if you publish to end users.
    pub fn custom(base: Self, mut overrides: impl FnMut(&mut Self)) -> Self {
        let mut p = base;
        overrides(&mut p);
        p
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light()
    }
}
