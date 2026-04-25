//! Modal [`Dialog`] — re-themes [`egui::Window`] with title / body / actions
//! layout and a backdrop scrim. [`ConfirmDialog`] is a turn-key
//! cancel-or-confirm wrapper.

use egui::{FontId, Stroke, StrokeKind, Ui};

use super::button::{Button, ButtonSize};

use super::{alpha, corner};
use crate::{Elevation, RADIUS, SPACING, palette_of};

/// Handed to [`Dialog::show`] callbacks; call [`DialogControl::close`] to
/// request the dialog close after the current frame.
pub struct DialogControl {
    close: bool,
}

impl DialogControl {
    fn new() -> Self {
        Self { close: false }
    }
    /// Ask the dialog to close. The dialog stays visible for the rest of
    /// this frame; [`Dialog::show`] returns `true`, and the caller toggles
    /// its own visibility flag.
    pub fn close(&mut self) {
        self.close = true;
    }
}

/// Modal dialog: blocks the rest of the UI with a scrim and renders a
/// centered surface containing `body` and `actions`.
///
/// Visibility is managed by the caller. Call [`Dialog::show`] only when
/// your own `open` flag is `true`; it returns `true` when the user
/// requested closing (× icon or an action that called
/// [`DialogControl::close`]).
pub struct Dialog<'a> {
    title: &'a str,
    width: f32,
}

impl<'a> Dialog<'a> {
    /// New dialog with the given title.
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            width: 420.0,
        }
    }
    /// Override the dialog width.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Render the dialog. `body` draws the content; `actions` draws the
    /// bottom button row. Both receive a [`DialogControl`] so they can
    /// request closure. Returns `true` if close was requested.
    pub fn show(
        self,
        ctx: &egui::Context,
        body: impl FnOnce(&mut Ui, &mut DialogControl),
        actions: impl FnOnce(&mut Ui, &mut DialogControl),
    ) -> bool {
        let palette = palette_of(ctx);

        // Scrim behind the dialog.
        let screen = ctx.content_rect();
        let scrim_layer = egui::LayerId::new(egui::Order::Background, egui::Id::new("sauge_scrim"));
        let scrim_painter = ctx.layer_painter(scrim_layer);
        scrim_painter.rect_filled(screen, corner(0.0), alpha(egui::Color32::BLACK, 0.35));

        let title = self.title.to_string();
        let width = self.width;
        let mut control = DialogControl::new();

        egui::Window::new(&title)
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(width)
            .frame(
                egui::Frame::default()
                    .fill(palette.bg_surface)
                    .stroke(Stroke::new(1.0, palette.border_default))
                    .corner_radius(corner(RADIUS.lg))
                    .inner_margin(egui::Margin::same(SPACING.s4 as i8))
                    .shadow(Elevation::Modal.shadow(palette.dark_mode)),
            )
            .show(ctx, |ui| {
                // Header row.
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&title)
                            .font(FontId::new(18.0, egui::FontFamily::Proportional))
                            .color(palette.text_primary),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (rect, resp) =
                            ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                        if resp.hovered() {
                            ui.painter().rect(
                                rect,
                                corner(RADIUS.sm),
                                alpha(palette.text_primary, 0.08),
                                Stroke::NONE,
                                StrokeKind::Inside,
                            );
                        }
                        crate::Icon::Close.paint(
                            ui.painter(),
                            rect.shrink(5.0),
                            palette.text_secondary,
                        );
                        if resp.clicked() {
                            control.close();
                        }
                    });
                });
                ui.add_space(SPACING.s3);
                // Separator line.
                let sep_y = ui.cursor().top();
                ui.painter().line_segment(
                    [
                        egui::pos2(ui.min_rect().left(), sep_y),
                        egui::pos2(ui.min_rect().right(), sep_y),
                    ],
                    Stroke::new(1.0, palette.border_subtle),
                );
                ui.add_space(SPACING.s3);

                body(ui, &mut control);

                ui.add_space(SPACING.s4);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    actions(ui, &mut control);
                });
            });

        control.close
    }
}

/// Turn-key cancel / confirm dialog. Use this when the body is a single
/// short paragraph; fall back to [`Dialog`] for richer content.
///
/// The confirm button is right-most (per platform conventions: see
/// `GUIDE.md`); destructive actions use the danger variant. Default
/// confirm/cancel labels are localized through [`crate::set_locale`]; pass
/// custom strings via [`Self::confirm_label`] / [`Self::cancel_label`] when
/// you want bespoke wording (e.g. `"Delete"` / `"Keep"`).
///
/// Returns `Some(true)` on confirm, `Some(false)` on cancel, `None` while
/// the dialog is still open.
pub struct ConfirmDialog<'a> {
    title: &'a str,
    body: &'a str,
    confirm_label: Option<&'a str>,
    cancel_label: Option<&'a str>,
    danger: bool,
}

impl<'a> ConfirmDialog<'a> {
    /// New dialog with a title and body paragraph.
    pub fn new(title: &'a str, body: &'a str) -> Self {
        Self {
            title,
            body,
            confirm_label: None,
            cancel_label: None,
            danger: false,
        }
    }
    /// Style the confirm button as destructive (red).
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
    /// Override the confirm button label.
    pub fn confirm_label(mut self, label: &'a str) -> Self {
        self.confirm_label = Some(label);
        self
    }
    /// Override the cancel button label.
    pub fn cancel_label(mut self, label: &'a str) -> Self {
        self.cancel_label = Some(label);
        self
    }

    /// Render the dialog. Returns:
    /// - `Some(true)` when the user clicked confirm.
    /// - `Some(false)` when the user clicked cancel or the close icon.
    /// - `None` while the dialog is still open.
    pub fn show(self, ctx: &egui::Context) -> Option<bool> {
        use crate::theme::locale::{Key, tr};
        let locale = crate::locale_of(ctx);
        let confirm_label = self
            .confirm_label
            .unwrap_or_else(|| tr(locale, Key::ConfirmDefault));
        let cancel_label = self
            .cancel_label
            .unwrap_or_else(|| tr(locale, Key::CancelDefault));
        let mut outcome: Option<bool> = None;
        let close = Dialog::new(self.title).show(
            ctx,
            |ui, _ctrl| {
                let palette = crate::palette_of(ui.ctx());
                ui.label(
                    egui::RichText::new(self.body)
                        .text_style(egui::TextStyle::Body)
                        .color(palette.text_secondary),
                );
            },
            |ui, ctrl| {
                let confirm = if self.danger {
                    Button::danger(confirm_label)
                } else {
                    Button::primary(confirm_label)
                };
                if ui.add(confirm.size(ButtonSize::Md)).clicked() {
                    outcome = Some(true);
                    ctrl.close();
                }
                ui.add_space(SPACING.s2);
                if ui.add(Button::secondary(cancel_label)).clicked() {
                    outcome = Some(false);
                    ctrl.close();
                }
            },
        );
        // If the user clicked × or pressed Escape (close icon), treat as cancel.
        if outcome.is_none() && close {
            outcome = Some(false);
        }
        outcome
    }
}
