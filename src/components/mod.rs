//! Ready-to-use components built on top of [`crate::Palette`] +
//! [`crate::apply_theme`]. Each component reads the active palette/density
//! from the [`egui::Context`] so you don't have to pass them around.

use egui::{Color32, CornerRadius};

/// Build a [`CornerRadius`] from a pixel value.
pub(crate) fn corner(px: f32) -> CornerRadius {
    CornerRadius::same(px.round().clamp(0.0, 255.0) as u8)
}

/// Multiply the alpha channel of `c` by `factor` (clamped to `[0, 1]`).
pub(crate) fn alpha(c: Color32, factor: f32) -> Color32 {
    let [r, g, b, a] = c.to_array();
    let a = (a as f32 * factor.clamp(0.0, 1.0))
        .round()
        .clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

mod alert;
mod button;
mod card;
mod data;
mod dialog;
mod feedback;
mod header;
mod input;
mod menu;
mod nav;
mod section;
mod select;
mod stat;
mod status;
mod switch;
mod toast;
mod tooltip;

pub use alert::{Alert, Level};
pub use button::{Button, ButtonSize, ButtonVariant, IconButton};
pub use card::{Card, EmptyState};
pub use data::{KeyValue, LogLevel, LogLine, Skeleton};
pub use dialog::{ConfirmDialog, Dialog, DialogControl};
pub use feedback::{ProgressBar, Spinner};
pub use header::PageHeader;
pub use input::InputField;
pub use menu::{MenuItem, SubMenu};
pub use nav::{Breadcrumb, NavItem, Tabs};
pub use section::{CodeBlock, Section};
pub use select::SelectField;
pub use stat::{Stat, Trend};
pub use status::{Badge, BadgeTone, Kbd, StatusDot, StatusLevel, Tag};
pub use switch::Switch;
pub use toast::{Toast, Toasts};
pub use tooltip::{TooltipExt, tooltip};
