//! Themed dropdown [`MenuItem`] used inside [`egui::menu::menu_button`] or
//! any other popup. Pair with `ui.separator()` between groups.

use egui::{Color32, FontId, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget, vec2};

use super::{alpha, corner};
use crate::{Icon, Palette, RADIUS, SPACING, palette_of};

/// One row in a dropdown menu. Renders as `icon + label + optional shortcut`,
/// styles the danger variant with the error color, supports disabled state.
pub struct MenuItem<'a> {
    label: &'a str,
    icon: Option<Icon>,
    shortcut: Option<&'a str>,
    danger: bool,
    disabled: bool,
}

impl<'a> MenuItem<'a> {
    /// New plain menu item.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            shortcut: None,
            danger: false,
            disabled: false,
        }
    }
    /// Convenience: icon + label.
    pub fn with_icon(icon: Icon, label: &'a str) -> Self {
        Self::new(label).icon(icon)
    }
    /// Set the leading icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Right-aligned keyboard shortcut hint (e.g. `"⌘K"`).
    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }
    /// Style as a destructive action (error color).
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for MenuItem<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let height = 28.0;
        let width = ui.available_width().max(180.0);
        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);

        let fg = item_color(&palette, self.danger, self.disabled);
        let bg = if !self.disabled && response.hovered() {
            if self.danger {
                alpha(palette.error, 0.10)
            } else {
                palette.bg_hover
            }
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, corner(RADIUS.sm), bg);

        let pad = SPACING.s2;
        let mut x = rect.left() + pad;
        let cy = rect.center().y;
        let icon_size = 14.0;

        if let Some(icon) = self.icon {
            let r =
                Rect::from_min_size(egui::pos2(x, cy - icon_size / 2.0), Vec2::splat(icon_size));
            icon.paint(ui.painter(), r, fg);
            x += icon_size + SPACING.s2;
        } else {
            // Reserve space so labels align even without an icon.
            x += icon_size + SPACING.s2;
        }

        let font = FontId::new(13.0, egui::FontFamily::Proportional);
        let galley = ui.painter().layout_no_wrap(self.label.into(), font, fg);
        ui.painter()
            .galley(egui::pos2(x, cy - galley.size().y / 2.0), galley, fg);

        if let Some(shortcut) = self.shortcut {
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

        if response.has_focus() && !self.disabled {
            ui.painter().rect_stroke(
                rect.expand(1.0),
                corner(RADIUS.sm),
                Stroke::new(2.0, palette.focus_ring),
                StrokeKind::Outside,
            );
        }
        response
    }
}

fn item_color(p: &Palette, danger: bool, disabled: bool) -> Color32 {
    let base = if danger { p.error } else { p.text_primary };
    if disabled { alpha(base, 0.45) } else { base }
}

// -- SubMenu ----------------------------------------------------------------

/// A nested menu trigger. Renders like a [`MenuItem`] with a trailing chevron,
/// and opens a submenu popup on hover/click. Use **inside** a parent menu's
/// `content` closure (e.g. `ui.menu_button("More", |ui| { … })`).
///
/// ```ignore
/// ui.menu_button("Actions", |ui| {
///     ui.add(MenuItem::with_icon(Icon::Edit, "Edit"));
///     SubMenu::with_icon(Icon::Download, "Export").show(ui, |ui| {
///         ui.add(MenuItem::new("CSV"));
///         ui.add(MenuItem::new("JSON"));
///         ui.add(MenuItem::new("PDF"));
///     });
///     ui.separator();
///     ui.add(MenuItem::with_icon(Icon::Trash, "Delete").danger());
/// });
/// ```
pub struct SubMenu<'a> {
    label: &'a str,
    icon: Option<Icon>,
    disabled: bool,
}

impl<'a> SubMenu<'a> {
    /// New submenu trigger with a label.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            disabled: false,
        }
    }
    /// Convenience: icon + label.
    pub fn with_icon(icon: Icon, label: &'a str) -> Self {
        Self::new(label).icon(icon)
    }
    /// Set the leading icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Disable the submenu trigger (won't open).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Render the trigger row + open the submenu on hover/click.
    /// Returns the trigger row's [`Response`].
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> Response {
        let palette = palette_of(ui.ctx());
        let height = 28.0;
        let width = ui.available_width().max(180.0);
        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);

        let fg = item_color(&palette, false, self.disabled);
        // Visual is "open" while a submenu state is recorded as open for this
        // widget id. egui exposes the bookkeeping via `MenuState`.
        let is_open = egui::containers::menu::MenuState::from_ui(ui, |state, _| {
            state.open_item
                == Some(egui::containers::menu::SubMenu::id_from_widget_id(
                    response.id,
                ))
        });
        let bg = if !self.disabled && (response.hovered() || is_open) {
            palette.bg_hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, corner(RADIUS.sm), bg);

        let pad = SPACING.s2;
        let mut x = rect.left() + pad;
        let cy = rect.center().y;
        let icon_size = 14.0;

        if let Some(icon) = self.icon {
            let r =
                Rect::from_min_size(egui::pos2(x, cy - icon_size / 2.0), Vec2::splat(icon_size));
            icon.paint(ui.painter(), r, fg);
            x += icon_size + SPACING.s2;
        } else {
            x += icon_size + SPACING.s2;
        }

        let font = FontId::new(13.0, egui::FontFamily::Proportional);
        let galley = ui.painter().layout_no_wrap(self.label.into(), font, fg);
        ui.painter()
            .galley(egui::pos2(x, cy - galley.size().y / 2.0), galley, fg);

        // Trailing chevron-right indicating a nested submenu.
        let chev_size = 12.0;
        let chev_rect = Rect::from_min_size(
            egui::pos2(rect.right() - pad - chev_size, cy - chev_size / 2.0),
            Vec2::splat(chev_size),
        );
        Icon::ChevronRight.paint(ui.painter(), chev_rect, palette.text_tertiary);

        if response.has_focus() && !self.disabled {
            ui.painter().rect_stroke(
                rect.expand(1.0),
                corner(RADIUS.sm),
                Stroke::new(2.0, palette.focus_ring),
                StrokeKind::Outside,
            );
        }

        // Defer the actual popup management to egui's SubMenu.
        if !self.disabled {
            egui::containers::menu::SubMenu::new().show(ui, &response, content);
        }
        response
    }
}
