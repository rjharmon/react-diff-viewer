//! The Dioxus component apps mount to show the differences between two texts.
//!
//! Architecture: ARCH-m4dkxzw6hh (DiffViewer). The component asks the engine
//! for a line diff and renders from it (ARCH-atczcqvdsz, Line diff hand-off).

use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::consumer_callbacks::{HiddenLines, LineContent, LineNumberClick};
use crate::fold_planning::{PlannedRow, plan_rows};
use crate::fold_reset_trigger::{ExpandedFolds, FoldBasis, FoldResetTrigger};
use crate::line_diff_engine::line_diff;
use crate::line_diff_options::{CompareMethod, LineDiffOptions};
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

/// Shows the differences between an old and a new text.
///
/// Only the two texts are required; every other prop has a default.
#[component]
pub fn DiffViewer(
    /// The text before the change.
    old_text: ReadSignal<String>,
    /// The text after the change.
    new_text: ReadSignal<String>,
    /// Split or inline layout.
    #[props(default)]
    view: DiffView,
    /// Which palette this viewer reads; the default follows the reader.
    #[props(default)]
    theme: DiffTheme,
    /// How modified lines are compared when marking inline changes.
    #[props(default)]
    compare: ReadSignal<CompareMethod>,
    /// Whether modified lines mark the tokens changed within them.
    #[props(default = true)]
    mark_inline_changes: ReadSignal<bool>,
    /// Each side's first line is numbered one more than this.
    #[props(default)]
    line_offset: ReadSignal<usize>,
    /// Whether line numbers show.
    #[props(default = true)]
    show_line_numbers: bool,
    /// Whether unchanged lines far from every change fold away.
    #[props(default = true)]
    fold_unchanged_lines: bool,
    /// How many unchanged lines stay shown around each change.
    #[props(default = 3_usize)]
    surrounding_line_count: ReadSignal<usize>,
    /// Renders a fold row's content in place of "Expand N lines ...".
    fold_row_renderer: Option<Callback<HiddenLines, Element>>,
    /// Lets the app return every expanded fold to folded.
    fold_reset_trigger: Option<FoldResetTrigger>,
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
    // ARCH-atczcqvdsz (Line diff hand-off): the engine runs again only when the
    // texts or the engine's options change.
    let diff = use_memo(move || {
        let options = LineDiffOptions {
            compare: compare(),
            mark_inline_changes: mark_inline_changes(),
            line_offset: line_offset(),
        };
        line_diff(&old_text.read(), &new_text.read(), &options)
    });
    // REQT-zen8fyae28 (Rendered content identity): a change to either text
    // gives every row a new identity, so consumer-rendered content remounts.
    let texts_hash = use_memo(move || {
        let old_text = old_text.read();
        let mut hasher = blake3::Hasher::new();
        hasher.update(&(old_text.len() as u64).to_le_bytes());
        hasher.update(old_text.as_bytes());
        hasher.update(new_text.read().as_bytes());
        hasher.finalize()
    });
    let input_generation = use_fold_input_generation(old_text, new_text, surrounding_line_count);
    let mut expanded_folds = use_signal(ExpandedFolds::default);

    // REQT-869jyzdes7 (Fold reset) with REQT-v748c7mjr6 (Expanding folds):
    // expansions hold until a reset or a change of texts or surrounding-line
    // count gives the folds a new basis.
    let basis = FoldBasis {
        input_generation: input_generation(),
        resets: fold_reset_trigger.map_or(0, |trigger| trigger.resets()),
    };
    let diff = diff.read();
    // REQT-qcnxhemvhn (Folded unchanged lines): folding is on unless turned off.
    let folding = fold_unchanged_lines.then(|| *surrounding_line_count.read());
    let planned_rows = plan_rows(&diff, folding, |start| {
        expanded_folds.read().is_expanded(basis, start)
    });

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
                for texts_hash in std::iter::once(texts_hash()) {
                    Fragment { key: "{texts_hash}",
                        // The keyed Fragment is the loop's own item, so Dioxus
                        // matches rows by key rather than by position.
                        for planned in planned_rows.iter().copied() {
                            Fragment { key: "{RowKey::new(&diff, planned)}",
                                match planned {
                                    PlannedRow::Entry(position) => rows.entry_rows(&diff.entries[position]),
                                    PlannedRow::Fold(fold) => rows.fold_row(fold, &diff.entries[fold.start], move || {
                                        expanded_folds.write().expand(basis, fold.start);
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

/// A number that changes whenever the compared texts or the surrounding-line
/// count change, and at no other time.
fn use_fold_input_generation(
    old_text: ReadSignal<String>,
    new_text: ReadSignal<String>,
    surrounding_line_count: ReadSignal<usize>,
) -> Memo<u64> {
    let generations = use_hook(|| Rc::new(Cell::new(0_u64)));
    use_memo(move || {
        // Reading subscribes the memo to each input.
        old_text.read();
        new_text.read();
        surrounding_line_count.read();
        let generation = generations.get() + 1;
        generations.set(generation);
        generation
    })
}
