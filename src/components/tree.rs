//! [`TreeView`] — recursive hierarchical list (file tree, infra tree,
//! org chart). Each node has a label, optional icon, and optional
//! children that expand/collapse on click.
//!
//! Selection is bound to `&mut Option<T>` for typed single-pick
//! navigation; expansion state is persisted in `egui` memory via a
//! caller-supplied salt.

use egui::{Color32, FontId, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, vec2};

use super::{alpha, corner};
use crate::{Icon, RADIUS, SPACING, palette_of};

/// One node in a [`TreeView`]. Build trees as nested `TreeNode`s.
pub struct TreeNode<'a, T> {
    /// Value bound to the tree's `selected`.
    pub value: T,
    /// Label.
    pub label: &'a str,
    /// Optional leading icon. Files default to [`Icon::File`], folders
    /// pick `Folder`/`FolderOpen` based on expansion when `icon = None`.
    pub icon: Option<Icon>,
    /// Children. Empty = leaf.
    pub children: Vec<TreeNode<'a, T>>,
    /// Open by default the first time the node is rendered.
    pub default_open: bool,
}

impl<'a, T> TreeNode<'a, T> {
    /// New leaf node.
    pub fn leaf(value: T, label: &'a str) -> Self {
        Self {
            value,
            label,
            icon: None,
            children: Vec::new(),
            default_open: false,
        }
    }
    /// New folder-style node containing `children`.
    pub fn folder(value: T, label: &'a str, children: Vec<TreeNode<'a, T>>) -> Self {
        Self {
            value,
            label,
            icon: None,
            children,
            default_open: false,
        }
    }
    /// Override the icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Open by default.
    pub fn open(mut self) -> Self {
        self.default_open = true;
        self
    }
}

/// Hierarchical list bound to `selected`.
pub struct TreeView<'a, T: Clone + PartialEq> {
    selected: &'a mut Option<T>,
    nodes: Vec<TreeNode<'a, T>>,
    salt: egui::Id,
}

impl<'a, T: Clone + PartialEq> TreeView<'a, T> {
    /// New tree bound to `selected` (use `&mut None` to start without
    /// any selection). `id_salt` distinguishes multiple trees on the
    /// same screen for expansion-state persistence.
    pub fn new(selected: &'a mut Option<T>, id_salt: impl std::hash::Hash) -> Self {
        Self {
            selected,
            nodes: Vec::new(),
            salt: egui::Id::new(("sauge_tree", id_salt)),
        }
    }
    /// Append a top-level node.
    pub fn node(mut self, node: TreeNode<'a, T>) -> Self {
        self.nodes.push(node);
        self
    }
    /// Append several top-level nodes.
    pub fn nodes(mut self, nodes: impl IntoIterator<Item = TreeNode<'a, T>>) -> Self {
        self.nodes.extend(nodes);
        self
    }

    /// Render the tree. Returns the response of the last rendered row.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let Self {
            selected,
            nodes,
            salt,
        } = self;
        let mut last: Option<Response> = None;
        for (idx, node) in nodes.iter().enumerate() {
            let path = salt.with(idx);
            paint_node(ui, node, selected, &palette, 0, path, &mut last);
        }
        last.unwrap_or_else(|| ui.allocate_response(vec2(0.0, 0.0), Sense::hover()))
    }
}

fn paint_node<T: Clone + PartialEq>(
    ui: &mut Ui,
    node: &TreeNode<'_, T>,
    selected: &mut Option<T>,
    palette: &crate::Palette,
    depth: usize,
    id: egui::Id,
    last_resp: &mut Option<Response>,
) {
    // Persisted expansion state.
    let mut expanded = ui
        .ctx()
        .data(|d| d.get_temp::<bool>(id))
        .unwrap_or(node.default_open);

    let row_h = 24.0;
    let row_w = ui.available_width();
    let (rect, mut response) = ui.allocate_exact_size(vec2(row_w, row_h), Sense::click());
    let is_selected = selected.as_ref() == Some(&node.value);
    let has_children = !node.children.is_empty();

    let bg = if is_selected {
        alpha(palette.brand_default, 0.14)
    } else if response.hovered() {
        palette.bg_hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, corner(RADIUS.sm), bg);

    let indent = SPACING.s3 + depth as f32 * 14.0;
    let cy = rect.center().y;
    let mut x = rect.left() + indent;

    // Chevron (or invisible spacer for leaves).
    let chev_size = 12.0;
    if has_children {
        let chev_rect =
            Rect::from_min_size(egui::pos2(x, cy - chev_size / 2.0), Vec2::splat(chev_size));
        let chev_resp = ui.interact(chev_rect, id.with("chev"), Sense::click());
        let icon = if expanded {
            Icon::ChevronDown
        } else {
            Icon::ChevronRight
        };
        icon.paint(ui.painter(), chev_rect, palette.text_tertiary);
        if chev_resp.clicked() {
            expanded = !expanded;
            response.mark_changed();
        }
    }
    x += chev_size + 4.0;

    // Icon.
    let default_icon = if has_children {
        if expanded {
            Icon::FolderOpen
        } else {
            Icon::Folder
        }
    } else {
        Icon::File
    };
    let icon = node.icon.unwrap_or(default_icon);
    let icon_size = 14.0;
    let icon_rect =
        Rect::from_min_size(egui::pos2(x, cy - icon_size / 2.0), Vec2::splat(icon_size));
    let icon_color = if is_selected {
        palette.brand_default
    } else if has_children {
        palette.text_secondary
    } else {
        palette.text_tertiary
    };
    icon.paint(ui.painter(), icon_rect, icon_color);
    x += icon_size + SPACING.s2;

    // Label.
    let label_color = if is_selected {
        palette.text_primary
    } else {
        palette.text_secondary
    };
    let font = FontId::new(13.0, egui::FontFamily::Proportional);
    let galley = ui
        .painter()
        .layout_no_wrap(node.label.into(), font, label_color);
    ui.painter().galley(
        egui::pos2(x, cy - galley.size().y / 2.0),
        galley,
        label_color,
    );

    // Whole-row click.
    if response.clicked() {
        if has_children {
            expanded = !expanded;
        } else {
            *selected = Some(node.value.clone());
        }
        response.mark_changed();
    }

    // Persist expansion.
    ui.ctx().data_mut(|d| d.insert_temp::<bool>(id, expanded));

    if response.has_focus() {
        ui.painter().rect_stroke(
            rect.expand(1.0),
            corner(RADIUS.sm),
            Stroke::new(2.0, palette.focus_ring),
            StrokeKind::Outside,
        );
    }
    *last_resp = Some(response);

    // Children.
    if has_children && expanded {
        for (i, child) in node.children.iter().enumerate() {
            paint_node(
                ui,
                child,
                selected,
                palette,
                depth + 1,
                id.with(i),
                last_resp,
            );
        }
    }
}
