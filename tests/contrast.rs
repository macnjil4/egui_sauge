//! Validates WCAG AA contrast on every (text, background) pair used by the
//! palette. AA = 4.5:1 for normal text, 3:1 for large text (18pt / 14pt bold).

use egui_sauge::Palette;

fn luminance(c: egui::Color32) -> f64 {
    fn ch(v: u8) -> f64 {
        let v = v as f64 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }
    let [r, g, b, _] = c.to_array();
    0.2126 * ch(r) + 0.7152 * ch(g) + 0.0722 * ch(b)
}

fn ratio(fg: egui::Color32, bg: egui::Color32) -> f64 {
    let l1 = luminance(fg);
    let l2 = luminance(bg);
    let (hi, lo) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (hi + 0.05) / (lo + 0.05)
}

#[track_caller]
fn assert_min(label: &str, fg: egui::Color32, bg: egui::Color32, min: f64) {
    let r = ratio(fg, bg);
    assert!(r >= min, "{label}: contrast {r:.2} below required {min:.2}");
}

fn check(p: &Palette, mode: &str) {
    // Normal-size text (AA = 4.5:1).
    assert_min(
        &format!("{mode}: primary on bg_app"),
        p.text_primary,
        p.bg_app,
        4.5,
    );
    assert_min(
        &format!("{mode}: primary on bg_surface"),
        p.text_primary,
        p.bg_surface,
        4.5,
    );
    assert_min(
        &format!("{mode}: primary on bg_surface_alt"),
        p.text_primary,
        p.bg_surface_alt,
        4.5,
    );
    assert_min(
        &format!("{mode}: secondary on bg_app"),
        p.text_secondary,
        p.bg_app,
        4.5,
    );
    assert_min(
        &format!("{mode}: secondary on bg_surface"),
        p.text_secondary,
        p.bg_surface,
        4.5,
    );
    assert_min(
        &format!("{mode}: tertiary on bg_app"),
        p.text_tertiary,
        p.bg_app,
        4.5,
    );
    assert_min(
        &format!("{mode}: on_brand on brand_default"),
        p.text_on_brand,
        p.brand_default,
        4.5,
    );

    // Semantic colors used as text against the app background. 4.5 is the
    // normal-text bar; if a given color is only ever used as a large-text
    // pill/badge, 3:1 is acceptable, but we hold the full AA bar here.
    assert_min(
        &format!("{mode}: success on bg_app"),
        p.success,
        p.bg_app,
        4.5,
    );
    assert_min(
        &format!("{mode}: warning on bg_app"),
        p.warning,
        p.bg_app,
        4.5,
    );
    assert_min(&format!("{mode}: error on bg_app"), p.error, p.bg_app, 4.5);
    assert_min(&format!("{mode}: info on bg_app"), p.info, p.bg_app, 4.5);
}

#[test]
fn light_palette_is_wcag_aa() {
    check(&Palette::light(), "light");
}

#[test]
fn dark_palette_is_wcag_aa() {
    check(&Palette::dark(), "dark");
}
