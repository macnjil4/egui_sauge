//! [`SelectField`] — a labelled wrapper around [`egui::ComboBox`] with the
//! same error/helper presentation as [`crate::components::InputField`].

use egui::{FontId, TextStyle, Ui};

use crate::{SPACING, palette_of};

/// [`egui::ComboBox`] with label / helper / error rows.
pub struct SelectField<'a> {
    id: &'a str,
    label: Option<&'a str>,
    helper: Option<&'a str>,
    error: Option<&'a str>,
    width: Option<f32>,
}

impl<'a> SelectField<'a> {
    /// New select bound to `id` (the unique id used by the combo popup).
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            label: None,
            helper: None,
            error: None,
            width: None,
        }
    }
    /// Add a label above.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    /// Helper text below.
    pub fn helper(mut self, helper: &'a str) -> Self {
        self.helper = Some(helper);
        self
    }
    /// Error text below (takes precedence over helper).
    pub fn error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }
    /// Fixed width.
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    /// Render the select. `current` is the currently selected label shown in
    /// the closed combo. `options` are (value, label) pairs; the selected
    /// value (if any) is returned via `selection`.
    pub fn show<T: Clone + PartialEq>(
        self,
        ui: &mut Ui,
        selection: &mut T,
        current_label: &str,
        options: impl IntoIterator<Item = (T, &'a str)>,
    ) -> egui::Response {
        let palette = palette_of(ui.ctx());
        let width = self.width.unwrap_or(220.0);

        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                if let Some(label) = self.label {
                    ui.label(
                        egui::RichText::new(label)
                            .font(FontId::new(12.0, egui::FontFamily::Proportional))
                            .color(palette.text_secondary),
                    );
                    ui.add_space(4.0);
                }
                let resp = egui::ComboBox::from_id_salt(self.id)
                    .selected_text(current_label)
                    .width(width)
                    .show_ui(ui, |ui| {
                        for (value, label) in options {
                            ui.selectable_value(selection, value, label);
                        }
                    })
                    .response;

                if let Some(err) = self.error {
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(err)
                            .text_style(TextStyle::Small)
                            .color(palette.error),
                    );
                } else if let Some(help) = self.helper {
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(help)
                            .text_style(TextStyle::Small)
                            .color(palette.text_tertiary),
                    );
                }
                ui.add_space(SPACING.s1);
                resp
            },
        )
        .inner
    }
}
