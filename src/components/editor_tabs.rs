//! [`EditorTabs`] — file-style tab bar with per-tab close × and a
//! "modified" indicator dot. Distinct from the simple
//! [`crate::components::Tabs`] — those are section navigation, these
//! represent open documents in an editor.

use egui::{Color32, FontId, Rect, Response, Sense, Stroke, Ui, Vec2, vec2};

use super::{alpha, corner};
use crate::{Icon, RADIUS, SPACING, palette_of};

/// One file tab.
pub struct EditorTab<'a, T> {
    /// Value bound to the bar's `selected`.
    pub value: T,
    /// Tab label (typically file name).
    pub label: &'a str,
    /// Optional leading icon (file type, language).
    pub icon: Option<Icon>,
    /// Show a modified-content dot in place of the close icon when
    /// not hovered.
    pub modified: bool,
    /// Disable the close affordance (shown but does nothing).
    pub closable: bool,
}

impl<'a, T> EditorTab<'a, T> {
    /// New tab.
    pub fn new(value: T, label: &'a str) -> Self {
        Self {
            value,
            label,
            icon: None,
            modified: false,
            closable: true,
        }
    }
    /// Convenience: icon + value + label.
    pub fn with_icon(value: T, icon: Icon, label: &'a str) -> Self {
        Self::new(value, label).icon(icon)
    }
    /// Override the icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Show the modified indicator dot.
    pub fn modified(mut self, modified: bool) -> Self {
        self.modified = modified;
        self
    }
    /// Hide the close × (e.g. for a "pinned" tab).
    pub fn pinned(mut self) -> Self {
        self.closable = false;
        self
    }
}

/// Action returned by [`EditorTabs::show`].
#[derive(Debug, Clone)]
pub enum EditorTabAction<T> {
    /// User selected a tab.
    Selected(T),
    /// User clicked the × on a tab.
    Closed(T),
}

/// File-style tab bar.
pub struct EditorTabs<'a, T: Clone + PartialEq> {
    selected: &'a mut T,
    tabs: Vec<EditorTab<'a, T>>,
    height: f32,
}

impl<'a, T: Clone + PartialEq> EditorTabs<'a, T> {
    /// Empty tab bar bound to `selected`.
    pub fn new(selected: &'a mut T) -> Self {
        Self {
            selected,
            tabs: Vec::new(),
            height: 32.0,
        }
    }
    /// Append a tab.
    pub fn tab(mut self, tab: EditorTab<'a, T>) -> Self {
        self.tabs.push(tab);
        self
    }
    /// Override the bar height (default 32 px).
    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    /// Render. Returns `Some(action)` if the user selected or closed a
    /// tab this frame.
    pub fn show(self, ui: &mut Ui) -> Option<EditorTabAction<T>> {
        let palette = palette_of(ui.ctx());
        let Self {
            selected,
            tabs,
            height,
        } = self;
        let mut action: Option<EditorTabAction<T>> = None;

        let frame = egui::Frame::default()
            .fill(palette.bg_surface_alt)
            .inner_margin(egui::Margin::ZERO);
        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.set_height(height);
                let scroll = egui::ScrollArea::horizontal()
                    .id_salt("sauge_editor_tabs")
                    .auto_shrink([false, true]);
                scroll.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        for tab in &tabs {
                            if let Some(a) = paint_tab(ui, tab, selected, height, &palette) {
                                action = Some(a);
                            }
                        }
                    });
                });
            });
            // Bottom hairline.
            let r = ui.min_rect();
            ui.painter().line_segment(
                [
                    egui::pos2(r.left(), r.bottom() - 0.5),
                    egui::pos2(r.right(), r.bottom() - 0.5),
                ],
                Stroke::new(1.0, palette.border_default),
            );
        });
        action
    }
}

fn paint_tab<T: Clone + PartialEq>(
    ui: &mut Ui,
    tab: &EditorTab<'_, T>,
    selected: &mut T,
    height: f32,
    palette: &crate::Palette,
) -> Option<EditorTabAction<T>> {
    let pad = SPACING.s3;
    let icon_w = if tab.icon.is_some() { 16.0 + 6.0 } else { 0.0 };
    let close_w = 22.0;
    let font = FontId::new(13.0, egui::FontFamily::Proportional);
    let galley = ui
        .painter()
        .layout_no_wrap(tab.label.into(), font.clone(), palette.text_primary);
    let width = galley.size().x + icon_w + close_w + pad * 2.0;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    let is_selected = *selected == tab.value;

    let bg = if is_selected {
        palette.bg_surface
    } else if response.hovered() {
        palette.bg_hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, corner(0.0), bg);

    // Active accent strip on top.
    if is_selected {
        let bar = Rect::from_min_max(
            egui::pos2(rect.left(), rect.top()),
            egui::pos2(rect.right(), rect.top() + 2.0),
        );
        ui.painter()
            .rect_filled(bar, corner(0.0), palette.brand_default);
    }
    // Trailing separator (between tabs).
    ui.painter().line_segment(
        [
            egui::pos2(rect.right() - 0.5, rect.top() + 6.0),
            egui::pos2(rect.right() - 0.5, rect.bottom() - 6.0),
        ],
        Stroke::new(1.0, palette.border_subtle),
    );

    let mut x = rect.left() + pad;
    let cy = rect.center().y;
    let fg = if is_selected {
        palette.text_primary
    } else {
        palette.text_secondary
    };

    if let Some(icon) = tab.icon {
        let r = Rect::from_min_size(egui::pos2(x, cy - 8.0), Vec2::splat(16.0));
        icon.paint(ui.painter(), r, fg);
        x += 16.0 + 6.0;
    }
    let galley = ui.painter().layout_no_wrap(tab.label.into(), font, fg);
    ui.painter()
        .galley(egui::pos2(x, cy - galley.size().y / 2.0), galley, fg);

    // Close × (or modified dot when not hovered).
    let close_size = 14.0;
    let close_rect = Rect::from_min_size(
        egui::pos2(rect.right() - pad - close_size, cy - close_size / 2.0),
        Vec2::splat(close_size),
    );
    let close_hover = response.hovered() || is_selected;
    let mut close_resp: Option<Response> = None;
    if tab.closable && close_hover {
        let r = ui.interact(
            close_rect.expand(2.0),
            ui.id().with(("editor_tab_close", tab.label)),
            Sense::click(),
        );
        if r.hovered() {
            ui.painter().rect_filled(
                close_rect.expand(2.0),
                corner(RADIUS.sm),
                alpha(palette.text_primary, 0.10),
            );
        }
        Icon::Close.paint(ui.painter(), close_rect, palette.text_secondary);
        close_resp = Some(r);
    } else if tab.modified {
        let center = close_rect.center();
        ui.painter()
            .circle_filled(center, 3.0, palette.text_secondary);
    }

    let mut out: Option<EditorTabAction<T>> = None;
    if let Some(cr) = close_resp
        && cr.clicked()
    {
        out = Some(EditorTabAction::Closed(tab.value.clone()));
    } else if response.clicked() && !is_selected {
        *selected = tab.value.clone();
        out = Some(EditorTabAction::Selected(tab.value.clone()));
    }
    out
}
