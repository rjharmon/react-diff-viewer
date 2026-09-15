//! Show the differences between two texts in Dioxus apps.
//!
//! This crate is a port of the `react-diff-viewer` TypeScript package at the
//! repository root, with alignment computed by the `similar` crate.
//!
//! Architecture: ARCH-h2qwqte1g6 (dioxus-diff-viewer).

mod line_diff_engine;
mod line_diff_options;
mod line_diff_output;

pub use line_diff_engine::line_diff;
pub use line_diff_options::{CompareMethod, LineDiffOptions};
pub use line_diff_output::{
    ChangeKind, InlineChanges, InlineToken, LineDiff, LineEndingChange, LineSide, PairedLineEntry,
    TokenKind,
};
