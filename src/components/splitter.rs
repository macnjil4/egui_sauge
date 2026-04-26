//! [`Splitter`] — resizable side / bottom panel with a themed drag
//! handle. Thin wrapper on top of [`egui::Panel`] that applies our
//! palette (background, border, hover-tinted handle).

use egui::{InnerResponse, Stroke, Ui};

use super::corner;
use crate::{RADIUS, palette_of};

type ActionsSlot<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// Side or bottom of the workbench.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitterSide {
    /// Left tool window (e.g. file tree).
    Left,
    /// Right tool window (e.g. inspector, outline).
    Right,
    /// Bottom tool window (e.g. terminal).
    Bottom,
}

/// Themed resizable splitter.
pub struct Splitter<'a> {
    id: egui::Id,
    side: SplitterSide,
    default_size: f32,
    min_size: f32,
    max_size: f32,
    title: Option<&'a str>,
    actions: Option<ActionsSlot<'a>>,
}

impl<'a> Splitter<'a> {
    /// New splitter on `side` with a unique `id` (used to persist size
    /// across frames).
    pub fn new(id: impl Into<egui::Id>, side: SplitterSide) -> Self {
        Self {
            id: id.into(),
            side,
            default_size: 240.0,
            min_size: 160.0,
            max_size: 800.0,
            title: None,
            actions: None,
        }
    }
    /// Optional title strip rendered at the top of the panel (with the
    /// trailing actions slot).
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
    /// Right-aligned action slot in the title bar (collapse button,
    /// settings cog…). No-op when [`Self::title`] isn't set.
    pub fn actions(mut self, actions: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.actions = Some(Box::new(actions));
        self
    }
    /// Default size on first show (width for Left/Right, height for Bottom).
    pub fn default_size(mut self, size: f32) -> Self {
        self.default_size = size;
        self
    }
    /// Minimum size the user can drag to.
    pub fn min_size(mut self, size: f32) -> Self {
        self.min_size = size;
        self
    }
    /// Maximum size the user can drag to.
    pub fn max_size(mut self, size: f32) -> Self {
        self.max_size = size;
        self
    }

    /// Render the panel. Returns the inner response.
    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let palette = palette_of(ui.ctx());
        let frame = egui::Frame::default()
            .fill(palette.bg_surface_alt)
            .stroke(Stroke::new(1.0, palette.border_subtle))
            .inner_margin(egui::Margin::ZERO);

        let title = self.title;
        let actions = self.actions;

        let render = |ui: &mut Ui| {
            if let Some(t) = title {
                paint_title(ui, t, actions, &palette);
            }
            ui.separator();
            body(ui)
        };

        match self.side {
            SplitterSide::Left => egui::Panel::left(self.id)
                .resizable(true)
                .default_size(self.default_size)
                .min_size(self.min_size)
                .max_size(self.max_size)
                .frame(frame)
                .show_inside(ui, render),
            SplitterSide::Right => egui::Panel::right(self.id)
                .resizable(true)
                .default_size(self.default_size)
                .min_size(self.min_size)
                .max_size(self.max_size)
                .frame(frame)
                .show_inside(ui, render),
            SplitterSide::Bottom => egui::Panel::bottom(self.id)
                .resizable(true)
                .default_size(self.default_size)
                .min_size(self.min_size)
                .max_size(self.max_size)
                .frame(frame)
                .show_inside(ui, render),
        }
    }
}

fn paint_title(
    ui: &mut Ui,
    title: &str,
    actions: Option<ActionsSlot<'_>>,
    palette: &crate::Palette,
) {
    let height = 28.0;
    let frame = egui::Frame::default()
        .fill(palette.bg_surface)
        .corner_radius(corner(0.0))
        .inner_margin(egui::Margin::symmetric(crate::SPACING.s3 as i8, 0));
    frame.show(ui, |ui| {
        ui.set_height(height);
        ui.horizontal_centered(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .font(egui::FontId::new(11.0, egui::FontFamily::Proportional))
                    .color(palette.text_secondary),
            );
            if let Some(actions) = actions {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), actions);
            }
        });
    });
    ui.painter().line_segment(
        [
            egui::pos2(ui.min_rect().left(), ui.cursor().top()),
            egui::pos2(ui.min_rect().right(), ui.cursor().top()),
        ],
        Stroke::new(1.0, palette.border_subtle),
    );
    let _ = RADIUS;
}
