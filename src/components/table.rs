//! Typed [`Table`] of `T` rows with custom cell rendering, optional
//! sortable columns, optional row selection, zebra striping, and a
//! density-aware row height.

use std::collections::HashSet;

use egui::{FontId, Rect, Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, vec2};

use super::{alpha, corner};
use crate::{Density, Icon, RADIUS, SPACING, density_of, palette_of};

// -- Column alignment ------------------------------------------------------

/// Cell horizontal alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnAlign {
    /// Default — most columns.
    #[default]
    Left,
    /// Numeric columns, IDs, durations.
    Right,
    /// Status icons, single chips.
    Center,
}

// -- Sort state ------------------------------------------------------------

/// Which column the table is sorted by, ascending or descending.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SortState {
    /// Index into the column list (0-based).
    pub column: usize,
    /// `true` for ascending, `false` for descending.
    pub ascending: bool,
}

impl SortState {
    /// Sort ascending on column `column`.
    pub fn asc(column: usize) -> Self {
        Self {
            column,
            ascending: true,
        }
    }
    /// Sort descending on column `column`.
    pub fn desc(column: usize) -> Self {
        Self {
            column,
            ascending: false,
        }
    }
}

// -- Column ----------------------------------------------------------------

type CellFn<'a, T> = Box<dyn Fn(&mut Ui, &T) + 'a>;

/// One table column.
pub struct Column<'a, T> {
    header: &'a str,
    align: ColumnAlign,
    cell: CellFn<'a, T>,
    sortable: bool,
    width: Option<f32>,
}

impl<'a, T> Column<'a, T> {
    /// Convert this column to right-aligned (numbers, durations, IDs).
    pub fn align_right(mut self) -> Self {
        self.align = ColumnAlign::Right;
        self
    }
    /// Center the cell content (status chips, icons).
    pub fn align_center(mut self) -> Self {
        self.align = ColumnAlign::Center;
        self
    }
    /// Make the column header clickable to toggle sorting on it. The
    /// caller is responsible for re-ordering its data when the
    /// [`Table`]'s `sort` reference changes.
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }
    /// Pin the column width (default: distributed evenly).
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
}

// -- Table -----------------------------------------------------------------

/// Typed table over a slice of `T`.
pub struct Table<'a, T> {
    items: &'a [T],
    columns: Vec<Column<'a, T>>,
    selection: Option<&'a mut HashSet<usize>>,
    sort: Option<&'a mut Option<SortState>>,
    zebra: bool,
    on_row_clicked: Option<Box<dyn FnMut(usize) + 'a>>,
}

impl<'a, T> Table<'a, T> {
    /// New table over `items`. Add columns with [`Self::column`] /
    /// [`Self::column_text`].
    pub fn new(items: &'a [T]) -> Self {
        Self {
            items,
            columns: Vec::new(),
            selection: None,
            sort: None,
            zebra: true,
            on_row_clicked: None,
        }
    }

    /// Add a column whose cell is rendered by the closure
    /// `cell(ui, row)`. Use [`crate::components::Badge`], `ui.label`,
    /// or any widget inside.
    ///
    /// ```ignore
    /// table = table.column("Status", |ui, row| {
    ///     ui.add(egui_sauge::components::Badge::new("Active").tone(BadgeTone::Success));
    /// });
    /// ```
    pub fn column<F>(mut self, header: &'a str, cell: F) -> Self
    where
        F: Fn(&mut Ui, &T) + 'a,
    {
        self.columns.push(Column {
            header,
            align: ColumnAlign::Left,
            cell: Box::new(cell),
            sortable: false,
            width: None,
        });
        self
    }

    /// Convenience: a text column whose cell is `to_text(row)`.
    pub fn column_text<F>(mut self, header: &'a str, to_text: F) -> Self
    where
        F: Fn(&T) -> String + 'a,
    {
        self.columns.push(Column {
            header,
            align: ColumnAlign::Left,
            cell: Box::new(move |ui, row| {
                let palette = palette_of(ui.ctx());
                ui.label(
                    egui::RichText::new(to_text(row))
                        .text_style(TextStyle::Body)
                        .color(palette.text_primary),
                );
            }),
            sortable: false,
            width: None,
        });
        self
    }

    /// Mutate the last-added column (used by [`Self::column`] /
    /// [`Self::column_text`]) so you can chain `.align_right()` etc. on
    /// the table builder itself.
    pub fn last(mut self, cfg: impl FnOnce(Column<'a, T>) -> Column<'a, T>) -> Self {
        if let Some(col) = self.columns.pop() {
            self.columns.push(cfg(col));
        }
        self
    }

    /// Bind a selection set (row indices). Adds a leading checkbox
    /// column.
    pub fn selectable(mut self, selection: &'a mut HashSet<usize>) -> Self {
        self.selection = Some(selection);
        self
    }

    /// Bind the active sort state. Pass `&mut None` to start unsorted;
    /// the user is responsible for actually re-ordering its data when
    /// the value changes.
    pub fn sort(mut self, sort: &'a mut Option<SortState>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Disable the alternating row background.
    pub fn no_zebra(mut self) -> Self {
        self.zebra = false;
        self
    }

    /// Callback invoked with the row index when the user clicks on a
    /// row body (anywhere except interactive cells).
    pub fn on_row_clicked<F>(mut self, on_clicked: F) -> Self
    where
        F: FnMut(usize) + 'a,
    {
        self.on_row_clicked = Some(Box::new(on_clicked));
        self
    }

    /// Render the table.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = palette_of(ui.ctx());
        let density = density_of(ui.ctx());
        let row_height = match density {
            Density::Spacious => 44.0,
            Density::Comfortable => 36.0,
            Density::Compact => 28.0,
        };
        let header_height = match density {
            Density::Spacious => 40.0,
            Density::Comfortable => 32.0,
            Density::Compact => 26.0,
        };

        let Self {
            items,
            columns,
            selection,
            sort,
            zebra,
            mut on_row_clicked,
        } = self;

        let total_width = ui.available_width();
        let checkbox_w = if selection.is_some() { 32.0 } else { 0.0 };
        // Distribute remaining width across columns.
        let fixed_w: f32 = columns.iter().filter_map(|c| c.width).sum();
        let flexible_count = columns.iter().filter(|c| c.width.is_none()).count() as f32;
        let flexible_per_col = if flexible_count > 0.0 {
            ((total_width - checkbox_w - fixed_w) / flexible_count).max(60.0)
        } else {
            0.0
        };
        let col_widths: Vec<f32> = columns
            .iter()
            .map(|c| c.width.unwrap_or(flexible_per_col))
            .collect();

        // Take a mutable copy of the current sort so we can mutate during header click handling
        // without holding `self.sort` across the loop.
        let mut sort_local: Option<SortState> = sort.as_deref().copied().flatten();
        let resp = ui
            .vertical(|ui| {
                // Header row.
                let header_rect = ui
                    .horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        if selection.is_some() {
                            ui.allocate_exact_size(vec2(checkbox_w, header_height), Sense::hover());
                        }
                        for (i, col) in columns.iter().enumerate() {
                            let w = col_widths[i];
                            let cell_rect_resp =
                                paint_header_cell(ui, col, w, header_height, sort_local, i);
                            if col.sortable && cell_rect_resp.clicked() {
                                sort_local = Some(match sort_local {
                                    Some(s) if s.column == i => SortState {
                                        column: i,
                                        ascending: !s.ascending,
                                    },
                                    _ => SortState::asc(i),
                                });
                            }
                        }
                    })
                    .response
                    .rect;
                // Header bottom hairline.
                ui.painter().line_segment(
                    [
                        egui::pos2(header_rect.left(), header_rect.bottom()),
                        egui::pos2(header_rect.right(), header_rect.bottom()),
                    ],
                    Stroke::new(1.0, palette.border_default),
                );

                // Hold the selection mutable borrow at most for one row at a time.
                let mut selection_ref = selection;
                for (idx, item) in items.iter().enumerate() {
                    let row_resp = ui
                        .horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            let row_top = ui.cursor().top();
                            let row_left = ui.cursor().left();
                            // Reserve a full row rect for background painting.
                            let row_rect = Rect::from_min_size(
                                egui::pos2(row_left, row_top),
                                vec2(total_width, row_height),
                            );
                            let row_resp =
                                ui.interact(row_rect, ui.id().with(("row", idx)), Sense::click());

                            // Background.
                            let row_bg = if row_resp.hovered() {
                                palette.bg_hover
                            } else if zebra && idx % 2 == 1 {
                                alpha(palette.bg_surface_alt, 0.5)
                            } else {
                                egui::Color32::TRANSPARENT
                            };
                            ui.painter().rect_filled(row_rect, corner(0.0), row_bg);

                            // Selection checkbox column.
                            if let Some(sel) = selection_ref.as_deref_mut() {
                                let checkbox_rect = Rect::from_min_size(
                                    egui::pos2(row_left, row_top),
                                    vec2(checkbox_w, row_height),
                                );
                                let center = checkbox_rect.center();
                                let box_size: f32 = 16.0;
                                let box_rect =
                                    Rect::from_center_size(center, Vec2::splat(box_size));
                                let cb_resp = ui.interact(
                                    box_rect,
                                    ui.id().with(("row_cb", idx)),
                                    Sense::click(),
                                );
                                let on = sel.contains(&idx);
                                let (fill, stroke_color) = if on {
                                    (palette.brand_default, palette.brand_default)
                                } else if cb_resp.hovered() {
                                    (palette.bg_surface, palette.brand_default)
                                } else {
                                    (palette.bg_surface, palette.border_default)
                                };
                                ui.painter().rect(
                                    box_rect,
                                    corner(RADIUS.sm),
                                    fill,
                                    Stroke::new(1.5, stroke_color),
                                    StrokeKind::Inside,
                                );
                                if on {
                                    Icon::Check.paint(
                                        ui.painter(),
                                        box_rect.shrink(2.0),
                                        palette.text_on_brand,
                                    );
                                }
                                if cb_resp.clicked() {
                                    if on {
                                        sel.remove(&idx);
                                    } else {
                                        sel.insert(idx);
                                    }
                                }
                                ui.advance_cursor_after_rect(checkbox_rect);
                            }

                            // Cells.
                            for (i, col) in columns.iter().enumerate() {
                                let w = col_widths[i];
                                let cell_rect = Rect::from_min_size(
                                    egui::pos2(ui.cursor().left(), row_top),
                                    vec2(w, row_height),
                                );
                                paint_cell_align(ui, cell_rect, col.align, |ui| {
                                    (col.cell)(ui, item)
                                });
                            }

                            if row_resp.clicked()
                                && let Some(cb) = on_row_clicked.as_mut()
                            {
                                cb(idx);
                            }
                        })
                        .response;
                    let _ = row_resp;
                }
            })
            .response;
        // Write back the sort state if we had a mutable reference.
        if let Some(s) = sort {
            *s = sort_local;
        }
        resp
    }
}

fn paint_header_cell<T>(
    ui: &mut Ui,
    col: &Column<'_, T>,
    width: f32,
    height: f32,
    current_sort: Option<SortState>,
    col_idx: usize,
) -> Response {
    let palette = palette_of(ui.ctx());
    let pad = SPACING.s2;
    let cell_rect = Rect::from_min_size(
        egui::pos2(ui.cursor().left(), ui.cursor().top()),
        vec2(width, height),
    );
    let sense = if col.sortable {
        Sense::click()
    } else {
        Sense::hover()
    };
    let resp = ui.interact(cell_rect, ui.id().with(("hdr", col_idx)), sense);
    let bg = if col.sortable && resp.hovered() {
        palette.bg_hover
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(cell_rect, corner(0.0), bg);

    // Header text.
    let font = FontId::new(12.0, egui::FontFamily::Proportional);
    let galley = ui
        .painter()
        .layout_no_wrap(col.header.into(), font, palette.text_secondary);

    // Trailing sort chevron if active.
    let active = current_sort.is_some_and(|s| s.column == col_idx);
    let chev_w = if col.sortable { 14.0 } else { 0.0 };
    let inner_w = width - pad * 2.0 - chev_w;
    let text_x = match col.align {
        ColumnAlign::Left => cell_rect.left() + pad,
        ColumnAlign::Center => cell_rect.left() + pad + (inner_w - galley.size().x) / 2.0,
        ColumnAlign::Right => cell_rect.right() - pad - chev_w - galley.size().x,
    };
    ui.painter().galley(
        egui::pos2(text_x, cell_rect.center().y - galley.size().y / 2.0),
        galley,
        palette.text_secondary,
    );
    if col.sortable {
        let chev_rect = Rect::from_min_size(
            egui::pos2(cell_rect.right() - pad - 14.0, cell_rect.center().y - 7.0),
            Vec2::splat(14.0),
        );
        let icon = match current_sort {
            Some(s) if s.column == col_idx => {
                if s.ascending {
                    Icon::ChevronUp
                } else {
                    Icon::ChevronDown
                }
            }
            _ => Icon::ChevronDown,
        };
        let color = if active {
            palette.brand_default
        } else {
            alpha(palette.text_tertiary, 0.6)
        };
        icon.paint(ui.painter(), chev_rect, color);
    }
    ui.advance_cursor_after_rect(cell_rect);
    resp
}

fn paint_cell_align(ui: &mut Ui, rect: Rect, align: ColumnAlign, body: impl FnOnce(&mut Ui)) {
    let pad = SPACING.s2;
    let inner = match align {
        ColumnAlign::Left => Rect::from_min_max(
            egui::pos2(rect.left() + pad, rect.top()),
            egui::pos2(rect.right() - pad, rect.bottom()),
        ),
        ColumnAlign::Right => rect.shrink2(vec2(pad, 0.0)),
        ColumnAlign::Center => rect.shrink2(vec2(pad, 0.0)),
    };
    let layout = match align {
        ColumnAlign::Left => egui::Layout::left_to_right(egui::Align::Center),
        ColumnAlign::Right => egui::Layout::right_to_left(egui::Align::Center),
        ColumnAlign::Center => egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
    };
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(layout));
    body(&mut child);
    ui.advance_cursor_after_rect(rect);
}
