//! Navigation primitives: [`NavItem`] for sidebars, [`Tabs`] for tab bars,
//! [`Breadcrumb`] for hierarchical paths.

use egui::{Color32, FontId, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget, vec2};

use super::{alpha, corner};
use crate::{Icon, RADIUS, SPACING, palette_of};

/// One row in a sidebar nav. Renders as `icon + label + optional badge`,
/// reacts to hover, and renders a brand-colored left bar when selected.
pub struct NavItem<'a> {
    label: &'a str,
    icon: Option<Icon>,
    selected: bool,
    badge: Option<&'a str>,
    disabled: bool,
}

impl<'a> NavItem<'a> {
    /// New item with just a label.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            selected: false,
            badge: None,
            disabled: false,
        }
    }
    /// Add a leading icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Mark as the active route.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    /// Right-aligned badge text (e.g. count, "new").
    pub fn badge(mut self, text: &'a str) -> Self {
        self.badge = Some(text);
        self
    }
    /// Disable interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for NavItem<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let height = 32.0;
        let width = ui.available_width().max(180.0);
        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);

        // Background: brand-tint when selected, hover bg on hover.
        let bg = if self.disabled {
            Color32::TRANSPARENT
        } else if self.selected {
            alpha(palette.brand_default, 0.12)
        } else if response.hovered() {
            palette.bg_hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, corner(RADIUS.md), bg);

        // Left accent bar when selected.
        if self.selected {
            let bar = Rect::from_min_max(
                egui::pos2(rect.left(), rect.top() + 6.0),
                egui::pos2(rect.left() + 3.0, rect.bottom() - 6.0),
            );
            ui.painter()
                .rect_filled(bar, corner(RADIUS.sm), palette.brand_default);
        }

        let pad = SPACING.s3;
        let mut x = rect.left() + pad;
        let cy = rect.center().y;

        // Icon.
        if let Some(icon) = self.icon {
            let size = 16.0;
            let r = Rect::from_min_size(egui::pos2(x, cy - size / 2.0), Vec2::splat(size));
            let c = if self.disabled {
                alpha(palette.text_secondary, 0.45)
            } else if self.selected {
                palette.brand_default
            } else {
                palette.text_secondary
            };
            icon.paint(ui.painter(), r, c);
            x += size + SPACING.s2;
        }

        // Label.
        let text_color = if self.disabled {
            alpha(palette.text_primary, 0.45)
        } else if self.selected {
            palette.text_primary
        } else {
            palette.text_secondary
        };
        let font = FontId::new(13.0, egui::FontFamily::Proportional);
        let galley = ui
            .painter()
            .layout_no_wrap(self.label.into(), font, text_color);
        ui.painter().galley(
            egui::pos2(x, cy - galley.size().y / 2.0),
            galley,
            text_color,
        );

        // Right badge.
        if let Some(badge_text) = self.badge {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(pad);
                let bf = FontId::new(11.0, egui::FontFamily::Proportional);
                let bg_galley =
                    ui.painter()
                        .layout_no_wrap(badge_text.into(), bf, palette.text_secondary);
                let bw = bg_galley.size().x + 10.0;
                let bh = 18.0;
                let by = cy - bh / 2.0;
                let bx = rect.right() - pad - bw;
                let badge_rect = Rect::from_min_size(egui::pos2(bx, by), vec2(bw, bh));
                ui.painter()
                    .rect_filled(badge_rect, corner(RADIUS.full), palette.bg_surface_alt);
                ui.painter().galley(
                    egui::pos2(bx + 5.0, cy - bg_galley.size().y / 2.0),
                    bg_galley,
                    palette.text_secondary,
                );
            });
        }

        if response.has_focus() && !self.disabled {
            ui.painter().rect_stroke(
                rect.expand(2.0),
                corner(RADIUS.md),
                Stroke::new(2.0, palette.focus_ring),
                StrokeKind::Outside,
            );
        }
        response
    }
}

/// A tab bar bound to a value of type `T`. Returns `true` when the selected
/// tab changed.
pub struct Tabs<'a, T: Clone + PartialEq> {
    selected: &'a mut T,
    items: Vec<(T, &'a str, Option<Icon>)>,
}

impl<'a, T: Clone + PartialEq> Tabs<'a, T> {
    /// New tab bar bound to `selected`.
    pub fn new(selected: &'a mut T) -> Self {
        Self {
            selected,
            items: Vec::new(),
        }
    }
    /// Add a tab.
    pub fn tab(mut self, value: T, label: &'a str) -> Self {
        self.items.push((value, label, None));
        self
    }
    /// Add a tab with a leading icon.
    pub fn tab_with_icon(mut self, value: T, icon: Icon, label: &'a str) -> Self {
        self.items.push((value, label, Some(icon)));
        self
    }
    /// Render the tab bar.
    pub fn show(self, ui: &mut Ui) -> bool {
        let palette = palette_of(ui.ctx());
        let mut changed = false;

        let row_resp = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for (value, label, icon) in self.items.iter() {
                    let is_selected = self.selected == value;
                    let font = FontId::new(13.0, egui::FontFamily::Proportional);
                    let galley =
                        ui.painter()
                            .layout_no_wrap((*label).into(), font, palette.text_primary);
                    let icon_w = if icon.is_some() { 16.0 + 6.0 } else { 0.0 };
                    let pad = SPACING.s3;
                    let height = 36.0;
                    let width = galley.size().x + icon_w + pad * 2.0;
                    let (rect, resp) = ui.allocate_exact_size(vec2(width, height), Sense::click());

                    // Bg on hover/selected.
                    let bg = if is_selected {
                        Color32::TRANSPARENT
                    } else if resp.hovered() {
                        palette.bg_hover
                    } else {
                        Color32::TRANSPARENT
                    };
                    ui.painter().rect_filled(rect, corner(RADIUS.sm), bg);

                    // Content.
                    let mut x = rect.left() + pad;
                    let cy = rect.center().y;
                    let fg = if is_selected {
                        palette.text_primary
                    } else {
                        palette.text_secondary
                    };
                    if let Some(icon) = icon {
                        let r = Rect::from_min_size(egui::pos2(x, cy - 8.0), Vec2::splat(16.0));
                        icon.paint(ui.painter(), r, fg);
                        x += icon_w;
                    }
                    let galley = ui.painter().layout_no_wrap(
                        (*label).into(),
                        FontId::new(13.0, egui::FontFamily::Proportional),
                        fg,
                    );
                    ui.painter()
                        .galley(egui::pos2(x, cy - galley.size().y / 2.0), galley, fg);

                    // Underline.
                    if is_selected {
                        let bar = Rect::from_min_max(
                            egui::pos2(rect.left() + pad, rect.bottom() - 2.0),
                            egui::pos2(rect.right() - pad, rect.bottom()),
                        );
                        ui.painter()
                            .rect_filled(bar, corner(0.0), palette.brand_default);
                    }

                    if resp.clicked() && !is_selected {
                        *self.selected = value.clone();
                        changed = true;
                    }
                }
            })
            .response;

        // Bottom hairline along the entire row.
        let r = row_resp.rect;
        ui.painter().line_segment(
            [
                egui::pos2(r.left(), r.bottom() - 1.0),
                egui::pos2(r.right(), r.bottom() - 1.0),
            ],
            Stroke::new(1.0, palette.border_subtle),
        );

        changed
    }
}

/// Hierarchical breadcrumb path. Returns `Some(i)` if the user clicked the
/// `i`-th segment (the last one is non-clickable, like Finder).
pub struct Breadcrumb<'a> {
    items: Vec<&'a str>,
}

impl<'a> Breadcrumb<'a> {
    /// New breadcrumb from a slice of segments.
    pub fn new(items: &[&'a str]) -> Self {
        Self {
            items: items.to_vec(),
        }
    }
    /// Push a segment.
    pub fn item(mut self, label: &'a str) -> Self {
        self.items.push(label);
        self
    }
    /// Render. Returns the index of the clicked segment, if any.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let palette = palette_of(ui.ctx());
        let mut clicked = None;
        ui.horizontal(|ui| {
            for (i, label) in self.items.iter().enumerate() {
                let is_last = i == self.items.len() - 1;
                let color = if is_last {
                    palette.text_primary
                } else {
                    palette.text_secondary
                };
                let resp = ui.add(
                    egui::Label::new(
                        egui::RichText::new(*label)
                            .font(FontId::new(13.0, egui::FontFamily::Proportional))
                            .color(color),
                    )
                    .sense(if is_last {
                        Sense::hover()
                    } else {
                        Sense::click()
                    }),
                );
                if !is_last && resp.clicked() {
                    clicked = Some(i);
                }
                if !is_last {
                    let (rect, _) = ui.allocate_exact_size(vec2(12.0, 12.0), Sense::hover());
                    Icon::ChevronRight.paint(ui.painter(), rect, palette.text_tertiary);
                }
            }
        });
        clicked
    }
}
