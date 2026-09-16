//! Pre-work test drafts for the line diff engine (WORK-p7e70a5mwf).
//!
//! Drafted by the Rust Tester during pre-work review, before the crate exists.
//! Every test names one in-scope requirement. The call shape below (`line_diff`,
//! `LineDiffOptions`, `LineDiff`, `PairedLineEntry`) follows the candidate
//! architecture: ARCH-jf3s5npp9s (LineDiffEngine), ARCH-n7wmjnmt65
//! (LineDiffOptions), ARCH-exgfx6rwdt (LineDiff), ARCH-kjjykjtt5r
//! (PairedLineEntry). The worker owns the exact spelling; what these tests hold
//! to is the observable output each requirement names.

use dioxus_diff_viewer::{
    ChangeKind, CompareMethod, LineDiff, LineDiffOptions, TokenKind, line_diff,
};
use proptest::prelude::*;

// ---------------------------------------------------------------------------
// Reading helpers. These keep each test's assert section to one line of intent.
// ---------------------------------------------------------------------------

/// A row as the tests read it: change kind, then each side's number and text.
type Row = (ChangeKind, Option<(usize, String)>, Option<(usize, String)>);

/// One `Row` per entry: change kind, old side, new side.
fn rows(diff: &LineDiff) -> Vec<Row> {
    diff.entries
        .iter()
        .map(|entry| {
            (
                entry.change,
                entry
                    .old
                    .as_ref()
                    .map(|side| (side.number, side.text.clone())),
                entry
                    .new
                    .as_ref()
                    .map(|side| (side.number, side.text.clone())),
            )
        })
        .collect()
}

fn unchanged(number: usize, text: &str) -> Row {
    (
        ChangeKind::Unchanged,
        Some((number, text.into())),
        Some((number, text.into())),
    )
}

fn added(number: usize, text: &str) -> Row {
    (ChangeKind::Added, None, Some((number, text.into())))
}

fn removed(number: usize, text: &str) -> Row {
    (ChangeKind::Removed, Some((number, text.into())), None)
}

/// Old-side inline tokens of one entry, rejoined.
fn old_tokens_rejoined(diff: &LineDiff, index: usize) -> String {
    assert_inline_changes(diff, index);
    diff.entries[index]
        .old_inline_tokens()
        .map(|(_, text)| text)
        .collect()
}

fn new_tokens_rejoined(diff: &LineDiff, index: usize) -> String {
    assert_inline_changes(diff, index);
    diff.entries[index]
        .new_inline_tokens()
        .map(|(_, text)| text)
        .collect()
}

/// One side's tokens as kind-and-text pairs, for tests that check granularity.
fn old_tokens(diff: &LineDiff, index: usize) -> Vec<(TokenKind, &str)> {
    assert_inline_changes(diff, index);
    diff.entries[index].old_inline_tokens().collect()
}

fn new_tokens(diff: &LineDiff, index: usize) -> Vec<(TokenKind, &str)> {
    assert_inline_changes(diff, index);
    diff.entries[index].new_inline_tokens().collect()
}

fn assert_inline_changes(diff: &LineDiff, index: usize) {
    assert!(
        diff.entries[index].inline_changes.is_some(),
        "entry {index} carries inline changes"
    );
}

// ---------------------------------------------------------------------------
// Line changes: REQT-wqffp3d26e and children
// ---------------------------------------------------------------------------

/// REQT-hmsfnfe5wc (Line marking)
#[test]
fn a_line_only_the_new_text_has_is_added_and_the_shared_line_is_unchanged() {
    let diff = line_diff("test", "test\n    newLine", &LineDiffOptions::default());

    assert_eq!(
        rows(&diff),
        vec![unchanged(1, "test"), added(2, "    newLine")]
    );
}

/// REQT-hmsfnfe5wc (Line marking)
#[test]
fn a_line_only_the_old_text_has_is_removed() {
    let diff = line_diff("test\n    oldLine", "test", &LineDiffOptions::default());

    assert_eq!(
        rows(&diff),
        vec![unchanged(1, "test"), removed(2, "    oldLine")]
    );
}

/// REQT-hmsfnfe5wc (Line marking): lines differing only in their terminator align as unchanged.
#[test]
fn the_same_two_lines_under_windows_and_unix_endings_are_unchanged() {
    let diff = line_diff(
        "first\r\nsecond",
        "first\nsecond",
        &LineDiffOptions::default(),
    );

    assert_eq!(
        rows(&diff),
        vec![unchanged(1, "first"), unchanged(2, "second")],
        "line identity ignores the terminator"
    );
}

/// REQT-dqxm8fa7ts (Modified lines)
#[test]
fn a_removed_line_followed_by_an_added_line_is_one_modified_entry() {
    let options = LineDiffOptions {
        mark_inline_changes: false,
        ..LineDiffOptions::default()
    };

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
    let options = LineDiffOptions {
        mark_inline_changes: false,
        ..LineDiffOptions::default()
    };

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
    let diff = line_diff(
        "Hello World",
        "My Updated Name",
        &LineDiffOptions::default(),
    );

    assert_eq!(old_tokens_rejoined(&diff, 0), "Hello World");
    assert_eq!(new_tokens_rejoined(&diff, 0), "My Updated Name");
}

/// REQT-4nz35dscrn (Inline changes)
#[test]
fn a_modified_line_carries_no_inline_changes_when_they_are_turned_off() {
    let options = LineDiffOptions {
        mark_inline_changes: false,
        ..LineDiffOptions::default()
    };

    let diff = line_diff("Hello World", "My Updated Name", &options);

    assert!(diff.entries[0].inline_changes.is_none());
}

/// REQT-9tze98pt6g (Trailing whitespace)
#[test]
fn trailing_blank_lines_in_either_text_are_not_a_change() {
    let diff = line_diff(
        "test\n\n\n    ",
        "test\n\n    ",
        &LineDiffOptions::default(),
    );

    assert_eq!(rows(&diff), vec![unchanged(1, "test")]);
    assert!(diff.changed_positions.is_empty());
}

/// REQT-rtwn1qresp (Line ending changes)
#[test]
fn a_pair_whose_terminators_differ_reports_the_ending_change_with_both_terminators() {
    let diff = line_diff(
        "first\r\nsecond",
        "first\nsecond",
        &LineDiffOptions::default(),
    );

    let ending_change = diff.entries[0]
        .line_ending_change
        .as_ref()
        .expect("the first pair's terminators differ");
    assert_eq!(
        (ending_change.old.as_str(), ending_change.new.as_str()),
        ("\r\n", "\n")
    );
    assert!(
        diff.entries[1].line_ending_change.is_none(),
        "the last lines have no terminator to differ in"
    );
}

/// REQT-rtwn1qresp (Line ending changes) with REQT-qcnxhemvhn (Folded unchanged
/// lines): the pair holds a changed position so the component cannot fold its
/// chip away, while both sides still read unchanged.
#[test]
fn a_pair_carrying_a_line_ending_change_holds_a_changed_position() {
    let diff = line_diff(
        "first\r\nsecond",
        "first\nsecond",
        &LineDiffOptions::default(),
    );

    assert_eq!(diff.changed_positions, vec![0]);
    assert_eq!(
        rows(&diff),
        vec![unchanged(1, "first"), unchanged(2, "second")],
        "the pair still reads unchanged in both views"
    );
}

// ---------------------------------------------------------------------------
// Compare methods: REQT-rvcg21axa8 and children
// ---------------------------------------------------------------------------

/// REQT-czecf8krqc (Character comparison): the default splits below word level.
#[test]
fn inline_changes_compare_characters_unless_another_method_is_chosen() {
    let diff = line_diff("Hello World", "Hello Word", &LineDiffOptions::default());

    let removed_text: String = old_tokens(&diff, 0)
        .into_iter()
        .filter(|(kind, _)| *kind == TokenKind::Removed)
        .map(|(_, text)| text)
        .collect();
    assert_eq!(removed_text, "l", "only the dropped character is marked");
}

/// REQT-xzc8n354h1 (Word comparison): each whitespace run and each non-whitespace run is one token.
#[test]
fn word_comparison_keeps_whitespace_runs_as_their_own_tokens() {
    let options = LineDiffOptions {
        compare: CompareMethod::Word,
        ..LineDiffOptions::default()
    };

    let diff = line_diff("Hello World", "Hello Rust", &options);

    let new_side = new_tokens(&diff, 0);
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
    let options = LineDiffOptions {
        compare: CompareMethod::Line,
        ..LineDiffOptions::default()
    };

    let diff = line_diff("Hello World", "Hello Rust", &options);

    let new_side = new_tokens(&diff, 0);
    assert_eq!(new_side, vec![(TokenKind::Added, "Hello Rust")]);
}

// ---------------------------------------------------------------------------
// Line numbers: REQT-smd01rma2q
// ---------------------------------------------------------------------------

/// REQT-smd01rma2q (Independent numbering)
#[test]
fn each_side_counts_its_own_lines_from_one_more_than_the_offset() {
    let options = LineDiffOptions {
        line_offset: 5,
        mark_inline_changes: false,
        ..LineDiffOptions::default()
    };

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
// All nine reference cases are accounted for across the three tests below.
// ---------------------------------------------------------------------------

/// Carried over: the paired lines of every reference case, named by that case.
///
/// The reference records an added-only row as an empty `left` object; the
/// engine records it as an absent old side, which is the same row.
#[test]
fn every_reference_case_pairs_its_lines_as_the_reference_does() {
    let name_pair = (
        ChangeKind::Modified,
        Some((1, "Hello World".into())),
        Some((1, "My Updated Name".into())),
    );
    let cases: Vec<(&str, &str, &str, LineDiffOptions, Vec<Row>)> = vec![
        (
            "Should it avoid trailing spaces",
            "test\n\n\n    ",
            "test\n\n    ",
            LineDiffOptions::default(),
            vec![unchanged(1, "test")],
        ),
        (
            "Should identify line addition",
            "test",
            "test\n    newLine",
            LineDiffOptions::default(),
            vec![unchanged(1, "test"), added(2, "    newLine")],
        ),
        (
            "Should identify line deletion",
            "test\n    oldLine",
            "test",
            LineDiffOptions::default(),
            vec![unchanged(1, "test"), removed(2, "    oldLine")],
        ),
        (
            "Should identify line modification",
            "test\n    oldLine",
            "test\n    newLine",
            LineDiffOptions {
                mark_inline_changes: false,
                ..LineDiffOptions::default()
            },
            vec![
                unchanged(1, "test"),
                (
                    ChangeKind::Modified,
                    Some((2, "    oldLine".into())),
                    Some((2, "    newLine".into())),
                ),
            ],
        ),
        (
            "Should identify word diff",
            "test\n    oldLine",
            "test\n    newLine",
            LineDiffOptions::default(),
            vec![
                unchanged(1, "test"),
                (
                    ChangeKind::Modified,
                    Some((2, "    oldLine".into())),
                    Some((2, "    newLine".into())),
                ),
            ],
        ),
        (
            "Should call \"diffChars\" jsDiff method when compareMethod is not provided",
            "Hello World",
            "My Updated Name\nAlso this info",
            LineDiffOptions::default(),
            vec![name_pair.clone(), added(2, "Also this info")],
        ),
        (
            "Should call \"diffWords\" jsDiff method when a compareMethod IS provided",
            "Hello World",
            "My Updated Name\nAlso this info",
            LineDiffOptions {
                compare: CompareMethod::Word,
                ..LineDiffOptions::default()
            },
            vec![name_pair.clone(), added(2, "Also this info")],
        ),
        (
            "Should not call jsDiff method and not diff text when disableWordDiff is true",
            "Hello World",
            "My Updated Name\nAlso this info",
            LineDiffOptions {
                mark_inline_changes: false,
                ..LineDiffOptions::default()
            },
            vec![name_pair, added(2, "Also this info")],
        ),
        (
            "Should start line counting from offset",
            "Hello World",
            "My Updated Name\nAlso this info",
            LineDiffOptions {
                compare: CompareMethod::Word,
                mark_inline_changes: false,
                line_offset: 5,
            },
            vec![
                (
                    ChangeKind::Modified,
                    Some((6, "Hello World".into())),
                    Some((6, "My Updated Name".into())),
                ),
                added(7, "Also this info"),
            ],
        ),
    ];

    for (reference_case, old_text, new_text, options, expected) in cases {
        let diff = line_diff(old_text, new_text, &options);

        assert_eq!(rows(&diff), expected, "reference case {reference_case:?}");
    }
}

/// Differs, cause: a defect in the reference.
///
/// Reference cases "diffChars default", "diffWords", "disableWordDiff", and
/// "line counting from offset" all expect `diffLines` of [0, 2] over two rows.
/// The reference's row counter increments twice for a modified pair
/// (src/compute-lines.ts:197-211 with :251), so the second index lands past the
/// last row. The engine reports the real entry positions.
#[test]
fn changed_positions_index_the_entries_that_hold_a_change() {
    let options = LineDiffOptions {
        mark_inline_changes: false,
        ..LineDiffOptions::default()
    };

    let diff = line_diff("Hello World", "My Updated Name\nAlso this info", &options);
    assert_eq!(diff.changed_positions, vec![0, 1]);

    // Carried over unchanged: one changed row at index 1.
    let diff = line_diff("test", "test\n    newLine", &options);
    assert_eq!(diff.changed_positions, vec![1]);

    let diff = line_diff("test\n    oldLine", "test", &options);
    assert_eq!(diff.changed_positions, vec![1]);

    // Carried over: no changed rows when only trailing whitespace differs.
    let diff = line_diff("test\n\n\n    ", "test\n\n    ", &options);
    assert!(diff.changed_positions.is_empty());
}

/// Differs, cause: `similar`'s token granularity.
///
/// Reference cases "Should identify word diff" and "diffChars default" expect
/// consecutive same-kind characters merged into one run, because jsdiff's
/// change objects are runs. `similar` reports one change per token, so the
/// engine emits one inline token per character. No information is lost: runs
/// are recoverable from tokens, not the other way round, and merging would
/// contradict word comparison, where adjacent unchanged tokens stay separate.
#[test]
fn character_comparison_emits_one_inline_token_per_character() {
    let diff = line_diff(
        "test\n    oldLine",
        "test\n    newLine",
        &LineDiffOptions::default(),
    );

    let old_side = old_tokens(&diff, 1);
    assert_eq!(
        old_side,
        vec![
            (TokenKind::Unchanged, " "),
            (TokenKind::Unchanged, " "),
            (TokenKind::Unchanged, " "),
            (TokenKind::Unchanged, " "),
            (TokenKind::Removed, "o"),
            (TokenKind::Removed, "l"),
            (TokenKind::Removed, "d"),
            (TokenKind::Unchanged, "L"),
            (TokenKind::Unchanged, "i"),
            (TokenKind::Unchanged, "n"),
            (TokenKind::Unchanged, "e"),
        ],
        "the reference merges these into '    ', 'old', 'Line'"
    );
}

// ---------------------------------------------------------------------------
// Properties
//
// The generated properties below deliberately do not restate how the engine
// splits lines; a test that re-implements the code under test cannot fail with
// it. They assert what must hold whatever the split is: no character is lost or
// invented, and each side's numbers run contiguously. The exact split of chosen
// texts is asserted by the example test that follows them, and by the
// reference-case table above.
// ---------------------------------------------------------------------------

/// A text built from short lines, each ended by one of the three terminators
/// the engine recognizes. The last terminator makes the text end on a line
/// break as often as not.
fn text() -> impl Strategy<Value = String> {
    proptest::collection::vec(
        (
            "[a-zA-Z ]{0,10}",
            prop_oneof![Just("\n"), Just("\r\n"), Just("\r")],
        ),
        0..8,
    )
    .prop_map(|lines| {
        lines
            .iter()
            .map(|(line, ending)| format!("{line}{ending}"))
            .collect()
    })
}

/// One side's line texts, in entry order.
fn side_texts(diff: &LineDiff, side: Side) -> Vec<String> {
    diff.entries
        .iter()
        .filter_map(|entry| match side {
            Side::Old => entry.old.as_ref(),
            Side::New => entry.new.as_ref(),
        })
        .map(|line| line.text.clone())
        .collect()
}

#[derive(Clone, Copy)]
enum Side {
    Old,
    New,
}

/// That side's characters, minus the terminators the engine holds aside.
fn characters_without_terminators(text: &str) -> String {
    text.trim_end()
        .chars()
        .filter(|character| *character != '\n' && *character != '\r')
        .collect()
}

proptest! {
    /// Each side's line texts together hold exactly that side's characters,
    /// in order, with the terminators removed: nothing lost, nothing invented.
    #[test]
    fn each_side_s_line_texts_hold_every_character_of_that_side_s_trimmed_input(
        old_text in text(),
        new_text in text(),
    ) {
        let diff = line_diff(&old_text, &new_text, &LineDiffOptions::default());

        prop_assert_eq!(
            side_texts(&diff, Side::Old).concat(),
            characters_without_terminators(&old_text)
        );
        prop_assert_eq!(
            side_texts(&diff, Side::New).concat(),
            characters_without_terminators(&new_text)
        );
    }

    /// Each side's line numbers run contiguously from one more than the offset.
    #[test]
    fn every_side_s_line_numbers_run_contiguously_from_the_offset(
        old_text in text(),
        new_text in text(),
        line_offset in 0usize..50,
    ) {
        let options = LineDiffOptions {
            line_offset,
            ..LineDiffOptions::default()
        };
        let diff = line_diff(&old_text, &new_text, &options);

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
        prop_assert_eq!(old_numbers, expected_old);
        prop_assert_eq!(new_numbers, expected_new);
    }
}

/// The chosen pairs the generated properties cannot name: each text's exact
/// split, including trailing blank lines and each of the three terminators.
#[test]
fn chosen_text_pairs_split_into_the_lines_expected() {
    let cases: Vec<(&str, &str, Vec<&str>, Vec<&str>)> = vec![
        (
            "test",
            "test\n    newLine",
            vec!["test"],
            vec!["test", "    newLine"],
        ),
        (
            "test\n    oldLine",
            "test",
            vec!["test", "    oldLine"],
            vec!["test"],
        ),
        (
            "Hello World",
            "My Updated Name\nAlso this info",
            vec!["Hello World"],
            vec!["My Updated Name", "Also this info"],
        ),
        (
            "first\r\nsecond",
            "first\nsecond",
            vec!["first", "second"],
            vec!["first", "second"],
        ),
        (
            "a\nb\nc\n\n  ",
            "a\nB\nc",
            vec!["a", "b", "c"],
            vec!["a", "B", "c"],
        ),
        ("a\rb", "a\nb", vec!["a", "b"], vec!["a", "b"]),
    ];

    for (old_text, new_text, old_lines, new_lines) in cases {
        let diff = line_diff(old_text, new_text, &LineDiffOptions::default());

        assert_eq!(
            side_texts(&diff, Side::Old),
            old_lines,
            "old side of {old_text:?}"
        );
        assert_eq!(
            side_texts(&diff, Side::New),
            new_lines,
            "new side of {new_text:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Reference quirks the engine deliberately does not carry
// ---------------------------------------------------------------------------

/// The reference substitutes a single space for an empty line's text
/// (`src/compute-lines.ts:192`, `line || ' '`), which is display padding. The
/// engine keeps the line empty and leaves padding to the component.
#[test]
fn an_empty_removed_line_keeps_its_empty_text_rather_than_a_space() {
    let options = LineDiffOptions {
        mark_inline_changes: false,
        ..LineDiffOptions::default()
    };

    let diff = line_diff("first\n\nlast", "first\nlast", &options);

    assert_eq!(
        rows(&diff),
        vec![
            unchanged(1, "first"),
            removed(2, ""),
            (
                ChangeKind::Unchanged,
                Some((3, "last".into())),
                Some((2, "last".into()))
            ),
        ]
    );
}
