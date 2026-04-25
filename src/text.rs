//! Font installation and text style registration.

use egui::{FontFamily, FontId, TextStyle};

/// Install the canonical text-style scale on `ctx`
/// (display / heading / h2 / h3 / body-lg / body / button / small / monospace),
/// and register the Phosphor icon font so [`crate::Icon`] glyphs render.
///
/// When built with the `embedded-fonts` feature, also registers Inter and
/// `JetBrains Mono` from `assets/fonts/`. Without that feature, the host's
/// existing font stack is used (egui's default fonts, or whatever the
/// application installed previously).
pub fn install_fonts(ctx: &egui::Context) {
    install_egui_fonts(ctx);
    install_text_styles(ctx);
}

fn install_egui_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    #[cfg(feature = "embedded-fonts")]
    add_embedded_ui_fonts(&mut fonts);

    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    ctx.set_fonts(fonts);
}

#[cfg(feature = "embedded-fonts")]
fn add_embedded_ui_fonts(fonts: &mut egui::FontDefinitions) {
    use egui::FontData;
    use std::sync::Arc;

    fonts.font_data.insert(
        "Inter-Regular".into(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "Inter-Medium".into(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-Medium.ttf"
        ))),
    );
    fonts.font_data.insert(
        "Inter-SemiBold".into(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-SemiBold.ttf"
        ))),
    );
    fonts.font_data.insert(
        "JetBrainsMono".into(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/JetBrainsMono-Regular.ttf"
        ))),
    );

    let prop = fonts.families.entry(FontFamily::Proportional).or_default();
    prop.insert(0, "Inter-Regular".into());

    let mono = fonts.families.entry(FontFamily::Monospace).or_default();
    mono.insert(0, "JetBrainsMono".into());
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
