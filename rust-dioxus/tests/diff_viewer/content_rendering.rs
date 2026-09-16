//! Content rendering: REQT-3ekk7hre3k and children.

use dioxus::prelude::*;
use dioxus_diff_viewer::styling_hooks::DXDIFF__INLINE_TOKEN;
use dioxus_diff_viewer::{DiffView, DiffViewer, LineContent};

use crate::mounted_app::MountedApp;

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
                line_content_renderer: move |content: LineContent| rsx! { em { "[{content.text()}]" } },
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

/// REQT-vbaqm4y5zk (Column titles): titles span the columns of the lines below
/// them, in both views, with line numbers shown or hidden.
#[test]
fn titles_span_the_columns_of_the_lines_below_them() {
    fn split_numbered() -> Element {
        rsx! { DiffViewer { old_text: "a", new_text: "b", left_title: rsx! { "Old" }, right_title: rsx! { "New" } } }
    }
    fn split_unnumbered() -> Element {
        rsx! {
            DiffViewer {
                old_text: "a",
                new_text: "b",
                show_line_numbers: false,
                left_title: rsx! { "Old" },
                right_title: rsx! { "New" },
            }
        }
    }
    fn inline_numbered() -> Element {
        rsx! { DiffViewer { old_text: "a", new_text: "b", view: DiffView::Inline, left_title: rsx! { "Old" } } }
    }
    fn inline_unnumbered() -> Element {
        rsx! {
            DiffViewer {
                old_text: "a",
                new_text: "b",
                view: DiffView::Inline,
                show_line_numbers: false,
                left_title: rsx! { "Old" },
            }
        }
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

/// REQT-vxtax4x0vs (Custom line content): the empty side of a split row is not
/// sent through the consumer's renderer.
#[test]
fn a_side_with_no_line_is_not_sent_through_the_renderer() {
    fn app() -> Element {
        rsx! {
            DiffViewer {
                old_text: "a",
                new_text: "a\nb",
                line_content_renderer: move |content: LineContent| rsx! { em { "[{content.text()}]" } },
            }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings()[1], " |  |  | 2 | + | [b]");
}
