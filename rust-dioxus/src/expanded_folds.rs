//! The folds a reader or an app has expanded, and the basis they hold under.
//!
//! Architecture: ARCH-8tce9rry3b (Diff), which owns this state.
//! REQT-869jyzdes7 (Where fold state lives): the diff keeps it, so no
//! viewer keeps fold state of its own.

use std::collections::BTreeSet;

/// What a set of expansions was made under.
///
/// REQT-v748c7mjr6 (Expanding folds): a fold position means what it means only
/// for one pair of texts planned by one surrounding-line count, so a change to
/// either is what returns the folds it planned to folded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FoldBasis {
    /// The identity of the two compared texts.
    pub(crate) content_identity: blake3::Hash,
    /// The surrounding-line count the rows are planned by, absent when folding
    /// is off.
    pub(crate) folding: Option<usize>,
}

/// The folds expanded so far, by each fold's first entry position.
#[derive(Debug, Default)]
pub(crate) struct ExpandedFolds {
    /// What the expansions below were made under; absent while none are held.
    basis: Option<FoldBasis>,
    starts: BTreeSet<usize>,
}

impl ExpandedFolds {
    /// Whether the fold starting at `start` is expanded under `basis`.
    ///
    /// Expansions made under an earlier basis read as folded, without any
    /// write during rendering.
    pub(crate) fn is_expanded(&self, basis: FoldBasis, start: usize) -> bool {
        self.basis == Some(basis) && self.starts.contains(&start)
    }

    /// Expands the folds starting at `starts` under `basis`, dropping every
    /// expansion made under an earlier basis.
    ///
    /// One action expands one fold or every fold, so both arrive here as a run
    /// of starts and the basis is reckoned once for the action.
    pub(crate) fn expand(&mut self, basis: FoldBasis, starts: impl IntoIterator<Item = usize>) {
        if self.basis != Some(basis) {
            self.basis = Some(basis);
            self.starts.clear();
        }
        self.starts.extend(starts);
    }

    /// Returns every expanded fold to folded.
    ///
    /// REQT-ps5zx85jvc (Resetting folds): an app refolds everything in one
    /// action, which acts rather than rendering, so it writes here directly.
    pub(crate) fn reset(&mut self) {
        self.starts.clear();
    }
}
