//! Content rendering: REQT-3ekk7hre3k and children.

use std::cell::Cell;

use dioxus::prelude::*;
use dioxus_diff_viewer::styling_hooks::DXDIFF__INLINE_TOKEN;
use dioxus_diff_viewer::{DiffView, DiffViewer, LineContent, use_diff};

use crate::mounted_app::MountedApp;

/// REQT-vbaqm4y5zk (Column titles): the split view shows each title above its
/// column, as text or rendered content.
#[test]
fn the_split_view_shows_each_title_above_its_column() {
    fn app() -> Element {
        let diff = use_diff("a", "b");
        rsx! {
            DiffViewer {
                diff,
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
        let diff = use_diff("a", "b");
        rsx! {
            DiffViewer {
                diff,
                view: DiffView::Inline,
                left_title: rsx! { "Old" },
                right_title: rsx! { "New" },
            }
            DiffViewer { diff, view: DiffView::Inline, right_title: rsx! { "New" } }
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
        let diff = use_diff("same\nab", "same\nac");
        rsx! {
            DiffViewer {
                diff,
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
        let diff = use_diff("a", "b");
        rsx! { DiffViewer { diff, left_title: rsx! { "Old" }, right_title: rsx! { "New" } } }
    }
    fn split_unnumbered() -> Element {
        let diff = use_diff("a", "b");
        rsx! {
            DiffViewer {
                diff,
                show_line_numbers: false,
                left_title: rsx! { "Old" },
                right_title: rsx! { "New" },
            }
        }
    }
    fn inline_numbered() -> Element {
        let diff = use_diff("a", "b");
        rsx! { DiffViewer { diff, view: DiffView::Inline, left_title: rsx! { "Old" } } }
    }
    fn inline_unnumbered() -> Element {
        let diff = use_diff("a", "b");
        rsx! {
            DiffViewer {
                diff,
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
        let diff = use_diff("a", "a\nb");
        rsx! {
            DiffViewer {
                diff,
                line_content_renderer: move |content: LineContent| rsx! { em { "[{content.text()}]" } },
            }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(viewer.row_readings()[1], " |  |  | 2 | + | [b]");
}

thread_local! {
    /// Counts mounts of `MountStamp` on this test thread.
    static MOUNT_COUNT: Cell<usize> = const { Cell::new(0) };
}

/// Consumer content holding state: the mount it was created in, shown after
/// its text as `text#mount`.
#[component]
fn MountStamp(text: String) -> Element {
    let mount = use_hook(|| {
        MOUNT_COUNT.with(|count| {
            count.set(count.get() + 1);
            count.get()
        })
    });
    rsx! { "{text}#{mount}" }
}

/// Ten lines with the sixth changed, each line stamped with its mount, and a
/// button that edits the last line of the new text.
fn stamped_ten_lines() -> Element {
    let mut new_text = use_signal(|| "l1\nl2\nl3\nl4\nl5\nL6\nl7\nl8\nl9\nl10".to_string());
    let diff = use_diff("l1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\nl9\nl10", new_text());
    rsx! {
        button {
            onclick: move |_| new_text.set("l1\nl2\nl3\nl4\nl5\nL6\nl7\nl8\nl9\nL10".into()),
            "edit the last line"
        }
        DiffViewer {
            diff,
            line_content_renderer: move |content: LineContent| rsx! {
                MountStamp { text: content.text().to_string() }
            },
        }
    }
}

/// REQT-zen8fyae28 (Rendered content identity): while the texts stay the
/// same, content rendered for a line stays mounted, even as folds expand.
#[test]
fn rendered_content_stays_mounted_while_the_texts_stay_the_same() {
    let mut viewer = MountedApp::new(stamped_ten_lines);
    let line_three_before = viewer.row_readings()[1].clone();

    viewer.expand_first_fold();

    assert_eq!(viewer.row_readings()[2], line_three_before);
}

/// REQT-zen8fyae28 (Rendered content identity): when either text changes,
/// content rendered for every line remounts, including unchanged lines.
#[test]
fn rendered_content_remounts_when_a_text_changes() {
    let mut viewer = MountedApp::new(stamped_ten_lines);
    let line_three_before = viewer.row_readings()[1].clone();

    viewer.click_button("edit the last line");

    let line_three_after = viewer.row_readings()[1].clone();
    assert!(
        line_three_before.starts_with("3 |  | l3#"),
        "{line_three_before}"
    );
    assert!(
        line_three_after.starts_with("3 |  | l3#"),
        "{line_three_after}"
    );
    assert_ne!(line_three_after, line_three_before);
}
