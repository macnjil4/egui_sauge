//! Font installation and text style registration.

use egui::{FontFamily, FontId, TextStyle};

/// Install the canonical text-style scale on `ctx`
/// (display / heading / h2 / h3 / body-lg / body / button / small / monospace),
/// and register the Phosphor icon font so [`crate::Icon`] glyphs render.
///
/// UI text uses egui's bundled fonts. To embed a custom UI typeface (e.g.
/// Inter, JetBrains Mono), register it on the `Context` *before* calling
/// `install_fonts`, or build your own [`egui::FontDefinitions`]:
///
/// ```ignore
/// use std::sync::Arc;
/// use egui::{FontData, FontDefinitions, FontFamily};
///
/// let mut fonts = FontDefinitions::default();
/// fonts.font_data.insert(
///     "Inter".into(),
///     Arc::new(FontData::from_static(include_bytes!("../path/to/Inter-Regular.ttf"))),
/// );
/// fonts
///     .families
///     .entry(FontFamily::Proportional)
///     .or_default()
///     .insert(0, "Inter".into());
///
/// // Then let egui_sauge layer Phosphor + the type scale on top:
/// egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
/// ctx.set_fonts(fonts);
/// egui_sauge::install_fonts(ctx); // sets the text-style scale
/// ```
pub fn install_fonts(ctx: &egui::Context) {
    install_egui_fonts(ctx);
    install_text_styles(ctx);
}

fn install_egui_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}

/// Register an additional [`egui_phosphor::Variant`] on top of the regular
/// font that [`install_fonts`] already wires.
///
/// Useful when you want filled or bold icons for selected / active states.
/// Enable the matching Cargo feature first (`icons-bold`, `icons-fill`,
/// `icons-light`, `icons-thin`) so the variant's constants are available.
///
/// ```ignore
/// # // Cargo.toml: egui_sauge = { version = "1.1", features = ["icons-fill"] }
/// egui_sauge::install_fonts(ctx);
/// egui_sauge::install_phosphor_variant(ctx, egui_phosphor::Variant::Fill);
/// // Then anywhere:
/// egui_sauge::Icon::Glyph(egui_phosphor::fill::HEART).show(ui, 16.0, color);
/// ```
///
/// Calling this overrides the previously registered Phosphor font with
/// the new variant. To use multiple weights simultaneously you currently
/// need to manage `FontDefinitions` yourself.
pub fn install_phosphor_variant(ctx: &egui::Context, variant: egui_phosphor::Variant) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, variant);
    ctx.set_fonts(fonts);
}

fn install_text_styles(ctx: &egui::Context) {
    // Apply to BOTH stored styles (egui keeps separate light/dark Style values
    // and we don't know which one egui will pick depending on the system /
    // user theme preference). Without this, switching themes drops our named
    // text styles and panics with "Failed to find Name(\"h2\") in
    // Style::text_styles".
    let text_styles: std::collections::BTreeMap<TextStyle, FontId> = [
        (
            TextStyle::Name("display".into()),
            FontId::new(40.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Heading,
            FontId::new(28.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("h2".into()),
            FontId::new(20.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("h3".into()),
            FontId::new(16.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("body-lg".into()),
            FontId::new(16.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(12.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(13.0, FontFamily::Monospace),
        ),
    ]
    .into_iter()
    .collect();

    ctx.all_styles_mut(|style| style.text_styles = text_styles.clone());
}
