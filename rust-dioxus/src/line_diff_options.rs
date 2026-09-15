//! The choices a consumer makes about how two texts are compared.
//!
//! Architecture: ARCH-n7wmjnmt65 (LineDiffOptions).

/// How a modified line's old and new text are compared when marking inline changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompareMethod {
    /// One token per character. REQT-czecf8krqc (Character comparison).
    #[default]
    Character,
    /// One token per whitespace run and per non-whitespace run.
    /// REQT-xzc8n354h1 (Word comparison).
    Word,
    /// The whole modified line as one token. REQT-spzdk2z1pk (Line comparison).
    Line,
}

/// The engine's input choices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineDiffOptions {
    /// How modified lines are compared when marking inline changes.
    pub compare: CompareMethod,
    /// Whether modified lines carry the tokens removed and added within them.
    /// REQT-4nz35dscrn (Inline changes): on unless the consumer turns them off.
    pub inline_changes: bool,
    /// Each side's first line is numbered one more than this.
    /// REQT-smd01rma2q (Independent numbering).
    pub line_offset: usize,
}

impl Default for LineDiffOptions {
    fn default() -> Self {
        Self {
            // REQT-czecf8krqc (Character comparison): the default compare method.
            compare: CompareMethod::Character,
            // REQT-4nz35dscrn (Inline changes): on unless turned off.
            inline_changes: true,
            // REQT-smd01rma2q (Independent numbering): the offset defaults to 0.
            line_offset: 0,
        }
    }
}
