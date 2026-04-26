# Contributing to egui_sauge

Thanks for taking the time. This guide covers PR workflow, commit
conventions, how to add a component, and how to run the test gates.

If you only want to consume the library, you don't need this file —
read `README.md` and `GUIDE.md` instead.

---

## Setup

```bash
git clone https://github.com/macnjil4/egui_sauge.git
cd egui_sauge
cargo build
cargo run --example showcase    # smoke check
```

Toolchain: Rust **1.92+** (MSRV), edition **2024**.

---

## Test gates (must pass before pushing)

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --no-deps --all-features
```

CI runs these on Linux / macOS / Windows × MSRV / stable. PRs that
break any gate won't merge.

The contrast test (`tests/contrast.rs`) validates **WCAG AA** on every
text/background pair in both palettes. If you touch `Palette::light()`
or `Palette::dark()`, this test will catch contrast regressions —
fix the colors, don't loosen the test.

---

## Commit conventions

Follow Conventional Commits, scoped by the area touched:

```
feat(components): add Pagination
fix(theme): apply density scale to icon_spacing
docs(GUIDE): update modal vs side panel section
chore(ci): bump actions/checkout to v5
```

Common scopes: `theme`, `components`, `icons`, `text`, `ci`, `docs`,
`showcase`.

Always create new commits — never amend pushed commits. Keep a single
logical change per commit; large component additions can split into
multiple commits (skeleton, tests, showcase).

---

## Adding a new component

The smallest reference implementation is `src/components/switch.rs`
(106 lines). Use it as a template.

1. **Create the file** under `src/components/<name>.rs`.
2. **Builder pattern**: a struct, `new(...)` constructor, chained
   setters returning `Self`. Required args go in `new`, optional in
   setters.
3. **Render path**: either `impl Widget for X<'_>` (atoms — `Button`,
   `Badge`, `Switch`) or a `pub fn show(self, ui: &mut Ui, …)` method
   (containers — `Card`, `Dialog`, `Section`).
4. **Read the active theme** via `palette_of(ctx)`,
   `density_of(ctx)`, `locale_of(ctx)`, `reduce_motion(ctx)`. Never
   thread these as parameters.
5. **Helpers**: use `corner(px)` and `alpha(color, factor)` from
   `src/components/mod.rs`.
6. **Re-export** in `src/components/mod.rs`.
7. **Document** every `pub` item with rustdoc (`missing_docs = "warn"`
   in `Cargo.toml` enforces this).
8. **Demo** in `examples/showcase.rs` — add to an existing section or
   create a new `Card` block.
9. **Update**:
   - `CHANGELOG.md` under `[Unreleased]`.
   - `README.md` component table.
   - `GUIDE.md` if the new component changes a UX recommendation.

Open the PR with a screenshot of the showcase block (light + dark).

---

## Adding a locale

Tiny addition (~15 lines):

1. In `src/theme/locale.rs`, add the variant to `enum Locale`.
2. Extend `Locale::from_lang_code` with the BCP-47 prefix.
3. Add the translations in `tr()` for every `Key`.
4. Add the entry to the showcase's `ComboBox` for `locale`.
5. Mention it in `CHANGELOG.md`.

---

## Adding an icon (built-in)

`Icon` is curated for IT apps; we don't add every Phosphor glyph by
default. Check first: can `Icon::Glyph(egui_phosphor::regular::FOO)`
solve your case? If it can, use that.

If the icon is broadly useful, add it to `Icon` enum:

1. In `src/icons.rs`, add the variant with a one-line doc comment.
2. Add the codepoint mapping in `Icon::codepoint()`.
3. Demo in the showcase's Icons section.

For non-regular weights (`bold`, `fill`, etc.), enable the
corresponding Cargo feature (`icons-bold`, `icons-fill`, …) and use
`Icon::Glyph(egui_phosphor::bold::FOO)` — see the rustdoc on
`install_phosphor_variant`.

---

## Cargo.lock

We commit `Cargo.lock`. Standard Rust guidance says libraries don't,
but `egui_sauge` ships an example (`showcase`) that's effectively a
binary, and the lock file makes CI builds reproducible. Don't delete
it without discussion.

---

## Releases

Maintainers only. Steps:

```bash
# Update Cargo.toml version + html_root_url in src/lib.rs
# Update CHANGELOG.md (move [Unreleased] entries into [<version>] - <date>)
# Update README compat table if the egui pin moved
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo publish --dry-run                # final gate
git commit -am "release vX.Y.Z"
git tag -a vX.Y.Z -m "egui_sauge X.Y.Z"
git push origin main vX.Y.Z
cargo publish
```

GitHub Releases page: copy the matching `CHANGELOG.md` entry as the
release body.

---

## Questions

Open an issue. PRs welcome.
