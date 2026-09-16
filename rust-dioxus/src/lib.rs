//! Show the differences between two texts in Dioxus apps.
//!
//! This crate is a port of the `react-diff-viewer` TypeScript package at the
//! repository root, with alignment computed by the `similar` crate.
//!
//! Architecture: ARCH-h2qwqte1g6 (dioxus-diff-viewer).

mod consumer_callbacks;
mod diff_viewer;
mod diff_viewer_rows;
mod fold_planning;
mod fold_reset_trigger;
mod line_diff_engine;
mod line_diff_options;
mod line_diff_output;
mod line_id;
pub mod styling_hooks;

pub use consumer_callbacks::{HiddenLines, LineContent, LineNumberClick, ModifierKeys};
pub use diff_viewer::{DiffView, DiffViewer, DiffViewerProps};
pub use fold_reset_trigger::{FoldResetTrigger, use_fold_reset_trigger};
pub use line_diff_engine::line_diff;
pub use line_diff_options::{CompareMethod, LineDiffOptions};
pub use line_diff_output::{
    ChangeKind, InlineChanges, InlineToken, LineDiff, LineEndingChange, LineSide, PairedLineEntry,
    TokenKind,
};
pub use line_id::{LineId, LineIdParseError};
