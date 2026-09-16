//! Line selection: REQT-p2gkf77kjj and children.

use dioxus::prelude::*;
use dioxus_diff_viewer::styling_hooks::DXDIFF__HIGHLIGHTED;
use dioxus_diff_viewer::{DiffView, DiffViewer, LineId, LineNumberClick};
use dioxus_html::Modifiers;

use crate::mounted_viewer::MountedApp;

/// REQT-zaens35zqy (Highlighted lines): a line id joins its side and number
/// with a hyphen.
#[test]
fn a_line_id_reads_as_its_side_and_number_joined_by_a_hyphen() {
    assert_eq!(LineId::Old(20).to_string(), "L-20");
    assert_eq!(LineId::New(3).to_string(), "R-3");
    assert_eq!("L-20".parse(), Ok(LineId::Old(20)));
    assert_eq!("R-3".parse(), Ok(LineId::New(3)));
    for not_a_line_id in ["", "L20", "X-1", "L-", "R-x"] {
        assert!(
            not_a_line_id.parse::<LineId>().is_err(),
            "{not_a_line_id:?}"
        );
    }
}

/// REQT-zaens35zqy (Highlighted lines): in the split view, the listed side's
/// cells and their row are highlighted.
#[test]
fn a_listed_line_is_highlighted_on_its_own_side() {
    fn app() -> Element {
        rsx! { DiffViewer { old_text: "a\nb", new_text: "a\nc", highlighted_lines: vec![LineId::New(2)] } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.texts_with_class(DXDIFF__HIGHLIGHTED),
        vec!["2-b2+c", "2", "+", "c"],
        "the row, then the new side's gutter, marker, and content"
    );
}

/// REQT-zaens35zqy (Highlighted lines): in the inline view, an unchanged line
/// shown once is highlighted when either of its ids is listed.
#[test]
fn an_inline_unchanged_line_is_highlighted_by_either_of_its_ids() {
    fn app() -> Element {
        rsx! {
            DiffViewer {
                old_text: "gone\na",
                new_text: "a",
                view: DiffView::Inline,
                highlighted_lines: vec![LineId::New(1)],
            }
        }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        viewer.texts_with_class(DXDIFF__HIGHLIGHTED),
        vec!["21a", "2", "1", "", "a"]
    );
}

/// Records every line number click it receives, one list item each.
fn recording_clicks(view: DiffView) -> Element {
    let mut clicks = use_signal(Vec::<LineNumberClick>::new);
    rsx! {
        DiffViewer {
            old_text: "a\nb",
            new_text: "a\nc\nd",
            view,
            on_line_number_click: move |click| clicks.write().push(click),
        }
        ol {
            for click in clicks() {
                li { class: "received-click",
                    "{click.line_id} shift={click.modifier_keys.shift} control={click.modifier_keys.control}"
                }
            }
        }
    }
}

/// REQT-bqm9w6v4ms (Line number clicks): the handler gets each clicked line's
/// id and the modifier keys held.
#[test]
fn each_line_number_click_calls_the_handler_with_the_line_id_and_held_keys() {
    fn app() -> Element {
        recording_clicks(DiffView::Split)
    }
    let mut viewer = MountedApp::new(app);

    viewer.click_line_number(1, 1, Modifiers::SHIFT);
    viewer.click_line_number(0, 0, Modifiers::CONTROL);

    assert_eq!(
        viewer.texts_with_class("received-click"),
        vec![
            "R-2 shift=true control=false",
            "L-1 shift=false control=true"
        ]
    );
}

/// REQT-bqm9w6v4ms (Line number clicks): in the inline view, each gutter reports
/// its own side's line, and a gutter without a number reports nothing.
#[test]
fn an_inline_gutter_reports_its_own_side_s_line() {
    fn app() -> Element {
        recording_clicks(DiffView::Inline)
    }
    let mut viewer = MountedApp::new(app);

    viewer.click_line_number(0, 1, Modifiers::empty());
    viewer.click_line_number(2, 1, Modifiers::empty());
    viewer.click_line_number(2, 0, Modifiers::empty());

    assert_eq!(
        viewer.texts_with_class("received-click"),
        vec![
            "R-1 shift=false control=false",
            "R-2 shift=false control=false"
        ]
    );
}
