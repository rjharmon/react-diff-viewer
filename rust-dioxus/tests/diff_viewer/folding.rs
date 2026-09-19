//! Folding: REQT-qerexp825r and children.

use dioxus::prelude::*;
use dioxus_diff_viewer::{
    DiffOptions, DiffView, DiffViewer, HiddenLines, LineId, use_diff, use_diff_with,
};

use crate::mounted_app::MountedApp;

/// Ten lines with the sixth changed.
const OLD_TEN_LINES: &str = "l1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\nl9\nl10";
const NEW_TEN_LINES: &str = "l1\nl2\nl3\nl4\nl5\nL6\nl7\nl8\nl9\nl10";

fn unchanged_row(number: usize) -> String {
    format!("{number} |  | l{number} | {number} |  | l{number}")
}

fn fold_row(hidden_line_count: usize) -> String {
    format!(" |  | Expand {hidden_line_count} lines ...")
}

/// The ten-line fixture's rows with no fold expanded, around surrounding
/// count 3.
fn folded_ten_line_rows() -> Vec<String> {
    let mut rows = vec![fold_row(2)];
    rows.extend((3..=5).map(unchanged_row));
    rows.push("6 | - | l6 | 6 | + | L6".into());
    rows.extend((7..=9).map(unchanged_row));
    rows.push(fold_row(1));
    rows
}

/// REQT-qcnxhemvhn (Folded unchanged lines): lines more than three lines away
/// from every change fold by default, each run into one fold row.
#[test]
fn unchanged_lines_beyond_three_lines_from_a_change_fold_away() {
    fn app() -> Element {
        let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings(), folded_ten_line_rows());
}

/// REQT-qcnxhemvhn (Folded unchanged lines): the consumer's surrounding-line
/// count sets how many unchanged lines stay shown around a change.
#[test]
fn the_surrounding_line_count_sets_how_many_unchanged_lines_stay_shown() {
    fn app() -> Element {
        let diff = use_diff_with(
            OLD_TEN_LINES,
            NEW_TEN_LINES,
            DiffOptions {
                surrounding_line_count: 1,
                ..DiffOptions::default()
            },
        );
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings(),
        vec![
            fold_row(4),
            unchanged_row(5),
            "6 | - | l6 | 6 | + | L6".into(),
            unchanged_row(7),
            fold_row(3),
        ]
    );
}

/// REQT-qcnxhemvhn (Folded unchanged lines)
#[test]
fn every_line_shows_when_folding_is_turned_off() {
    fn app() -> Element {
        let diff = use_diff_with(
            OLD_TEN_LINES,
            NEW_TEN_LINES,
            DiffOptions {
                fold_unchanged_lines: false,
                ..DiffOptions::default()
            },
        );
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings().len(), 10);
    assert!(
        viewer
            .row_readings()
            .iter()
            .all(|row| !row.contains("Expand"))
    );
}

/// REQT-qcnxhemvhn (Folded unchanged lines): with no change at all, every line
/// is far from every change.
#[test]
fn identical_texts_fold_into_one_fold_row() {
    fn app() -> Element {
        let diff = use_diff(OLD_TEN_LINES, OLD_TEN_LINES);
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings(), vec![fold_row(10)]);
}

/// REQT-1tdrfvay4q (Fold rows): a consumer-rendered fold row receives the
/// hidden-line count and the first hidden line's old and new numbers.
#[test]
fn a_consumer_renders_a_fold_row_from_its_hidden_lines() {
    fn app() -> Element {
        let diff = use_diff(
            "gone\nl1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\nl9",
            "l1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\nl9",
        );
        rsx! {
            DiffViewer {
                diff,
                fold_row_renderer: move |hidden: HiddenLines| rsx! {
                    "{hidden.count} hidden from L-{hidden.first_old_number} R-{hidden.first_new_number}"
                },
            }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings().last().map(String::as_str),
        Some(" |  | 6 hidden from L-5 R-4")
    );
}

/// The ten-line fixture with buttons that change the viewer's inputs.
fn ten_lines_with_input_controls() -> Element {
    let mut view = use_signal(|| DiffView::Split);
    let mut new_text = use_signal(|| NEW_TEN_LINES.to_string());
    let mut surrounding_line_count = use_signal(|| 3_usize);
    let diff = use_diff_with(
        OLD_TEN_LINES,
        new_text(),
        DiffOptions {
            surrounding_line_count: surrounding_line_count(),
            ..DiffOptions::default()
        },
    );
    rsx! {
        button { onclick: move |_| view.set(DiffView::Inline), "show inline" }
        button { onclick: move |_| new_text.set(NEW_TEN_LINES.replace("L6", "M6")), "edit new text" }
        button { onclick: move |_| surrounding_line_count.set(2), "surround by two" }
        DiffViewer { diff, view: view() }
    }
}

/// REQT-v748c7mjr6 (Expanding folds)
#[test]
fn activating_a_fold_row_reveals_the_lines_it_hides() {
    let mut viewer = MountedApp::new(ten_lines_with_input_controls);

    viewer.expand_first_fold();

    let mut expected = vec![unchanged_row(1), unchanged_row(2)];
    expected.extend(folded_ten_line_rows().into_iter().skip(1));
    assert_eq!(viewer.row_readings(), expected);
}

/// REQT-v748c7mjr6 (Expanding folds): a change other than the texts or the
/// surrounding-line count leaves expansions alone.
#[test]
fn revealed_lines_stay_revealed_when_the_view_changes() {
    let mut viewer = MountedApp::new(ten_lines_with_input_controls);
    viewer.expand_first_fold();

    viewer.click_button("show inline");

    assert_eq!(
        viewer.row_readings()[..2],
        ["1 | 1 |  | l1", "2 | 2 |  | l2"]
    );
}

/// REQT-v748c7mjr6 (Expanding folds): changed texts return every fold to
/// folded, even where the folds keep their positions.
#[test]
fn revealed_lines_fold_again_when_the_texts_change() {
    let mut viewer = MountedApp::new(ten_lines_with_input_controls);
    viewer.expand_first_fold();

    viewer.click_button("edit new text");

    assert_eq!(viewer.row_readings()[0], fold_row(2));
}

/// REQT-v748c7mjr6 (Expanding folds): a changed surrounding-line count returns
/// every fold to folded, even where a fold keeps its first line.
#[test]
fn revealed_lines_fold_again_when_the_surrounding_line_count_changes() {
    let mut viewer = MountedApp::new(ten_lines_with_input_controls);
    viewer.expand_first_fold();

    viewer.click_button("surround by two");

    assert_eq!(viewer.row_readings()[0], fold_row(3));
}

fn ten_lines_with_fold_reset() -> Element {
    let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
    rsx! {
        button { onclick: move |_| diff.reset_folds(), "reset folds" }
        DiffViewer { diff }
    }
}

/// REQT-ps5zx85jvc (Resetting folds)
#[test]
fn an_app_returns_every_expanded_fold_to_folded_in_one_action() {
    let mut viewer = MountedApp::new(ten_lines_with_fold_reset);
    viewer.expand_first_fold();
    viewer.expand_first_fold();
    assert!(
        viewer
            .row_readings()
            .iter()
            .all(|row| !row.contains("Expand"))
    );

    viewer.click_button("reset folds");

    assert_eq!(viewer.row_readings(), folded_ten_line_rows());
}

/// REQT-ps5zx85jvc (Resetting folds): a reset does not stop later expansions.
#[test]
fn a_fold_expands_again_after_a_reset() {
    let mut viewer = MountedApp::new(ten_lines_with_fold_reset);
    viewer.expand_first_fold();
    viewer.click_button("reset folds");

    viewer.expand_first_fold();

    assert_eq!(viewer.row_readings()[0], unchanged_row(1));
}

/// The ten-line fixture with buttons driving the diff's fold actions from
/// app code, each naming a line rather than a fold.
fn ten_lines_with_fold_actions() -> Element {
    let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
    rsx! {
        button { onclick: move |_| diff.expand_fold_at_line(LineId::Old(1)), "reveal old line 1" }
        button { onclick: move |_| diff.expand_fold_at_line(LineId::New(10)), "reveal new line 10" }
        button { onclick: move |_| diff.expand_fold_at_line(LineId::Old(5)), "reveal old line 5" }
        button { onclick: move |_| diff.expand_fold_at_line(LineId::Old(99)), "reveal old line 99" }
        button { onclick: move |_| diff.expand_all_folds(), "expand everything" }
        DiffViewer { diff }
    }
}

/// REQT-g86vdmyyp9 (Expanding at a line): naming one hidden line reveals the
/// whole run of hidden lines it sits in.
#[test]
fn naming_a_hidden_line_reveals_the_whole_run_it_sits_in() {
    let mut viewer = MountedApp::new(ten_lines_with_fold_actions);

    viewer.click_button("reveal old line 1");

    let mut expected = vec![unchanged_row(1), unchanged_row(2)];
    expected.extend(folded_ten_line_rows().into_iter().skip(1));
    assert_eq!(viewer.row_readings(), expected);
}

/// REQT-g86vdmyyp9 (Expanding at a line): a line is named on either side, so a
/// new-side line id reaches the fold hiding it.
#[test]
fn naming_a_hidden_line_on_the_new_side_reveals_its_run() {
    let mut viewer = MountedApp::new(ten_lines_with_fold_actions);

    viewer.click_button("reveal new line 10");

    let mut expected = folded_ten_line_rows();
    expected.pop();
    expected.push(unchanged_row(10));
    assert_eq!(viewer.row_readings(), expected);
}

/// REQT-g86vdmyyp9 (Expanding at a line): a named line that is not hidden
/// leaves the shown lines alone.
#[test]
fn naming_a_line_that_is_already_shown_leaves_the_shown_lines_alone() {
    let mut viewer = MountedApp::new(ten_lines_with_fold_actions);

    viewer.click_button("reveal old line 5");

    assert_eq!(viewer.row_readings(), folded_ten_line_rows());
}

/// REQT-g86vdmyyp9 (Expanding at a line): a line the diff does not hold at all
/// leaves the shown lines alone rather than opening some other fold.
#[test]
fn naming_a_line_the_diff_does_not_hold_leaves_the_shown_lines_alone() {
    let mut viewer = MountedApp::new(ten_lines_with_fold_actions);

    viewer.click_button("reveal old line 99");

    assert_eq!(viewer.row_readings(), folded_ten_line_rows());
}

/// REQT-h8rxvhpj9g (Expanding everything): one action reveals every line the
/// folds hide, wherever those folds sit.
#[test]
fn an_app_reveals_every_hidden_line_in_one_action() {
    let mut viewer = MountedApp::new(ten_lines_with_fold_actions);

    viewer.click_button("expand everything");

    let mut expected: Vec<String> = (1..=5).map(unchanged_row).collect();
    expected.push("6 | - | l6 | 6 | + | L6".into());
    expected.extend((7..=10).map(unchanged_row));
    assert_eq!(viewer.row_readings(), expected);
}

/// REQT-ps5zx85jvc (Resetting folds) after REQT-h8rxvhpj9g (Expanding
/// everything): the two actions are inverses, so a reset refolds what
/// expanding everything opened.
#[test]
fn a_reset_refolds_everything_an_app_expanded_in_one_action() {
    fn app() -> Element {
        let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
        rsx! {
            button { onclick: move |_| diff.expand_all_folds(), "expand everything" }
            button { onclick: move |_| diff.reset_folds(), "reset folds" }
            DiffViewer { diff }
        }
    }
    let mut viewer = MountedApp::new(app);
    viewer.click_button("expand everything");

    viewer.click_button("reset folds");

    assert_eq!(viewer.row_readings(), folded_ten_line_rows());
}

/// REQT-1tdrfvay4q (Fold rows): a fold row spans the same columns as the line
/// rows around it, in both views, with line numbers shown or hidden.
#[test]
fn a_fold_row_spans_every_column_of_the_rows_around_it() {
    fn split_numbered() -> Element {
        let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
        rsx! { DiffViewer { diff } }
    }
    fn split_unnumbered() -> Element {
        let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
        rsx! { DiffViewer { diff, show_line_numbers: false } }
    }
    fn inline_numbered() -> Element {
        let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
        rsx! { DiffViewer { diff, view: DiffView::Inline } }
    }
    fn inline_unnumbered() -> Element {
        let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
        rsx! { DiffViewer { diff, view: DiffView::Inline, show_line_numbers: false } }
    }

    for (layout, app, columns) in [
        ("split, numbered", split_numbered as fn() -> Element, 6),
        ("split, unnumbered", split_unnumbered, 4),
        ("inline, numbered", inline_numbered, 4),
        ("inline, unnumbered", inline_unnumbered, 2),
    ] {
        let viewer = MountedApp::new(app);

        let counts = viewer.row_column_counts();
        assert!(
            counts.iter().all(|count| *count == columns),
            "{layout}: every row spans {columns} columns, got {counts:?}"
        );
    }
}

/// Two viewers over one diff, the second inline, with the fold rows of
/// the first alone reachable through `expand_first_fold`.
fn ten_lines_in_two_viewers() -> Element {
    let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
    rsx! {
        DiffViewer { diff }
        DiffViewer { diff, view: DiffView::Inline }
    }
}

/// REQT-dxbaat20ja (One plan per diff) with REQT-869jyzdes7 (Where fold
/// state lives): a fold opened through one viewer opens in the other, because
/// the diff holds the one plan both of them read.
/// How many viewers still hide the fixture's first two lines.
fn two_line_folds(viewer: &MountedApp) -> usize {
    viewer
        .row_readings()
        .iter()
        .filter(|row| row.contains("Expand 2 lines"))
        .count()
}

#[test]
fn a_fold_opened_in_one_viewer_over_one_diff_opens_in_the_other() {
    let mut viewer = MountedApp::new(ten_lines_in_two_viewers);
    assert_eq!(
        two_line_folds(&viewer),
        2,
        "each viewer hides the first two lines"
    );

    viewer.expand_first_fold();

    assert_eq!(
        two_line_folds(&viewer),
        0,
        "the fold opened in the viewer that was never clicked as well"
    );
    let rows = viewer.row_readings();
    assert_eq!(rows[..2], [unchanged_row(1), unchanged_row(2)]);
    assert!(
        rows.contains(&"1 | 1 |  | l1".to_owned()),
        "the inline viewer shows the first revealed line its own way: {rows:?}"
    );
}

/// Two viewers over one diff, with a button naming a hidden line from app
/// code rather than either viewer's own fold row.
fn ten_lines_in_two_viewers_with_a_fold_action() -> Element {
    let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
    rsx! {
        button { onclick: move |_| diff.expand_fold_at_line(LineId::Old(1)), "reveal old line 1" }
        DiffViewer { diff }
        DiffViewer { diff, view: DiffView::Inline }
    }
}

/// REQT-dxbaat20ja (One plan per diff) with REQT-g86vdmyyp9 (Expanding at
/// a line): a fold opened through the diff itself, naming no viewer,
/// opens in every viewer rendering that diff.
#[test]
fn a_fold_an_app_opens_by_naming_a_line_opens_in_every_viewer() {
    let mut viewer = MountedApp::new(ten_lines_in_two_viewers_with_a_fold_action);
    assert_eq!(
        two_line_folds(&viewer),
        2,
        "each viewer hides the first two lines"
    );

    viewer.click_button("reveal old line 1");

    assert_eq!(
        two_line_folds(&viewer),
        0,
        "the run opened in both viewers, neither of them named"
    );
}
