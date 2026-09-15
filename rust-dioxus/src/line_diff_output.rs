//! What the engine hands the component: paired line entries and the positions
//! of the entries holding a change.
//!
//! Architecture: ARCH-exgfx6rwdt (LineDiff), ARCH-kjjykjtt5r (PairedLineEntry).
//!
//! Every text here is owned rather than borrowed from the two input texts. A
//! Dioxus component keeps this output in state across renders, which needs
//! `'static`, and the component is this output's only consumer.

/// How one paired line entry changed between the two texts.
/// REQT-hmsfnfe5wc (Line marking), REQT-dqxm8fa7ts (Modified lines).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// Both sides carry the same line text.
    Unchanged,
    /// Only the old text has this line.
    Removed,
    /// Only the new text has this line.
    Added,
    /// A removed line paired with the added line that followed it.
    Modified,
}

/// One side of a paired line entry: its text and its line number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineSide {
    /// This side's line number, counted independently of the other side.
    /// REQT-smd01rma2q (Independent numbering).
    pub number: usize,
    /// The line's text, without its line terminator.
    /// REQT-hmsfnfe5wc (Line marking).
    pub text: String,
}

/// How one token within a modified line changed.
/// REQT-4nz35dscrn (Inline changes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// The token is present in both lines.
    Unchanged,
    /// The token is present only in the old line.
    Removed,
    /// The token is present only in the new line.
    Added,
}

/// One token of a modified line, on whichever side carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineToken {
    /// How this token changed.
    pub kind: TokenKind,
    /// The token's text.
    pub text: String,
}

/// The tokens of a modified line's old and new text.
/// Each side's tokens rejoin to that side's line text.
/// REQT-4nz35dscrn (Inline changes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineChanges {
    /// The old line's tokens, unchanged and removed ones in line order.
    pub old: Vec<InlineToken>,
    /// The new line's tokens, unchanged and added ones in line order.
    pub new: Vec<InlineToken>,
}

/// A paired entry whose two lines end with different line terminators.
/// REQT-rtwn1qresp (Line ending changes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineEndingChange {
    /// The old line's terminator: `\n`, `\r\n`, or `\r`.
    pub old: String,
    /// The new line's terminator: `\n`, `\r\n`, or `\r`.
    pub new: String,
}

/// One row of the diff as either view reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairedLineEntry {
    /// How this entry changed.
    pub change: ChangeKind,
    /// The old text's line, absent on an added entry.
    pub old: Option<LineSide>,
    /// The new text's line, absent on a removed entry.
    pub new: Option<LineSide>,
    /// The tokens changed within a modified line, absent when the entry is not
    /// modified or the consumer turned inline changes off.
    /// REQT-4nz35dscrn (Inline changes).
    pub inline_changes: Option<InlineChanges>,
    /// Each side's terminator when the two sides' terminators differ.
    /// REQT-rtwn1qresp (Line ending changes).
    pub line_ending_change: Option<LineEndingChange>,
}

/// The engine's output and the component's only input for views and folding.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LineDiff {
    /// The paired line entries in display order.
    pub entries: Vec<PairedLineEntry>,
    /// The positions in `entries` of the entries holding a change, ascending.
    pub changed_positions: Vec<usize>,
}
