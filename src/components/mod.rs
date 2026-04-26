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

mod accordion;
mod activity_bar;
mod alert;
mod avatar;
mod button;
mod card;
mod checkbox;
mod command;
mod data;
mod dialog;
mod diff;
mod drawer;
mod editor_tabs;
mod feedback;
mod gauge;
mod header;
mod input;
mod menu;
mod nav;
mod number;
mod pagination;
mod radio;
mod section;
mod select;
mod sparkline;
mod splitter;
mod stat;
mod status;
mod status_bar;
mod switch;
mod table;
mod timeline;
mod toast;
mod tooltip;
mod tree;
mod workbench;

pub use accordion::Accordion;
pub use activity_bar::{ActivityBar, ActivityItem};
pub use alert::{Alert, Level};
pub use avatar::{Avatar, AvatarGroup, AvatarSize};
pub use button::{Button, ButtonSize, ButtonVariant, IconButton};
pub use card::{Card, EmptyState};
pub use checkbox::Checkbox;
pub use command::{CommandAction, CommandPalette};
pub use data::{KeyValue, LogLevel, LogLine, Skeleton};
pub use dialog::{ConfirmDialog, Dialog, DialogControl};
pub use diff::{DiffKind, DiffLine, DiffView};
pub use drawer::Drawer;
pub use editor_tabs::{EditorTab, EditorTabAction, EditorTabs};
pub use feedback::{ProgressBar, Spinner};
pub use gauge::{Gauge, GaugeShape, Thresholds};
pub use header::PageHeader;
pub use input::InputField;
pub use menu::{MenuItem, SubMenu};
pub use nav::{Breadcrumb, NavItem, Tabs};
pub use number::NumberField;
pub use pagination::Pagination;
pub use radio::{RadioGroup, RadioLayout, RadioOption};
pub use section::{CodeBlock, Section};
pub use select::SelectField;
pub use sparkline::{Sparkline, SparklineKind};
pub use splitter::{Splitter, SplitterSide};
pub use stat::{Stat, Trend};
pub use status::{Badge, BadgeTone, Kbd, StatusDot, StatusLevel, Tag};
pub use status_bar::StatusBar;
pub use switch::Switch;
pub use table::{Column, ColumnAlign, SortState, Table};
pub use timeline::{Timeline, TimelineEvent};
pub use toast::{Toast, Toasts};
pub use tooltip::{TooltipExt, tooltip};
pub use tree::{TreeNode, TreeView};
pub use workbench::Workbench;
