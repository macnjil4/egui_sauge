//! Ambient theme state stored on [`egui::Context`].
//!
//! Components pull the active [`Palette`] and [`Density`] from the context so
//! callers don't have to thread them through every function signature.
//! [`crate::apply_theme`] / [`crate::apply_theme_with`] populate these slots.

use super::density::Density;
use super::locale::Locale;
use super::palette::Palette;

fn palette_id() -> egui::Id {
    egui::Id::new("egui_sauge::palette")
}

fn density_id() -> egui::Id {
    egui::Id::new("egui_sauge::density")
}

fn locale_id() -> egui::Id {
    egui::Id::new("egui_sauge::locale")
}

fn reduce_motion_id() -> egui::Id {
    egui::Id::new("egui_sauge::reduce_motion")
}

pub(crate) fn store(ctx: &egui::Context, palette: &Palette, density: Density) {
    ctx.data_mut(|d| {
        d.insert_temp(palette_id(), *palette);
        d.insert_temp(density_id(), density);
    });
}

/// Current [`Palette`] for `ctx`. Returns `Palette::light()` if no theme has
/// been applied yet.
pub fn palette_of(ctx: &egui::Context) -> Palette {
    ctx.data(|d| d.get_temp::<Palette>(palette_id()))
        .unwrap_or_else(Palette::light)
}

/// Current [`Density`] for `ctx`. Returns [`Density::Comfortable`] if no theme
/// has been applied yet.
pub fn density_of(ctx: &egui::Context) -> Density {
    ctx.data(|d| d.get_temp::<Density>(density_id()))
        .unwrap_or_default()
}

/// Current [`Locale`] for `ctx`. Defaults to [`Locale::En`].
///
/// This drives the few strings emitted by built-in components
/// (default `ConfirmDialog` button labels, [`crate::components::StatusDot`]
/// labels). Application-level strings are out of scope — pair this with
/// your own i18n crate (fluent, rust-i18n, …) for those.
pub fn locale_of(ctx: &egui::Context) -> Locale {
    ctx.data(|d| d.get_temp::<Locale>(locale_id()))
        .unwrap_or_default()
}

/// Set the active [`Locale`] on `ctx`. Independent from theming —
/// safe to call anytime; affected components pick up the change next frame.
pub fn set_locale(ctx: &egui::Context, locale: Locale) {
    ctx.data_mut(|d| d.insert_temp(locale_id(), locale));
}

/// Whether motion-sensitive UIs should freeze their animations.
///
/// Read by components that animate (`Spinner`, `Skeleton`). Defaults to
/// `false` — call [`set_reduce_motion`] from your app if your user opts
/// in, e.g. matching the OS-level "Reduce Motion" accessibility setting.
pub fn reduce_motion(ctx: &egui::Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(reduce_motion_id()))
        .unwrap_or(false)
}

/// Toggle the reduce-motion flag for `ctx`. See [`reduce_motion`].
pub fn set_reduce_motion(ctx: &egui::Context, on: bool) {
    ctx.data_mut(|d| d.insert_temp(reduce_motion_id(), on));
}
