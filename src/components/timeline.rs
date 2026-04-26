//! [`Timeline`] — vertical event list with a connector line and
//! semantic colored markers. Built for audit logs, deploy history,
//! incident timelines.

use egui::{Color32, FontId, Response, Sense, Stroke, TextStyle, Ui, Vec2, Widget, vec2};

use super::alert::Level;
use crate::{Icon, SPACING, palette_of};

/// Single timeline event.
pub struct TimelineEvent<'a> {
    /// Severity / category — drives the marker icon and color.
    pub level: Level,
    /// Optional explicit icon. Falls back to [`Level::icon`].
    pub icon: Option<Icon>,
    /// Override the marker color (defaults to `Level::color`).
    pub color: Option<Color32>,
    /// Timestamp text (e.g. `"14:02:11"` or `"2 min ago"`).
    pub timestamp: Option<&'a str>,
    /// Title of the event.
    pub title: &'a str,
    /// Optional descriptive paragraph.
    pub body: Option<&'a str>,
}

impl<'a> TimelineEvent<'a> {
    /// New event with required title + level.
    pub fn new(level: Level, title: &'a str) -> Self {
        Self {
            level,
            icon: None,
            color: None,
            timestamp: None,
            title,
            body: None,
        }
    }
    /// Override the marker icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Override the marker color.
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
    /// Add a timestamp shown above the title.
    pub fn timestamp(mut self, ts: &'a str) -> Self {
        self.timestamp = Some(ts);
        self
    }
    /// Add a body paragraph under the title.
    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }
}

/// Vertical list of [`TimelineEvent`]s.
pub struct Timeline<'a> {
    events: Vec<TimelineEvent<'a>>,
}

impl<'a> Default for Timeline<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Timeline<'a> {
    /// Empty timeline.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    /// Append an event.
    pub fn event(mut self, event: TimelineEvent<'a>) -> Self {
        self.events.push(event);
        self
    }
    /// Render. Returns the response of the last event.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let marker_size: f32 = 24.0;
        let connector_x = SPACING.s4 + marker_size / 2.0;
        let n = self.events.len();
        let mut last: Option<Response> = None;

        ui.vertical(|ui| {
            for (i, ev) in self.events.iter().enumerate() {
                let is_last = i == n - 1;
                let row_top = ui.cursor().top();
                let resp = ui
                    .horizontal_top(|ui| {
                        // Left column: marker (with optional connector to next).
                        let (marker_rect, _) = ui.allocate_exact_size(
                            vec2(connector_x * 2.0, marker_size + SPACING.s2),
                            Sense::hover(),
                        );
                        let center = egui::pos2(
                            marker_rect.left() + connector_x,
                            marker_rect.top() + marker_size / 2.0,
                        );
                        let color = ev.color.unwrap_or_else(|| ev.level.color(&palette));
                        // Connector down to next marker.
                        if !is_last {
                            ui.painter().line_segment(
                                [
                                    egui::pos2(center.x, center.y + marker_size / 2.0),
                                    egui::pos2(center.x, marker_rect.bottom() + 80.0),
                                ],
                                Stroke::new(1.5, palette.border_default),
                            );
                        }
                        // Marker disc.
                        ui.painter().circle_filled(
                            center,
                            marker_size / 2.0,
                            super::alpha(color, 0.18),
                        );
                        ui.painter()
                            .circle_filled(center, marker_size / 2.0 - 6.0, color);
                        let icon = ev.icon.unwrap_or_else(|| ev.level.icon());
                        icon.paint(
                            ui.painter(),
                            egui::Rect::from_center_size(center, Vec2::splat(14.0)),
                            palette.text_on_brand,
                        );

                        // Right column: text.
                        ui.add_space(SPACING.s2);
                        ui.vertical(|ui| {
                            if let Some(ts) = ev.timestamp {
                                ui.label(
                                    egui::RichText::new(ts)
                                        .text_style(TextStyle::Small)
                                        .color(palette.text_tertiary),
                                );
                            }
                            ui.label(
                                egui::RichText::new(ev.title)
                                    .font(FontId::new(14.0, egui::FontFamily::Proportional))
                                    .color(palette.text_primary),
                            );
                            if let Some(body) = ev.body {
                                ui.label(
                                    egui::RichText::new(body)
                                        .text_style(TextStyle::Body)
                                        .color(palette.text_secondary),
                                );
                            }
                        });
                    })
                    .response;
                let _ = row_top;
                if !is_last {
                    ui.add_space(SPACING.s3);
                }
                last = Some(resp);
            }
        });
        last.unwrap_or_else(|| ui.allocate_response(vec2(0.0, 0.0), Sense::hover()))
    }
}

impl<'a> Widget for Timeline<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}
