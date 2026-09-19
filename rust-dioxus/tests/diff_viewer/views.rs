//! Views, change markers, and chips: REQT-2k7j51afde and children, plus
//! REQT-m9r3k5b1ge (Hidden line numbers).

use dioxus::prelude::*;
use dioxus_diff_viewer::styling_hooks::{
    DXDIFF__GUTTER, DXDIFF__LINE_ENDING_ARROW, DXDIFF__LINE_ENDING_CHIP, DXDIFF__WHITESPACE_CHIP,
};
use dioxus_diff_viewer::{
    CompareMethod, DiffOptions, DiffView, DiffViewer, use_diff, use_diff_with,
};

use crate::mounted_app::MountedApp;

/// REQT-ys3yr5g185 (Split view), with the first acceptance criterion: two
/// texts alone show a split diff.
#[test]
fn two_texts_alone_show_old_lines_left_and_new_lines_right() {
    fn app() -> Element {
        let diff = use_diff("a\nb", "a\nc");
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings(),
        vec!["1 |  | a | 1 |  | a", "2 | - | b | 2 | + | c"]
    );
}

/// REQT-0xbgrj9ane (Inline view)
#[test]
fn the_inline_view_shows_a_modified_line_s_old_text_above_its_new_text() {
    fn app() -> Element {
        let diff = use_diff("a\nb", "a\nc");
        rsx! { DiffViewer { diff, view: DiffView::Inline } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings(),
        vec!["1 | 1 |  | a", "2 |  | - | b", " | 2 | + | c"],
        "the unchanged line shows once with both numbers"
    );
}

/// REQT-wjjyqjnjs6 (Change markers): in the split view, a removed line is
/// marked `-` and an added line `+`, beside an empty side.
#[test]
fn the_split_view_marks_removed_and_added_lines() {
    fn app() -> Element {
        let removal = use_diff("a\nold", "a");
        let addition = use_diff("a", "a\nnew");
        rsx! {
            DiffViewer { diff: removal }
            DiffViewer { diff: addition }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings(),
        vec![
            "1 |  | a | 1 |  | a",
            "2 | - | old |  |  | ",
            "1 |  | a | 1 |  | a",
            " |  |  | 2 | + | new",
        ]
    );
}

/// REQT-wjjyqjnjs6 (Change markers): in the inline view, a removed line is
/// marked `-` and an added line `+`, each numbered on its own side only.
#[test]
fn the_inline_view_marks_removed_and_added_lines() {
    fn app() -> Element {
        let removal = use_diff("a\nold", "a");
        let addition = use_diff("a", "a\nnew");
        rsx! {
            DiffViewer { diff: removal, view: DiffView::Inline }
            DiffViewer { diff: addition, view: DiffView::Inline }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings(),
        vec![
            "1 | 1 |  | a",
            "2 |  | - | old",
            "1 | 1 |  | a",
            " | 2 | + | new"
        ]
    );
}

/// REQT-4zyjfjrhd3 (Line ending chips): in the split view, each side's
/// terminator shows as escaped text on its own side's line.
#[test]
fn each_side_of_a_line_ending_change_shows_its_terminator_as_a_chip() {
    fn app() -> Element {
        let diff = use_diff("a\r\nb", "a\nb");
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.texts_with_class(DXDIFF__LINE_ENDING_CHIP),
        vec!["\\r\\n", "\\n"]
    );
    assert_eq!(viewer.row_readings()[0], "1 |  | a\\r\\n | 1 |  | a\\n");
}

/// REQT-4zyjfjrhd3 (Line ending chips): an inline unchanged line shows the old
/// side's chip, an arrow, then the new side's chip.
#[test]
fn an_inline_unchanged_line_shows_its_old_chip_an_arrow_and_its_new_chip() {
    fn app() -> Element {
        let diff = use_diff("a\r\nb", "a\nb");
        rsx! { DiffViewer { diff, view: DiffView::Inline } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings()[0], "1 | 1 |  | a\\r\\n→\\n");
    let arrows = viewer.tree().elements_with_class(DXDIFF__LINE_ENDING_ARROW);
    assert_eq!(arrows.len(), 1);
    assert_eq!(arrows[0].text(), "→");
    assert_eq!(
        arrows[0].attribute("aria-label"),
        Some("changed to"),
        "screen readers read the arrow as a phrase"
    );
}

/// REQT-27a1gxq15f (Whitespace chips): both sides' lines of a whitespace change
/// show a `WS` chip, in both views.
#[test]
fn both_sides_of_a_whitespace_change_show_a_ws_chip() {
    fn app() -> Element {
        let diff = use_diff_with(
            "  a\nz",
            "a\nz",
            DiffOptions {
                compare: CompareMethod::TrimmedLine,
                ..DiffOptions::default()
            },
        );
        rsx! {
            DiffViewer { diff }
            DiffViewer { diff, view: DiffView::Inline }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.texts_with_class(DXDIFF__WHITESPACE_CHIP),
        vec!["WS", "WS", "WS", "WS"]
    );
    assert_eq!(viewer.row_readings()[0], "1 | - |   aWS | 1 | + | aWS");
}

/// REQT-m9r3k5b1ge (Hidden line numbers)
#[test]
fn line_numbers_are_hidden_when_the_consumer_hides_them() {
    fn app() -> Element {
        let diff = use_diff("a\nb", "a\nc");
        rsx! {
            DiffViewer { diff, show_line_numbers: false }
            DiffViewer { diff, show_line_numbers: false, view: DiffView::Inline }
        }
    }

    let viewer = MountedApp::new(app);

    assert!(viewer.texts_with_class(DXDIFF__GUTTER).is_empty());
    assert_eq!(
        viewer.row_readings(),
        vec![" | a |  | a", "- | b | + | c", " | a", "- | b", "+ | c"]
    );
}
