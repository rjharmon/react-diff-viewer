//! The rows of both views: one row per shown line or pair, and fold rows.
//!
//! Architecture: ARCH-m4dkxzw6hh (DiffViewer). Rows render the engine's output
//! without diffing again.

use std::fmt;

use dioxus::prelude::*;

use crate::consumer_callbacks::{FoldRowLines, LineContent, LineNumberClick};
use crate::diff_analysis_output::{
    ChangeKind, DiffAnalysis, InlineToken, LineSide, PairedLineEntry,
};
use crate::diff_viewer::DiffView;
use crate::fold_planning::{Fold, PlannedRow};
use crate::line_id::LineId;
use crate::styling_hooks::*;

/// A planned row's key: `entry` or `fold`, then the line ids of the entry it
/// shows or first hides, as in `entry L-3 R-3`, `entry R-4`, or `fold L-1 R-1`.
///
/// REQT-zen8fyae28 (Rendered content identity): line numbers identify a row
/// while the compared texts stay the same.
pub(crate) struct RowKey<'a> {
    diff: &'a DiffAnalysis,
    planned: PlannedRow,
}

impl<'a> RowKey<'a> {
    pub(crate) fn new(diff: &'a DiffAnalysis, planned: PlannedRow) -> Self {
        Self { diff, planned }
    }
}

impl fmt::Display for RowKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (kind, entry) = match self.planned {
            PlannedRow::Entry(position) => ("entry", &self.diff.entries[position]),
            PlannedRow::Fold(fold) => ("fold", &self.diff.entries[fold.start]),
        };
        let old = entry.old.as_ref().map(|side| LineId::Old(side.number));
        let new = entry.new.as_ref().map(|side| LineId::New(side.number));
        write!(f, "{kind} ")?;
        match (old, new) {
            (Some(old), Some(new)) => write!(f, "{old} {new}"),
            (Some(line_id), None) | (None, Some(line_id)) => write!(f, "{line_id}"),
            (None, None) => Ok(()),
        }
    }
}

/// The choices every row reads, gathered once per render.
pub(crate) struct RowRendering<'a> {
    pub(crate) view: DiffView,
    pub(crate) show_line_numbers: bool,
    pub(crate) highlighted_lines: &'a [LineId],
    pub(crate) on_line_number_click: Option<EventHandler<LineNumberClick>>,
    pub(crate) line_content_renderer: Option<Callback<LineContent, Element>>,
    pub(crate) fold_row_renderer: Option<Callback<FoldRowLines, Element>>,
}

/// One line as a row shows it: its gutters' line ids, marker, text, and chips.
struct ShownLine<'a> {
    /// One slot per gutter column; an empty slot is a gutter with no number.
    /// The split view reads only the first slot.
    gutters: [Option<LineId>; 2],
    /// The side's state class: unchanged, removed, added, or empty.
    state: &'static str,
    /// `-` or `+`, when the line was removed or added.
    marker: &'static str,
    line: Option<&'a LineSide>,
    /// The line's inline-change tokens, on a modified line carrying them.
    tokens: Option<&'a [InlineToken]>,
    /// Each terminator to show as a line ending chip, in side order.
    line_endings: [Option<&'a str>; 2],
    whitespace_change: bool,
}

impl RowRendering<'_> {
    /// Gutter columns per shown line: one in the split view, where each side
    /// numbers its own line, and two inline, one per text.
    fn gutters_per_line(&self) -> usize {
        match self.view {
            DiffView::Split => 1,
            DiffView::Inline => 2,
        }
    }

    /// Gutter cells each shown line actually renders.
    fn rendered_gutters_per_line(&self) -> usize {
        if self.show_line_numbers {
            self.gutters_per_line()
        } else {
            0
        }
    }

    /// Cells per shown line: its gutters, the marker, and the content.
    pub(crate) fn cells_per_line(&self) -> usize {
        self.rendered_gutters_per_line() + 2
    }

    /// Cells across a whole row: two shown lines per split row, one inline.
    fn cells_per_row(&self) -> usize {
        match self.view {
            DiffView::Split => 2 * self.cells_per_line(),
            DiffView::Inline => self.cells_per_line(),
        }
    }

    /// The row or rows showing one entry in the current view.
    pub(crate) fn entry_rows(&self, entry: &PairedLineEntry) -> Element {
        match self.view {
            DiffView::Split => self.split_row(entry),
            DiffView::Inline => self.inline_rows(entry),
        }
    }

    /// REQT-zaens35zqy (Highlighted lines)
    fn is_highlighted(&self, line_ids: &[Option<LineId>]) -> bool {
        line_ids
            .iter()
            .flatten()
            .any(|line_id| self.highlighted_lines.contains(line_id))
    }

    /// REQT-ys3yr5g185 (Split view): old line left, new line right, one pair
    /// per row.
    fn split_row(&self, entry: &PairedLineEntry) -> Element {
        let old = shown_side(entry, Side::Old);
        let new = shown_side(entry, Side::New);
        // Each shown line is checked once; the row is highlighted when either is.
        let old_highlighted = self.is_highlighted(&old.gutters);
        let new_highlighted = self.is_highlighted(&new.gutters);
        let row_highlighted = old_highlighted || new_highlighted;
        let row_state_class = row_state(entry.change);
        rsx! {
            tr {
                class: "{DXDIFF__ROW}",
                class: "{row_state_class}",
                class: if row_highlighted { "{DXDIFF__HIGHLIGHTED}" },
                {self.line_cells(old, old_highlighted)}
                {self.line_cells(new, new_highlighted)}
            }
        }
    }

    /// REQT-0xbgrj9ane (Inline view): one column; a modified line's old text
    /// directly above its new text, and an unchanged line once with both
    /// numbers.
    fn inline_rows(&self, entry: &PairedLineEntry) -> Element {
        if entry.change == ChangeKind::Modified {
            let old = inline_side(entry, Side::Old);
            let new = inline_side(entry, Side::New);
            return rsx! {
                {self.inline_row(entry, old)}
                {self.inline_row(entry, new)}
            };
        }

        let old_side = entry.old.as_ref();
        let new_side = entry.new.as_ref();
        let line = old_side.or(new_side);
        let side = if old_side.is_some() {
            Side::Old
        } else {
            Side::New
        };
        let (state, marker) = line_state(line, side, entry.change);
        let line_endings = match &entry.line_ending_change {
            Some(change) => [Some(change.old.as_str()), Some(change.new.as_str())],
            None => [None, None],
        };
        let shown = ShownLine {
            gutters: [
                old_side.map(|side| LineId::Old(side.number)),
                new_side.map(|side| LineId::New(side.number)),
            ],
            state,
            marker,
            line,
            tokens: None,
            line_endings,
            whitespace_change: entry.whitespace_change,
        };
        self.inline_row(entry, shown)
    }

    fn inline_row(&self, entry: &PairedLineEntry, shown: ShownLine<'_>) -> Element {
        let highlighted = self.is_highlighted(&shown.gutters);
        let row_state_class = row_state(entry.change);
        rsx! {
            tr {
                class: "{DXDIFF__ROW}",
                class: "{row_state_class}",
                class: if highlighted { "{DXDIFF__HIGHLIGHTED}" },
                {self.line_cells(shown, highlighted)}
            }
        }
    }

    /// The gutter, marker, and content cells of one shown line.
    fn line_cells(&self, shown: ShownLine<'_>, highlighted: bool) -> Element {
        let state = shown.state;
        let gutters = &shown.gutters[..self.gutters_per_line()];
        rsx! {
            // REQT-m9r3k5b1ge (Hidden line numbers): gutters show unless the
            // consumer hides them.
            if self.show_line_numbers {
                for line_id in gutters.iter().copied() {
                    {self.gutter(line_id, state, highlighted)}
                }
            }
            td {
                class: "{DXDIFF__CHANGE_MARKER}",
                class: "{state}",
                class: if highlighted { "{DXDIFF__HIGHLIGHTED}" },
                // REQT-wjjyqjnjs6 (Change markers)
                pre { "{shown.marker}" }
            }
            td {
                class: "{DXDIFF__CONTENT}",
                class: "{state}",
                class: if highlighted { "{DXDIFF__HIGHLIGHTED}" },
                pre {
                    {self.line_text(&shown)}
                    // REQT-4zyjfjrhd3 (Line ending chips)
                    for (position, terminator) in shown.line_endings.iter().flatten().enumerate() {
                        Fragment { key: "{position}",
                            // An inline unchanged line carries both sides'
                            // chips: old, an arrow, then new.
                            if position == 1 {
                                span {
                                    class: "{DXDIFF__LINE_ENDING_ARROW}",
                                    class: "{state}",
                                    aria_label: "changed to",
                                    "→"
                                }
                            }
                            span {
                                class: "{DXDIFF__LINE_ENDING_CHIP}",
                                class: "{state}",
                                "{escaped_terminator(terminator)}"
                            }
                        }
                    }
                    // REQT-27a1gxq15f (Whitespace chips)
                    if shown.whitespace_change {
                        span { class: "{DXDIFF__WHITESPACE_CHIP}", class: "{state}", "WS" }
                    }
                }
            }
        }
    }

    fn gutter(&self, line_id: Option<LineId>, state: &'static str, highlighted: bool) -> Element {
        // REQT-bqm9w6v4ms (Line number clicks): each click on a line number
        // calls the consumer's handler with the line's id. Only a gutter with a
        // number, in a viewer given a handler, listens for clicks.
        let click_target = self.on_line_number_click.zip(line_id);
        let number = line_id.map(|line_id| match line_id {
            LineId::Old(number) | LineId::New(number) => number,
        });
        rsx! {
            if let Some((handler, line_id)) = click_target {
                td {
                    class: "{DXDIFF__GUTTER}",
                    // REQT-3928hx46s3 (Styling hooks): only this branch answers
                    // a click, so only this branch carries the class the
                    // stylesheet's hover affordance reaches.
                    class: "{DXDIFF__GUTTER_CLICKABLE}",
                    class: "{state}",
                    class: if highlighted { "{DXDIFF__HIGHLIGHTED}" },
                    onclick: move |event| handler.call(LineNumberClick::new(line_id, &event)),
                    pre {
                        if let Some(number) = number {
                            "{number}"
                        }
                    }
                }
            } else {
                td {
                    class: "{DXDIFF__GUTTER}",
                    class: "{state}",
                    class: if highlighted { "{DXDIFF__HIGHLIGHTED}" },
                    pre {
                        if let Some(number) = number {
                            "{number}"
                        }
                    }
                }
            }
        }
    }

    /// A line's text: its inline-change tokens on a modified line carrying
    /// them, otherwise the whole line.
    fn line_text(&self, shown: &ShownLine<'_>) -> Element {
        match (shown.line, shown.tokens) {
            (Some(line), Some(tokens)) => rsx! {
                for (position, token) in tokens.iter().enumerate() {
                    span {
                        key: "{position}",
                        class: "{DXDIFF__INLINE_TOKEN}",
                        class: "{token_state(token.kind)}",
                        {self.rendered_text(LineContent::token(&line.text, token.range.clone()))}
                    }
                }
            },
            (Some(line), None) => self.rendered_text(LineContent::whole_line(&line.text)),
            // REQT-vxtax4x0vs (Custom line content): a side with no line is
            // not sent through the consumer's renderer.
            (None, _) => rsx! {},
        }
    }

    /// REQT-vxtax4x0vs (Custom line content): text goes through the consumer's
    /// renderer when one is supplied, including each inline-change token.
    fn rendered_text(&self, content: LineContent) -> Element {
        match self.line_content_renderer {
            Some(renderer) => renderer.call(content),
            None => rsx! { "{content.text()}" },
        }
    }

    /// REQT-1tdrfvay4q (Fold rows): one row per fold, reading "Expand N lines
    /// ..." unless the consumer renders it.
    pub(crate) fn fold_row(
        &self,
        fold: Fold,
        first_hidden: &PairedLineEntry,
        mut on_expand: impl FnMut() + 'static,
    ) -> Element {
        let fold_row_lines = FoldRowLines {
            count: fold.len,
            first_old_number: first_hidden.old.as_ref().map_or(0, |side| side.number),
            first_new_number: first_hidden.new.as_ref().map_or(0, |side| side.number),
        };
        let gutters = self.rendered_gutters_per_line();
        // The fold row's gutters and marker line up with the first shown line's;
        // its content spans the rest of the row.
        let content_span = self.cells_per_row() - gutters - 1;
        let label = match self.fold_row_renderer {
            Some(renderer) => renderer.call(fold_row_lines),
            None => rsx! { "Expand {fold_row_lines.count} lines ..." },
        };
        rsx! {
            tr {
                class: "{DXDIFF__FOLD_ROW}",
                class: "{DXDIFF__UNCHANGED}",
                for _ in 0..gutters {
                    td { class: "{DXDIFF__GUTTER}", class: "{DXDIFF__UNCHANGED}" }
                }
                td { class: "{DXDIFF__CHANGE_MARKER}", class: "{DXDIFF__UNCHANGED}" }
                td {
                    colspan: "{content_span}",
                    // REQT-v748c7mjr6 (Expanding folds): activating the fold
                    // row reveals the lines it hides.
                    button {
                        class: "{DXDIFF__FOLD_CONTROL}",
                        r#type: "button",
                        onclick: move |_| on_expand(),
                        {label}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Side {
    Old,
    New,
}

fn side_of(entry: &PairedLineEntry, side: Side) -> Option<&LineSide> {
    match side {
        Side::Old => entry.old.as_ref(),
        Side::New => entry.new.as_ref(),
    }
}

/// A shown line's state class and change marker, in either view: empty where
/// the side has no line, unchanged on an unchanged entry, and otherwise removed
/// `-` on the old side or added `+` on the new side.
///
/// REQT-wjjyqjnjs6 (Change markers)
fn line_state(
    line: Option<&LineSide>,
    side: Side,
    change: ChangeKind,
) -> (&'static str, &'static str) {
    match (line, side, change) {
        (None, _, _) => (DXDIFF__EMPTY, ""),
        (Some(_), _, ChangeKind::Unchanged) => (DXDIFF__UNCHANGED, ""),
        (Some(_), Side::Old, _) => (DXDIFF__REMOVED, "-"),
        (Some(_), Side::New, _) => (DXDIFF__ADDED, "+"),
    }
}

/// One side of a modified pair, shown as its own inline row.
fn inline_side(entry: &PairedLineEntry, side: Side) -> ShownLine<'_> {
    let mut shown = shown_side(entry, side);
    // The old row numbers only the old gutter, the new row only the new one.
    shown.gutters = match side {
        Side::Old => [shown.gutters[0], None],
        Side::New => [None, shown.gutters[0]],
    };
    shown
}

/// One side of an entry as a shown line: a split row's side, or the start of
/// a modified pair's inline row.
fn shown_side(entry: &PairedLineEntry, side: Side) -> ShownLine<'_> {
    let line = side_of(entry, side);
    let (state, marker) = line_state(line, side, entry.change);
    let line_id = line.map(|line| match side {
        Side::Old => LineId::Old(line.number),
        Side::New => LineId::New(line.number),
    });
    let tokens = entry.inline_changes.as_ref().map(|changes| match side {
        Side::Old => changes.old.as_slice(),
        Side::New => changes.new.as_slice(),
    });
    let terminator = line
        .and(entry.line_ending_change.as_ref())
        .map(|change| match side {
            Side::Old => change.old.as_str(),
            Side::New => change.new.as_str(),
        });
    ShownLine {
        gutters: [line_id, None],
        state,
        marker,
        line,
        tokens,
        line_endings: [terminator, None],
        whitespace_change: line.is_some() && entry.whitespace_change,
    }
}

/// A line terminator as escaped text: `\n`, `\r\n`, or `\r`, formatted in place.
fn escaped_terminator(terminator: &str) -> std::str::EscapeDefault<'_> {
    terminator.escape_default()
}
