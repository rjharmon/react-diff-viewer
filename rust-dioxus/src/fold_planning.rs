//! Which rows a view shows: each paired line entry, or a fold row standing in
//! for a run of unchanged entries far from every change.
//!
//! This is presentation-free planning over the engine's output, kept apart from
//! rendering so both views read the same plan.

use crate::diff_analysis_output::DiffAnalysis;

/// One planned row: an entry to show, or a fold hiding a run of entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlannedRow {
    /// Show the entry at this position in the diff's entries.
    Entry(usize),
    /// Show one fold row in place of these entries.
    Fold(Fold),
}

/// A run of consecutive entries hidden behind one fold row.
///
/// A fold is identified by its first entry's position, which is stable for a
/// given diff and surrounding-line count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Fold {
    /// The position of the first hidden entry.
    pub(crate) start: usize,
    /// How many entries the fold hides.
    pub(crate) len: usize,
}

/// Plans the rows of a view.
///
/// With `surrounding_line_count` absent, folding is off and every entry shows.
/// `is_expanded` reports whether the fold starting at a position was expanded.
pub(crate) fn plan_rows(
    diff: &DiffAnalysis,
    surrounding_line_count: Option<usize>,
    is_expanded: impl Fn(usize) -> bool,
) -> Vec<PlannedRow> {
    let entry_count = diff.entries.len();
    let Some(surrounding_line_count) = surrounding_line_count else {
        return (0..entry_count).map(PlannedRow::Entry).collect();
    };

    let mut rows = Vec::with_capacity(entry_count);
    let mut position = 0;
    while position < entry_count {
        // REQT-qcnxhemvhn (Folded unchanged lines): an entry folds when it lies
        // more than the surrounding-line count away from every change.
        if !is_far_from_changes(diff, position, surrounding_line_count) {
            rows.push(PlannedRow::Entry(position));
            position += 1;
            continue;
        }

        let start = position;
        while position < entry_count && is_far_from_changes(diff, position, surrounding_line_count)
        {
            position += 1;
        }
        let fold = Fold {
            start,
            len: position - start,
        };
        // REQT-v748c7mjr6 (Expanding folds): an expanded fold shows its lines.
        if is_expanded(fold.start) {
            rows.extend((fold.start..position).map(PlannedRow::Entry));
        } else {
            rows.push(PlannedRow::Fold(fold));
        }
    }
    rows
}

/// Whether every changed entry lies more than `surrounding_line_count`
/// positions away. With no changes at all, every entry is far from them.
fn is_far_from_changes(
    diff: &DiffAnalysis,
    position: usize,
    surrounding_line_count: usize,
) -> bool {
    // The changed positions are ascending, so the nearest change is at the
    // insertion point or just before it.
    let changes = &diff.changed_positions;
    let next = changes.partition_point(|&changed| changed < position);
    let distance_after = changes.get(next).map(|&changed| changed - position);
    let distance_before = next
        .checked_sub(1)
        .map(|previous| position - changes[previous]);
    [distance_after, distance_before]
        .into_iter()
        .flatten()
        .all(|distance| distance > surrounding_line_count)
}
