//! Turns two texts and options into paired line information.
//!
//! Architecture: ARCH-jf3s5npp9s (DiffAnalysisEngine). The engine carries no
//! presentation: views, folding, and styling belong to the component.
//!
//! Alignment comes from `similar`, but not from its line tokenizer: that
//! tokenizer keeps `\n`, `\r\n`, and a bare `\r` inside each line token, so two
//! lines with the same text but different terminators would never match. This
//! module splits the lines itself, holds each terminator aside, and hands
//! `similar` only the line texts.

use std::sync::Arc;

use similar::{ChangeTag, DiffOp, TextDiff};

use crate::diff_analysis_options::{CompareMethod, DiffAnalysisOptions};
use crate::diff_analysis_output::{
    ChangeKind, DiffAnalysis, InlineChanges, InlineToken, LineEndingChange, LineSide,
    PairedLineEntry, TokenKind,
};

/// One line of an input text, with the terminator that ended it held aside.
/// The last line of a text is never terminated.
struct InputLine<'a> {
    text: &'a str,
    terminator: Option<&'a str>,
}

/// Computes paired line information for two texts.
///
/// Every pair of texts yields a `DiffAnalysis`; there is no failure case.
pub fn analyze_diff(old_text: &str, new_text: &str, options: &DiffAnalysisOptions) -> DiffAnalysis {
    // REQT-9tze98pt6g (Trailing whitespace): each text's end is trimmed, so
    // trailing blank lines never show as changes.
    let old_lines = split_lines(old_text.trim_end());
    let new_lines = split_lines(new_text.trim_end());

    // REQT-hmsfnfe5wc (Line marking): lines are compared by their text without
    // line terminators, so `\r\n` and `\n` endings align as unchanged.
    let old_texts: Vec<&str> = old_lines.iter().map(|line| line.text).collect();
    let new_texts: Vec<&str> = new_lines.iter().map(|line| line.text).collect();
    let alignment = TextDiff::from_slices(&old_texts, &new_texts);

    let mut builder = EntryBuilder::new(options);
    for op in alignment.ops() {
        match *op {
            DiffOp::Equal {
                old_index,
                new_index,
                len,
            } => {
                for offset in 0..len {
                    builder.push_unchanged(
                        &old_lines[old_index + offset],
                        &new_lines[new_index + offset],
                    );
                }
            }
            DiffOp::Delete {
                old_index, old_len, ..
            } => {
                for line in &old_lines[old_index..old_index + old_len] {
                    builder.push_removed(line);
                }
            }
            DiffOp::Insert {
                new_index, new_len, ..
            } => {
                for line in &new_lines[new_index..new_index + new_len] {
                    builder.push_added(line);
                }
            }
            // REQT-dqxm8fa7ts (Modified lines): removed lines immediately
            // followed by added lines pair in order, and the lines left without
            // a partner stay plain removals or additions.
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => {
                let paired = old_len.min(new_len);
                for offset in 0..paired {
                    builder.push_modified(
                        &old_lines[old_index + offset],
                        &new_lines[new_index + offset],
                    );
                }
                for line in &old_lines[old_index + paired..old_index + old_len] {
                    builder.push_removed(line);
                }
                for line in &new_lines[new_index + paired..new_index + new_len] {
                    builder.push_added(line);
                }
            }
        }
    }

    builder.finish()
}

/// Splits a text into its lines, holding each line's terminator aside.
///
/// This is `similar`'s own line tokenization (`\r\n`, a bare `\r`, and `\n` all
/// end a line) with the terminator kept out of the line's text, which is what
/// REQT-hmsfnfe5wc (Line marking) compares and what REQT-rtwn1qresp (Line
/// ending changes) reports.
fn split_lines(text: &str) -> Vec<InputLine<'_>> {
    let mut lines = Vec::new();
    let mut chars = text.char_indices().peekable();
    let mut start = 0;

    while let Some((index, character)) = chars.next() {
        let end = match character {
            '\r' => {
                if chars.peek().is_some_and(|(_, next)| *next == '\n') {
                    chars.next();
                    index + 2
                } else {
                    index + 1
                }
            }
            '\n' => index + 1,
            _ => continue,
        };
        lines.push(InputLine {
            text: &text[start..index],
            terminator: Some(&text[index..end]),
        });
        start = end;
    }

    if start < text.len() {
        lines.push(InputLine {
            text: &text[start..],
            terminator: None,
        });
    }

    lines
}

/// Accumulates paired entries while counting each side's lines independently.
struct EntryBuilder<'a> {
    options: &'a DiffAnalysisOptions,
    entries: Vec<PairedLineEntry>,
    changed_positions: Vec<usize>,
    // REQT-smd01rma2q (Independent numbering): each side counts its own lines,
    // starting at one more than the consumer's line offset.
    old_number: usize,
    new_number: usize,
}

impl<'a> EntryBuilder<'a> {
    fn new(options: &'a DiffAnalysisOptions) -> Self {
        Self {
            options,
            entries: Vec::new(),
            changed_positions: Vec::new(),
            old_number: options.line_offset,
            new_number: options.line_offset,
        }
    }

    fn next_old(&mut self, line: &InputLine<'_>) -> LineSide {
        self.old_number += 1;
        LineSide {
            number: self.old_number,
            text: Arc::from(line.text),
        }
    }

    fn next_new(&mut self, line: &InputLine<'_>) -> LineSide {
        self.new_number += 1;
        LineSide {
            number: self.new_number,
            text: Arc::from(line.text),
        }
    }

    fn push_unchanged(&mut self, old_line: &InputLine<'_>, new_line: &InputLine<'_>) {
        let old = self.next_old(old_line);
        let new = self.next_new(new_line);
        self.push(PairedLineEntry {
            change: ChangeKind::Unchanged,
            old: Some(old),
            new: Some(new),
            inline_changes: None,
            line_ending_change: line_ending_change(old_line, new_line),
            whitespace_change: false,
        });
    }

    fn push_removed(&mut self, line: &InputLine<'_>) {
        let old = self.next_old(line);
        self.push(PairedLineEntry {
            change: ChangeKind::Removed,
            old: Some(old),
            new: None,
            inline_changes: None,
            line_ending_change: None,
            whitespace_change: false,
        });
    }

    fn push_added(&mut self, line: &InputLine<'_>) {
        let new = self.next_new(line);
        self.push(PairedLineEntry {
            change: ChangeKind::Added,
            old: None,
            new: Some(new),
            inline_changes: None,
            line_ending_change: None,
            whitespace_change: false,
        });
    }

    fn push_modified(&mut self, old_line: &InputLine<'_>, new_line: &InputLine<'_>) {
        let old = self.next_old(old_line);
        let new = self.next_new(new_line);
        // REQT-4nz35dscrn (Inline changes): a modified line carries the tokens
        // removed from its old text and added in its new text, unless the
        // consumer turned inline changes off.
        let inline_changes = self
            .options
            .mark_inline_changes
            .then(|| compute_inline_changes(old_line.text, new_line.text, self.options.compare));
        // REQT-hq1fzjaxg8 (Leading or trailing whitespace changes): only
        // trimmed line comparison marks end-of-line whitespace edits.
        let whitespace_change = self.options.compare == CompareMethod::TrimmedLine
            && edge_whitespace_differs(old_line.text, new_line.text);
        self.push(PairedLineEntry {
            change: ChangeKind::Modified,
            old: Some(old),
            new: Some(new),
            inline_changes,
            line_ending_change: line_ending_change(old_line, new_line),
            whitespace_change,
        });
    }

    /// Appends one entry, recording its position when it holds a change.
    ///
    /// Every entry goes through here, so `changed_positions` cannot drift out
    /// of step with `entries`.
    ///
    /// A line ending change counts as a change: the pair's text is unchanged,
    /// but the component must show each side's terminator
    /// (REQT-rtwn1qresp, Line ending changes), so the entry cannot fold away
    /// (REQT-qcnxhemvhn, Folded unchanged lines).
    fn push(&mut self, entry: PairedLineEntry) {
        if entry.change != ChangeKind::Unchanged || entry.line_ending_change.is_some() {
            self.changed_positions.push(self.entries.len());
        }
        self.entries.push(entry);
    }

    fn finish(self) -> DiffAnalysis {
        DiffAnalysis {
            entries: self.entries,
            changed_positions: self.changed_positions,
        }
    }
}

/// Reports each side's terminator when a pair's two terminators differ.
///
/// REQT-rtwn1qresp (Line ending changes): both lines must end with a terminator
/// for the pair to carry an ending change, so the last line of each text, which
/// trimming leaves unterminated, never reports one.
fn line_ending_change(
    old_line: &InputLine<'_>,
    new_line: &InputLine<'_>,
) -> Option<LineEndingChange> {
    let old = old_line.terminator?;
    let new = new_line.terminator?;
    (old != new).then(|| LineEndingChange {
        old: old.to_owned(),
        new: new.to_owned(),
    })
}

/// Whether two lines differ in their leading or their trailing whitespace.
fn edge_whitespace_differs(old_line: &str, new_line: &str) -> bool {
    leading_whitespace(old_line) != leading_whitespace(new_line)
        || trailing_whitespace(old_line) != trailing_whitespace(new_line)
}

fn leading_whitespace(line: &str) -> &str {
    &line[..line.len() - line.trim_start().len()]
}

fn trailing_whitespace(line: &str) -> &str {
    &line[line.trim_end().len()..]
}

/// Marks the tokens removed from the old line and added in the new line.
fn compute_inline_changes(old_line: &str, new_line: &str, compare: CompareMethod) -> InlineChanges {
    let tokens = match compare {
        // REQT-z9r0pc53jg (Trimmed line comparison): each side is one token
        // covering its whole line, unchanged when only the line's leading or
        // trailing whitespace differs.
        CompareMethod::TrimmedLine => {
            return whole_line_tokens(old_line, new_line, old_line.trim() == new_line.trim());
        }
        // REQT-czecf8krqc (Character comparison)
        CompareMethod::Character => TextDiff::from_chars(old_line, new_line),
        // REQT-xzc8n354h1 (Word comparison): `similar` splits into alternating
        // whitespace runs and non-whitespace runs.
        CompareMethod::Word => TextDiff::from_words(old_line, new_line),
        // REQT-spzdk2z1pk (Line comparison): the whole line is one token.
        CompareMethod::Line => TextDiff::from_slices(&[old_line], &[new_line]),
    };

    let mut changes = InlineChanges {
        old: Vec::new(),
        new: Vec::new(),
    };
    // Each side's tokens are consecutive and cover that side's whole line, so
    // walking the changes in order gives every token's range into its own side.
    let mut old_at = 0;
    let mut new_at = 0;
    for change in tokens.iter_all_changes() {
        let len = change.value().len();
        match change.tag() {
            ChangeTag::Equal => {
                changes.old.push(InlineToken {
                    kind: TokenKind::Unchanged,
                    range: old_at..old_at + len,
                });
                changes.new.push(InlineToken {
                    kind: TokenKind::Unchanged,
                    range: new_at..new_at + len,
                });
                old_at += len;
                new_at += len;
            }
            ChangeTag::Delete => {
                changes.old.push(InlineToken {
                    kind: TokenKind::Removed,
                    range: old_at..old_at + len,
                });
                old_at += len;
            }
            ChangeTag::Insert => {
                changes.new.push(InlineToken {
                    kind: TokenKind::Added,
                    range: new_at..new_at + len,
                });
                new_at += len;
            }
        }
    }
    debug_assert_eq!(old_at, old_line.len(), "old tokens cover the old line");
    debug_assert_eq!(new_at, new_line.len(), "new tokens cover the new line");
    changes
}

/// One token per side covering that side's whole line, marked unchanged on
/// both sides or removed and added.
fn whole_line_tokens(old_line: &str, new_line: &str, unchanged: bool) -> InlineChanges {
    let (old_kind, new_kind) = if unchanged {
        (TokenKind::Unchanged, TokenKind::Unchanged)
    } else {
        (TokenKind::Removed, TokenKind::Added)
    };
    InlineChanges {
        old: vec![InlineToken {
            kind: old_kind,
            range: 0..old_line.len(),
        }],
        new: vec![InlineToken {
            kind: new_kind,
            range: 0..new_line.len(),
        }],
    }
}
