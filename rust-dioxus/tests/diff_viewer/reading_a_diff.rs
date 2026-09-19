//! Reading a diff: REQT-jyhvdaa75p and children.
//!
//! A diff's answers are values, and a diff lives inside the component
//! whose hook built it. Each app below therefore renders its answer as one row
//! per run, which is what an app reading the answer would do with it, and the
//! test reads those rows back.

use std::ops::RangeInclusive;

use dioxus::prelude::*;
use dioxus_diff_viewer::{DiffOptions, DiffViewer, LineRun, use_diff, use_diff_with};

use crate::mounted_app::MountedApp;

/// Ten lines with the sixth changed, as the folding tests use them.
const OLD_TEN_LINES: &str = "l1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\nl9\nl10";
const NEW_TEN_LINES: &str = "l1\nl2\nl3\nl4\nl5\nL6\nl7\nl8\nl9\nl10";

/// The class each app below renders one run onto.
const RUN_CLASS: &str = "run";

/// One run as an app might read it out: `L-2..3 R-2..3`, with a side reading
/// `L-none` or `R-none` where the run holds no line of that text.
fn run_reading(run: &LineRun) -> String {
    format!(
        "{} {}",
        side_reading("L", run.old.as_ref()),
        side_reading("R", run.new.as_ref())
    )
}

fn side_reading(side: &str, numbers: Option<&RangeInclusive<usize>>) -> String {
    match numbers {
        Some(numbers) => format!("{side}-{}..{}", numbers.start(), numbers.end()),
        None => format!("{side}-none"),
    }
}

/// Shows an answer the way an app reading it would: one row per run.
#[component]
fn Runs(runs: Vec<LineRun>) -> Element {
    rsx! {
        for run in runs {
            p { class: RUN_CLASS, "{run_reading(&run)}" }
        }
    }
}

fn run_readings(app: &MountedApp) -> Vec<String> {
    app.texts_with_class(RUN_CLASS)
}

/// REQT-f2affyt2h2 (Where the changes are): each change is one run, in
/// document order, carrying the line numbers it spans on both sides.
#[test]
fn a_diff_reports_each_of_its_changes_as_one_run_in_both_sides_line_numbers() {
    fn app() -> Element {
        let diff = use_diff("a\nb\nc\nd\ne\nf\ng\nh", "a\nB\nc\nd\ne\nf\nG\nh");
        rsx! { Runs { runs: diff.changed_line_runs() } }
    }

    let reading = MountedApp::new(app);

    assert_eq!(run_readings(&reading), ["L-2..2 R-2..2", "L-7..7 R-7..7"]);
}

/// REQT-f2affyt2h2 (Where the changes are): lines changed next to each other
/// are one run rather than one run apiece.
#[test]
fn changed_lines_lying_next_to_each_other_are_reported_as_one_run() {
    fn app() -> Element {
        let diff = use_diff("a\nb\nc\nd", "a\nB\nC\nd");
        rsx! { Runs { runs: diff.changed_line_runs() } }
    }

    let reading = MountedApp::new(app);

    assert_eq!(run_readings(&reading), ["L-2..3 R-2..3"]);
}

/// REQT-f2affyt2h2 (Where the changes are): a run holding no line of one text
/// reports that side as absent rather than inventing a number for it.
#[test]
fn a_run_of_added_lines_reports_no_old_side_and_a_run_of_removed_lines_no_new_side() {
    fn added() -> Element {
        let diff = use_diff("a\nd", "a\nb\nc\nd");
        rsx! { Runs { runs: diff.changed_line_runs() } }
    }
    fn removed() -> Element {
        let diff = use_diff("a\nb\nc\nd", "a\nd");
        rsx! { Runs { runs: diff.changed_line_runs() } }
    }

    assert_eq!(
        run_readings(&MountedApp::new(added)),
        ["L-none R-2..3"],
        "two added lines"
    );
    assert_eq!(
        run_readings(&MountedApp::new(removed)),
        ["L-2..3 R-none"],
        "two removed lines"
    );
}

/// REQT-f2affyt2h2 (Where the changes are): the lines counted as changed are
/// the ones folding measures from, so a pair whose terminators differ counts,
/// though its two lines read the same.
#[test]
fn a_pair_differing_only_in_its_line_terminators_is_reported_as_a_change() {
    fn app() -> Element {
        let diff = use_diff("a\r\nb\nc", "a\nb\nc");
        rsx! { Runs { runs: diff.changed_line_runs() } }
    }

    let reading = MountedApp::new(app);

    assert_eq!(run_readings(&reading), ["L-1..1 R-1..1"]);
}

/// REQT-f2affyt2h2 (Where the changes are)
#[test]
fn a_diff_of_two_identical_texts_reports_no_changes() {
    fn app() -> Element {
        let diff = use_diff(OLD_TEN_LINES, OLD_TEN_LINES);
        rsx! { Runs { runs: diff.changed_line_runs() } }
    }

    let reading = MountedApp::new(app);

    assert!(run_readings(&reading).is_empty());
}

/// The ten-line fixture, with each hidden run rendered above a viewer whose
/// fold rows the test can activate.
fn ten_lines_reporting_hidden_runs() -> Element {
    let diff = use_diff(OLD_TEN_LINES, NEW_TEN_LINES);
    rsx! {
        Runs { runs: diff.hidden_line_runs() }
        DiffViewer { diff }
    }
}

/// REQT-hsef5r7c4z (Which lines are hidden): each fold's hidden lines, in both
/// sides' line numbers.
#[test]
fn a_diff_reports_the_lines_its_folds_hide() {
    let reading = MountedApp::new(ten_lines_reporting_hidden_runs);

    assert_eq!(
        run_readings(&reading),
        ["L-1..2 R-1..2", "L-10..10 R-10..10"]
    );
}

/// REQT-hsef5r7c4z (Which lines are hidden): the answer is current as of the
/// asking, so a fold a reader has expanded is reported hidden no longer.
#[test]
fn a_fold_a_reader_has_expanded_is_reported_hidden_no_longer() {
    let mut reading = MountedApp::new(ten_lines_reporting_hidden_runs);

    reading.expand_first_fold();

    assert_eq!(run_readings(&reading), ["L-10..10 R-10..10"]);
}

/// REQT-hsef5r7c4z (Which lines are hidden)
#[test]
fn a_diff_folding_nothing_reports_no_hidden_lines() {
    fn app() -> Element {
        let diff = use_diff_with(
            OLD_TEN_LINES,
            NEW_TEN_LINES,
            DiffOptions {
                fold_unchanged_lines: false,
                ..DiffOptions::default()
            },
        );
        rsx! { Runs { runs: diff.hidden_line_runs() } }
    }

    let reading = MountedApp::new(app);

    assert!(run_readings(&reading).is_empty());
}
