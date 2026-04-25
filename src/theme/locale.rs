//! [`Locale`] — interface language for strings emitted *by the design
//! system itself* (default button labels, status dot labels, etc.).
//!
//! For your own application strings, plug in any i18n crate you like
//! (`fluent`, `rust-i18n`, `gettext-rs`, …): `egui_sauge` does not
//! ship a full i18n runtime, only the few labels it owns.

/// Language used for built-in component strings.
///
/// Default is [`Locale::En`]. Switch with [`crate::set_locale`] at any
/// time; affected components pick up the change on the next frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    /// English (default).
    #[default]
    En,
    /// French.
    Fr,
}

impl Locale {
    /// Best-effort parse from a BCP-47 / ISO-639 prefix, e.g. `"fr-FR"`,
    /// `"FR"`, `"french"`. Anything not French falls back to English.
    pub fn from_lang_code(code: &str) -> Self {
        let lower = code.to_ascii_lowercase();
        if lower.starts_with("fr") || lower.starts_with("french") {
            Self::Fr
        } else {
            Self::En
        }
    }
}

/// Translation keys for strings the design system emits itself.
/// Internal — components call [`tr`] which handles the lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key {
    StatusOnline,
    StatusDegraded,
    StatusOffline,
    StatusIdle,
    ConfirmDefault,
    CancelDefault,
}

/// Translate `key` in the given locale. All bundled translations are
/// `&'static str`; no allocation.
pub(crate) fn tr(locale: Locale, key: Key) -> &'static str {
    use Key::*;
    use Locale::*;
    match (locale, key) {
        (En, StatusOnline) => "Online",
        (En, StatusDegraded) => "Degraded",
        (En, StatusOffline) => "Offline",
        (En, StatusIdle) => "Idle",
        (En, ConfirmDefault) => "Confirm",
        (En, CancelDefault) => "Cancel",

        (Fr, StatusOnline) => "En ligne",
        (Fr, StatusDegraded) => "Dégradé",
        (Fr, StatusOffline) => "Hors ligne",
        (Fr, StatusIdle) => "Inactif",
        (Fr, ConfirmDefault) => "Confirmer",
        (Fr, CancelDefault) => "Annuler",
    }
}
