# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project status

Greenfield crate. `src/lib.rs` currently contains the default `cargo new --lib` template (`add` function + trivial test). `Cargo.toml` has only package metadata — no dependencies yet. The real implementation is described in **`egui_sauge-spec.md`**, which is the authoritative source of truth for the crate's design, API surface, and implementation plan.

**When asked to implement anything, read `egui_sauge-spec.md` first.** The spec is executable top-to-bottom: each milestone (M1–M5) lists the exact files to create, their full contents, acceptance criteria, and verification commands.

## What this crate is

`egui_sauge` is a design system for [egui](https://github.com/emilk/egui): a semantic color palette (sage/stone/clay), typography scale, spacing grid (4pt), radius/elevation tokens, and a one-call theme applier. Target API surface:

```rust
use egui_sauge::{Palette, apply_theme, install_fonts};
install_fonts(ctx);
apply_theme(ctx, &Palette::light()); // or Palette::dark()
```

The public surface (`Palette`, `apply_theme`, `install_fonts`, `SPACING`, `RADIUS`) is stable and versioned — do not change it when adapting to egui API changes.

## Architecture (target, once implemented)

- `src/lib.rs` — re-exports the public API only. No logic.
- `src/theme/palette.rs` — `Palette` struct with ~20 semantic color fields (`bg_app`, `text_primary`, `brand_default`, `focus_ring`, etc.), plus `const fn light()` / `const fn dark()` constructors. Both palettes are designed together (not derived from each other).
- `src/theme/tokens.rs` — numerical tokens: `SPACING` (4/8/12/16/24/32/48/64) and `RADIUS` (sm/md/lg/xl/full).
- `src/theme/apply.rs` — `apply_theme(ctx, palette)` / `apply_theme_with(ctx, palette, density)` write widget visuals + selection + window/popup shadows + spacing (scaled by density) into BOTH stored styles via `ctx.all_styles_mut`, then stash the active palette + density on `ctx.data` via `theme::state::store`. The `all_styles_mut` is load-bearing: egui 0.34 keeps separate light/dark `Style` values, and `set_global_style` only writes to the active one — if you only write to one, switching the system theme drops your text styles and panics with "Failed to find Name(\"h2\") in Style::text_styles". Safe to call every frame.
- `src/theme/state.rs` — ambient storage. `palette_of(ctx)` / `density_of(ctx)` / `locale_of(ctx)` are how components read the active theme without threading state through their signatures. `set_locale(ctx, locale)` flips the DS's own strings (independent of `apply_theme*`).
- `src/theme/locale.rs` — `Locale` enum (En default, Fr available) + private `tr(locale, Key)` for the ~6 strings the DS itself emits (`StatusDot` labels, `ConfirmDialog` confirm/cancel defaults). No i18n runtime dependency — application strings stay in the user's i18n crate of choice.
- `src/theme/density.rs` — `Density::{Comfortable, Compact}`; `.scale()` multiplies spacing tokens, `.interact_size()` sets minimum hit-target height.
- `src/theme/elevation.rs` — `Elevation` enum (Flat / Card / Popover / Modal) with `shadow(dark) -> Shadow`. `Palette.dark_mode` selects the right tuning.
- `src/icons.rs` — `Icon` enum backed by [Phosphor Icons](https://phosphoricons.com/) via `egui-phosphor`. ~80 named variants curated for IT apps. Two escape hatches: `Icon::Glyph(&'static str)` for any other Phosphor codepoint (`egui_phosphor::regular::ROCKET` etc.) and `Icon::Custom(fn(&Painter, Rect, Color32))` for hand-rolled shapes. The Phosphor regular font is registered automatically by `install_fonts`.
- `src/components/` — opinionated component library. Every component reads `palette_of(ctx)` (and `density_of(ctx)` where applicable), so none of them take theme state as parameters. Files:
  - Atoms: `button.rs` (Button + IconButton), `status.rs` (Badge + Tag + StatusDot + Kbd), `feedback.rs` (Spinner + ProgressBar), `switch.rs`.
  - Containers: `card.rs` (Card + EmptyState), `section.rs` (Section + CodeBlock), `stat.rs`, `alert.rs`.
  - Forms: `input.rs` (InputField), `select.rs` (SelectField).
  - Overlays: `dialog.rs` (Dialog + ConfirmDialog), `toast.rs` (Toast + Toasts), `menu.rs` (MenuItem + SubMenu — submenus wrap egui 0.34's `containers::menu::SubMenu` so they nest arbitrarily deep; trigger menus with `egui::Popup::menu(&themed_button_response)`), `tooltip.rs` (themed `tooltip(...)` + `TooltipExt::sauge_tooltip`).
  - Navigation: `nav.rs` (NavItem + Tabs + Breadcrumb), `header.rs` (PageHeader).
  - Data: `data.rs` (KeyValue + LogLine + Skeleton).
- `GUIDE.md` (root) — UX/UI playbook: composition patterns, navigation choice (sidebar / tabs / breadcrumb / drawer / modal), the modal-vs-side panel decision rule, button order (primary right-most by convention), typography hierarchy, feedback patterns, form rules, tables, empty states, a11y checklist, IT-app patterns. Read this before redesigning UIs in this repo.
- `src/text.rs` — `install_fonts` registers Inter (Regular/Medium/SemiBold) + JetBrains Mono via `include_bytes!` (gated on the `embedded-fonts` feature), then installs the 9-entry text style scale (display/h1/h2/h3/body-lg/body/button/small/mono).
- `tests/contrast.rs` — WCAG AA contrast assertions for every (text, background) pair in both palettes. Uses relative luminance + the 4.5:1 threshold.
- `examples/minimal.rs`, `examples/showcase.rs` — eframe apps demonstrating the theme.
- `assets/fonts/` — Inter + JetBrains Mono TTFs (SIL OFL 1.1, redistributable).

### Invariants the architecture enforces

1. **No raw color literals outside `palette.rs`.** `Color32::from_rgb(...)` only appears in `palette.rs`; numeric size/radius literals only in `tokens.rs`. Every other module consumes `palette.brand_default`, `SPACING.s3`, `RADIUS.md`, etc. A component referencing `sage-500` directly is a bug.
2. **Tokens are roles, not hues.** The palette maps semantic roles (e.g. `brand_default`, `text_on_brand`, `focus_ring`) to concrete `Color32`s. Consumers never see hue names.
3. **Dark mode is first-class.** Both palettes are hand-tuned; do not auto-derive dark from light.
4. **WCAG AA is non-negotiable.** `tests/contrast.rs` is the gate — any palette edit must keep both tests green (4.5:1 for normal text, 3:1 for large text).
5. **Focus is always visible as a ring**, never color-only. Disabled state uses 0.45 opacity.

## egui API compatibility

Target: **egui 0.34**, **eframe 0.34**, Rust edition **2024**, MSRV **1.92**. Pinned in `Cargo.toml`.

Key API shape in 0.34 (breaking changes since 0.29 — the spec's snippets use the 0.34 names):

- `CornerRadius` (was `Rounding`); `CornerRadius::same(u8)` — note the `u8`, cast from `f32` tokens at the boundary.
- `Visuals::window_corner_radius` (was `window_rounding`).
- `WidgetVisuals::corner_radius` (was `rounding`).
- `Margin::same(i8)` (was `f32`) — cast with defensive bounds.
- `FontDefinitions::font_data` is `BTreeMap<String, Arc<FontData>>` — wrap in `Arc::new`.
- Widget visuals still expose `{noninteractive, inactive, hovered, active, open}` with `{bg_fill, weak_bg_fill, bg_stroke, fg_stroke, corner_radius}`.
- `Visuals` still exposes `{selection, panel_fill, window_fill, extreme_bg_color, faint_bg_color, window_stroke, override_text_color, hyperlink_color, error_fg_color, warn_fg_color}`.

If a future egui minor bumps these names again, **adapt the implementation but preserve the public API shape** (`Palette`, `apply_theme`, `install_fonts`). Bumping egui = update `Cargo.toml`, the README compat table, MSRV if required, and spec §1 + §14.

## Commands

```bash
cargo check                       # typecheck
cargo build
cargo test                        # runs tests/contrast.rs (the WCAG gate)
cargo test --test contrast        # just the contrast tests
cargo test light_palette_is_wcag_aa -- --nocapture   # one test

cargo run --example minimal       # smallest live demo
cargo run --example showcase      # full palette/typo/spacing/states showcase

cargo clippy -- -D warnings       # must be zero warnings
cargo fmt --check
cargo doc --no-deps               # all pub items must be documented (missing_docs = "warn")
cargo publish --dry-run           # pre-flight crates.io metadata check
```

## Conventions from the spec

- **Edition:** 2024. MSRV: 1.92 (aligned with egui 0.34). Spec and `Cargo.toml` are in sync.
- **Lints:** `missing_docs = "warn"`, `unsafe_code = "forbid"`.
- **License:** dual `MIT OR Apache-2.0`.
- **Commits:** one commit per milestone, prefixed `feat(M1):`, `feat(M2):`, etc. No "wip" on main.
- **No new dependencies** beyond `egui` (runtime) and `eframe` (dev) without explicit justification.
- **Features:** `default = ["embedded-fonts"]`. When `embedded-fonts` is off, the host provides fonts; `install_fonts` still installs the text-style scale.

## Gotchas

- `cargo test` currently passes only because `src/lib.rs` is the default template. Once M2 lands, the contrast tests become the gate — treat a failing contrast test as a palette bug, not a test bug.
- The TTF files in `assets/fonts/` are not in the repo yet. Until they exist, `cargo build --features embedded-fonts` will fail on `include_bytes!`. Either add the fonts, or build with `--no-default-features` during early milestones.
- `Cargo.lock` is currently committed but `.gitignore` (per spec) will exclude it — it's a library, so this is intentional.
