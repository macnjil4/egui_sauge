# Changelog

## [Unreleased]

## [1.2.0] — 2026-04-26

IT-app components release. No breaking changes.

### Added

#### Forms
- `Checkbox` — themed two/three-state input with `indeterminate`,
  `error`, `disabled`, optional label.
- `RadioGroup<T>` — typed picker, vertical/horizontal layout, per-option
  helper text, group-level error, `RadioOption::disabled`.
- `NumberField` — numeric input with `+`/`−` stepper, `min`/`max`
  clamping, `step`, `decimals`, `suffix` (units like `%`, `MB`), helper
  / error states.

#### Data
- `Avatar` — circular badge from initials (deterministic brand-tinted
  background), an `Icon`, or an `egui::TextureId`. Sizes Xs-Xl, status
  dot, themed tooltip.
- `AvatarGroup` — overlapping stack with `+N` overflow chip.
- `Table<T>` — typed columns with custom cell renderers, sortable
  headers (caller re-orders the data — the component just signals via
  `&mut Option<SortState>`), multi-row selection (`&mut HashSet<usize>`),
  zebra stripes, density-aware row height, per-column alignment and
  fixed widths.
- `Pagination` — page navigation paired with `Table`. Page-size
  selector, prev/next, "Showing N–M of T" status, customisable size
  choices.

#### Layout
- `Drawer` — non-blocking right-side panel; companion to `Dialog`.
  Underlying surface stays interactive. Returns `true` when the user
  clicks the close ×.
- `Accordion` — themed wrapper around `egui::CollapsingHeader` with an
  optional icon, subtitle, and default-open flag.

### Changed
- `StatusLevel` now derives `PartialOrd` / `Ord` so it sorts
  predictably in tables.
- `Icon` exposes a private `pub(crate) glyph()` returning
  `Option<&'static str>` (used by `Accordion` to embed icons in a
  `LayoutJob`).
- `GUIDE.md` § Tables rewritten now that `Table<T>` exists.
- `examples/showcase.rs` gets a "v1.2 — Forms, Data, Layout" section
  demonstrating every new component.

## [1.1.0] — 2026-04-26

Polish & infrastructure release. No breaking changes.

### Added
- `Density::Spacious` (1.25× scale, 40 px interact size) — touch-first /
  accessibility large-target mode.
- `Palette::custom(base, |p| { … })` — build a brand-customised palette
  from `light()` or `dark()` without forking. WCAG AA is no longer
  guaranteed once you override roles; verify yourself.
- `Locale::De` (German) and `Locale::Es` (Spanish) translations.
- `set_reduce_motion(ctx, bool)` / `reduce_motion(ctx)` — when true,
  `Spinner` and `Skeleton` freeze their animations. Use to mirror the
  OS-level "Reduce Motion" accessibility preference.
- Cargo features `icons-bold`, `icons-fill`, `icons-light`, `icons-thin`
  — turn on the matching `egui-phosphor` weight; consume via
  `Icon::Glyph(egui_phosphor::bold::ROCKET)` after calling
  `install_phosphor_variant(ctx, Variant::Bold)`.
- `install_phosphor_variant(ctx, variant)` helper.
- `[package.metadata.docs.rs]` so docs.rs builds with `--all-features`
  and exposes the optional weight features as cfg flags.
- `documentation = "https://docs.rs/egui_sauge"` in `Cargo.toml`.
- README badges (CI, crates.io, docs.rs, license, MSRV).
- `CONTRIBUTING.md` — PR workflow, commit conventions, component / locale
  / icon addition recipes, release procedure.
- CI matrix: Linux + macOS + Windows × stable + MSRV (1.92), plus a
  separate `docs.rs build (strict)` job that fails on broken
  intra-doc links.

### Changed
- `examples/showcase.rs` topbar exposes the three densities, four
  locales, and the reduce-motion toggle.

## [1.0.0] — 2026-04-25

First stable release. Public API frozen for the 1.x line. Breaking changes
require 2.0.

### Removed (vs. pre-release drafts)
- The `embedded-fonts` Cargo feature. It referenced TTFs we never shipped
  (Inter / JetBrains Mono are not in the repo for size reasons), so the
  feature was broken under `--all-features` (also broke `docs.rs`). The
  README now shows the 5-line pattern to embed any UI typeface at the
  application level.

### Added
- Initial implementation: `Palette`, `apply_theme`, `install_fonts`.
- Light and dark palettes, WCAG AA validated by `tests/contrast.rs`.
- `SPACING` and `RADIUS` token tables.
- `Elevation` enum (Flat / Card / Popover / Modal) with per-mode `Shadow`;
  `apply_theme` wires `window_shadow` = Modal and `popup_shadow` = Popover.
- `Icon` enum backed by [Phosphor Icons](https://phosphoricons.com/) via the
  `egui-phosphor` crate. ~80 named variants curated for IT apps (status,
  navigation, infrastructure: server/database/cpu/cloud/network/lightning/
  package/rocket, files & code, git, security, comms, people, time, misc).
  Two escape hatches: `Icon::Glyph(&'static str)` for any other Phosphor
  codepoint, and `Icon::Custom(fn)` for hand-rolled painters.
- `install_fonts` now also registers the Phosphor regular font on `ctx`.
- `Density` preset (Comfortable / Compact) + `apply_theme_with(ctx, palette, density)`.
- `Locale` (En default, Fr) + `set_locale(ctx, locale)` / `locale_of(ctx)`. Translates the strings the DS itself owns (`StatusDot` default labels, `ConfirmDialog` default buttons). No runtime i18n dependency — application-level strings are out of scope; bring your own crate (fluent / rust-i18n / …) for those.
- Ambient theme state on `Context`: `palette_of(ctx)` / `density_of(ctx)` / `locale_of(ctx)`.
- `Palette.dark_mode` flag so callers and shadow helpers can branch cleanly.
- **Component library** under `egui_sauge::components`:
  - Atoms: `Button` (4 variants × 3 sizes, leading/trailing icons, full-width,
    disabled, focus ring), `IconButton` (with tooltip), `Switch`, `Badge`
    (6 tones × solid/soft), `Tag` (closable), `StatusDot` (online/degraded/
    offline/idle + pulse), `Kbd`, `Spinner`, `ProgressBar`.
  - Containers: `Card` (title/subtitle/elevation), `Section`, `EmptyState`,
    `Stat` (with `Trend::Up/Down/Flat` delta), `CodeBlock` (header + scroll).
  - Feedback: `Alert` (inline, dismissible), `Toasts` / `Toast` (top-right
    stack with auto-dismiss).
  - Forms: `InputField` (label / placeholder / helper / error / password /
    leading+trailing icons / focus ring), `SelectField` (themed ComboBox
    wrapper with label/helper/error).
  - Overlays: `Dialog` (modal with scrim, close icon, title/body/actions,
    `DialogControl::close()` for action-button-driven closure),
    `ConfirmDialog` (turn-key confirm/cancel wrapper, danger variant).
  - Navigation: `NavItem` (sidebar row with icon/badge/selected accent),
    `Tabs<T>` (typed tab bar with underline + optional icons),
    `Breadcrumb` (clickable path, last segment static),
    `PageHeader` (title + subtitle + breadcrumb + right-aligned actions).
  - Data: `KeyValue` (definition list), `LogLine` (timestamp + colored level
    + monospace message), `Skeleton` (animated loading placeholder: line,
    block, circle).
  - Menu: `MenuItem` for use inside `egui::menu::menu_button` (or
    `egui::Popup::menu(&trigger)` with a themed `Button`) — icon / label /
    shortcut hint / danger / disabled.
  - `SubMenu` — nested submenu trigger that wraps egui 0.34's
    `containers::menu::SubMenu`. Renders like a `MenuItem` with a trailing
    chevron, opens a flyout on hover. Submenus nest arbitrarily deep.
  - Tooltip: `tooltip(resp, text)` and `TooltipExt::sauge_tooltip(text)`
    extension trait — themed hover tooltip with Popover shadow.
- `GUIDE.md` — UX/UI playbook covering page composition, navigation
  patterns (sidebar/tabs/breadcrumb/topnav/drawer/modal), the modal-vs-side
  panel decision, button order convention (primary right-most), typography
  hierarchy, feedback (alert/toast/banner), forms, tables, empty states,
  accessibility, and IT-app patterns (health dashboard, resource list,
  pipeline, destructive confirmation).
- Examples `minimal` and `showcase`; showcase exercises every component,
  both density presets, both color modes, icon extensibility (`★` via
  `Icon::Custom`), toast levels, and a destructive-action dialog.

### Notes
- Three light-mode palette values were darkened from the spec in §4 so every
  text/background pair clears the AA 4.5:1 bar. Dark-mode values are unchanged.
  - `brand_default`: `#4A8B6B` → `#3F7A5D` (white-on-brand was 4.04:1).
  - `brand_hover`, `brand_pressed`, `focus_ring` shifted in step to preserve
    the tonal progression.
  - `success`: `#3F8A5C` → `#2E7048` (was 4.02:1 on `bg_app`).
  - `warning`: `#B8822A` → `#8A6317` (was 3.21:1 on `bg_app`).
- `embedded-fonts` is not a default feature in the initial commit — the TTFs
  live outside the repo. Spec default can be restored once fonts are added.
- Target toolchain: egui 0.34, eframe 0.34, Rust edition 2024, MSRV 1.92.
