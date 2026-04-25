//! Floating [`Toast`]s with auto-dismiss, managed by [`Toasts`].

use std::time::{Duration, Instant};

use egui::{Context, Rect, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, vec2};

use super::alert::Level;
use super::{alpha, corner};
use crate::{Elevation, Icon, RADIUS, SPACING, palette_of};

/// Single toast notification. Prefer pushing via [`Toasts`] rather than
/// constructing directly.
#[derive(Debug, Clone)]
pub struct Toast {
    /// Severity (drives icon + accent color).
    pub level: Level,
    /// Toast text.
    pub text: String,
    /// When the toast should disappear.
    pub deadline: Instant,
}

/// Toast queue. Create once, hold in your app state, call
/// [`Toasts::show`] every frame.
#[derive(Debug, Default)]
pub struct Toasts {
    queue: Vec<Toast>,
}

impl Toasts {
    /// New empty queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push an info toast (4 s default).
    pub fn info(&mut self, text: impl Into<String>) {
        self.push(Level::Info, text.into(), Duration::from_secs(4));
    }
    /// Push a success toast (4 s default).
    pub fn success(&mut self, text: impl Into<String>) {
        self.push(Level::Success, text.into(), Duration::from_secs(4));
    }
    /// Push a warning toast (5 s default).
    pub fn warning(&mut self, text: impl Into<String>) {
        self.push(Level::Warning, text.into(), Duration::from_secs(5));
    }
    /// Push an error toast (6 s default).
    pub fn error(&mut self, text: impl Into<String>) {
        self.push(Level::Error, text.into(), Duration::from_secs(6));
    }
    /// Push a custom toast.
    pub fn push(&mut self, level: Level, text: String, duration: Duration) {
        self.queue.push(Toast {
            level,
            text,
            deadline: Instant::now() + duration,
        });
    }

    /// Render the toast stack in the top-right corner. Call every frame.
    pub fn show(&mut self, ctx: &Context) {
        // Expire timed-out toasts.
        let now = Instant::now();
        self.queue.retain(|t| t.deadline > now);
        if self.queue.is_empty() {
            return;
        }
        // Ensure the UI keeps repainting so auto-dismiss happens on a quiet ctx.
        ctx.request_repaint_after(Duration::from_millis(250));

        let screen = ctx.content_rect();
        let panel_w = 340.0;
        let right = screen.right() - SPACING.s4;
        let top = screen.top() + SPACING.s4;

        let area_rect = Rect::from_min_max(
            egui::pos2(right - panel_w, top),
            egui::pos2(right, screen.bottom() - SPACING.s4),
        );

        let mut to_remove: Vec<usize> = Vec::new();
        egui::Area::new(egui::Id::new("sauge_toasts"))
            .order(egui::Order::Foreground)
            .fixed_pos(area_rect.min)
            .show(ctx, |ui| {
                ui.set_max_width(panel_w);
                for (i, toast) in self.queue.iter().enumerate() {
                    if paint_toast(ui, toast) {
                        to_remove.push(i);
                    }
                    ui.add_space(SPACING.s2);
                }
            });

        for i in to_remove.into_iter().rev() {
            self.queue.remove(i);
        }
    }
}

/// Returns true if the toast's close button was clicked.
fn paint_toast(ui: &mut Ui, toast: &Toast) -> bool {
    let palette = palette_of(ui.ctx());
    let accent = toast.level.color(&palette);
    let mut dismissed = false;
    egui::Frame::default()
        .fill(palette.bg_surface)
        .stroke(Stroke::new(1.0, palette.border_default))
        .corner_radius(corner(RADIUS.md))
        .inner_margin(egui::Margin::same(SPACING.s3 as i8))
        .shadow(Elevation::Popover.shadow(palette.dark_mode))
        .show(ui, |ui| {
            // Left accent bar.
            let row_rect = ui
                .horizontal_top(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(20.0, 20.0), Sense::hover());
                    toast.level.icon().paint(ui.painter(), rect, accent);
                    ui.add_space(SPACING.s2);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&toast.text)
                                .text_style(TextStyle::Body)
                                .color(palette.text_primary),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        let (rect, resp) =
                            ui.allocate_exact_size(Vec2::splat(18.0), Sense::click());
                        if resp.hovered() {
                            ui.painter().rect(
                                rect,
                                corner(RADIUS.sm),
                                alpha(palette.text_primary, 0.08),
                                Stroke::NONE,
                                StrokeKind::Inside,
                            );
                        }
                        Icon::Close.paint(ui.painter(), rect.shrink(3.0), palette.text_secondary);
                        if resp.clicked() {
                            dismissed = true;
                        }
                    });
                })
                .response
                .rect;
            // Left accent.
            ui.painter().rect_filled(
                Rect::from_min_max(
                    egui::pos2(row_rect.left() - SPACING.s3, row_rect.top() - 4.0),
                    egui::pos2(row_rect.left() - SPACING.s3 + 3.0, row_rect.bottom() + 4.0),
                ),
                corner(RADIUS.sm),
                accent,
            );
        });
    dismissed
}

impl Level {}
