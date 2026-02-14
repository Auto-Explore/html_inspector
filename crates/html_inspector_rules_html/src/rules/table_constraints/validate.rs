use html_inspector::{Category, Message, MessageSink, Severity, Span};

use super::state::TableState;

pub(super) fn validate_table(
    state: &TableState,
    out: &mut dyn MessageSink,
    table_span: Option<Span>,
    format: html_inspector::InputFormat,
) {
    let col_markup = state.has_col_markup.then_some(state.col_markup_count);
    let mut pending_warnings = Vec::new();
    let mut reported_overlap = false;

    // Validate headers references.
    for (ids, span) in &state.headers_checks {
        for id in ids {
            if !state.th_ids.contains(id) {
                out.push(Message::new(
                    "html.table.headers.missing_th",
                    Severity::Error,
                    Category::Html,
                    format!("The “headers” attribute on the element “td” refers to the ID “{id}”, but there is no “th” element with that ID in the same table."),
                    *span,
                ));
                break;
            }
        }
    }

    // Compute row widths and starting columns per row group.
    let mut col_has_start = Vec::new();
    let mut max_width: u32 = 0;
    let mut first_row_width: Option<u32> = None;

    for group in &state.groups {
        let mut rowspan_remaining = Vec::new();
        for (row_index, row) in group.rows.iter().enumerate() {
            let mut col = 0usize;

            // Columns occupied by rowspans from earlier rows contribute to row width.
            let occupied_width = rowspan_remaining
                .iter()
                .rposition(|&n| n > 0)
                .map(|i| i as u32 + 1)
                .unwrap_or(0);
            let mut row_width = occupied_width;

            for cell in &row.cells {
                while col < rowspan_remaining.len() && rowspan_remaining[col] > 0 {
                    col += 1;
                }
                if col_has_start.len() <= col {
                    col_has_start.resize(col + 1, false);
                }
                col_has_start[col] = true;

                let colspan = cell.colspan.max(1) as usize;
                let rowspan = cell.rowspan.max(1);
                let end_col = col.saturating_add(colspan - 1);

                if !reported_overlap {
                    let mut overlaps = false;
                    for c in col..=end_col {
                        if c < rowspan_remaining.len() && rowspan_remaining[c] > 0 {
                            overlaps = true;
                            break;
                        }
                    }
                    if overlaps {
                        reported_overlap = true;
                        out.push(Message::new(
                            "html.table.cell.overlapped",
                            Severity::Error,
                            Category::Html,
                            "Table cell is overlapped by later table cell.",
                            cell.span.or(table_span),
                        ));
                    }
                }

                row_width = row_width.max(end_col as u32 + 1);
                if rowspan_remaining.len() <= end_col {
                    rowspan_remaining.resize(end_col + 1, 0);
                }
                if rowspan > 1 {
                    let rem = rowspan;
                    for remaining in &mut rowspan_remaining[col..=end_col] {
                        *remaining = (*remaining).max(rem);
                    }
                }
                col = end_col + 1;
            }

            if row.cells.is_empty() {
                out.push(Message::new(
                    "html.table.row.no_cells",
                    Severity::Error,
                    Category::Html,
                    match format {
                        html_inspector::InputFormat::Xhtml => format!(
                            "Row {} of an implicit row group has no cells beginning on it.",
                            row_index + 1
                        ),
                        html_inspector::InputFormat::Html => format!(
                            "Row {} of a row group established by a “tbody” element has no cells beginning on it.",
                            row_index + 1
                        ),
                    },
                    table_span,
                ));
            }

            max_width = max_width.max(row_width);
            if first_row_width.is_none() {
                first_row_width = Some(row_width);
            }

            if let Some(markup) = col_markup {
                if row_width < markup {
                    out.push(Message::new(
                        "html.table.row.width.less_than_col_markup",
                        Severity::Error,
                        Category::Html,
                        format!(
                            "A table row was {} columns wide, which is less than the column count established using column markup ({}).",
                            row_width, markup
                        ),
                        table_span,
                    ));
                } else if row_width > markup {
                    out.push(Message::new(
                        "html.table.row.width.exceeds_col_markup",
                        Severity::Error,
                        Category::Html,
                        format!(
                            "A table row was {} columns wide and exceeded the column count established using column markup ({}).",
                            row_width, markup
                        ),
                        table_span,
                    ));
                }
            } else if let Some(first) = first_row_width {
                if row_width > first {
                    pending_warnings.push(Message::new(
                        "html.table.row.width.exceeds_first_row",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "A table row was {} columns wide and exceeded the column count established by the first row ({}).",
                            row_width, first
                        ),
                        table_span,
                    ));
                } else if row_width < first && row_index > 0 {
                    pending_warnings.push(Message::new(
                        "html.table.row.width.less_than_first_row",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "A table row was {} columns wide, which is less than the column count established by the first row ({}).",
                            row_width, first
                        ),
                        table_span,
                    ));
                }
            }

            // Advance to next row in this group.
            for rem in &mut rowspan_remaining {
                if *rem > 0 {
                    *rem -= 1;
                }
            }
        }

        if rowspan_remaining.iter().any(|&n| n > 0) {
            out.push(Message::new(
                "html.table.cell.spans_past_row_group",
                Severity::Error,
                Category::Html,
                "Table cell spans past the end of its row group established by a “tbody” element; clipped to the end of the row group.",
                table_span,
            ));
        }
    }

    // Columns established by a spanning cell but with no cell beginning in them.
    if let Some((start, end)) = missing_column_range(&col_has_start, max_width as usize) {
        if start == end {
            out.push(Message::new(
                "html.table.column.no_starting_cell",
                Severity::Error,
                Category::Html,
                format!(
                    "Table column {} established by element “td” has no cells beginning in it.",
                    start + 1
                ),
                table_span,
            ));
        } else {
            out.push(Message::new(
                "html.table.columns.no_starting_cells",
                Severity::Error,
                Category::Html,
                format!(
                    "Table columns in range {}…{} established by element “td” have no cells beginning in them.",
                    start + 1,
                    end + 1
                ),
                table_span,
            ));
        }
    }

    for w in pending_warnings {
        out.push(w);
    }
}

pub(super) fn missing_column_range(
    col_has_start: &[bool],
    max_width: usize,
) -> Option<(usize, usize)> {
    let mut i = 0usize;
    while i < max_width && col_has_start.get(i) == Some(&true) {
        i += 1;
    }
    if i == max_width {
        return None;
    }
    let start = i;
    while i < max_width && col_has_start.get(i) != Some(&true) {
        i += 1;
    }
    Some((start, i - 1))
}
