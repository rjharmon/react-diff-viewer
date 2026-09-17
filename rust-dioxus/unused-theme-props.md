# Theme colors the port does not read

The `react-diff-viewer` package names 24 colors per theme (`src/styles.ts:88-112`
light, `:117-141` dark). The Dioxus port carries them as `--dxdiff-` custom
properties, one per color, wherever a rule in `src/dxdiff.css` reads it.

This file records the exceptions: a reference color whose element or state the
port's markup does not distinguish, so no rule can read it. Each entry names
what the reference does with the color, why the port leaves it out, and the
feature that would make it active.

## `gutterBackgroundDark`

- **In the reference**: the hover background of a line-number gutter, set
  together with `cursor: pointer` (`src/styles.ts:298-311`). Light `#f3f1f1`,
  dark `#262933`.
- **Why the port leaves it out**: the reference gutter is always a click target,
  while the port's gutter listens for clicks only in a viewer given a
  line-number click handler, and only where the gutter carries a number
  (`rust-dioxus/src/row_rendering.rs:250-288`). Both kinds carry the same
  classes, `dxdiff-gutter` plus the change state plus `dxdiff-highlighted`, so a
  `.dxdiff-gutter:hover` rule would show a hover shade and a pointer cursor on
  gutters that do nothing when clicked.
- **What would make it active**: a styling hook marking a clickable gutter, so
  the hover rule can be scoped to it. That class is part of the vocabulary
  REQT-3928hx46s3 (Styling hooks) defines, so activating this color means
  growing that requirement first.

## The other direction

The port renders elements the reference has no color for at all: the line ending
chip, the line ending arrow, and the whitespace chip. REQT-8y5fp2aarz (Chip
color) governs those, and each theme colors them to contrast with every line
background they appear on.
