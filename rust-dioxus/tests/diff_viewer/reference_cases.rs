//! Reference cases from `test/react-diff-viewer-test.tsx`, each named by its
//! reference case.

use dioxus::prelude::*;
use dioxus_diff_viewer::{DiffView, DiffViewer, use_diff};

use crate::mounted_app::MountedApp;

const OLD_CODE: &str = "
const a = 123
const b = 456
const c = 4556
const d = 4566
const e = () => {
  console.log('c')
}
";

const NEW_CODE: &str = "
const a = 123
const b = 456
const c = 4556
const d = 4566
const aa = 123
const bb = 456
";

/// Carried over: "It should render a table".
#[test]
fn reference_case_it_should_render_a_table() {
    fn app() -> Element {
        let diff = use_diff(OLD_CODE, NEW_CODE);
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.tree().elements_with_tag("table").len(), 1);
}

/// Carried over: "It should render diff lines in diff view" (7 rows).
#[test]
fn reference_case_it_should_render_diff_lines_in_diff_view() {
    fn app() -> Element {
        let diff = use_diff(OLD_CODE, NEW_CODE);
        rsx! { DiffViewer { diff } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings().len(), 7);
}

/// Carried over: "It should render diff lines in inline view" (9 rows).
#[test]
fn reference_case_it_should_render_diff_lines_in_inline_view() {
    fn app() -> Element {
        let diff = use_diff(OLD_CODE, NEW_CODE);
        rsx! { DiffViewer { diff, view: DiffView::Inline } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings().len(), 9);
}
