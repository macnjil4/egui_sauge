# egui_sauge

A fresh, natural design system for [egui](https://github.com/emilk/egui) — sage palette, warm neutrals, WCAG AA contrast, and a ready-to-use component library aimed at IT applications.

```rust
use eframe::egui;
use egui_sauge::{Palette, Density, Locale, apply_theme_with, install_fonts, set_locale};
use egui_sauge::components::{Button, Card, Section};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "My app",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            install_fonts(&cc.egui_ctx);
            apply_theme_with(&cc.egui_ctx, &Palette::light(), Density::Comfortable);
            set_locale(&cc.egui_ctx, Locale::En); // or Locale::Fr
            Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)
        }),
    )
}
```

## What's in the box

**Theme**
- 20-role semantic palette (light + dark, both WCAG AA validated).
- 4-pt spacing scale, 5-step radius scale, 4-level elevation with real shadows.
- Density preset (`Comfortable` / `Compact`).
- Locale (`En` / `Fr`) for the strings the design system itself emits.
- ~80 named icons backed by [Phosphor](https://phosphoricons.com/), plus `Icon::Glyph` and `Icon::Custom` escape hatches.

**Components** (`egui_sauge::components`)

| Group | Items |
|---|---|
| Buttons | `Button` (Primary / Secondary / Ghost / Danger × Sm / Md / Lg, leading & trailing icons), `IconButton` (with tooltip) |
| Status & badges | `Badge`, `Tag` (closable), `StatusDot` (online / degraded / offline / idle, pulse), `Kbd` |
| Feedback | `Spinner`, `ProgressBar`, `Alert`, `Toast` / `Toasts` (auto-dismiss stack) |
| Containers | `Card`, `Section`, `EmptyState`, `Stat` (with trend), `CodeBlock` |
| Forms | `InputField` (label / helper / error / icons / password), `SelectField`, `Switch` |
| Overlays | `Dialog`, `ConfirmDialog`, `MenuItem`, `SubMenu` (nests arbitrarily deep), `tooltip(...)` / `TooltipExt` |
| Navigation | `NavItem`, `Tabs<T>`, `Breadcrumb`, `PageHeader` |
| Data | `KeyValue`, `LogLine` (timestamped, level-colored), `Skeleton` |

See `GUIDE.md` for the **UX/UI playbook** — page composition, when to use each navigation pattern, modal vs side panel, button order, typography hierarchy, accessibility checklist, IT-app patterns.

## Live demo

```bash
cargo run --example showcase
```

Every component, both color modes, both density presets, both locales, in one window.

## Adding to your project

The crate is not yet on crates.io. Pick the option that fits your workflow:

```toml
# Option 1 — local path (during development)
egui_sauge = { path = "../egui_sauge" }

# Option 2 — git
egui_sauge = { git = "https://github.com/<user>/egui_sauge", tag = "v1.0.0" }

# Option 3 — crates.io (once published)
egui_sauge = "1.0"

# In all cases, your egui must match egui_sauge's pinned version:
egui   = "0.34"
eframe = "0.34"
```

## Compatibility

| egui_sauge | egui    | eframe  | rustc (MSRV) | edition |
| ---------- | ------- | ------- | ------------ | ------- |
| 1.x        | 0.34.x  | 0.34.x  | 1.92         | 2024    |

## Fonts

By default, `install_fonts` installs the typographic scale and registers the Phosphor icon font. UI text uses egui's bundled fonts.

To also embed Inter + JetBrains Mono in your binary, drop these TTFs into `assets/fonts/`:

- `Inter-Regular.ttf`
- `Inter-Medium.ttf`
- `Inter-SemiBold.ttf`
- `JetBrainsMono-Regular.ttf`

…and enable the feature: `egui_sauge = { ..., features = ["embedded-fonts"] }`. Both families are SIL OFL 1.1 and redistributable.

## i18n

`egui_sauge` does not ship a full i18n runtime — only the ~6 strings it owns (default `ConfirmDialog` buttons, `StatusDot` labels) are translated, currently `En` (default) and `Fr`. For your application's strings, plug in any i18n crate you like (`fluent`, `rust-i18n`, …). See `GUIDE.md` § Internationalisation.

## Status

Stable. Following semver — breaking changes will land in 2.0.

## License

Apache-2.0
