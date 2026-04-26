//! [`Accordion`] — themed wrapper around [`egui::CollapsingHeader`].
//!
//! Use for FAQ-style content, long settings pages, or grouped form
//! sections that the user can fold away.

use egui::{FontId, Stroke, TextStyle, Ui};

use super::corner;
use crate::{Icon, RADIUS, SPACING, palette_of};

/// Disclosure section with a header row and a collapsible body.
pub struct Accordion<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    icon: Option<Icon>,
    open_default: bool,
    id: egui::Id,
}

impl<'a> Accordion<'a> {
    /// New collapsed accordion titled `title`.
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            icon: None,
            open_default: false,
            id: egui::Id::new(("sauge_accordion", title)),
        }
    }
    /// Add a subtitle line under the title.
    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }
    /// Leading icon in the header.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Open by default the first time it's rendered.
    pub fn open(mut self) -> Self {
        self.open_default = true;
        self
    }
    /// Override the persistent state id (rare — for stacking duplicates).
    pub fn id(mut self, id: egui::Id) -> Self {
        self.id = id;
        self
    }

    /// Render the accordion. Returns the [`egui::CollapsingResponse`] of
    /// the inner body (use `.fully_open()` to detect open state).
    pub fn show<R>(
        self,
        ui: &mut Ui,
        body: impl FnOnce(&mut Ui) -> R,
    ) -> egui::CollapsingResponse<R> {
        let palette = palette_of(ui.ctx());
        let frame = egui::Frame::default()
            .fill(palette.bg_surface)
            .stroke(Stroke::new(1.0, palette.border_subtle))
            .corner_radius(corner(RADIUS.md))
            .inner_margin(egui::Margin::symmetric(SPACING.s3 as i8, SPACING.s2 as i8));
        // We can't easily pierce CollapsingHeader's icon, so we just style
        // the surrounding frame and rely on its default chevron.
        let mut header = egui::CollapsingHeader::new(format_header(
            self.title,
            self.subtitle,
            &palette,
            self.icon,
        ))
        .id_salt(self.id)
        .default_open(self.open_default);
        if self.icon.is_some() {
            // header already includes the icon as a glyph in the text;
            // nothing else to do.
        }
        let mut inner: Option<egui::CollapsingResponse<R>> = None;
        frame.show(ui, |ui| {
            header = std::mem::replace(
                &mut header,
                egui::CollapsingHeader::new("placeholder").id_salt(self.id),
            );
            inner = Some(header.show(ui, body));
        });
        inner.expect("CollapsingHeader::show always runs")
    }
}

fn format_header(
    title: &str,
    subtitle: Option<&str>,
    palette: &crate::Palette,
    icon: Option<Icon>,
) -> egui::WidgetText {
    use egui::text::LayoutJob;
    let mut job = LayoutJob::default();
    if let Some(icon) = icon
        && let Some(cp) = icon.glyph()
    {
        job.append(
            cp,
            0.0,
            egui::TextFormat {
                font_id: FontId::new(15.0, egui::FontFamily::Proportional),
                color: palette.text_secondary,
                ..Default::default()
            },
        );
        job.append(
            "  ",
            0.0,
            egui::TextFormat {
                font_id: FontId::new(15.0, egui::FontFamily::Proportional),
                color: palette.text_secondary,
                ..Default::default()
            },
        );
    }
    job.append(
        title,
        0.0,
        egui::TextFormat {
            font_id: FontId::new(15.0, egui::FontFamily::Proportional),
            color: palette.text_primary,
            ..Default::default()
        },
    );
    if let Some(sub) = subtitle {
        job.append(
            "    ",
            0.0,
            egui::TextFormat {
                font_id: FontId::new(15.0, egui::FontFamily::Proportional),
                color: palette.text_secondary,
                ..Default::default()
            },
        );
        job.append(
            sub,
            0.0,
            egui::TextFormat {
                font_id: FontId::new(12.0, egui::FontFamily::Proportional),
                color: palette.text_secondary,
                ..Default::default()
            },
        );
    }
    let _ = TextStyle::Body;
    egui::WidgetText::LayoutJob(job.into())
}
