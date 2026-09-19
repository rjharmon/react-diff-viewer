//! The Dioxus component apps mount to show the differences between two texts.
//!
//! Architecture: ARCH-m4dkxzw6hh (DiffViewer). The component renders the diff
//! an app hands it and keeps no diff state of its own
//! (ARCH:dcisn-xgpr1asvyn).

use dioxus::prelude::*;

use crate::consumer_callbacks::{FoldRowLines, LineContent, LineNumberClick};
use crate::diff::Diff;
use crate::fold_planning::PlannedRow;
use crate::line_id::LineId;
use crate::row_rendering::{RowKey, RowRendering};
use crate::styling_hooks::*;

/// The viewer's own styles, embedded so that an app mounting a viewer adds no
/// stylesheet of its own.
///
/// REQT-q356bvvv15 (Style delivery), ARCH:dcisn-pgydgmajhx (Themes shipped as
/// one layered stylesheet).
const DXDIFF_STYLESHEET: &str = include_str!("dxdiff.css");

/// Which palette a viewer reads.
///
/// ARCH-7kwnstr5rt (DiffTheme). An app restyles a viewer with ordinary CSS
/// against the `dxdiff` classes and the `--dxdiff-` custom property names; the
/// crate offers no Rust type for that.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffTheme {
    /// Follow the reader's light or dark color-scheme preference.
    /// REQT-sc8expw3q8 (Theme selection): the default.
    #[default]
    Auto,
    /// Read the light palette whatever the reader prefers.
    Light,
    /// Read the dark palette whatever the reader prefers.
    Dark,
}

impl DiffTheme {
    /// The choice as the viewer element reports it, which is what the
    /// stylesheet selects an explicit theme's palette on.
    fn attribute_value(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// How the viewer lays out the two texts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffView {
    /// Old lines on the left, new lines on the right.
    /// REQT-ys3yr5g185 (Split view): the default.
    #[default]
    Split,
    /// One column, with a modified line's old text above its new text.
    /// REQT-0xbgrj9ane (Inline view).
    Inline,
}

/// Shows the differences an app's diff holds.
///
/// Only the diff is required; every other prop has a default.
#[component]
pub fn DiffViewer(
    /// The diff to render, built with `use_diff` or `use_diff_with`.
    ///
    /// REQT-m776z5vdhe (Rendering a diff): the viewer's only data prop.
    diff: Diff,
    /// Split or inline layout.
    #[props(default)]
    view: DiffView,
    /// Which palette this viewer reads; the default follows the reader.
    #[props(default)]
    theme: DiffTheme,
    /// Whether line numbers show.
    #[props(default = true)]
    show_line_numbers: bool,
    /// Renders a fold row's content in place of "Expand N lines ...".
    fold_row_renderer: Option<Callback<FoldRowLines, Element>>,
    /// Lines to highlight, by line id.
    #[props(default)]
    highlighted_lines: Vec<LineId>,
    /// Called with the line id and held modifier keys of each clicked line number.
    on_line_number_click: Option<EventHandler<LineNumberClick>>,
    /// Renders line text, and each inline-change token on modified lines.
    line_content_renderer: Option<Callback<LineContent, Element>>,
    /// Shown above the old column, and above the only column inline.
    left_title: Option<Element>,
    /// Shown above the new column in the split view.
    right_title: Option<Element>,
) -> Element {
    // Reading the diff's state here subscribes this viewer to it, so a
    // change reaches every viewer over one diff whatever its own props say.
    let analysis = diff.analysis();
    let analysis = analysis.read();
    // REQT-dxbaat20ja (One plan per diff): the rows come from the diff's
    // own plan rather than from a plan this viewer makes.
    let planned_rows = diff.planned_rows();
    let planned_rows = planned_rows.read();
    let content_identity = diff.content_identity();

    let rows = RowRendering {
        view,
        show_line_numbers,
        highlighted_lines: &highlighted_lines,
        on_line_number_click,
        line_content_renderer,
        fold_row_renderer,
    };
    let view_class = match view {
        DiffView::Split => DXDIFF__SPLIT_VIEW,
        DiffView::Inline => DXDIFF__INLINE_VIEW,
    };
    let title_span = rows.cells_per_line();
    // REQT-vbaqm4y5zk (Column titles): the right title shows only in the split
    // view.
    let right_title = right_title.filter(|_| view == DiffView::Split);
    let has_title = left_title.is_some() || right_title.is_some();

    rsx! {
        // REQT-q356bvvv15 (Style delivery): the sheet goes into the document's
        // head and leaves nothing in the viewer's own tree. Two mounted viewers
        // insert two identical copies, which costs nothing: the layer may be
        // declared repeatedly and both carry the same declarations.
        document::Style { "{DXDIFF_STYLESHEET}" }
        table {
            class: "{DXDIFF__VIEWER}",
            class: "{view_class}",
            // REQT-sc8expw3q8 (Theme selection): the choice rides on the
            // viewer element, so two mounted viewers may differ.
            "data-dxdiff-theme": "{theme.attribute_value()}",
            tbody {
                if has_title {
                    // REQT-3928hx46s3 (Styling hooks): titles carry only their
                    // kind class; a title shows no line, so no change or
                    // highlight state is relevant to it.
                    tr {
                        td { class: "{DXDIFF__TITLE}", colspan: "{title_span}", pre { {left_title} } }
                        if view == DiffView::Split {
                            td { class: "{DXDIFF__TITLE}", colspan: "{title_span}", pre { {right_title} } }
                        }
                    }
                }
                // A one-item keyed list: a new key replaces every row beneath it.
                for content_identity in std::iter::once(content_identity) {
                    Fragment { key: "{content_identity}",
                        // The keyed Fragment is the loop's own item, so Dioxus
                        // matches rows by key rather than by position.
                        for planned in planned_rows.iter().copied() {
                            Fragment { key: "{RowKey::new(&analysis, planned)}",
                                match planned {
                                    PlannedRow::Entry(position) => rows.entry_rows(&analysis.entries[position]),
                                    PlannedRow::Fold(fold) => rows.fold_row(fold, &analysis.entries[fold.start], move || {
                                        diff.expand_fold_at_position(fold.start);
                                    }),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
