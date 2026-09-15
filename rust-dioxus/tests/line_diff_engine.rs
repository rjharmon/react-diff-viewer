//! Pre-work test drafts for the line diff engine (WORK-p7e70a5mwf).
//!
//! Drafted by the Rust Tester during pre-work review, before the crate exists.
//! Every test names one in-scope requirement. The call shape below (`line_diff`,
//! `LineDiffOptions`, `LineDiff`, `PairedLineEntry`) follows the candidate
//! architecture: ARCH-jf3s5npp9s (LineDiffEngine), ARCH-n7wmjnmt65
//! (LineDiffOptions), ARCH-exgfx6rwdt (LineDiff), ARCH-kjjykjtt5r
//! (PairedLineEntry). The worker owns the exact spelling; what these tests hold
//! to is the observable output each requirement names.

use dioxus_diff_viewer::{ChangeKind, CompareMethod, LineDiff, LineDiffOptions, TokenKind, line_diff};

// ---------------------------------------------------------------------------
// Reading helpers. These keep each test's assert section to one line of intent.
// ---------------------------------------------------------------------------

/// (change kind, old number and text, new number and text) per entry.
fn rows(diff: &LineDiff) -> Vec<(ChangeKind, Option<(usize, String)>, Option<(usize, String)>)> {
    diff.entries
        .iter()
        .map(|entry| {
            (
                entry.change,
                entry.old.as_ref().map(|side| (side.number, side.text.clone())),
                entry.new.as_ref().map(|side| (side.number, side.text.clone())),
            )
        })
        .collect()
}

fn unchanged(number: usize, text: &str) -> (ChangeKind, Option<(usize, String)>, Option<(usize, String)>) {
    (ChangeKind::Unchanged, Some((number, text.into())), Some((number, text.into())))
}

fn added(number: usize, text: &str) -> (ChangeKind, Option<(usize, String)>, Option<(usize, String)>) {
    (ChangeKind::Added, None, Some((number, text.into())))
}

fn removed(number: usize, text: &str) -> (ChangeKind, Option<(usize, String)>, Option<(usize, String)>) {
    (ChangeKind::Removed, Some((number, text.into())), None)
}

/// Old-side inline tokens of one entry, rejoined; panics when the entry carries none.
fn old_tokens_rejoined(diff: &LineDiff, index: usize) -> String {
    diff.entries[index]
        .inline_changes
        .as_ref()
        .expect("modified entry carries inline changes")
        .old
        .iter()
        .map(|token| token.text.as_str())
        .collect()
}

fn new_tokens_rejoined(diff: &LineDiff, index: usize) -> String {
    diff.entries[index]
        .inline_changes
        .as_ref()
        .expect("modified entry carries inline changes")
        .new
        .iter()
        .map(|token| token.text.as_str())
        .collect()
}

// ---------------------------------------------------------------------------
// Line changes: REQT-wqffp3d26e and children
// ---------------------------------------------------------------------------

/// REQT-hmsfnfe5wc (Line marking)
#[test]
fn a_line_only_the_new_text_has_is_added_and_the_shared_line_is_unchanged() {
    let diff = line_diff("test", "test\n    newLine", &LineDiffOptions::default());

    assert_eq!(rows(&diff), vec![unchanged(1, "test"), added(2, "    newLine")]);
}

/// REQT-hmsfnfe5wc (Line marking)
#[test]
fn a_line_only_the_old_text_has_is_removed() {
    let diff = line_diff("test\n    oldLine", "test", &LineDiffOptions::default());

    assert_eq!(rows(&diff), vec![unchanged(1, "test"), removed(2, "    oldLine")]);
}

/// REQT-hmsfnfe5wc (Line marking): lines differing only in their terminator align as unchanged.
#[test]
fn the_same_two_lines_under_windows_and_unix_endings_are_unchanged() {
    let diff = line_diff("first\r\nsecond", "first\nsecond", &LineDiffOptions::default());

    assert_eq!(
        rows(&diff),
        vec![unchanged(1, "first"), unchanged(2, "second")],
        "line identity ignores the terminator"
    );
}

/// REQT-dqxm8fa7ts (Modified lines)
#[test]
fn a_removed_line_followed_by_an_added_line_is_one_modified_entry() {
    let options = LineDiffOptions { inline_changes: false, ..LineDiffOptions::default() };

    let diff = line_diff("test\n    oldLine", "test\n    newLine", &options);

    assert_eq!(
        rows(&diff),
        vec![
            unchanged(1, "test"),
            (
                ChangeKind::Modified,
                Some((2, "    oldLine".into())),
                Some((2, "    newLine".into()))
            ),
        ]
    );
}

/// REQT-dqxm8fa7ts (Modified lines): more added lines than removed leaves the extras plain.
#[test]
fn an_added_line_beyond_the_removed_ones_stays_a_plain_addition() {
    let options = LineDiffOptions { inline_changes: false, ..LineDiffOptions::default() };

    let diff = line_diff("Hello World", "My Updated Name\nAlso this info", &options);

    assert_eq!(
        rows(&diff),
        vec![
            (
                ChangeKind::Modified,
                Some((1, "Hello World".into())),
                Some((1, "My Updated Name".into()))
            ),
            added(2, "Also this info"),
        ]
    );
}

/// REQT-4nz35dscrn (Inline changes): on unless the consumer turns them off.
#[test]
fn a_modified_line_carries_inline_changes_by_default() {
    let diff = line_diff("Hello World", "My Updated Name", &LineDiffOptions::default());

    assert_eq!(old_tokens_rejoined(&diff, 0), "Hello World");
    assert_eq!(new_tokens_rejoined(&diff, 0), "My Updated Name");
}

/// REQT-4nz35dscrn (Inline changes)
#[test]
fn a_modified_line_carries_no_inline_changes_when_they_are_turned_off() {
    let options = LineDiffOptions { inline_changes: false, ..LineDiffOptions::default() };

    let diff = line_diff("Hello World", "My Updated Name", &options);

    assert!(diff.entries[0].inline_changes.is_none());
}

/// REQT-9tze98pt6g (Trailing whitespace)
#[test]
fn trailing_blank_lines_in_either_text_are_not_a_change() {
    let diff = line_diff("test\n\n\n    ", "test\n\n    ", &LineDiffOptions::default());

    assert_eq!(rows(&diff), vec![unchanged(1, "test")]);
    assert!(diff.changed_positions.is_empty());
}

/// REQT-rtwn1qresp (Line ending changes)
#[test]
fn a_pair_whose_terminators_differ_reports_the_ending_change_with_both_terminators() {
    let diff = line_diff("first\r\nsecond", "first\nsecond", &LineDiffOptions::default());

    let ending_change = diff.entries[0]
        .line_ending_change
        .as_ref()
        .expect("the first pair's terminators differ");
    assert_eq!((ending_change.old.as_str(), ending_change.new.as_str()), ("\r\n", "\n"));
    assert!(
        diff.entries[1].line_ending_change.is_none(),
        "the last lines have no terminator to differ in"
    );
}

// ---------------------------------------------------------------------------
// Compare methods: REQT-rvcg21axa8 and children
// ---------------------------------------------------------------------------

/// REQT-czecf8krqc (Character comparison): the default splits below word level.
#[test]
fn inline_changes_compare_characters_unless_another_method_is_chosen() {
    let diff = line_diff("Hello World", "Hello Word", &LineDiffOptions::default());

    let removed_text: String = diff.entries[0]
        .inline_changes
        .as_ref()
        .expect("modified entry carries inline changes")
        .old
        .iter()
        .filter(|token| token.kind == TokenKind::Removed)
        .map(|token| token.text.as_str())
        .collect();
    assert_eq!(removed_text, "l", "only the dropped character is marked");
}

/// REQT-xzc8n354h1 (Word comparison): each whitespace run and each non-whitespace run is one token.
#[test]
fn word_comparison_keeps_whitespace_runs_as_their_own_tokens() {
    let options = LineDiffOptions { compare: CompareMethod::Word, ..LineDiffOptions::default() };

    let diff = line_diff("Hello World", "Hello Rust", &options);

    let new_side: Vec<(TokenKind, &str)> = diff.entries[0]
        .inline_changes
        .as_ref()
        .expect("modified entry carries inline changes")
        .new
        .iter()
        .map(|token| (token.kind, token.text.as_str()))
        .collect();
    assert_eq!(
        new_side,
        vec![
            (TokenKind::Unchanged, "Hello"),
            (TokenKind::Unchanged, " "),
            (TokenKind::Added, "Rust"),
        ]
    );
}

/// REQT-spzdk2z1pk (Line comparison): the whole modified line is one token.
#[test]
fn line_comparison_marks_the_whole_line_as_one_token() {
    let options = LineDiffOptions { compare: CompareMethod::Line, ..LineDiffOptions::default() };

    let diff = line_diff("Hello World", "Hello Rust", &options);

    let new_side: Vec<(TokenKind, &str)> = diff.entries[0]
        .inline_changes
        .as_ref()
        .expect("modified entry carries inline changes")
        .new
        .iter()
        .map(|token| (token.kind, token.text.as_str()))
        .collect();
    assert_eq!(new_side, vec![(TokenKind::Added, "Hello Rust")]);
}

// ---------------------------------------------------------------------------
// Line numbers: REQT-smd01rma2q
// ---------------------------------------------------------------------------

/// REQT-smd01rma2q (Independent numbering)
#[test]
fn each_side_counts_its_own_lines_from_one_more_than_the_offset() {
    let options = LineDiffOptions { line_offset: 5, inline_changes: false, ..LineDiffOptions::default() };

    let diff = line_diff("Hello World", "My Updated Name\nAlso this info", &options);

    assert_eq!(
        rows(&diff),
        vec![
            (
                ChangeKind::Modified,
                Some((6, "Hello World".into())),
                Some((6, "My Updated Name".into()))
            ),
            added(7, "Also this info"),
        ]
    );
}

// ---------------------------------------------------------------------------
// Reference cases from test/compute-lines-test.ts
//
// WORK:crit-rdjdbfyxnt: each case is carried over, or recorded here as
// differing with its cause: `similar`'s alignment, or a defect in the reference.
// ---------------------------------------------------------------------------

#[test]
fn changed_positions_index_the_entries_that_hold_a_change() {
    let options = LineDiffOptions { inline_changes: false, ..LineDiffOptions::default() };

    // Reference cases "diffChars default", "diffWords", "disableWordDiff", and
    // "line counting from offset" all expect [0, 2] over two rows. That is a
    // defect in the reference: its row counter increments twice for a modified
    // pair (src/compute-lines.ts:197-211 with :251), so the second index lands
    // past the last row. The engine reports the real entry positions.
    let diff = line_diff("Hello World", "My Updated Name\nAlso this info", &options);
    assert_eq!(diff.changed_positions, vec![0, 1]);

    // Carried over unchanged: one changed row at index 1.
    let diff = line_diff("test", "test\n    newLine", &options);
    assert_eq!(diff.changed_positions, vec![1]);

    let diff = line_diff("test\n    oldLine", "test", &options);
    assert_eq!(diff.changed_positions, vec![1]);
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

/// Each side's line texts rejoin to that side's trimmed input.
#[test]
fn every_side_s_line_texts_rejoin_to_that_side_s_trimmed_input() {
    let cases = vec![
        ("test", "test\n    newLine"),
        ("test\n    oldLine", "test"),
        ("Hello World", "My Updated Name\nAlso this info"),
        ("first\r\nsecond", "first\nsecond"),
        ("a\nb\nc\n\n  ", "a\nB\nc"),
    ];

    for (old_text, new_text) in cases {
        let diff = line_diff(old_text, new_text, &LineDiffOptions::default());

        let old_rejoined: Vec<String> = diff
            .entries
            .iter()
            .filter_map(|entry| entry.old.as_ref().map(|side| side.text.clone()))
            .collect();
        let new_rejoined: Vec<String> = diff
            .entries
            .iter()
            .filter_map(|entry| entry.new.as_ref().map(|side| side.text.clone()))
            .collect();

        assert_eq!(
            old_rejoined.join("\n"),
            old_text.trim_end().replace("\r\n", "\n"),
            "old side of {old_text:?} against {new_text:?}"
        );
        assert_eq!(
            new_rejoined.join("\n"),
            new_text.trim_end().replace("\r\n", "\n"),
            "new side of {old_text:?} against {new_text:?}"
        );
    }
}

/// Each side's line numbers run contiguously from one more than the offset.
#[test]
fn every_side_s_line_numbers_run_contiguously_from_the_offset() {
    let cases = vec![
        (0, "test", "test\n    newLine"),
        (5, "Hello World", "My Updated Name\nAlso this info"),
        (12, "a\nb\nc", "a\nB\nc\nd"),
    ];

    for (line_offset, old_text, new_text) in cases {
        let options = LineDiffOptions { line_offset, ..LineDiffOptions::default() };
        let diff = line_diff(old_text, new_text, &options);

        let old_numbers: Vec<usize> = diff
            .entries
            .iter()
            .filter_map(|entry| entry.old.as_ref().map(|side| side.number))
            .collect();
        let new_numbers: Vec<usize> = diff
            .entries
            .iter()
            .filter_map(|entry| entry.new.as_ref().map(|side| side.number))
            .collect();

        let expected_old: Vec<usize> = (1..=old_numbers.len()).map(|n| n + line_offset).collect();
        let expected_new: Vec<usize> = (1..=new_numbers.len()).map(|n| n + line_offset).collect();
        assert_eq!(old_numbers, expected_old, "old side at offset {line_offset}");
        assert_eq!(new_numbers, expected_new, "new side at offset {line_offset}");
    }
}
