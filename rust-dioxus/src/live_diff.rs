//! The live diff an app holds and viewers render.
//!
//! Architecture: ARCH-8tce9rry3b (Diff), ARCH-87bw15t11s (UseDiffHooks),
//! ARCH:dcisn-xgpr1asvyn (the app holds the live diff, the viewer renders it).
//!
//! The state sits outside any view's code path, which is what lets it answer
//! an app's questions without publishing them from an effect.

use dioxus::prelude::*;

use crate::diff_analysis_engine::analyze_diff;
use crate::diff_analysis_output::DiffAnalysis;
use crate::diff_options::DiffOptions;
use crate::entry_line_numbers::{LineRun, changed_runs, line_run_of, position_of_line};
use crate::expanded_folds::{ExpandedFolds, FoldBasis};
use crate::fold_planning::{PlannedRow, plan_rows};
use crate::line_id::LineId;

/// The two texts and the options they are compared and folded under.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DiffInputs {
    old_text: String,
    new_text: String,
    options: DiffOptions,
}

/// A diff an app holds and viewers render.
///
/// Build one with [`use_diff`] or [`use_diff_with`] in the app's own
/// component and pass it to each viewer's `diff` prop. It owns the compared
/// texts, the engine's analysis of them, the identity consumer-rendered
/// content is kept mounted by, the expanded folds, and the rows every viewer
/// over it shows.
#[derive(Clone, Copy, PartialEq)]
pub struct Diff {
    analysis: Memo<DiffAnalysis>,
    content_identity: Memo<blake3::Hash>,
    fold_basis: Memo<FoldBasis>,
    expanded_folds: Signal<ExpandedFolds>,
    planned_rows: Memo<Vec<PlannedRow>>,
}

impl Diff {
    /// Where the changes are, in document order, each as one run of
    /// consecutive changed lines.
    ///
    /// REQT-f2affyt2h2 (Where the changes are): the runs come from the same
    /// changed positions folding measures its surrounding-line count from, so
    /// the lines reported changed are the lines folding treats as changed.
    pub fn changed_line_runs(&self) -> Vec<LineRun> {
        let analysis = self.analysis.read();
        changed_runs(&analysis.changed_positions)
            .map(|positions| line_run_of(&analysis.entries, positions))
            .collect()
    }

    /// Which lines this diff's folds hide, as of this call.
    ///
    /// REQT-hsef5r7c4z (Which lines are hidden): the answer reads the row plan
    /// the viewers read, so a fold a reader has already expanded is reported as
    /// hidden no longer.
    pub fn hidden_line_runs(&self) -> Vec<LineRun> {
        let analysis = self.analysis.read();
        self.planned_rows
            .read()
            .iter()
            .filter_map(|planned| match planned {
                PlannedRow::Fold(fold) => Some(fold.hidden_positions()),
                PlannedRow::Entry(_) => None,
            })
            .map(|positions| line_run_of(&analysis.entries, positions))
            .collect()
    }

    /// Returns every expanded fold to folded.
    ///
    /// REQT-ps5zx85jvc (Resetting folds): one action, whose result shows in
    /// every viewer over this diff.
    pub fn reset_folds(&self) {
        let mut expanded_folds = self.expanded_folds;
        expanded_folds.write().reset();
    }

    /// The engine's analysis of the two texts, which viewers render from.
    pub(crate) fn analysis(&self) -> Memo<DiffAnalysis> {
        self.analysis
    }

    /// What consumer-rendered content stays mounted by: it changes when either
    /// text changes and at no other time.
    ///
    /// REQT-zen8fyae28 (Rendered content identity)
    pub(crate) fn content_identity(&self) -> blake3::Hash {
        *self.content_identity.read()
    }

    /// The rows to show, planned once for every viewer over this diff.
    ///
    /// REQT-dxbaat20ja (One plan per live diff)
    pub(crate) fn planned_rows(&self) -> Memo<Vec<PlannedRow>> {
        self.planned_rows
    }

    /// Reveals the run of hidden lines holding the line an app names.
    ///
    /// REQT-g86vdmyyp9 (Expanding at a line): a line the diff does not hold,
    /// and a line already shown, both leave the shown lines alone.
    pub fn expand_fold_at_line(&self, line: LineId) {
        if let Some(start) = self.fold_start_hiding(line) {
            self.expand_folds([start]);
        }
    }

    /// Reveals every line this diff's folds hide.
    ///
    /// REQT-h8rxvhpj9g (Expanding everything): one action, whose result shows
    /// in every viewer over this diff.
    pub fn expand_all_folds(&self) {
        self.expand_folds(self.fold_starts());
    }

    /// Reveals the lines hidden by the fold starting at `start`.
    ///
    /// REQT-v748c7mjr6 (Expanding folds): what a fold row activates.
    pub(crate) fn expand_fold_at_position(&self, start: usize) {
        self.expand_folds([start]);
    }

    /// Where the fold hiding `line` starts, absent when the diff holds no such
    /// line or the line is shown.
    ///
    /// REQT-g86vdmyyp9 (Expanding at a line): the named line becomes a
    /// position through `entry_line_numbers`, the same relation the reading
    /// answers report through, and the row plan says which fold covers it.
    fn fold_start_hiding(&self, line: LineId) -> Option<usize> {
        let position = position_of_line(&self.analysis.read().entries, line)?;
        self.planned_rows
            .read()
            .iter()
            .find_map(|planned| match planned {
                PlannedRow::Fold(fold) if fold.hidden_positions().contains(&position) => {
                    Some(fold.start)
                }
                _ => None,
            })
    }

    /// Where each fold still standing in the row plan starts.
    fn fold_starts(&self) -> Vec<usize> {
        self.planned_rows
            .read()
            .iter()
            .filter_map(|planned| match planned {
                PlannedRow::Fold(fold) => Some(fold.start),
                PlannedRow::Entry(_) => None,
            })
            .collect()
    }

    /// Expands the folds starting at `starts`, in one write.
    ///
    /// The row plan is read before this call rather than during it, so no read
    /// guard on the plan is held while the state it is planned from changes.
    fn expand_folds(&self, starts: impl IntoIterator<Item = usize>) {
        let basis = *self.fold_basis.read();
        let mut expanded_folds = self.expanded_folds;
        expanded_folds.write().expand(basis, starts);
    }
}

impl std::fmt::Debug for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Diff").finish_non_exhaustive()
    }
}

/// Builds a live diff from an old and a new text.
///
/// This is a hook: call it at the top of a component, unconditionally. The
/// calling component owns the state, which is what lets it outlive a render
/// without a viewer holding it.
///
/// REQT-7tk9vxv9wd (Building a live diff): the two texts alone, compared and
/// folded under [`DiffOptions::default`].
pub fn use_diff(old_text: impl AsRef<str>, new_text: impl AsRef<str>) -> Diff {
    use_diff_with(old_text, new_text, DiffOptions::default())
}

/// Builds a live diff from an old and a new text and the options they are
/// compared and folded under.
///
/// This is a hook: call it at the top of a component, unconditionally.
///
/// REQT-7tk9vxv9wd (Building a live diff): the form taking one options value.
pub fn use_diff_with(
    old_text: impl AsRef<str>,
    new_text: impl AsRef<str>,
    options: DiffOptions,
) -> Diff {
    let inputs = use_diff_inputs(old_text.as_ref(), new_text.as_ref(), options);

    // ARCH-atczcqvdsz (Line diff hand-off): the live diff asks the engine, and
    // only when the texts or the engine's options change.
    let analysis = use_memo(move || {
        let inputs = inputs.read();
        analyze_diff(
            &inputs.old_text,
            &inputs.new_text,
            &inputs.options.analysis_options(),
        )
    });
    // REQT-zen8fyae28 (Rendered content identity): keyed by the texts alone,
    // so a compare method or a fold opening leaves content mounted.
    let content_identity = use_memo(move || {
        let inputs = inputs.read();
        content_identity_of(&inputs.old_text, &inputs.new_text)
    });
    let fold_basis = use_memo(move || FoldBasis {
        content_identity: content_identity(),
        folding: inputs.read().options.folding(),
    });
    let expanded_folds = use_signal(ExpandedFolds::default);
    // REQT-dxbaat20ja (One plan per live diff): one plan, which every viewer
    // over this diff reads, so a fold opened here opens in all of them.
    let planned_rows = use_memo(move || {
        let basis = fold_basis();
        plan_rows(&analysis.read(), basis.folding, |start| {
            expanded_folds.read().is_expanded(basis, start)
        })
    });

    Diff {
        analysis,
        content_identity,
        fold_basis,
        expanded_folds,
        planned_rows,
    }
}

/// Holds the caller's plain values as reactive state, refreshed during
/// rendering only when they actually differ.
///
/// The hooks take plain texts rather than signals: a hook receives none of the
/// in-place prop updating a component gets, so a caller's changing text would
/// otherwise never reach the memos above. This is `use_reactive`'s mechanism,
/// written out here so that the three values are held as one record and so
/// that an unchanged render compares the texts rather than copying them.
fn use_diff_inputs(old_text: &str, new_text: &str, options: DiffOptions) -> Signal<DiffInputs> {
    let mut held = use_signal(|| DiffInputs {
        old_text: old_text.to_owned(),
        new_text: new_text.to_owned(),
        options: options.clone(),
    });
    let changed = {
        let held = held.peek();
        held.old_text != old_text || held.new_text != new_text || held.options != options
    };
    if changed {
        held.set(DiffInputs {
            old_text: old_text.to_owned(),
            new_text: new_text.to_owned(),
            options,
        });
    }
    held
}

/// The identity of a pair of texts, which changes whenever either changes.
fn content_identity_of(old_text: &str, new_text: &str) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&(old_text.len() as u64).to_le_bytes());
    hasher.update(old_text.as_bytes());
    hasher.update(new_text.as_bytes());
    hasher.finalize()
}
