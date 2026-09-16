//! The class names the viewer's markup carries, so apps and themes can style
//! it without depending on its element structure.
//!
//! REQT-3928hx46s3 (Styling hooks): every class is prefixed with `dxdiff`. Each
//! element carries one class naming its kind and one naming its change or
//! highlight state.

use crate::line_diff_output::{ChangeKind, TokenKind};

/// The table holding the whole viewer.
pub const DXDIFF__VIEWER: &str = "dxdiff-viewer";
/// On the viewer, when it shows the split view.
pub const DXDIFF__SPLIT_VIEW: &str = "dxdiff-split-view";
/// On the viewer, when it shows the inline view.
pub const DXDIFF__INLINE_VIEW: &str = "dxdiff-inline-view";

/// A column title cell.
pub const DXDIFF__TITLE: &str = "dxdiff-title";
/// A row showing one or two lines.
pub const DXDIFF__ROW: &str = "dxdiff-row";
/// A line number cell.
pub const DXDIFF__GUTTER: &str = "dxdiff-gutter";
/// The cell holding a line's `-` or `+`.
pub const DXDIFF__CHANGE_MARKER: &str = "dxdiff-change-marker";
/// The cell holding a line's text.
pub const DXDIFF__CONTENT: &str = "dxdiff-content";
/// One inline-change token within a modified line's text.
pub const DXDIFF__INLINE_TOKEN: &str = "dxdiff-inline-token";
/// The chip showing a line's terminator in a line ending change.
pub const DXDIFF__LINE_ENDING_CHIP: &str = "dxdiff-line-ending-chip";
/// The arrow between an inline unchanged line's old and new terminator chips.
pub const DXDIFF__LINE_ENDING_ARROW: &str = "dxdiff-line-ending-arrow";
/// The chip reading `WS` on a pair carrying a whitespace change.
pub const DXDIFF__WHITESPACE_CHIP: &str = "dxdiff-whitespace-chip";
/// A row standing in for folded unchanged lines.
pub const DXDIFF__FOLD_ROW: &str = "dxdiff-fold-row";

/// State: the element shows an unchanged line or token.
pub const DXDIFF__UNCHANGED: &str = "dxdiff-unchanged";
/// State: the element shows a removed line or token.
pub const DXDIFF__REMOVED: &str = "dxdiff-removed";
/// State: the element shows an added line or token.
pub const DXDIFF__ADDED: &str = "dxdiff-added";
/// State: the row shows a modified pair.
pub const DXDIFF__MODIFIED: &str = "dxdiff-modified";
/// State: the element sits on a side with no line, as the old side of an
/// added line in the split view.
pub const DXDIFF__EMPTY: &str = "dxdiff-empty";
/// State: the element shows a line the consumer highlighted.
pub const DXDIFF__HIGHLIGHTED: &str = "dxdiff-highlighted";

/// A row's state class for its entry's change kind.
pub(crate) fn row_state(change: ChangeKind) -> &'static str {
    match change {
        ChangeKind::Unchanged => DXDIFF__UNCHANGED,
        ChangeKind::Removed => DXDIFF__REMOVED,
        ChangeKind::Added => DXDIFF__ADDED,
        ChangeKind::Modified => DXDIFF__MODIFIED,
    }
}

/// An inline-change token's state class.
pub(crate) fn token_state(kind: TokenKind) -> &'static str {
    match kind {
        TokenKind::Unchanged => DXDIFF__UNCHANGED,
        TokenKind::Removed => DXDIFF__REMOVED,
        TokenKind::Added => DXDIFF__ADDED,
    }
}
