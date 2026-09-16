//! The expanded-fold state the viewer keeps, and the optional trigger through
//! which an app returns every expanded fold to folded.
//!
//! Architecture: ARCH-y9545npjzg (FoldResetTrigger).
//!
//! REQT-869jyzdes7 (Fold reset): the viewer keeps the state itself. An app
//! never names a fold; it only asks for a reset.

use std::collections::BTreeSet;

use dioxus::prelude::*;

/// An opaque handle whose one action returns every expanded fold to folded.
///
/// Create it with [`use_fold_reset_trigger`] in the app's component and pass it
/// to the viewer's `fold_reset_trigger` prop. It is optional: a viewer given
/// none keeps its folds to itself.
#[derive(Clone, Copy, PartialEq)]
pub struct FoldResetTrigger {
    resets: Signal<u64>,
}

impl FoldResetTrigger {
    /// Returns every fold this trigger's viewers have expanded to folded.
    pub fn reset(&self) {
        let mut resets = self.resets;
        *resets.write() += 1;
    }

    /// How many resets have been asked for, which the viewer compares against
    /// the count its expansions were made under.
    pub(crate) fn resets(&self) -> u64 {
        *self.resets.read()
    }
}

impl std::fmt::Debug for FoldResetTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FoldResetTrigger").finish_non_exhaustive()
    }
}

/// Creates a fold reset trigger owned by the calling component.
///
/// This is a hook: call it at the top of a component, unconditionally.
pub fn use_fold_reset_trigger() -> FoldResetTrigger {
    FoldResetTrigger {
        resets: use_signal(|| 0),
    }
}

/// What the viewer's expansions were made under. Expansions made under an
/// earlier basis no longer apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct FoldBasis {
    /// Bumped whenever the compared texts or the surrounding-line count change,
    /// which is when fold positions stop meaning what they meant.
    pub(crate) input_generation: u64,
    /// The fold reset trigger's reset count, or 0 without a trigger.
    pub(crate) resets: u64,
}

/// The folds a reader expanded, by each fold's first entry position.
#[derive(Debug, Default)]
pub(crate) struct ExpandedFolds {
    basis: FoldBasis,
    starts: BTreeSet<usize>,
}

impl ExpandedFolds {
    /// Whether the fold starting at `start` is expanded under `basis`.
    ///
    /// REQT-v748c7mjr6 (Expanding folds): expansions made before the texts,
    /// the surrounding-line count, or a reset changed the basis read as
    /// folded, without any write during rendering.
    pub(crate) fn is_expanded(&self, basis: FoldBasis, start: usize) -> bool {
        self.basis == basis && self.starts.contains(&start)
    }

    /// Expands the fold starting at `start` under `basis`, dropping every
    /// expansion made under an earlier basis.
    pub(crate) fn expand(&mut self, basis: FoldBasis, start: usize) {
        if self.basis != basis {
            self.basis = basis;
            self.starts.clear();
        }
        self.starts.insert(start);
    }
}
