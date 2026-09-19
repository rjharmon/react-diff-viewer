//! The one relation between entry positions and each side's line numbers.
//!
//! The engine hands over entries in display order and reports the positions
//! that changed; folding plans rows by position too. An app asks and is
//! answered in each side's plain line numbers. Every answer crossing that
//! boundary reads this module, so a line reported hidden and a line resolved
//! from an app's own request cannot be reckoned two different ways.

use std::ops::{Range, RangeInclusive};

use crate::diff_analysis_output::{LineSide, PairedLineEntry};
use crate::line_id::LineId;

/// The line numbers a run of consecutive entries spans, on each side.
///
/// A side is absent when the run holds no line of that text: the old side of a
/// run of added lines, or the new side of a run of removed lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineNumberSpan {
    /// The old text's first and last line numbers in this run.
    pub old: Option<RangeInclusive<usize>>,
    /// The new text's first and last line numbers in this run.
    pub new: Option<RangeInclusive<usize>>,
}

/// The line numbers the entries at `positions` span, on each side.
///
/// REQT-f2affyt2h2 (Where the changes are), REQT-hsef5r7c4z (Which lines are
/// hidden): both answers read their two sides' numbers here.
pub(crate) fn line_number_span_of(
    entries: &[PairedLineEntry],
    positions: Range<usize>,
) -> LineNumberSpan {
    let run = &entries[positions];
    LineNumberSpan {
        old: side_run(run.iter().filter_map(|entry| entry.old.as_ref())),
        new: side_run(run.iter().filter_map(|entry| entry.new.as_ref())),
    }
}

/// One side's first and last line numbers over the entries carrying that side,
/// absent when none of them does.
///
/// Each side numbers its own lines upward across the entries
/// (REQT-smd01rma2q, Independent numbering), so the first and last carrying
/// entries hold the lowest and highest numbers.
fn side_run<'a>(mut sides: impl Iterator<Item = &'a LineSide>) -> Option<RangeInclusive<usize>> {
    let first = sides.next()?.number;
    let last = sides.last().map_or(first, |side| side.number);
    Some(first..=last)
}

/// The changed positions grouped into runs of consecutive entries.
///
/// REQT-f2affyt2h2 (Where the changes are): the lines counted as changed are
/// the engine's own changed positions, which is what folding measures its
/// surrounding-line count from.
pub(crate) fn changed_runs(changed_positions: &[usize]) -> impl Iterator<Item = Range<usize>> {
    changed_positions
        .chunk_by(|position, next| *next == position + 1)
        .map(|run| run[0]..run[run.len() - 1] + 1)
}

/// The position of the entry carrying `line`, absent when the diff holds no
/// such line.
///
/// REQT-g86vdmyyp9 (Expanding at a line): an app names a line and the crate
/// maps it onto its own folds, so the name becomes a position here, in the
/// same relation the reading answers report through.
pub(crate) fn position_of_line(entries: &[PairedLineEntry], line: LineId) -> Option<usize> {
    entries.iter().position(|entry| entry_carries(entry, line))
}

/// Whether `entry` carries `line` on the side the line names.
fn entry_carries(entry: &PairedLineEntry, line: LineId) -> bool {
    let (side, number) = match line {
        LineId::Old(number) => (entry.old.as_ref(), number),
        LineId::New(number) => (entry.new.as_ref(), number),
    };
    side.is_some_and(|side| side.number == number)
}
