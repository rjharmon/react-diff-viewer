//! Writes a page showing the viewer in each theme, so the first acceptance
//! criterion of the theming work can be checked by eye against the
//! `react-diff-viewer` package.
//!
//! No test reaches "reads as the package does". What the suite checks is that
//! every color a rule reads is carried from `src/styles.ts` for the same
//! element; how that looks is for a person to see.
//!
//! `cargo run --example theme_preview` writes `theme-preview.html` beside the
//! crate and names the path it wrote.

use std::fs;

use dioxus::prelude::*;
use dioxus_diff_viewer::{CompareMethod, DiffTheme, DiffView, DiffViewer, LineId, LineNumberClick};

/// A code edit: modified lines with word-level changes, a run of unchanged
/// lines long enough to fold, a highlighted line, and a removal with no
/// counterpart, so an empty side shows too.
const OLD_CODE: &str = "const a = 123\nconst b = 456\nconst c = 4556\nconst d = 4566\nconst e = () => {\n  console.log('c')\n}\nunchanged one\nunchanged two\nunchanged three\nunchanged four\nunchanged five\nunchanged six\nunchanged seven\nunchanged eight\nconst removed = true";

const NEW_CODE: &str = "const a = 123\nconst b = 789\nconst c = 4556\nconst d = 4566\nconst e = () => {\n  console.log('d')\n}\nunchanged one\nunchanged two\nunchanged three\nunchanged four\nunchanged five\nunchanged six\nunchanged seven\nunchanged eight";

/// Edge whitespace and a terminator change, which is what brings out the
/// whitespace chip, the line ending chips, and the arrow between them.
const OLD_CHIPS: &str = "terminator changed\r\ntrailing space here   \nplain line";

const NEW_CHIPS: &str = "terminator changed\n   trailing space here\nplain line";

/// One viewer under a heading naming what it shows.
#[component]
fn Sample(
    heading: String,
    theme: DiffTheme,
    view: DiffView,
    old_text: String,
    new_text: String,
    compare: CompareMethod,
) -> Element {
    rsx! {
        h2 { "{heading}" }
        DiffViewer {
            old_text,
            new_text,
            theme,
            view,
            compare,
            highlighted_lines: vec![LineId::New(3)],
            left_title: rsx! { "before" },
            right_title: rsx! { "after" },
            on_line_number_click: move |_: LineNumberClick| {},
        }
    }
}

fn preview() -> Element {
    rsx! {
        for (heading, theme, view, old_text, new_text, compare) in [
            ("light, split", DiffTheme::Light, DiffView::Split, OLD_CODE, NEW_CODE, CompareMethod::Word),
            ("dark, split", DiffTheme::Dark, DiffView::Split, OLD_CODE, NEW_CODE, CompareMethod::Word),
            ("light, inline", DiffTheme::Light, DiffView::Inline, OLD_CODE, NEW_CODE, CompareMethod::Word),
            ("dark, inline", DiffTheme::Dark, DiffView::Inline, OLD_CODE, NEW_CODE, CompareMethod::Word),
            ("light, chips and the arrow", DiffTheme::Light, DiffView::Inline, OLD_CHIPS, NEW_CHIPS, CompareMethod::TrimmedLine),
            ("dark, chips and the arrow", DiffTheme::Dark, DiffView::Inline, OLD_CHIPS, NEW_CHIPS, CompareMethod::TrimmedLine),
            ("the reader's preference, split", DiffTheme::Auto, DiffView::Split, OLD_CODE, NEW_CODE, CompareMethod::Word),
        ] {
            Sample {
                key: "{heading}",
                heading: heading.to_string(),
                theme,
                view,
                old_text: old_text.to_string(),
                new_text: new_text.to_string(),
                compare,
            }
        }
    }
}

fn main() {
    // The component delivers its styles through the document, which server-side
    // rendering does not carry, so the page states the same sheet the crate
    // embeds.
    let stylesheet = include_str!("../src/dxdiff.css");
    let body = dioxus_ssr::render_element(preview());
    let page = format!(
        "<!doctype html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>dxdiff theme preview</title>\n<style>\n{stylesheet}\n</style>\n<style>\nbody {{ font-family: system-ui, sans-serif; margin: 2rem; }}\nh2 {{ margin-top: 2rem; font-size: 1rem; font-weight: 600; }}\n</style>\n</head>\n<body>\n{body}\n</body>\n</html>\n"
    );

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/theme-preview.html");
    fs::write(path, page).expect("the preview page is written");
    println!("wrote {path}");
}
