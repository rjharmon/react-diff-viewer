# Mission

> **Loading gate for agents**: before acting on this mission, load and study
> `skillz/automatic-agent/automatic-agent.SKILL.md`.

## Mission objective
Port react-diff-viewer to a Rust crate in `rust-dioxus/` that renders as a Dioxus component and diffs with the `similar` crate. Load the Rust Roundup with its UI group.

## Mission outcomes
- [ ] Split and inline views render two strings with line numbers, folded unchanged lines, and expandable folds.
- [ ] Modified lines show word-level diffs, with character-level diffs also available.
- [ ] Each prop in `README.md` "Props" has a Rust equivalent or a recorded omission.
- [ ] Light and dark themes carry the `src/styles.ts` palettes, and consumers can override them.

## Mission constraints
- The TypeScript package at the repo root stays unchanged.
- Diff alignment follows `similar`; matching the TypeScript output or tests exactly is not a goal.

## Mission failure-outcomes
- Modified lines render as whole-line replacements with no word-level diff.

## Connections
- Reference implementation: `src/`, `test/`, `README.md` at the repo root.
