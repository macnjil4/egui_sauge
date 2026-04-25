//! Elevation scale: four levels of shadow, from flat to modal.
//!
//! Spec §8 describes shadows in CSS-ish terms (offset, blur/spread, color).
//! egui's [`egui::epaint::Shadow`] uses `i8` / `u8` for everything, so
//! values are rounded accordingly.

use egui::{Color32, epaint::Shadow};

/// Four elevation levels: 0 flat, 1 card, 2 popover, 3 modal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Elevation {
    /// Flat. No shadow.
    Flat = 0,
    /// Cards, inline surfaces.
    Card = 1,
    /// Popovers, menus, tooltips.
    Popover = 2,
    /// Modals, dialogs.
    Modal = 3,
}

impl Elevation {
    /// Return the [`Shadow`] for this elevation in the given mode.
    ///
    /// `dark` selects the darker, denser shadow tuning used in dark mode
    /// (spec §8: alpha × 1.6, color shifted to pure black).
    pub fn shadow(self, dark: bool) -> Shadow {
        let (offset_y, blur, spread, alpha_light) = match self {
            Self::Flat => return Shadow::NONE,
            // Spec: (0,1) blur 2, alpha 0.06
            Self::Card => (1i8, 2u8, 0u8, 15u8),
            // Spec: (0,4) blur 12, alpha 0.10
            Self::Popover => (4, 12, 0, 26),
            // Spec: (0,12) blur 32, alpha 0.14
            Self::Modal => (12, 32, 0, 36),
        };

        let color = if dark {
            // Dark mode: pure black, alpha × 1.6 (clamped to u8).
            let a = ((alpha_light as u16) * 160 / 100).min(255) as u8;
            Color32::from_rgba_premultiplied(0, 0, 0, a)
        } else {
            // Light mode: greenish-black so shadows stay on-brand.
            Color32::from_rgba_premultiplied(20, 25, 20, alpha_light)
        };

        Shadow {
            offset: [0, offset_y],
            blur,
            spread,
            color,
        }
    }
}
