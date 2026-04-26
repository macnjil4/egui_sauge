//! [`CommandPalette`] — modal ⌘K-style action picker. Fuzzy-substring
//! search, optional grouping, keyboard navigation, themed shadow.
//!
//! ```ignore
//! let mut palette_open = false;
//! // Open on ⌘K / Ctrl+K
//! if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
//!     palette_open = true;
//! }
//! let chosen = CommandPalette::new()
//!     .action(CommandAction::new("New project").icon(Icon::Plus).group("Files"))
//!     .action(CommandAction::new("Open settings").icon(Icon::Settings).group("App"))
//!     .show(ctx, &mut palette_open);
//! match chosen {
//!     Some(0) => { /* new project */ }
//!     Some(1) => { /* open settings */ }
//!     _ => {}
//! }
//! ```

use egui::{
    Color32, FontId, Id, Key, Rect, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, vec2,
};

use super::{alpha, corner};
use crate::{Elevation, Icon, RADIUS, SPACING, palette_of};

/// One entry in a [`CommandPalette`].
pub struct CommandAction<'a> {
    label: String,
    group: Option<&'a str>,
    icon: Option<Icon>,
    shortcut: Option<&'a str>,
    keywords: Vec<&'a str>,
}

impl<'a> CommandAction<'a> {
    /// New action with a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            group: None,
            icon: None,
            shortcut: None,
            keywords: Vec::new(),
        }
    }
    /// Section header to group actions under (e.g. "Files", "Navigation").
    pub fn group(mut self, group: &'a str) -> Self {
        self.group = Some(group);
        self
    }
    /// Leading icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Right-aligned keyboard shortcut hint (e.g. `"⌘N"`).
    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }
    /// Extra search keywords (synonyms, French/English mixes, etc.) that
    /// don't appear in the label but should still match the query.
    pub fn keywords(mut self, keywords: impl IntoIterator<Item = &'a str>) -> Self {
        self.keywords = keywords.into_iter().collect();
        self
    }
}

#[derive(Clone, Default)]
struct PaletteState {
    query: String,
    selected: usize,
    /// Whether the search input has been focused at least once. Reset
    /// every time the palette is reopened.
    focused: bool,
}

/// Modal ⌘K-style action picker.
pub struct CommandPalette<'a> {
    actions: Vec<CommandAction<'a>>,
    placeholder: &'a str,
    width: f32,
    max_height: f32,
}

impl<'a> Default for CommandPalette<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CommandPalette<'a> {
    /// Empty palette.
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            placeholder: "Type a command…",
            width: 540.0,
            max_height: 380.0,
        }
    }
    /// Append one action.
    pub fn action(mut self, action: CommandAction<'a>) -> Self {
        self.actions.push(action);
        self
    }
    /// Search-input placeholder text.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }
    /// Override the palette width (default 540 px).
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    /// Override the results-list max height (default 380 px).
    pub fn max_height(mut self, h: f32) -> Self {
        self.max_height = h;
        self
    }

    /// Render the palette while `open` is `true`. Returns `Some(index)`
    /// of the activated action (the original action index, not the
    /// filtered one), or `None` when nothing was activated this frame.
    /// Sets `*open = false` on selection or on Escape.
    pub fn show(self, ctx: &egui::Context, open: &mut bool) -> Option<usize> {
        if !*open {
            // Reset state so reopening starts clean.
            ctx.data_mut(|d| d.remove::<PaletteState>(state_id()));
            return None;
        }

        let palette = palette_of(ctx);
        let mut state: PaletteState = ctx.data(|d| d.get_temp(state_id())).unwrap_or_default();

        // Filter the actions by the current query (case-insensitive
        // substring on label OR keywords).
        let q = state.query.trim().to_ascii_lowercase();
        let filtered: Vec<usize> = self
            .actions
            .iter()
            .enumerate()
            .filter(|(_, a)| {
                if q.is_empty() {
                    true
                } else {
                    a.label.to_ascii_lowercase().contains(&q)
                        || a.keywords
                            .iter()
                            .any(|k| k.to_ascii_lowercase().contains(&q))
                }
            })
            .map(|(i, _)| i)
            .collect();

        if state.selected >= filtered.len() {
            state.selected = filtered.len().saturating_sub(1);
        }

        // Keyboard.
        let mut activate_now = false;
        ctx.input(|i| {
            if i.key_pressed(Key::Escape) {
                *open = false;
            }
            if i.key_pressed(Key::ArrowDown) && state.selected + 1 < filtered.len() {
                state.selected += 1;
            }
            if i.key_pressed(Key::ArrowUp) && state.selected > 0 {
                state.selected -= 1;
            }
            if i.key_pressed(Key::Enter) {
                activate_now = true;
            }
        });

        // Scrim.
        let screen = ctx.content_rect();
        let scrim_layer = egui::LayerId::new(egui::Order::Background, Id::new("sauge_cmd_scrim"));
        ctx.layer_painter(scrim_layer).rect_filled(
            screen,
            corner(0.0),
            alpha(Color32::BLACK, 0.30),
        );

        let mut activated: Option<usize> = None;

        let frame = egui::Frame::default()
            .fill(palette.bg_surface)
            .stroke(Stroke::new(1.0, palette.border_default))
            .corner_radius(corner(RADIUS.lg))
            .inner_margin(egui::Margin::same(0))
            .shadow(Elevation::Modal.shadow(palette.dark_mode));

        egui::Window::new("sauge_cmd_palette")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, [0.0, 80.0])
            .default_width(self.width)
            .frame(frame)
            .show(ctx, |ui| {
                ui.set_width(self.width);

                // Search row.
                let search_h = 48.0;
                let (search_rect, _) =
                    ui.allocate_exact_size(vec2(self.width, search_h), Sense::hover());
                let pad = SPACING.s3;
                let icon_size = 18.0;
                let icon_rect = Rect::from_min_size(
                    egui::pos2(
                        search_rect.left() + pad,
                        search_rect.center().y - icon_size / 2.0,
                    ),
                    Vec2::splat(icon_size),
                );
                Icon::Search.paint(ui.painter(), icon_rect, palette.text_secondary);

                let edit_rect = Rect::from_min_max(
                    egui::pos2(icon_rect.right() + SPACING.s2, search_rect.top() + 6.0),
                    egui::pos2(search_rect.right() - pad, search_rect.bottom() - 6.0),
                );
                let edit = egui::TextEdit::singleline(&mut state.query)
                    .frame(egui::Frame::NONE)
                    .margin(egui::Margin::ZERO)
                    .desired_width(edit_rect.width())
                    .hint_text(self.placeholder)
                    .font(FontId::new(15.0, egui::FontFamily::Proportional))
                    .text_color(palette.text_primary);
                let edit_resp = ui.put(edit_rect, edit);
                if !state.focused {
                    edit_resp.request_focus();
                    state.focused = true;
                }

                // Bottom hairline of the search row.
                ui.painter().line_segment(
                    [
                        egui::pos2(search_rect.left(), search_rect.bottom()),
                        egui::pos2(search_rect.right(), search_rect.bottom()),
                    ],
                    Stroke::new(1.0, palette.border_subtle),
                );

                // Results.
                egui::ScrollArea::vertical()
                    .max_height(self.max_height)
                    .show(ui, |ui| {
                        if filtered.is_empty() {
                            ui.add_space(SPACING.s5);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("No results")
                                        .text_style(TextStyle::Body)
                                        .color(palette.text_tertiary),
                                );
                            });
                            ui.add_space(SPACING.s5);
                            return;
                        }
                        // Group by `group` while preserving order.
                        let mut last_group: Option<&str> = None;
                        for (visible_idx, &orig_idx) in filtered.iter().enumerate() {
                            let action = &self.actions[orig_idx];
                            if action.group != last_group {
                                if let Some(g) = action.group {
                                    ui.add_space(SPACING.s2);
                                    ui.label(
                                        egui::RichText::new(g)
                                            .text_style(TextStyle::Small)
                                            .color(palette.text_tertiary),
                                    );
                                }
                                last_group = action.group;
                            }
                            let active = visible_idx == state.selected;
                            let resp = paint_row(ui, action, &palette, active);
                            if resp.clicked() || (active && activate_now) {
                                activated = Some(orig_idx);
                                *open = false;
                            }
                        }
                    });

                ui.add_space(SPACING.s2);
                // Footer: kbd hints.
                ui.horizontal(|ui| {
                    ui.add_space(SPACING.s3);
                    let kbd = |ui: &mut Ui, text: &str| {
                        let g = ui.painter().layout_no_wrap(
                            text.into(),
                            FontId::new(11.0, egui::FontFamily::Monospace),
                            palette.text_secondary,
                        );
                        let pad = vec2(6.0, 2.0);
                        let size = g.size() + pad * 2.0;
                        let (r, _) = ui.allocate_exact_size(size, Sense::hover());
                        ui.painter().rect(
                            r,
                            corner(RADIUS.sm),
                            palette.bg_surface_alt,
                            Stroke::new(1.0, palette.border_subtle),
                            StrokeKind::Inside,
                        );
                        ui.painter().galley(
                            egui::pos2(r.left() + pad.x, r.center().y - g.size().y / 2.0),
                            g,
                            palette.text_secondary,
                        );
                    };
                    kbd(ui, "↑↓");
                    ui.label(
                        egui::RichText::new("navigate")
                            .text_style(TextStyle::Small)
                            .color(palette.text_tertiary),
                    );
                    ui.add_space(SPACING.s3);
                    kbd(ui, "↵");
                    ui.label(
                        egui::RichText::new("select")
                            .text_style(TextStyle::Small)
                            .color(palette.text_tertiary),
                    );
                    ui.add_space(SPACING.s3);
                    kbd(ui, "esc");
                    ui.label(
                        egui::RichText::new("close")
                            .text_style(TextStyle::Small)
                            .color(palette.text_tertiary),
                    );
                });
                ui.add_space(SPACING.s2);
            });

        // Persist state.
        ctx.data_mut(|d| d.insert_temp(state_id(), state));
        activated
    }
}

fn paint_row(
    ui: &mut Ui,
    action: &CommandAction<'_>,
    palette: &crate::Palette,
    active: bool,
) -> Response {
    let row_h = 36.0;
    let total_w = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(total_w, row_h), Sense::click());
    let bg = if active {
        alpha(palette.brand_default, 0.12)
    } else if response.hovered() {
        palette.bg_hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, corner(RADIUS.sm), bg);

    if active {
        let bar = Rect::from_min_max(
            egui::pos2(rect.left(), rect.top() + 4.0),
            egui::pos2(rect.left() + 3.0, rect.bottom() - 4.0),
        );
        ui.painter()
            .rect_filled(bar, corner(RADIUS.sm), palette.brand_default);
    }

    let pad = SPACING.s3;
    let mut x = rect.left() + pad;
    let cy = rect.center().y;
    let icon_size = 16.0;
    if let Some(icon) = action.icon {
        let r = Rect::from_min_size(egui::pos2(x, cy - icon_size / 2.0), Vec2::splat(icon_size));
        let c = if active {
            palette.brand_default
        } else {
            palette.text_secondary
        };
        icon.paint(ui.painter(), r, c);
        x += icon_size + SPACING.s2;
    } else {
        x += icon_size + SPACING.s2;
    }
    let font = FontId::new(14.0, egui::FontFamily::Proportional);
    let galley = ui
        .painter()
        .layout_no_wrap(action.label.clone(), font, palette.text_primary);
    ui.painter().galley(
        egui::pos2(x, cy - galley.size().y / 2.0),
        galley,
        palette.text_primary,
    );

    if let Some(shortcut) = action.shortcut {
        let mono = FontId::new(11.0, egui::FontFamily::Monospace);
        let g = ui
            .painter()
            .layout_no_wrap(shortcut.into(), mono, palette.text_tertiary);
        ui.painter().galley(
            egui::pos2(rect.right() - pad - g.size().x, cy - g.size().y / 2.0),
            g,
            palette.text_tertiary,
        );
    }
    response
}

fn state_id() -> Id {
    Id::new("egui_sauge::cmd_palette_state")
}
