//! Show the differences between two texts in Dioxus apps.
//!
//! This crate is a port of the `react-diff-viewer` TypeScript package at the
//! repository root, with alignment computed by the `similar` crate.
//!
//! Architecture: ARCH-h2qwqte1g6 (dioxus-diff-viewer).

mod consumer_callbacks;
mod diff;
mod diff_analysis_engine;
mod diff_analysis_options;
mod diff_analysis_output;
mod diff_options;
mod diff_viewer;
mod entry_line_numbers;
mod expanded_folds;
mod fold_planning;
mod line_id;
mod row_rendering;
pub mod styling_hooks;

pub use consumer_callbacks::{HiddenLines, LineContent, LineNumberClick, ModifierKeys};
pub use diff::{Diff, use_diff, use_diff_with};
pub use diff_analysis_engine::analyze_diff;
pub use diff_analysis_options::{CompareMethod, DiffAnalysisOptions};
pub use diff_analysis_output::{
    ChangeKind, DiffAnalysis, InlineChanges, InlineToken, LineEndingChange, LineSide,
    PairedLineEntry, TokenKind,
};
pub use diff_options::DiffOptions;
pub use diff_viewer::{DiffTheme, DiffView, DiffViewer, DiffViewerProps};
pub use entry_line_numbers::LineRun;
pub use line_id::{LineId, LineIdParseError};
