//! REQT-3928hx46s3 (Styling hooks): each element kind carries its `dxdiff`
//! class and a class for its change or highlight state. These assert classes
//! rather than element structure, which is what apps and themes rely on.

use dioxus::prelude::*;
use dioxus_diff_viewer::styling_hooks::*;
use dioxus_diff_viewer::{CompareMethod, DiffViewer, LineId};

use crate::mounted_app::MountedApp;

/// Each element carrying `kind_class`, read as its other `dxdiff` classes.
fn states_of(viewer: &MountedApp, kind_class: &str) -> Vec<String> {
    viewer
        .tree()
        .elements_with_class(kind_class)
        .iter()
        .map(|element| {
            element
                .classes()
                .into_iter()
                .filter(|class| *class != kind_class && class.starts_with("dxdiff-"))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

/// Two modified pairs (the first with a whitespace change and a line ending
/// change, the second highlighted), ten unchanged lines whose middle four fold,
/// and a removed line.
fn app() -> Element {
    rsx! {
        DiffViewer {
            old_text: "  kept\r\nab\nl1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\nl9\nl10\ngone",
            new_text: "kept\nac\nl1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\nl9\nl10",
            compare: CompareMethod::TrimmedLine,
            highlighted_lines: vec![LineId::New(2)],
            left_title: rsx! { "Old" },
        }
    }
}

#[test]
fn rows_carry_their_kind_and_change_state() {
    let viewer = MountedApp::new(app);

    let mut expected_rows = vec!["dxdiff-modified", "dxdiff-modified dxdiff-highlighted"];
    expected_rows.extend(["dxdiff-unchanged"; 6]);
    expected_rows.push("dxdiff-removed");
    assert_eq!(states_of(&viewer, DXDIFF__ROW), expected_rows);
    assert_eq!(
        states_of(&viewer, DXDIFF__FOLD_ROW),
        vec!["dxdiff-unchanged"]
    );
    // The control expanding a fold carries its own kind class, so an app
    // restyles it without naming the element the fold row holds it in. It
    // shows no line, so no change or highlight state is relevant to it.
    assert_eq!(states_of(&viewer, DXDIFF__FOLD_CONTROL), vec![""]);
    assert_eq!(states_of(&viewer, DXDIFF__TITLE), vec!["", ""]);
}

#[test]
fn line_cells_carry_their_kind_and_side_state() {
    let viewer = MountedApp::new(app);

    let mut expected_line_cells = vec![
        "dxdiff-removed",
        "dxdiff-added",
        "dxdiff-removed",
        "dxdiff-added dxdiff-highlighted",
    ];
    expected_line_cells.extend(["dxdiff-unchanged"; 12]);
    expected_line_cells.extend(["dxdiff-removed", "dxdiff-empty"]);
    assert_eq!(states_of(&viewer, DXDIFF__CONTENT), expected_line_cells);

    // Marker and gutter cells also appear in the fold row, after the first
    // three unchanged rows.
    let mut expected_with_fold_row = expected_line_cells.clone();
    expected_with_fold_row.insert(10, "dxdiff-unchanged");
    assert_eq!(
        states_of(&viewer, DXDIFF__CHANGE_MARKER),
        expected_with_fold_row
    );
    assert_eq!(states_of(&viewer, DXDIFF__GUTTER), expected_with_fold_row);
}

#[test]
fn tokens_and_chips_carry_their_kind_and_state() {
    let viewer = MountedApp::new(app);

    assert_eq!(
        states_of(&viewer, DXDIFF__INLINE_TOKEN),
        vec![
            "dxdiff-unchanged",
            "dxdiff-unchanged",
            "dxdiff-removed",
            "dxdiff-added"
        ]
    );
    assert_eq!(
        states_of(&viewer, DXDIFF__LINE_ENDING_CHIP),
        vec!["dxdiff-removed", "dxdiff-added"]
    );
    assert!(
        states_of(&viewer, DXDIFF__LINE_ENDING_ARROW).is_empty(),
        "the split view shows no arrow"
    );
    assert_eq!(
        states_of(&viewer, DXDIFF__WHITESPACE_CHIP),
        vec!["dxdiff-removed", "dxdiff-added"]
    );
}

#[test]
fn the_viewer_carries_its_view() {
    let viewer = MountedApp::new(app);

    assert_eq!(
        states_of(&viewer, DXDIFF__VIEWER),
        vec!["dxdiff-split-view"]
    );
}

#[test]
fn the_inline_line_ending_arrow_carries_its_kind_and_state() {
    fn inline_app() -> Element {
        rsx! {
            DiffViewer { old_text: "a\r\nb", new_text: "a\nb", view: dioxus_diff_viewer::DiffView::Inline }
        }
    }

    let viewer = MountedApp::new(inline_app);

    assert_eq!(
        states_of(&viewer, DXDIFF__LINE_ENDING_ARROW),
        vec!["dxdiff-unchanged"]
    );
    assert_eq!(
        states_of(&viewer, DXDIFF__LINE_ENDING_CHIP),
        vec!["dxdiff-unchanged", "dxdiff-unchanged"]
    );
}
