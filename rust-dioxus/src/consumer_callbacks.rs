//! What the viewer hands the app's callbacks: plain data carrying document
//! coordinates, never fold identities or framework event types.
//!
//! Architecture: ARCH-jakkqkh6e1 (FoldRowRenderer), ARCH-sx4az00gan
//! (LineNumberClickHandler).

use std::ops::Range;
use std::sync::Arc;

use dioxus::prelude::{Modifiers, ModifiersInteraction, MouseEvent};

use crate::line_id::LineId;

/// What the line content renderer receives: a whole line, or one inline-change
/// token of it, sharing the line's text rather than copying it.
///
/// REQT-vxtax4x0vs (Custom line content). Architecture: ARCH-rs0yp4vc5q
/// (LineContentRenderer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineContent {
    line: Arc<str>,
    range: Range<usize>,
}

impl LineContent {
    /// A whole line's text.
    pub(crate) fn whole_line(line: &Arc<str>) -> Self {
        Self {
            line: Arc::clone(line),
            range: 0..line.len(),
        }
    }

    /// One token of a line, by its range in the line's text.
    pub(crate) fn token(line: &Arc<str>, range: Range<usize>) -> Self {
        Self {
            line: Arc::clone(line),
            range,
        }
    }

    /// The text to render: the whole line, or the token.
    pub fn text(&self) -> &str {
        &self.line[self.range.clone()]
    }
}

/// The lines one fold row hides, as the fold row renderer receives them.
///
/// REQT-1tdrfvay4q (Fold rows): the hidden-line count, and the old and new line
/// numbers of the first hidden line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoldRowLines {
    /// How many lines the fold hides.
    pub count: usize,
    /// The old-text line number of the first hidden line.
    pub first_old_number: usize,
    /// The new-text line number of the first hidden line.
    pub first_new_number: usize,
}

/// The modifier keys held during a click.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModifierKeys {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub meta: bool,
}

impl From<Modifiers> for ModifierKeys {
    fn from(modifiers: Modifiers) -> Self {
        Self {
            shift: modifiers.shift(),
            control: modifiers.ctrl(),
            alt: modifiers.alt(),
            meta: modifiers.meta(),
        }
    }
}

/// A click on a line number, as the line number click handler receives it.
///
/// REQT-bqm9w6v4ms (Line number clicks): the clicked line's id and the modifier
/// keys held, as plain data an app can use to build range selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineNumberClick {
    /// The clicked line.
    pub line_id: LineId,
    /// The modifier keys held during the click.
    pub modifier_keys: ModifierKeys,
}

impl LineNumberClick {
    pub(crate) fn new(line_id: LineId, event: &MouseEvent) -> Self {
        Self {
            line_id,
            modifier_keys: event.modifiers().into(),
        }
    }
}
