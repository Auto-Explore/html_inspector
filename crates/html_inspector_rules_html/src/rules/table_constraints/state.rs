use html_inspector_core::Span;
use rustc_hash::FxHashSet;

#[derive(Default)]
pub(super) struct TableState {
    pub(super) col_markup_count: u32,
    pub(super) has_col_markup: bool,
    pub(super) colgroup_stack: Vec<ColgroupFrame>,
    pub(super) implicit_group_index: Option<usize>,
    pub(super) current_group_index: Option<usize>,
    pub(super) groups: Vec<RowGroup>,
    pub(super) current_row: Option<RowState>,
    pub(super) th_ids: FxHashSet<String>,
    pub(super) headers_checks: Vec<(Vec<String>, Option<Span>)>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ColgroupFrame {
    pub(super) span_attr_present: bool,
    pub(super) pending_span: u32,
    pub(super) span: Option<Span>,
}

pub(super) struct RowGroup {
    pub(super) rows: Vec<RowState>,
}

#[derive(Default)]
pub(super) struct RowState {
    pub(super) cells: Vec<CellState>,
}

pub(super) struct CellState {
    pub(super) colspan: u32,
    pub(super) rowspan: u32,
    pub(super) span: Option<Span>,
}

pub(super) fn start_row_group(state: &mut TableState) {
    let idx = state.groups.len();
    state.groups.push(RowGroup { rows: Vec::new() });
    state.current_group_index = Some(idx);
}

pub(super) fn current_group_index(state: &mut TableState) -> usize {
    if let Some(idx) = state.current_group_index {
        return idx;
    }
    if let Some(idx) = state.implicit_group_index {
        return idx;
    }
    let idx = state.groups.len();
    state.groups.push(RowGroup { rows: Vec::new() });
    state.implicit_group_index = Some(idx);
    idx
}
