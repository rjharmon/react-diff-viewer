//! What a diff is built with: the engine's compare choices together with
//! the folding choices.
//!
//! Architecture: ARCH-smcbvznq0z (DiffOptions). Folding sits here rather than
//! on the viewer because the row plan the diff owns cannot be computed
//! without it.

use crate::diff_analysis_options::{CompareMethod, DiffAnalysisOptions};

/// The choices an app makes when building a diff.
///
/// REQT-7tk9vxv9wd (Building a diff): every field is defaulted, so an app
/// states only what it wants to change and takes the rest from
/// [`DiffOptions::default`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffOptions {
    /// How modified lines are compared when marking inline changes.
    pub compare: CompareMethod,
    /// Whether modified lines carry the tokens removed and added within them.
    pub mark_inline_changes: bool,
    /// Each side's first line is numbered one more than this.
    pub line_offset: usize,
    /// Whether unchanged lines far from every change fold away.
    pub fold_unchanged_lines: bool,
    /// How many unchanged lines stay shown around each change.
    pub surrounding_line_count: usize,
}

impl Default for DiffOptions {
    fn default() -> Self {
        // The engine's own defaults stay where the engine states them; this
        // value adds the folding choices to them.
        let DiffAnalysisOptions {
            compare,
            mark_inline_changes,
            line_offset,
        } = DiffAnalysisOptions::default();
        Self {
            compare,
            mark_inline_changes,
            line_offset,
            // REQT-qcnxhemvhn (Folded unchanged lines): folding is on unless
            // turned off, and three unchanged lines surround each change.
            fold_unchanged_lines: true,
            surrounding_line_count: 3,
        }
    }
}

impl DiffOptions {
    /// The engine's own choices, as the engine takes them.
    pub(crate) fn analysis_options(&self) -> DiffAnalysisOptions {
        DiffAnalysisOptions {
            compare: self.compare,
            mark_inline_changes: self.mark_inline_changes,
            line_offset: self.line_offset,
        }
    }

    /// The surrounding-line count to fold by, or `None` when folding is off.
    ///
    /// REQT-qcnxhemvhn (Folded unchanged lines)
    pub(crate) fn folding(&self) -> Option<usize> {
        self.fold_unchanged_lines
            .then_some(self.surrounding_line_count)
    }
}
