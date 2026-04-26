//! [`ActivityBar`] — vertical icon-only navigation strip used along the
//! left or right edge of an IDE-style workbench. Each item toggles the
//! associated tool window; clicking the active item collapses the
//! adjacent panel (`selected` becomes `None`).

use egui::{Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget, vec2};

use super::{alpha, corner};
use crate::{Icon, RADIUS, SPACING, palette_of};

/// One [`ActivityBar`] entry.
pub struct ActivityItem<'a, T> {
    /// Value bound to the bar's `selected`. Click sets it to `Some(value)`,
    /// clicking the already-selected item flips back to `None`.
    pub value: T,
    /// Icon (use [`Icon::Glyph`] / [`Icon::Custom`] for non-bundled glyphs).
    pub icon: Icon,
    /// Hover tooltip — typically the panel name + shortcut.
    pub tooltip: Option<&'a str>,
    /// Show a small dot (e.g. unread indicator).
    pub badge: bool,
    /// Disable interaction.
    pub disabled: bool,
}

impl<'a, T> ActivityItem<'a, T> {
    /// New item.
    pub fn new(value: T, icon: Icon) -> Self {
        Self {
            value,
            icon,
            tooltip: None,
            badge: false,
            disabled: false,
        }
    }
    /// Hover tooltip.
    pub fn tooltip(mut self, text: &'a str) -> Self {
        self.tooltip = Some(text);
        self
    }
    /// Show an unread / new-event dot.
    pub fn badge(mut self, on: bool) -> Self {
        self.badge = on;
        self
    }
    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Vertical icon-only nav. Renders as a 44px-wide column. Pair with
/// [`crate::components::Splitter`] / [`egui::Panel::left`] for the
/// adjacent tool window.
pub struct ActivityBar<'a, T: PartialEq + Clone> {
    selected: &'a mut Option<T>,
    items: Vec<ActivityItem<'a, T>>,
    bottom: Vec<ActivityItem<'a, T>>,
    width: f32,
}

impl<'a, T: PartialEq + Clone> ActivityBar<'a, T> {
    /// Empty bar bound to `selected`. `*selected = None` means no panel
    /// is open; clicking an item sets it.
    pub fn new(selected: &'a mut Option<T>) -> Self {
        Self {
            selected,
            items: Vec::new(),
            bottom: Vec::new(),
            width: 44.0,
        }
    }
    /// Append an item to the top group.
    pub fn item(mut self, item: ActivityItem<'a, T>) -> Self {
        self.items.push(item);
        self
    }
    /// Append an item to the bottom group (e.g. Settings, Account). They
    /// stick to the bottom of the bar with a separator above.
    pub fn bottom_item(mut self, item: ActivityItem<'a, T>) -> Self {
        self.bottom.push(item);
        self
    }
    /// Override the bar width (default 44 px).
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Render the bar. Returns the [`Response`] of the last rendered
    /// item (for compose-up positioning).
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let Self {
            selected,
            items,
            bottom,
            width,
        } = self;

        let mut last: Option<Response> = None;
        ui.vertical(|ui| {
            ui.set_width(width);
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.add_space(SPACING.s2);
            for item in &items {
                let r = paint_item(ui, item, selected, width);
                last = Some(r);
            }

            if !bottom.is_empty() {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(SPACING.s2);
                    for item in bottom.iter().rev() {
                        let r = paint_item(ui, item, selected, width);
                        last = Some(r);
                    }
                    // Separator above the bottom group.
                    let y = ui.cursor().top() - 4.0;
                    ui.painter().line_segment(
                        [
                            egui::pos2(ui.min_rect().left() + 8.0, y),
                            egui::pos2(ui.min_rect().right() - 8.0, y),
                        ],
                        Stroke::new(1.0, palette.border_subtle),
                    );
                });
            }
        });
        last.unwrap_or_else(|| ui.allocate_response(vec2(0.0, 0.0), Sense::hover()))
    }
}

fn paint_item<T: PartialEq + Clone>(
    ui: &mut Ui,
    item: &ActivityItem<'_, T>,
    selected: &mut Option<T>,
    width: f32,
) -> Response {
    let palette = palette_of(ui.ctx());
    let h = 36.0;
    let sense = if item.disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, mut response) = ui.allocate_exact_size(vec2(width, h), sense);
    let is_selected = selected.as_ref() == Some(&item.value);

    let bg = if item.disabled {
        Color32::TRANSPARENT
    } else if is_selected {
        alpha(palette.brand_default, 0.14)
    } else if response.hovered() {
        palette.bg_hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, corner(RADIUS.sm), bg);

    // Left accent bar when active.
    if is_selected {
        let bar = Rect::from_min_max(
            egui::pos2(rect.left(), rect.top() + 6.0),
            egui::pos2(rect.left() + 2.5, rect.bottom() - 6.0),
        );
        ui.painter()
            .rect_filled(bar, corner(RADIUS.sm), palette.brand_default);
    }

    let icon_size: f32 = 18.0;
    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
    let color = if item.disabled {
        alpha(palette.text_secondary, 0.45)
    } else if is_selected {
        palette.brand_default
    } else {
        palette.text_secondary
    };
    item.icon.paint(ui.painter(), icon_rect, color);

    if item.badge {
        let dot_r = 3.0;
        let center = egui::pos2(rect.right() - 9.0, rect.top() + 9.0);
        ui.painter()
            .circle_filled(center, dot_r + 1.5, palette.bg_app);
        ui.painter().circle_filled(center, dot_r, palette.error);
    }

    if response.clicked() && !item.disabled {
        if is_selected {
            *selected = None;
        } else {
            *selected = Some(item.value.clone());
        }
        response.mark_changed();
    }
    if response.has_focus() && !item.disabled {
        ui.painter().rect_stroke(
            rect.expand(1.0),
            corner(RADIUS.sm),
            Stroke::new(2.0, palette.focus_ring),
            StrokeKind::Outside,
        );
    }
    if let Some(tip) = item.tooltip {
        return crate::components::tooltip(&response, tip);
    }
    response
}

impl<'a, T: PartialEq + Clone> Widget for ActivityBar<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}
