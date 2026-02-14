use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

mod state;
mod util;
mod validate;

use state::{current_group_index, start_row_group, CellState, ColgroupFrame, RowState, TableState};
use util::{attr_u32, attr_value, is, is_row_group};
use validate::validate_table;

#[derive(Default)]
pub struct TableConstraints {
    stack: Vec<TableState>,
}

impl Rule for TableConstraints {
    fn id(&self) -> &'static str {
        "html.table.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name, attrs, span, ..
            } => {
                if is(ctx, name, "table") {
                    self.stack.push(TableState::default());
                    return;
                }
                let Some(state) = self.stack.last_mut() else {
                    return;
                };

                if is_row_group(ctx, name) {
                    start_row_group(state);
                    return;
                }

                if is(ctx, name, "colgroup") {
                    let span_attr_present = ctx.has_attr(attrs, "span");
                    let span_value = attr_u32(ctx, attrs, "span").unwrap_or(0);
                    let span_value = if span_value > 1000 {
                        out.push(Message::new(
                            "html.table.colgroup.span.max",
                            Severity::Error,
                            Category::Html,
                            "The value of the “span” attribute must be less than or equal to 1000.",
                            *span,
                        ));
                        1000
                    } else {
                        span_value
                    };
                    // A non-zero colgroup span counts as column markup.
                    if span_value > 0 {
                        state.has_col_markup = true;
                    }
                    state.colgroup_stack.push(ColgroupFrame {
                        span_attr_present,
                        pending_span: span_value,
                        span: *span,
                    });
                    return;
                }

                if is(ctx, name, "col") {
                    if ctx.format == html_inspector_core::InputFormat::Xhtml
                        && ctx.current_parent().is_some_and(|p| p == "table")
                    {
                        out.push(Message::new(
                            "html.table.col.parent_table",
                            Severity::Error,
                            Category::Html,
                            "Element “col” not allowed as child of “table” in this context.",
                            *span,
                        ));
                        return;
                    }

                    let span_value = attr_u32(ctx, attrs, "span").unwrap_or(1);
                    if span_value > 1000 {
                        out.push(Message::new(
                            "html.table.col.span.max",
                            Severity::Error,
                            Category::Html,
                            "The value of the “span” attribute must be less than or equal to 1000.",
                            *span,
                        ));
                    }

                    // If this col is a child of a colgroup that has a pending span, warn that
                    // the colgroup span is ignored (matches Java TableChecker behavior).
                    if ctx.current_parent().is_some_and(|p| is(ctx, p, "colgroup"))
                        && let Some(frame) = state.colgroup_stack.last_mut() {
                            if frame.span_attr_present {
                                out.push(Message::new(
                                    "html.table.col.not_allowed_in_colgroup_with_span",
                                    Severity::Error,
                                    Category::Html,
                                    "Element “col” not allowed as child of “colgroup” in this context.",
                                    *span,
                                ));
                            }
                            if frame.pending_span > 0 {
                                out.push(Message::new(
                                    "html.table.colgroup.span.ignored",
                                    Severity::Warning,
                                    Category::Html,
                                    format!(
                                        "A col element causes a span attribute with value {} to be ignored on the parent colgroup.",
                                        frame.pending_span
                                    ),
                                    frame.span.or(*span),
                                ));
                                frame.pending_span = 0;
                            }
                        }
                    state.has_col_markup = true;
                    state.col_markup_count =
                        state.col_markup_count.saturating_add(span_value.min(1000));
                    return;
                }

                if is(ctx, name, "tr") {
                    state.current_row = Some(RowState::default());
                    return;
                }

                if is(ctx, name, "th") || is(ctx, name, "td") {
                    let Some(row) = state.current_row.as_mut() else {
                        return;
                    };

                    if let Some(colspan_raw) = attr_value(ctx, attrs, "colspan")
                        && colspan_raw.trim().parse::<u32>().ok() == Some(0) {
                            let elem = if is(ctx, name, "td") { "td" } else { "th" };
                            out.push(Message::new(
                                "html.table.cell.colspan.zero",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "Bad value “0” for attribute “colspan” on element “{elem}”."
                                ),
                                *span,
                            ));
                        }

                    let colspan = attr_u32(ctx, attrs, "colspan").unwrap_or(1).max(1);
                    if colspan > 1000 {
                        out.push(Message::new(
                            "html.table.cell.colspan.max",
                            Severity::Error,
                            Category::Html,
                            "The value of the “colspan” attribute must be less than or equal to 1000.",
                            *span,
                        ));
                    }

                    let rowspan = attr_u32(ctx, attrs, "rowspan").unwrap_or(1).max(1);
                    if rowspan > 65534 {
                        out.push(Message::new(
                            "html.table.cell.rowspan.max",
                            Severity::Error,
                            Category::Html,
                            "The value of the “rowspan” attribute must be less than or equal to 65534.",
                            *span,
                        ));
                    }

                    if is(ctx, name, "th")
                        && let Some(id) = attr_value(ctx, attrs, "id")
                            && !id.is_empty() {
                                state.th_ids.insert(id.to_string());
                            }

                    if let Some(headers_raw) = attr_value(ctx, attrs, "headers") {
                        let ids: Vec<String> = headers_raw
                            .split_ascii_whitespace()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect();
                        if !ids.is_empty() {
                            state.headers_checks.push((ids, *span));
                        }
                    }

                    row.cells.push(CellState {
                        colspan: colspan.min(1000),
                        rowspan: rowspan.min(65534),
                        span: *span,
                    });
                }
            }
            ParseEvent::EndTag { name, span } => {
                if is(ctx, name, "tr") {
                    let Some(state) = self.stack.last_mut() else {
                        return;
                    };
                    if let Some(row) = state.current_row.take() {
                        let group_idx = current_group_index(state);
                        state.groups[group_idx].rows.push(row);
                    }
                    return;
                }

                if is_row_group(ctx, name) {
                    let Some(state) = self.stack.last_mut() else {
                        return;
                    };
                    state.current_group_index = None;
                    return;
                }

                if is(ctx, name, "colgroup") {
                    let Some(state) = self.stack.last_mut() else {
                        return;
                    };
                    if let Some(frame) = state.colgroup_stack.pop()
                        && frame.pending_span > 0 {
                            state.has_col_markup = true;
                            state.col_markup_count = state
                                .col_markup_count
                                .saturating_add(frame.pending_span.min(1000));
                        }
                    return;
                }

                if is(ctx, name, "table") {
                    let Some(state) = self.stack.pop() else {
                        return;
                    };
                    validate_table(&state, out, *span, ctx.format);
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::validate::missing_column_range;
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet, ValidationContext};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn missing_column_range_treats_out_of_bounds_as_missing() {
        assert_eq!(missing_column_range(&[true], 1), None);
        assert_eq!(missing_column_range(&[false, true], 2), Some((0, 0)));
        assert_eq!(missing_column_range(&[true], 3), Some((1, 2)));
    }

    #[test]
    fn missing_column_range_returns_first_missing_run_only() {
        assert_eq!(
            missing_column_range(&[true, false, true, false, false], 5),
            Some((1, 1))
        );
    }

    #[test]
    fn missing_column_range_with_zero_width_is_none() {
        assert_eq!(missing_column_range(&[], 0), None);
        assert_eq!(missing_column_range(&[false], 0), None);
    }

    #[test]
    fn missing_column_range_with_empty_flags_and_positive_width_is_full_range() {
        assert_eq!(missing_column_range(&[], 1), Some((0, 0)));
        assert_eq!(missing_column_range(&[], 3), Some((0, 2)));
    }

    #[test]
    fn missing_column_range_with_full_coverage_is_none() {
        assert_eq!(missing_column_range(&[true, true, true], 3), None);
        assert_eq!(missing_column_range(&[true, true, true], 2), None);
    }

    #[test]
    fn missing_column_range_respects_max_width() {
        assert_eq!(missing_column_range(&[true, false], 1), None);
        assert_eq!(missing_column_range(&[false, true], 1), Some((0, 0)));
    }

    #[test]
    fn xhtml_col_direct_child_of_table_is_disallowed() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Xhtml, "<table><col/></table>").unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.col.parent_table"));
    }

    #[test]
    fn headers_reference_missing_th_emits_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tr><td headers=a>1</td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.headers.missing_th"));
    }

    #[test]
    fn row_without_cells_and_rowspan_past_group_are_errors() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tbody><tr></tr><tr><td rowspan=2>1</td></tr></tbody></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.no_cells"));
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.cell.spans_past_row_group"));
    }

    #[test]
    fn row_with_only_rowspan_continuations_is_an_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tbody><tr><td rowspan=2>1</td></tr><tr></tr></tbody></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.no_cells"));
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.table.cell.spans_past_row_group"));
    }

    #[test]
    fn colspan_establishes_missing_columns_without_starts() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tr><td colspan=2></td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.column.no_starting_cell"));
    }

    #[test]
    fn missing_column_range_finds_first_contiguous_run() {
        assert_eq!(missing_column_range(&[true, true, true], 3), None);
        assert_eq!(missing_column_range(&[false, false, true], 3), Some((0, 1)));
        assert_eq!(
            missing_column_range(&[true, false, false, false], 4),
            Some((1, 3))
        );
    }

    #[test]
    fn missing_column_range_treats_beyond_len_as_missing() {
        assert_eq!(missing_column_range(&[true, true], 4), Some((2, 3)));
        assert_eq!(missing_column_range(&[], 2), Some((0, 1)));
    }

    #[test]
    fn colspan_zero_is_invalid() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tr><td colspan=0></td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.cell.colspan.zero"));
    }

    #[test]
    fn tfoot_row_group_is_tracked() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tfoot><tr><td>1</td></tr></tfoot></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report.messages.is_empty());
    }

    #[test]
    fn th_ids_satisfy_headers_checks() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tr><th id=a>h</th><td headers=a>1</td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.table.headers.missing_th"));
    }

    #[test]
    fn row_width_warnings_without_col_markup_cover_both_directions() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><tr><td colspan=2></td></tr><tr><td colspan=3></td></tr><tr><td></td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.width.exceeds_first_row"));
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.width.less_than_first_row"));
    }

    #[test]
    fn column_markup_mismatch_emits_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><col span=2><tr><td></td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.width.less_than_col_markup"));
    }

    #[test]
    fn colgroup_span_establishes_column_markup_count() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><colgroup span=2></colgroup><tr><td></td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.width.less_than_col_markup"));
    }

    #[test]
    fn colgroup_span_is_ignored_when_col_children_exist() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<table><colgroup span=2><col></colgroup><tr><td></td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.colgroup.span.ignored"));
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.width.less_than_col_markup"));
    }

    #[test]
    fn rule_ignores_unhandled_events() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = TableConstraints::default();
        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
        html_inspector_core::MessageSink::push(
            &mut sink,
            html_inspector_core::Message::new(
                "test.dummy",
                html_inspector_core::Severity::Info,
                html_inspector_core::Category::Html,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }

    #[test]
    fn xhtml_row_without_cells_uses_implicit_row_group_message_variant() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Xhtml, "<table><tr></tr></table>").unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        let msg = report
            .messages
            .iter()
            .find(|m| m.code == "html.table.row.no_cells")
            .expect("expected row.no_cells error");
        assert!(msg.message.contains("implicit row group"));
    }

    #[test]
    fn xhtml_headers_attribute_is_case_sensitive_and_is_parsed() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<table><tr><td headers=a>1</td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.table.headers.missing_th"));

        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<table><tr><th id=a>h</th><td headers=a>1</td></tr></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.table.headers.missing_th"));
    }

    #[test]
    fn xhtml_thead_row_group_is_tracked() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<table><thead><tr><td>1</td></tr></thead></table>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TableConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report.messages.is_empty());
    }
}
