//! Content rendering: REQT-3ekk7hre3k and children.

use dioxus::prelude::*;
use dioxus_diff_viewer::styling_hooks::DXDIFF__INLINE_TOKEN;
use dioxus_diff_viewer::{DiffView, DiffViewer};

use crate::mounted_viewer::MountedApp;

/// REQT-vbaqm4y5zk (Column titles): the split view shows each title above its
/// column, as text or rendered content.
#[test]
fn the_split_view_shows_each_title_above_its_column() {
    fn app() -> Element {
        rsx! {
            DiffViewer {
                old_text: "a",
                new_text: "b",
                left_title: rsx! { "Old" },
                right_title: rsx! { b { "New" } },
            }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings()[0], "Old | New");
}

/// REQT-vbaqm4y5zk (Column titles): the inline view shows only the left title.
#[test]
fn the_inline_view_shows_only_the_left_title() {
    fn app() -> Element {
        rsx! {
            DiffViewer {
                old_text: "a",
                new_text: "b",
                view: DiffView::Inline,
                left_title: rsx! { "Old" },
                right_title: rsx! { "New" },
            }
            DiffViewer {
                old_text: "a",
                new_text: "b",
                view: DiffView::Inline,
                right_title: rsx! { "New" },
            }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings(),
        vec![
            "Old",
            "1 |  | - | a",
            " | 1 | + | b",
            "1 |  | - | a",
            " | 1 | + | b"
        ]
    );
}

/// REQT-vxtax4x0vs (Custom line content): whole lines, and each inline-change
/// token on a modified line, go through the consumer's renderer.
#[test]
fn a_consumer_renderer_shapes_whole_lines_and_each_inline_token() {
    fn app() -> Element {
        rsx! {
            DiffViewer {
                old_text: "same\nab",
                new_text: "same\nac",
                line_content_renderer: move |text: String| rsx! { em { "[{text}]" } },
            }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.row_readings(),
        vec![
            "1 |  | [same] | 1 |  | [same]",
            "2 | - | [a][b] | 2 | + | [a][c]",
        ]
    );
    let tokens = viewer.tree().elements_with_class(DXDIFF__INLINE_TOKEN);
    assert_eq!(tokens.len(), 4);
    assert!(
        tokens.iter().all(|token| token
            .child_elements()
            .iter()
            .any(|child| child.tag() == "em")),
        "the change wrapping stays around each rendered token"
    );
}
