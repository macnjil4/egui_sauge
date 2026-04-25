# egui_sauge — Spec d'implémentation

**Crate :** `egui_sauge` · **Version cible :** `0.1.0`
**Thème :** Modern · Accessible · Frais · Nature
**Audience :** Claude Code (implémentation du crate à partir de ce document)

---

## 0. Comment utiliser ce document

Ce document est **exécutable de haut en bas**. Chaque jalon (`M1` à `M5`) contient :

- la liste exacte des fichiers à créer,
- leur contenu complet (pas de fragments),
- les critères d'acceptation (ce qui doit passer),
- les commandes à lancer pour vérifier.

**Règles générales pour l'implémentation :**

1. Ne pas inventer de valeurs : toutes les couleurs, tailles, rayons viennent des tables de ce document.
2. Ne jamais mettre de `Color32::from_rgb(...)` ou de valeur numérique en dur **hors de `palette.rs` et `tokens.rs`**. Le reste du crate consomme ces constantes.
3. Cible **egui 0.34** (edition 2024, MSRV 1.92). Pinner `egui = "0.34"` dans `Cargo.toml` et **valider l'API réelle contre cette version** avant d'écrire du code `Style`. Depuis 0.29, `Rounding` a été renommé `CornerRadius` et certains champs `rounding` sur les visuals sont devenus `corner_radius` — les snippets de ce document utilisent déjà les noms 0.34. Si un champ a encore bougé, adapter mais **conserver les noms et la structure publique** (`Palette`, `apply_theme`, `install_fonts`).
4. Commits atomiques par jalon. Messages : `feat(M1): ...`, `feat(M2): ...`, etc.

---

## 1. Cible & métadonnées

| Champ          | Valeur                                                                     |
| -------------- | -------------------------------------------------------------------------- |
| Nom crate      | `egui_sauge`                                                               |
| Version        | `0.1.0` (ou `0.1.0-alpha.0` pour un premier `cargo publish` de réservation) |
| Édition Rust   | `2024`                                                                     |
| MSRV           | `1.92` (aligné sur egui 0.34)                                              |
| Licence        | `MIT OR Apache-2.0` (dual standard écosystème Rust)                        |
| egui           | `0.34` — voir `Cargo.toml` ci-dessous, **pinner explicitement**            |
| Description    | `A fresh, natural design system for egui — sage palette, warm neutrals, WCAG AA.` |
| Keywords       | `egui`, `design-system`, `theme`, `ui`, `accessibility`                    |
| Categories     | `gui`, `visualization`                                                     |

---

## 2. Structure du projet

```
egui_sauge/
├── Cargo.toml
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── CHANGELOG.md
├── .gitignore
├── src/
│   ├── lib.rs
│   ├── theme/
│   │   ├── mod.rs
│   │   ├── palette.rs
│   │   ├── tokens.rs
│   │   └── apply.rs
│   └── text.rs
├── assets/
│   └── fonts/
│       ├── Inter-Regular.ttf
│       ├── Inter-Medium.ttf
│       ├── Inter-SemiBold.ttf
│       └── JetBrainsMono-Regular.ttf
├── examples/
│   ├── showcase.rs
│   └── minimal.rs
└── tests/
    └── contrast.rs
```

---

## 3. Principes directeurs

1. **Nature d'abord** — palette inspirée du végétal (sauge, forêt, lin, pierre). Pas de couleurs saturées "SaaS".
2. **Lisibilité non négociable** — tout texte respecte WCAG AA (4.5:1 normal, 3:1 large). Cibles interactives ≥ 32 px par défaut, ≥ 24 px en densité compact.
3. **Calme visuel** — peu d'ombres, peu de bordures dures, beaucoup d'air. La hiérarchie passe par la typo et l'espace.
4. **Tokens → `egui::Style`** — aucune valeur en dur dans les composants applicatifs. Tout passe par `Palette` + tokens.
5. **Dark mode de première classe** — les deux modes sont conçus ensemble, pas l'un dérivé de l'autre.

---

## 4. Tokens de couleur

### 4.1 Palette brute

#### Sauge (brand)

| Token    | Hex       | Usage                                  |
| -------- | --------- | -------------------------------------- |
| sage-50  | `#F1F8F4` | surface teintée très légère            |
| sage-100 | `#D8ECDF` | hover subtil, badges info douce        |
| sage-200 | `#B6D9C2` | bordures actives, séparateurs accent   |
| sage-400 | `#6FA98A` | brand dark mode, icônes                |
| sage-500 | `#4A8B6B` | **brand principal (light)**            |
| sage-600 | `#376B51` | hover brand, texte sur sage-100        |
| sage-700 | `#264E3B` | texte haute emphase sur fond clair     |
| sage-900 | `#14291F` | fond app dark mode                     |

#### Lin & pierre (neutres chauds)

| Token     | Hex       | Usage                          |
| --------- | --------- | ------------------------------ |
| stone-0   | `#FFFFFF` | surface élevée                 |
| stone-50  | `#FAFAF7` | **fond app (light)**           |
| stone-100 | `#F1EFE8` | surface secondaire             |
| stone-200 | `#E3DFD4` | bordures légères               |
| stone-300 | `#CFC9BA` | bordures standard              |
| stone-500 | `#8A8473` | texte tertiaire dark, placeholder |
| stone-700 | `#4A4638` | texte secondaire               |
| stone-900 | `#1C1B16` | texte principal                |

#### Clay (accent chaud, usage parcimonieux)

| Token    | Hex       | Usage                    |
| -------- | --------- | ------------------------ |
| clay-400 | `#D98C6B` | accent, focus alternatif |
| clay-600 | `#A85F3F` | CTA alternatif           |

#### Sémantique

| Rôle    | Light     | Dark      | Contraste texte garanti |
| ------- | --------- | --------- | ----------------------- |
| success | `#3F8A5C` | `#6FB98A` | ≥ 4.5:1                 |
| warning | `#B8822A` | `#E0B056` | ≥ 4.5:1                 |
| error   | `#B24A3E` | `#E58578` | ≥ 4.5:1                 |
| info    | `#3A7A8C` | `#7FB8C7` | ≥ 4.5:1                 |

### 4.2 Tokens sémantiques (API publique)

| Token                   | Light     | Dark      |
| ----------------------- | --------- | --------- |
| `bg_app`                | stone-50  | sage-900  |
| `bg_surface`            | stone-0   | `#1F2E25` |
| `bg_surface_alt`        | stone-100 | `#26362C` |
| `bg_hover`              | stone-100 | `#2E3F35` |
| `bg_pressed`            | stone-200 | `#38493F` |
| `border_subtle`         | stone-200 | `#2E3F35` |
| `border_default`        | stone-300 | `#3E5247` |
| `border_strong`         | stone-500 | sage-400  |
| `text_primary`          | stone-900 | `#ECEAE1` |
| `text_secondary`        | stone-700 | `#C3BFB1` |
| `text_tertiary`         | `#6E685A` | `#9A9485` |
| `text_on_brand`         | stone-0   | sage-900  |
| `brand_default`         | sage-500  | sage-400  |
| `brand_hover`           | sage-600  | `#85BFA1` |
| `brand_pressed`         | sage-700  | `#5A9279` |
| `focus_ring`            | sage-600  | sage-400  |
| `success` / `warning` / `error` / `info` | voir table ci-dessus |

> **Règle :** un composant ne référence jamais `sage-500` directement. Il consomme `palette.brand_default`.

---

## 5. Typographie

### 5.1 Familles

| Rôle | Famille            | Fallback              |
| ---- | ------------------ | --------------------- |
| UI   | **Inter**          | system-ui, sans-serif |
| Mono | **JetBrains Mono** | monospace             |

Une seule famille UI : Inter. Le caractère "frais/nature" passe par la palette et l'espacement, pas par la typo.

### 5.2 Échelle

| Token          | Taille | Line-height | Poids | Usage                    |
| -------------- | ------ | ----------- | ----- | ------------------------ |
| `text.display` | 40 px  | 48 px       | 600   | titre d'écran, hero      |
| `text.h1`      | 28 px  | 34 px       | 600   | section principale       |
| `text.h2`      | 20 px  | 26 px       | 600   | sous-section             |
| `text.h3`      | 16 px  | 22 px       | 600   | titre de carte           |
| `text.body-lg` | 16 px  | 26 px       | 400   | lecture, paragraphes     |
| `text.body`    | 14 px  | 22 px       | 400   | **taille par défaut UI** |
| `text.small`   | 12 px  | 18 px       | 500   | labels, métadonnées      |
| `text.mono`    | 13 px  | 20 px       | 400   | code, identifiants       |

> Minimum texte interactif : 13 px.

---

## 6. Espacement (grille 4 pt)

| Token     | Valeur |
| --------- | ------ |
| `space.1` | 4      |
| `space.2` | 8      |
| `space.3` | 12     |
| `space.4` | 16     |
| `space.5` | 24     |
| `space.6` | 32     |
| `space.7` | 48     |
| `space.8` | 64     |

**Defaults egui :**
- `item_spacing` : `(8, 6)`
- `button_padding` : `(12, 8)`
- `window_margin` : `16`
- `indent` : `16`
- `interact_size.y` : `32.0`

---

## 7. Rayons

| Token         | Valeur | Usage                         |
| ------------- | ------ | ----------------------------- |
| `radius.sm`   | 4      | inputs, petits badges         |
| `radius.md`   | 8      | **boutons, cartes — défaut**  |
| `radius.lg`   | 12     | modales, popovers             |
| `radius.xl`   | 16     | grandes surfaces              |
| `radius.full` | 9999   | avatars, pills, toggles       |

---

## 8. Élévation

| Niveau   | Ombre (offset, spread, color)              | Bordure          | Usage    |
| -------- | ------------------------------------------ | ---------------- | -------- |
| `elev.0` | aucune                                     | `border_subtle`  | plat     |
| `elev.1` | `(0, 1)` spread 2, `rgba(20,25,20,0.06)`   | `border_subtle`  | cartes   |
| `elev.2` | `(0, 4)` spread 12, `rgba(20,25,20,0.10)`  | `border_default` | popovers |
| `elev.3` | `(0, 12)` spread 32, `rgba(20,25,20,0.14)` | `border_default` | modales  |

Dark mode : alpha × 1.6 et couleur `rgba(0,0,0,…)`.

---

## 9. États d'interaction

Tout élément interactif définit **5 états** : `rest`, `hover`, `active/pressed`, `focus`, `disabled`.

| État     | Transformation                                                 |
| -------- | -------------------------------------------------------------- |
| hover    | background → `bg_hover` (ou `brand_hover` pour surfaces brand) |
| active   | background → `bg_pressed` / `brand_pressed`                    |
| focus    | anneau **2 px** `focus_ring`, offset 2 px, jamais supprimé     |
| disabled | opacité **0.45**, pas de hover                                 |

---

## 10. Accessibilité

- Contraste vérifié pour chaque paire texte/fond (AA min, AAA sur `text_primary`).
- Taille cible ≥ 32 px en preset `comfortable`, ≥ 28 px en `compact`.
- Focus visible toujours matérialisé par un anneau, jamais par couleur seule.
- Flag `reduce_motion: bool` dans `Theme` pour désactiver les tweens.
- Préfixer chaque warning/error/success par une icône (couleur jamais seule).

---

## 11. Jalons d'implémentation

### M1 — Squelette & métadonnées

**But :** crate qui compile, licence en place, crates.io-ready.

**Fichiers à créer :**

#### `Cargo.toml`

```toml
[package]
name = "egui_sauge"
version = "0.1.0"
edition = "2024"
rust-version = "1.92"
authors = ["<AUTEUR>"]
description = "A fresh, natural design system for egui — sage palette, warm neutrals, WCAG AA."
repository = "https://github.com/<USER>/egui_sauge"
homepage = "https://github.com/<USER>/egui_sauge"
license = "MIT OR Apache-2.0"
readme = "README.md"
keywords = ["egui", "design-system", "theme", "ui", "accessibility"]
categories = ["gui", "visualization"]
exclude = ["/assets/fonts/*.md", "/.github"]

[dependencies]
# Pinner sur la version cible. Vérifier l'API de Style/Visuals avant implémentation.
egui = "0.34"

[dev-dependencies]
# eframe 0.34 : `wgpu` + `glow` + `accesskit` + `default_fonts` + `wayland` + `x11` + `web_screen_reader`
# sont inclus dans `default`. On reste sur `default-features = true` pour les exemples.
eframe = "0.34"

[features]
default = ["embedded-fonts"]
# Embarque Inter + JetBrains Mono directement dans le binaire via include_bytes!.
# Désactiver si l'hôte fournit ses propres polices.
embedded-fonts = []

[lints.rust]
missing_docs = "warn"
unsafe_code = "forbid"
```

#### `.gitignore`

```
/target
Cargo.lock
.DS_Store
```

#### `LICENSE-MIT`

Texte MIT standard, copyright `<ANNÉE> <AUTEUR>`.

#### `LICENSE-APACHE`

Texte Apache-2.0 standard.

#### `README.md`

````markdown
# egui_sauge

A fresh, natural design system for [egui](https://github.com/emilk/egui) — sage palette, warm neutrals, WCAG AA.

## Quickstart

```rust
use egui_sauge::{Palette, apply_theme, install_fonts};

fn setup(ctx: &egui::Context) {
    install_fonts(ctx);
    apply_theme(ctx, &Palette::light());
}
```

## Features

- Semantic color tokens (light & dark)
- 4pt spacing grid, 5-step radius scale, 4-level elevation
- WCAG AA contrast on every text/background pair
- Inter + JetBrains Mono embedded (optional feature)
- Zero runtime cost beyond a one-time `apply_theme` call

## egui compatibility

| egui_sauge | egui    | rustc (MSRV) |
| ---------- | ------- | ------------ |
| 0.1.x      | 0.34.x  | 1.92         |

## License

MIT OR Apache-2.0.
````

#### `CHANGELOG.md`

```markdown
# Changelog

## [Unreleased]

## [0.1.0] — <DATE>
### Added
- Initial release: `Palette`, `apply_theme`, `install_fonts`.
- Light and dark palettes, WCAG AA validated.
- Example `showcase` reproducing the reference HTML showcase in egui.
```

#### `src/lib.rs`

```rust
//! # egui_sauge
//!
//! A fresh, natural design system for [egui]. Provides a semantic color
//! palette, typography scale, spacing grid and a one-call theme applier.
//!
//! ## Quickstart
//!
//! ```no_run
//! use egui_sauge::{Palette, apply_theme, install_fonts};
//!
//! # fn demo(ctx: &egui::Context) {
//! install_fonts(ctx);
//! apply_theme(ctx, &Palette::light());
//! # }
//! ```
//!
//! [egui]: https://github.com/emilk/egui

#![doc(html_root_url = "https://docs.rs/egui_sauge/0.1.0")]

mod theme;
mod text;

pub use theme::{
    apply::apply_theme,
    palette::Palette,
    tokens::{Radius, Spacing, RADIUS, SPACING},
};
pub use text::install_fonts;
```

**Acceptance M1**

- `cargo check` passe.
- `cargo doc --no-deps` compile sans warning.
- `cargo publish --dry-run` n'émet aucune erreur de métadonnées.

---

### M2 — Palette & tokens

**But :** rendre `Palette::light()` / `Palette::dark()` et les tokens numériques disponibles.

#### `src/theme/mod.rs`

```rust
pub mod apply;
pub mod palette;
pub mod tokens;
```

#### `src/theme/palette.rs`

```rust
//! Semantic color palette. Each field maps a *role* (not a hue) to a
//! concrete [`egui::Color32`]. Components should consume roles, never
//! raw hues.

use egui::Color32;

/// A full set of semantic colors for one theme mode (light or dark).
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    // Backgrounds
    pub bg_app: Color32,
    pub bg_surface: Color32,
    pub bg_surface_alt: Color32,
    pub bg_hover: Color32,
    pub bg_pressed: Color32,

    // Borders
    pub border_subtle: Color32,
    pub border_default: Color32,
    pub border_strong: Color32,

    // Text
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_tertiary: Color32,
    pub text_on_brand: Color32,

    // Brand
    pub brand_default: Color32,
    pub brand_hover: Color32,
    pub brand_pressed: Color32,

    // Focus
    pub focus_ring: Color32,

    // Semantic
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub info: Color32,
}

impl Palette {
    /// The default light palette.
    pub const fn light() -> Self {
        Self {
            bg_app:         Color32::from_rgb(0xFA, 0xFA, 0xF7),
            bg_surface:     Color32::WHITE,
            bg_surface_alt: Color32::from_rgb(0xF1, 0xEF, 0xE8),
            bg_hover:       Color32::from_rgb(0xF1, 0xEF, 0xE8),
            bg_pressed:     Color32::from_rgb(0xE3, 0xDF, 0xD4),

            border_subtle:  Color32::from_rgb(0xE3, 0xDF, 0xD4),
            border_default: Color32::from_rgb(0xCF, 0xC9, 0xBA),
            border_strong:  Color32::from_rgb(0x8A, 0x84, 0x73),

            text_primary:   Color32::from_rgb(0x1C, 0x1B, 0x16),
            text_secondary: Color32::from_rgb(0x4A, 0x46, 0x38),
            text_tertiary:  Color32::from_rgb(0x6E, 0x68, 0x5A),
            text_on_brand:  Color32::WHITE,

            brand_default:  Color32::from_rgb(0x4A, 0x8B, 0x6B),
            brand_hover:    Color32::from_rgb(0x37, 0x6B, 0x51),
            brand_pressed:  Color32::from_rgb(0x26, 0x4E, 0x3B),
            focus_ring:     Color32::from_rgb(0x37, 0x6B, 0x51),

            success:        Color32::from_rgb(0x3F, 0x8A, 0x5C),
            warning:        Color32::from_rgb(0xB8, 0x82, 0x2A),
            error:          Color32::from_rgb(0xB2, 0x4A, 0x3E),
            info:           Color32::from_rgb(0x3A, 0x7A, 0x8C),
        }
    }

    /// The default dark palette.
    pub const fn dark() -> Self {
        Self {
            bg_app:         Color32::from_rgb(0x14, 0x29, 0x1F),
            bg_surface:     Color32::from_rgb(0x1F, 0x2E, 0x25),
            bg_surface_alt: Color32::from_rgb(0x26, 0x36, 0x2C),
            bg_hover:       Color32::from_rgb(0x2E, 0x3F, 0x35),
            bg_pressed:     Color32::from_rgb(0x38, 0x49, 0x3F),

            border_subtle:  Color32::from_rgb(0x2E, 0x3F, 0x35),
            border_default: Color32::from_rgb(0x3E, 0x52, 0x47),
            border_strong:  Color32::from_rgb(0x6F, 0xA9, 0x8A),

            text_primary:   Color32::from_rgb(0xEC, 0xEA, 0xE1),
            text_secondary: Color32::from_rgb(0xC3, 0xBF, 0xB1),
            text_tertiary:  Color32::from_rgb(0x9A, 0x94, 0x85),
            text_on_brand:  Color32::from_rgb(0x14, 0x29, 0x1F),

            brand_default:  Color32::from_rgb(0x6F, 0xA9, 0x8A),
            brand_hover:    Color32::from_rgb(0x85, 0xBF, 0xA1),
            brand_pressed:  Color32::from_rgb(0x5A, 0x92, 0x79),
            focus_ring:     Color32::from_rgb(0x6F, 0xA9, 0x8A),

            success:        Color32::from_rgb(0x6F, 0xB9, 0x8A),
            warning:        Color32::from_rgb(0xE0, 0xB0, 0x56),
            error:          Color32::from_rgb(0xE5, 0x85, 0x78),
            info:           Color32::from_rgb(0x7F, 0xB8, 0xC7),
        }
    }
}

impl Default for Palette {
    fn default() -> Self { Self::light() }
}
```

#### `src/theme/tokens.rs`

```rust
//! Numerical design tokens: spacing grid and border radii.

/// 4-point spacing scale.
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub s1: f32, pub s2: f32, pub s3: f32, pub s4: f32,
    pub s5: f32, pub s6: f32, pub s7: f32, pub s8: f32,
}

/// The canonical spacing scale (4, 8, 12, 16, 24, 32, 48, 64).
pub const SPACING: Spacing = Spacing {
    s1: 4.0,  s2: 8.0,  s3: 12.0, s4: 16.0,
    s5: 24.0, s6: 32.0, s7: 48.0, s8: 64.0,
};

/// Border radius scale.
#[derive(Debug, Clone, Copy)]
pub struct Radius {
    pub sm: f32, pub md: f32, pub lg: f32, pub xl: f32, pub full: f32,
}

/// The canonical radius scale.
pub const RADIUS: Radius = Radius {
    sm: 4.0, md: 8.0, lg: 12.0, xl: 16.0, full: 9999.0,
};
```

**Acceptance M2**

- `cargo check` passe.
- Les 20 champs de `Palette` sont présents et non-mutables (`Copy`).
- Les deux constructeurs `Palette::light()` et `Palette::dark()` sont `const`.
- `SPACING.s1 == 4.0 && RADIUS.md == 8.0`.

---

### M3 — Application du thème

**But :** `apply_theme(&Context, &Palette)` produit un `egui::Style` cohérent.

> **Remarque sur l'API egui 0.34 :** les snippets ci-dessous utilisent les noms actuels.
> Sur 0.34, le type s'appelle `CornerRadius` (et non plus `Rounding`), et les champs
> correspondants sur `WidgetVisuals` / `Visuals` s'appellent `corner_radius` /
> `window_corner_radius`. Vérifier contre la version pinée :
> `Visuals::widgets.{noninteractive,inactive,hovered,active,open}`,
> `WidgetVisuals::{bg_fill, weak_bg_fill, bg_stroke, fg_stroke, corner_radius}`,
> `Visuals::{selection.{bg_fill, stroke}, window_corner_radius, window_stroke,
> panel_fill, window_fill, extreme_bg_color, faint_bg_color, override_text_color,
> hyperlink_color, error_fg_color, warn_fg_color}`, `Style::{visuals, spacing}`.
> Si un champ a été renommé à nouveau, adapter sans changer l'API publique.

#### `src/theme/apply.rs`

```rust
//! Applies a [`Palette`] to an [`egui::Context`] as a complete [`egui::Style`].

use egui::{CornerRadius, Stroke, Style};

use super::palette::Palette;
use super::tokens::{RADIUS, SPACING};

/// Push a theme based on `palette` onto `ctx`. Safe to call on every frame if
/// the palette may change (e.g. light/dark toggle); otherwise call once at
/// startup.
pub fn apply_theme(ctx: &egui::Context, palette: &Palette) {
    let mut style: Style = (*ctx.style()).clone();

    apply_visuals(&mut style, palette);
    apply_spacing(&mut style);

    ctx.set_style(style);
}

fn apply_visuals(style: &mut Style, p: &Palette) {
    let v = &mut style.visuals;
    // egui 0.34: `CornerRadius::same` takes a u8. Tokens are f32 in our
    // scale — cast at the boundary and clamp defensively (radii are small).
    let r = |px: f32| CornerRadius::same(px.round().clamp(0.0, 255.0) as u8);
    let r_md = r(RADIUS.md);

    // Surfaces
    v.window_fill         = p.bg_surface;
    v.panel_fill          = p.bg_app;
    v.extreme_bg_color    = p.bg_surface;
    v.faint_bg_color      = p.bg_hover;
    v.window_stroke       = Stroke::new(1.0, p.border_default);
    v.window_corner_radius = r(RADIUS.lg);

    // Default text color
    v.override_text_color = Some(p.text_primary);

    // Non-interactive widgets (labels, separators)
    v.widgets.noninteractive.bg_fill      = p.bg_surface;
    v.widgets.noninteractive.weak_bg_fill = p.bg_surface;
    v.widgets.noninteractive.bg_stroke    = Stroke::new(1.0, p.border_subtle);
    v.widgets.noninteractive.fg_stroke    = Stroke::new(1.0, p.text_secondary);
    v.widgets.noninteractive.corner_radius = r_md;

    // Inactive (rest) interactive widgets
    v.widgets.inactive.bg_fill      = p.bg_surface;
    v.widgets.inactive.weak_bg_fill = p.bg_surface;
    v.widgets.inactive.bg_stroke    = Stroke::new(1.0, p.border_default);
    v.widgets.inactive.fg_stroke    = Stroke::new(1.0, p.text_primary);
    v.widgets.inactive.corner_radius = r_md;

    // Hovered
    v.widgets.hovered.bg_fill      = p.bg_hover;
    v.widgets.hovered.weak_bg_fill = p.bg_hover;
    v.widgets.hovered.bg_stroke    = Stroke::new(1.0, p.brand_default);
    v.widgets.hovered.fg_stroke    = Stroke::new(1.0, p.text_primary);
    v.widgets.hovered.corner_radius = r_md;

    // Active (pressed)
    v.widgets.active.bg_fill       = p.bg_pressed;
    v.widgets.active.weak_bg_fill  = p.bg_pressed;
    v.widgets.active.bg_stroke     = Stroke::new(1.5, p.brand_hover);
    v.widgets.active.fg_stroke     = Stroke::new(1.0, p.text_primary);
    v.widgets.active.corner_radius = r_md;

    // Open (e.g. open combobox)
    v.widgets.open.bg_fill         = p.bg_surface_alt;
    v.widgets.open.weak_bg_fill    = p.bg_surface_alt;
    v.widgets.open.bg_stroke       = Stroke::new(1.0, p.brand_default);
    v.widgets.open.fg_stroke       = Stroke::new(1.0, p.text_primary);
    v.widgets.open.corner_radius   = r_md;

    // Selection + focus
    v.selection.bg_fill = multiply_alpha(p.brand_default, 0.25);
    v.selection.stroke  = Stroke::new(2.0, p.focus_ring);

    // Hyperlink
    v.hyperlink_color = p.brand_hover;

    // Error/warning tints used by egui internals
    v.error_fg_color = p.error;
    v.warn_fg_color  = p.warning;
}

fn apply_spacing(style: &mut Style) {
    let s = &mut style.spacing;
    s.item_spacing    = egui::vec2(SPACING.s2, 6.0);
    s.button_padding  = egui::vec2(SPACING.s3, SPACING.s2);
    // egui 0.34: `Margin::same` takes an i8.
    s.window_margin   = egui::Margin::same(SPACING.s4 as i8);
    s.indent          = SPACING.s4;
    s.interact_size.y = 32.0;
    s.icon_width      = 16.0;
    s.icon_spacing    = SPACING.s2;
}

fn multiply_alpha(c: egui::Color32, factor: f32) -> egui::Color32 {
    let [r, g, b, a] = c.to_array();
    let a = (a as f32 * factor).round().clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}
```

**Acceptance M3**

- `cargo check` passe.
- Un exemple minimal (voir M4) compile et s'ouvre sans panique.
- Visuellement : bouton au repos/hover/pressé change d'état comme spécifié au §9.

---

### M4 — Polices & exemples

**But :** installer Inter + JetBrains Mono, fournir un exemple `showcase` complet et un `minimal`.

> **Assets :** placer les fichiers TTF dans `assets/fonts/`. Les polices **Inter** (SIL OFL 1.1) et **JetBrains Mono** (SIL OFL 1.1) sont redistribuables ; ajouter les `.ttf` (ou `.otf`) requis. Si l'utilisateur du crate préfère fournir ses propres polices, il désactive `features = ["embedded-fonts"]`.

#### `src/text.rs`

```rust
//! Font installation and text style registration.

use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};

/// Register Inter + JetBrains Mono and set the canonical text style scale
/// (display/h1/h2/h3/body/body-lg/small/mono).
pub fn install_fonts(ctx: &egui::Context) {
    #[cfg(feature = "embedded-fonts")]
    install_embedded_fonts(ctx);

    install_text_styles(ctx);
}

#[cfg(feature = "embedded-fonts")]
fn install_embedded_fonts(ctx: &egui::Context) {
    use std::sync::Arc;

    let mut fonts = FontDefinitions::default();

    // egui 0.34: `font_data` is `BTreeMap<String, Arc<FontData>>`.
    fonts.font_data.insert(
        "Inter-Regular".into(),
        Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Inter-Regular.ttf"))),
    );
    fonts.font_data.insert(
        "Inter-Medium".into(),
        Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Inter-Medium.ttf"))),
    );
    fonts.font_data.insert(
        "Inter-SemiBold".into(),
        Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Inter-SemiBold.ttf"))),
    );
    fonts.font_data.insert(
        "JetBrainsMono".into(),
        Arc::new(FontData::from_static(include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf"))),
    );

    // Default proportional stack: Inter first, then egui's fallbacks (which
    // stay in the list for coverage on non-Latin scripts).
    let prop = fonts.families.entry(FontFamily::Proportional).or_default();
    prop.insert(0, "Inter-Regular".into());

    let mono = fonts.families.entry(FontFamily::Monospace).or_default();
    mono.insert(0, "JetBrainsMono".into());

    ctx.set_fonts(fonts);
}

fn install_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    use TextStyle as TS;
    style.text_styles = [
        (TS::Name("display".into()), FontId::new(40.0, FontFamily::Proportional)),
        (TS::Heading,                FontId::new(28.0, FontFamily::Proportional)),
        (TS::Name("h2".into()),      FontId::new(20.0, FontFamily::Proportional)),
        (TS::Name("h3".into()),      FontId::new(16.0, FontFamily::Proportional)),
        (TS::Name("body-lg".into()), FontId::new(16.0, FontFamily::Proportional)),
        (TS::Body,                   FontId::new(14.0, FontFamily::Proportional)),
        (TS::Button,                 FontId::new(14.0, FontFamily::Proportional)),
        (TS::Small,                  FontId::new(12.0, FontFamily::Proportional)),
        (TS::Monospace,              FontId::new(13.0, FontFamily::Monospace)),
    ]
    .into();

    ctx.set_style(style);
}
```

#### `examples/minimal.rs`

```rust
//! Minimal app showing the theme applied to a handful of widgets.
//!
//! Run with: `cargo run --example minimal`

use eframe::egui;
use egui_sauge::{apply_theme, install_fonts, Palette};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_sauge — minimal",
        options,
        Box::new(|cc| {
            install_fonts(&cc.egui_ctx);
            apply_theme(&cc.egui_ctx, &Palette::light());
            // egui 0.34: the creator returns a boxed `dyn App`.
            Ok(Box::new(App::default()) as Box<dyn eframe::App>)
        }),
    )
}

#[derive(Default)]
struct App {
    name: String,
    dark: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui_sauge");
            ui.label("A fresh, natural design system for egui.");
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add_space(8.0);
            if ui.checkbox(&mut self.dark, "Dark mode").changed() {
                let p = if self.dark { Palette::dark() } else { Palette::light() };
                apply_theme(ctx, &p);
            }

            ui.add_space(16.0);
            ui.horizontal(|ui| {
                let _ = ui.button("Primary");
                let _ = ui.button("Secondary");
            });
        });
    }
}
```

#### `examples/showcase.rs`

**But :** reproduire les sept sections du showcase HTML (palette, typo, espacement, rayons, élévation, états, composants) en egui natif. Un seul écran, scrollable, avec toggle light/dark en topbar.

Structure attendue (à implémenter par Claude Code en s'appuyant sur l'API connue d'egui) :

- `egui::TopBottomPanel::top` → nom du crate + toggle clair/sombre.
- `egui::CentralPanel` avec `ScrollArea::vertical` contenant sept `ui.collapsing` ou `ui.group` :
  1. **Palette** — pour chaque couleur de `Palette`, `ui.allocate_response` avec un rect coloré + label nom + hex.
  2. **Typographie** — `ui.label(RichText::new("…").text_style(TextStyle::Name("display".into())))` pour chaque token de l'échelle.
  3. **Espacement** — une ligne par token avec une barre horizontale de la largeur correspondante (`ui.allocate_exact_size`).
  4. **Rayons** — pour chaque valeur, un carré `Painter::rect_filled(..., CornerRadius::same(r as u8), brand_default)`.
  5. **Élévation** — quatre "cartes" dessinées avec ombres simulées (voir plus bas).
  6. **États** — cinq boutons figés dans chaque état (possible via `Response::hovered/pressed` simulés par un `Widget` custom ou simplement en affichant des vues statiques peintes à la main).
  7. **Composants** — un `TextEdit`, trois boutons (primary/secondary/ghost), badges sémantiques peints en pills, une carte regroupant titre/texte/actions.

**Acceptance M4**

- `cargo run --example minimal` ouvre une fenêtre, les polices Inter/Mono sont visibles, le toggle fonctionne.
- `cargo run --example showcase` ouvre la fenêtre, les sept sections s'affichent sans panique, le toggle clair/sombre rafraîchit toutes les sections.
- Aucun `unwrap()` dans les exemples hors des points légitimes (`eframe::run_native` retour).

---

### M5 — Tests & qualité

**But :** lock-in de la stabilité de la palette + vérification automatique du contraste.

#### `tests/contrast.rs`

```rust
//! Validates WCAG AA contrast on every (text, background) pair used by the
//! palette. AA = 4.5:1 for normal text, 3:1 for large text (18pt / 14pt bold).

use egui_sauge::Palette;

fn luminance(c: egui::Color32) -> f64 {
    fn ch(v: u8) -> f64 {
        let v = v as f64 / 255.0;
        if v <= 0.03928 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
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
fn assert_aa(label: &str, fg: egui::Color32, bg: egui::Color32, min: f64) {
    let r = ratio(fg, bg);
    assert!(r >= min, "{label}: contrast {r:.2} below required {min:.2}");
}

fn check(p: &Palette, mode: &str) {
    // Normal text against every background it may appear on.
    assert_aa(&format!("{mode}: primary on bg_app"),       p.text_primary,   p.bg_app,         4.5);
    assert_aa(&format!("{mode}: primary on bg_surface"),   p.text_primary,   p.bg_surface,     4.5);
    assert_aa(&format!("{mode}: primary on bg_alt"),       p.text_primary,   p.bg_surface_alt, 4.5);
    assert_aa(&format!("{mode}: secondary on bg_app"),     p.text_secondary, p.bg_app,         4.5);
    assert_aa(&format!("{mode}: secondary on bg_surface"), p.text_secondary, p.bg_surface,     4.5);
    assert_aa(&format!("{mode}: tertiary on bg_app"),      p.text_tertiary,  p.bg_app,         4.5);
    assert_aa(&format!("{mode}: on_brand on brand"),       p.text_on_brand,  p.brand_default,  4.5);

    // Semantic against app background (3:1 is acceptable for non-text UI
    // but we aim higher for text — 4.5 when used as text).
    assert_aa(&format!("{mode}: success on bg_app"), p.success, p.bg_app, 4.5);
    assert_aa(&format!("{mode}: warning on bg_app"), p.warning, p.bg_app, 4.5);
    assert_aa(&format!("{mode}: error on bg_app"),   p.error,   p.bg_app, 4.5);
    assert_aa(&format!("{mode}: info on bg_app"),    p.info,    p.bg_app, 4.5);
}

#[test]
fn light_palette_is_wcag_aa() {
    check(&Palette::light(), "light");
}

#[test]
fn dark_palette_is_wcag_aa() {
    check(&Palette::dark(), "dark");
}
```

**Acceptance M5**

- `cargo test` : tous les tests passent.
- `cargo clippy -- -D warnings` : zéro warning.
- `cargo fmt --check` passe.
- `cargo doc --no-deps` sans warning.
- `cargo publish --dry-run` OK.

---

## 12. Checklist de livraison v0.1.0

- [ ] `Cargo.toml` : licence dual, description, keywords, categories, repository.
- [ ] `README.md` avec quickstart, table de compat egui, licence.
- [ ] `LICENSE-MIT` et `LICENSE-APACHE` présents.
- [ ] `CHANGELOG.md` avec l'entrée `0.1.0`.
- [ ] `Palette::light()` et `Palette::dark()` exposées et `const`.
- [ ] `apply_theme` et `install_fonts` exposés à la racine du crate.
- [ ] Tests de contraste verts pour les deux palettes.
- [ ] `cargo clippy -- -D warnings` propre.
- [ ] `cargo fmt --check` propre.
- [ ] `cargo doc --no-deps` sans warning (toutes les `pub` items documentées).
- [ ] `cargo publish --dry-run` OK.
- [ ] Tag git `v0.1.0` créé après le `cargo publish`.

---

## 13. Hors périmètre v0.1

- Composants haut-niveau (Button, Input, Select, Toast, Dialog) — v0.2.
- Layouts (app shell, side nav, tabs) — v0.2.
- Set d'icônes complet — v0.3.
- Palette catégorielle / séquentielle pour data-viz — v0.3.
- Densité `compact` paramétrable — v0.2.

---

## 14. Notes pour Claude Code

- **Cible moderne (2026-Q2) :** Rust edition **2024**, MSRV **1.92**, toolchain testé avec `rustc 1.95` ; `egui = "0.34"`, `eframe = "0.34"`. Ne pas descendre sous ces versions.
- **Breaking changes notables depuis egui 0.29 → 0.34** à connaître avant de coder :
  - `Rounding` → `CornerRadius` (`u8` par composante, plus de `f32`).
  - `Visuals::window_rounding` → `window_corner_radius`.
  - `WidgetVisuals::rounding` → `corner_radius`.
  - `Margin::same(i8)` — plus de `f32`, attention aux casts.
  - `FontDefinitions::font_data` stocke désormais `Arc<FontData>` (plus `FontData` nu).
  - Architecture "More `Ui`, less `Context`" (0.34) : privilégier les méthodes sur `Ui` aux accès via `Context` quand elles existent.
- **En cas de mismatch avec l'API egui** : préférer adapter les noms de types/méthodes plutôt que changer la structure publique. La surface `Palette` / `apply_theme` / `install_fonts` est stable et versionnée.
- **Ne jamais ajouter de dépendance** non listée dans `Cargo.toml` sans justification explicite.
- **Polices manquantes** : si les TTF ne sont pas disponibles localement, désactiver `default-features` et documenter dans le README comment l'utilisateur fournit ses polices. Proposer un lien de téléchargement dans `assets/fonts/README.md`.
- **Commits** : un commit par jalon M1 → M5. Pas de "wip" dans `main`.
- **Avant tout `cargo publish`** : lancer `cargo publish --dry-run`, puis `cargo publish` avec version alpha si c'est le premier upload pour réserver le nom.
- **Bumps futurs** : quand egui publie une nouvelle minor, mettre à jour `Cargo.toml`, la table de compat dans le README, la MSRV si elle bouge, et ce document (section 1, section 14, snippets M3/M4 si l'API a encore dérivé). Incrémenter `egui_sauge` d'une minor en miroir.
