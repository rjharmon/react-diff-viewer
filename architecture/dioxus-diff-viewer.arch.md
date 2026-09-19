
# dioxus-diff-viewer - Architecture

> ⚠️ Automatically generated from related JSONL file; maintain using the Eidos Architect skill.

*local; DREAMED; ARCH-h2qwqte1g6*

Dioxus diff viewer crate built on a diff analysis engine.

**Activities**:

- Show the differences between two texts in Dioxus apps

**Responsibilities**:

- Crate identity and public vocabulary drawn from requirement terms

**Supports Requirements**: REQT-9273mztsx4, REQT-wxe0svnb22, REQT-x8e364jbqm





**Maturity**:

- DREAMED: 20/28
- draft: 4/28
- consented: 3/28
- abandoned: 1/28

## In this document

[Components and Concerns](#components-and-concerns)&nbsp;&nbsp; [Components](#components)&nbsp;&nbsp; [Interactions](#interactions)&nbsp;&nbsp; [Software Objects](#software-objects)&nbsp;&nbsp; [Decisions](#decisions)&nbsp;&nbsp; [Design Patterns](#design-patterns)&nbsp;&nbsp; [Files](#files)&nbsp;&nbsp; [Collaboration Summary](#collaboration-summary)&nbsp;&nbsp; [Open Questions](#open-questions)&nbsp;&nbsp; [Discovery Notes](#discovery-notes)

## Components and Concerns




- [blake3](#blake3-arch-v811rxcq9d) (external): Third-party Rust crate hashing the two texts into one identity.
- [DiffAnalysisEngine](#diffanalysisengine-arch-jf3s5npp9s) (internal): Turns two texts and options into paired line information.
- [DiffViewer](#diffviewer-arch-m4dkxzw6hh) (internal): The Dioxus component apps mount to show two texts' differences, rendering a diff in split or inline view.
- [similar](#similar-arch-wdrt39xvh4) (external): Third-party Rust crate computing text alignment.



## Components

<a id="blake3-arch-v811rxcq9d"></a>

### Component: blake3 (external; DREAMED - ARCH-v811rxcq9d)

Third-party Rust crate hashing the two texts into one identity.

**Activities**:

- Hash the compared texts into a content identity



**Supports Requirements**: REQT-zen8fyae28




---

<a id="diffanalysisengine-arch-jf3s5npp9s"></a>

### Component: DiffAnalysisEngine (internal; DREAMED - ARCH-jf3s5npp9s)

Turns two texts and options into paired line information.

**Activities**:

- Pair old and new lines
- Mark changes, line endings, and inline changes
- Number lines on each side



**Exclusions** (boundary clarifications):
- Does NOT handle Views, folding, and styling → see ARCH-m4dkxzw6hh (DiffViewer) - The engine carries no presentation.


**Supports Requirements**: REQT-czecf8krqc, REQT-xzc8n354h1, REQT-spzdk2z1pk, REQT-z9r0pc53jg, REQT-hmsfnfe5wc, REQT-dqxm8fa7ts, REQT-4nz35dscrn, REQT-9tze98pt6g, REQT-rtwn1qresp, REQT-hq1fzjaxg8, REQT-smd01rma2q

**Concerns and Responsibilities**:
- **Responsibility**: Line change marking as `similar` aligns lines
- **Responsibility**: Inline change tokens under each compare method
- **Responsibility**: Independent line numbering from the offset
- **Responsibility**: Positions of entries holding a change


**Interactions**: [Diff analysis hand-off](#interaction-ARCH-atczcqvdsz)



---

<a id="diffviewer-arch-m4dkxzw6hh"></a>

### Component: DiffViewer (internal; DREAMED - ARCH-m4dkxzw6hh)

The Dioxus component apps mount to show two texts' differences, rendering a diff in split or inline view.

**Activities**:

- Render split and inline views from a diff
- Show fold rows and expand one a reader activates
- Report line-number clicks to the app



**Exclusions** (boundary clarifications):
- Does NOT handle Whitespace-insensitive word, sentence, and CSS comparison (react-diff-viewer's WORDS, SENTENCES, CSS) - Not ported; the compare methods offered are character, word, line, and trimmed line, and CompareMethod leaves room for more.
- Does NOT handle Reporting whether a reset found any expanded fold (the boolean react-diff-viewer's resetCodeBlocks() returns) - Resetting returns nothing; an app that wants to know reads which lines are hidden before and after.
- Does NOT handle Closing one expanded region from app code - Steering carries expand-at-a-line, expand-everything and reset. An app reaches the same end state by reading which lines are hidden, resetting, and re-expanding what it wants kept; the reference package offers no per-region close either.


**Supports Requirements**: REQT-2k7j51afde, REQT-qerexp825r, REQT-p2gkf77kjj, REQT-3ekk7hre3k, REQT-m9r3k5b1ge, REQT-3928hx46s3, REQT-zen8fyae28, REQT-78dg0g6a2h, REQT-q356bvvv15, REQT-kcm5ba45sp, REQT-sc8expw3q8, REQT-aaxrz33x1n, REQT-ef5a9d2paw, REQT-m776z5vdhe

**Concerns and Responsibilities**:
- **Responsibility**: Rendering a diff without diffing or planning again
- **Responsibility**: Line highlighting and selection
- **Responsibility**: Consumer rendering of line content and fold rows
- **Responsibility**: Keeping consumer-rendered content mounted with its line
- **Responsibility**: Styling hook classes on its markup
- **Responsibility**: Style delivery and theme selection
- **Responsibility**: Hover feedback on the row under the pointer




---

<a id="similar-arch-wdrt39xvh4"></a>

### Component: similar (external; DREAMED - ARCH-wdrt39xvh4)

Third-party Rust crate computing text alignment.

**Activities**:

- Align lines and tokens between two texts



**Interactions**: [Diff analysis hand-off](#interaction-ARCH-atczcqvdsz)



---


### Nested Components

> These components declare a `parentComponent` and should each have a separate architectural breakdown document. The summary here is a placeholder until that breakdown exists.

| Component | Parent | Summary |
|-----------|--------|---------|
| [DiffAnalysisEngine](#diffanalysisengine-arch-jf3s5npp9s) | [dioxus-diff-viewer](#dioxus-diff-viewer-arch-h2qwqte1g6) | Turns two texts and options into paired line information. |
| [DiffViewer](#diffviewer-arch-m4dkxzw6hh) | [dioxus-diff-viewer](#dioxus-diff-viewer-arch-h2qwqte1g6) | The Dioxus component apps mount to show two texts' differences, rendering a diff in split or inline view. |



## Interactions



<a id="interaction-ARCH-atczcqvdsz"></a>

### Interaction: Diff analysis hand-off (library_call; DREAMED - ARCH-atczcqvdsz)

[Diff](#diff-arch-8tce9rry3b), [DiffAnalysisEngine](#diffanalysisengine-arch-jf3s5npp9s), [similar](#similar-arch-wdrt39xvh4) - Trigger: Either text or the engine's options change.

The diff asks the engine for a DiffAnalysis and every viewer over it renders from that; the engine splits the lines itself and asks `similar` to align their texts.

**Payload**: two texts and DiffAnalysisOptions; DiffAnalysis

**Error**: None on text input: every pair of texts yields a DiffAnalysis.
**Supports Requirements**: REQT-hmsfnfe5wc



![Diff analysis hand-off: collaboration](./diagrams/ARCH-atczcqvdsz-collaboration.svg)


---




## Data Flows





## Software Objects


<a id="software-object-ARCH-n7wmjnmt65"></a>

### DiffAnalysisOptions (struct; DREAMED - ARCH-n7wmjnmt65)

**Component**: DiffAnalysisEngine (ARCH-jf3s5npp9s)

```
compare method (character, word, line, trimmed line); inline changes on or off; line offset
```

The engine's own input choices, which a diff's options value composes.

**Supports Requirements**: REQT-czecf8krqc, REQT-xzc8n354h1, REQT-spzdk2z1pk, REQT-z9r0pc53jg, REQT-4nz35dscrn, REQT-smd01rma2q



<a id="software-object-ARCH-exgfx6rwdt"></a>

### DiffAnalysis (struct; DREAMED - ARCH-exgfx6rwdt)

**Component**: DiffAnalysisEngine (ARCH-jf3s5npp9s)

```
paired line entries in display order; positions of entries holding a change, including a pair whose terminators differ
```

The engine's output, and the diff's only input for rows and folding.

**Supports Requirements**: REQT-hmsfnfe5wc, REQT-dqxm8fa7ts, REQT-rtwn1qresp, REQT-qcnxhemvhn



<a id="software-object-ARCH-kjjykjtt5r"></a>

### PairedLineEntry (struct; DREAMED - ARCH-kjjykjtt5r)

**Component**: DiffAnalysisEngine (ARCH-jf3s5npp9s)

```
old side and new side, each optional with shared line text and line number; change kind (unchanged, removed, added, modified); inline change tokens on modified entries, each side's tokens marked unchanged, removed, or added and rejoining to that side's text; line ending change with each side's terminator; whether a modified pair differs in leading or trailing whitespace
```

One row of the diff as either view reads it.

**Supports Requirements**: REQT-hmsfnfe5wc, REQT-dqxm8fa7ts, REQT-4nz35dscrn, REQT-rtwn1qresp, REQT-hq1fzjaxg8, REQT-smd01rma2q



<a id="software-object-ARCH-y9545npjzg"></a>

### FoldResetTrigger (struct; abandoned - ARCH-y9545npjzg)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
an opaque copyable handle whose one action returns every expanded fold to folded
```

Retired with the viewer's own fold state. An app resets through the diff (ARCH-8tce9rry3b), which keeps the expanded folds and takes two more fold actions besides; its rule that an app never names a fold carried over.



<a id="software-object-ARCH-rs0yp4vc5q"></a>

### LineContentRenderer (callback; DREAMED - ARCH-rs0yp4vc5q)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
a whole line or one inline-change token, as shared line text with a range, to rendered content
```

Optional; lets an app shape line text, as for syntax highlighting. The viewer keeps the change wrapping around each token, so on modified lines the renderer sees fragments. Line text is shared with the engine output, so no call copies it.

**Supports Requirements**: REQT-vxtax4x0vs



<a id="software-object-ARCH-jakkqkh6e1"></a>

### FoldRowRenderer (callback; DREAMED - ARCH-jakkqkh6e1)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
hidden-line count, and the first hidden line's old and new numbers, to rendered content
```

Optional; replaces the default fold-row text. Receives document coordinates only, never a fold identity.

**Supports Requirements**: REQT-1tdrfvay4q



<a id="software-object-ARCH-sx4az00gan"></a>

### LineNumberClickHandler (callback; DREAMED - ARCH-sx4az00gan)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
the clicked line's id, as in L-20, and the modifier keys held; returns nothing
```

Optional; with highlighted lines, lets an app build line and range selection. Modifier keys travel as plain data rather than the framework's event.

**Supports Requirements**: REQT-bqm9w6v4ms, REQT-zaens35zqy



<a id="software-object-ARCH-mt182qa8vt"></a>

### LineId (enum; DREAMED - ARCH-mt182qa8vt)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
old or new side plus a line number; text form `L-20` or `R-3`, readable back from text
```

How an app names a line, both to highlight it and in each line-number click.

**Supports Requirements**: REQT-zaens35zqy, REQT-bqm9w6v4ms, REQT-jgkndzqkyz



<a id="software-object-ARCH-38ktaqm4xw"></a>

### StylingHookClasses (constants; draft - ARCH-38ktaqm4xw)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
one `dxdiff` class per element kind, plus view classes, state classes (unchanged, removed, added, modified, empty, highlighted), and a class on a gutter that reports its clicks
```

The public class vocabulary apps and the theming unit style against, independent of element structure. The clickable-gutter class is what lets a hover affordance reach only the gutters that answer a click, which is what makes the reference's gutter hover carryable.

**Supports Requirements**: REQT-3928hx46s3



<a id="software-object-ARCH-d87pprx5vn"></a>

### ThemePalette (constants - ARCH-d87pprx5vn)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
one `--dxdiff`-prefixed custom property per theme color, defaulted at the document root, redeclared for the dark theme, and carried on a viewer given an explicit theme
```

What an app reassigns to restyle the viewer; the stylesheet reads nothing else for color.

**Supports Requirements**: REQT-78dg0g6a2h, REQT-1accrb4jhp, REQT-8y5fp2aarz, REQT-ef5a9d2paw



<a id="software-object-ARCH-7kwnstr5rt"></a>

### DiffTheme (enum - ARCH-7kwnstr5rt)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
auto, light, or dark; auto follows the reader's color-scheme preference
```

The prop choosing a viewer's palette, written onto the viewer element so two mounted viewers may differ.

**Supports Requirements**: REQT-sc8expw3q8



<a id="software-object-ARCH-8tce9rry3b"></a>

### Diff (struct; DREAMED - ARCH-8tce9rry3b)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
the two texts, the analysis, the content identity, the expanded folds with their basis, and the planned rows
```

The state an app holds and viewers render. It answers where the changes are and which lines are hidden, and it takes expand-at-a-line, expand-everything and reset. Sitting outside the view's code path lets it answer without publishing from an effect.

**Supports Requirements**: REQT-m776z5vdhe, REQT-dxbaat20ja, REQT-f2affyt2h2, REQT-hsef5r7c4z, REQT-g86vdmyyp9, REQT-h8rxvhpj9g, REQT-ps5zx85jvc, REQT-869jyzdes7



<a id="software-object-ARCH-smcbvznq0z"></a>

### DiffOptions (struct; DREAMED - ARCH-smcbvznq0z)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
the engine's compare choices, plus whether unchanged lines fold and how many surround each change; every field defaulted
```

What a diff is built with. Folding sits here rather than on the viewer because the row plan the diff owns cannot be computed without it (ARCH-phde62wxvn).

**Supports Requirements**: REQT-7tk9vxv9wd, REQT-qcnxhemvhn



<a id="software-object-ARCH-87bw15t11s"></a>

### UseDiffHooks (hook pair; DREAMED - ARCH-87bw15t11s)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
use_diff(old, new) and use_diff_with(old, new, options), both returning a Diff owned by the calling component
```

How an app reaches a diff. The pair keeps the common call short while the second form carries the options, and the calling component's ownership is what makes the state outlive a render without the viewer holding it.

**Supports Requirements**: REQT-7tk9vxv9wd



<a id="software-object-ARCH-hrwskjav44"></a>

### LineNumberSpan (struct; DREAMED - ARCH-hrwskjav44)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
each side's first and last line numbers, either side absent when the run holds no line of that text
```

The shape both of the diff's reading answers come back in: the line numbers one run of entries spans, rather than the run itself. A run of added lines carries no old side and a run of removed lines no new side, which is why each side is optional.

**Supports Requirements**: REQT-f2affyt2h2, REQT-hsef5r7c4z



<a id="software-object-ARCH-syx4s6r1kf"></a>

### EntryLineNumberRelation (module; DREAMED - ARCH-syx4s6r1kf)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)
**Source**: `rust-dioxus/src/entry_line_numbers.rs`

```
a run of entry positions to each side's line-number span; a line id back to the entry position carrying it
```

The only translator between the engine's entry positions and the line numbers an app reads and names. Both reading answers translate outward through it and expanding at a named line translates inward, so no answer grows a second reckoning of where a line sits.

**Supports Requirements**: REQT-f2affyt2h2, REQT-hsef5r7c4z, REQT-g86vdmyyp9






## Decisions


<a id="decision-ARCH:dcisn-wma0h4mndz"></a>

### Engine separate from component (accepted; DREAMED - ARCH:dcisn-wma0h4mndz)

**Subject**: DiffAnalysisEngine (ARCH-jf3s5npp9s)

**Context**: Views and folding need line diff information without diffing inside rendering.

> **Situation**: The reference component calls its line computation during rendering (`src/index.tsx:479`).
>
> **Impact**: Diffing inside the component ties engine tests to a renderer and repeats work across renders.
>
> **Vision**: A presentation-free engine whose output the component renders.

The engine returns a ~~LineDiff~~ DiffAnalysis from two texts and ~~LineDiffOptions~~ DiffAnalysisOptions; ~~the component builds split and inline views and folding from that output alone~~ the diff plans the rows from that output alone and the viewer builds split and inline views from the plan (~~`rust-dioxus/src/live_diff.rs`~~ `rust-dioxus/src/diff.rs`, `rust-dioxus/src/diff_viewer.rs`). The engine stays presentation-free either way, which is what this decision settled.


**Also involves**:
- DiffViewer (ARCH-m4dkxzw6hh): ~~Renders views and folding from LineDiff alone~~ Renders views from the diff's row plan



<a id="decision-ARCH:dcisn-zyt966vq09"></a>

### Engine tokenizes lines, similar aligns them (accepted; DREAMED - ARCH:dcisn-zyt966vq09)

**Subject**: DiffAnalysisEngine (ARCH-jf3s5npp9s)

**Context**: Line identity must ignore terminators while each terminator stays reportable.

> **Situation**: `similar`'s line tokenizer keeps `\n`, `\r\n`, and a bare `\r` inside each line token.
>
> **Impact**: Lines with the same text and different terminators never match, so unchanged lines read as modified.
>
> **Vision**: The engine splits lines, holds each terminator aside, and hands `similar` only the line texts.

The engine owns line tokenization; `similar` aligns the resulting line texts and the tokens within a modified pair.


**Also involves**:
- similar (ARCH-wdrt39xvh4): Aligns the line texts the engine hands it


**Supports Requirements**: REQT-hmsfnfe5wc, REQT-rtwn1qresp



<a id="decision-ARCH:dcisn-pgydgmajhx"></a>

### Themes shipped as one layered stylesheet (accepted - ARCH:dcisn-pgydgmajhx)

**Subject**: DiffViewer (ARCH-m4dkxzw6hh)

**Context**: The viewer needs consumer style overrides, and emotion style objects have no Rust counterpart.

> **Situation**: The reference builds every rule at runtime from a per-instance `styles` prop through emotion (`src/styles.ts:78-145`).
>
> **Impact**: A Rust port of that shape would need a typed field per CSS declaration and would still reach less than CSS does.
>
> **Vision**: One stylesheet the crate embeds, whose colors are named custom properties an app reassigns in its own CSS.

The viewer renders its embedded stylesheet through `document::Style`, inside a cascade layer, so an app's own rules take effect at any specificity.


**Supports Requirements**: REQT-q356bvvv15, REQT-kcm5ba45sp, REQT-1accrb4jhp, REQT-ey9f1s27r1



<a id="decision-ARCH:dcisn-x8fa9g2hm8"></a>

### Engine renamed so the viewer can own the word diff (accepted; draft - ARCH:dcisn-x8fa9g2hm8)

**Subject**: DiffAnalysisEngine (ARCH-jf3s5npp9s)

**Context**: The app-held state this port is adding needs a name, and the fitting one already belongs to the engine.

> **Situation**: `line_diff` and `LineDiff` are public (`rust-dioxus/src/lib.rs:22`, `:25`) and carry roughly forty-five call sites in the engine's test file.
>
> **Impact**: One word would mean both the alignment the engine computes and the state an app holds, in a single namespace a consumer imports whole.
>
> **Vision**: Two names a reader cannot mistake for each other, each saying which of the two it is.

The engine's function becomes `analyze_diff` and its output becomes `DiffAnalysis`; the state an app holds takes the plain `Diff`, reached through `use_diff`. The rename reaches the engine modules and their filenames, the crate root, the engine tests, ARCH-exgfx6rwdt, the requirements' file list, and the glossary. Chosen over leaving the engine alone and qualifying the newcomer, which costs no paperwork but leaves the call site a consumer writes most often carrying the longer name. ~~Recorded ahead of the change: the code still carries the earlier names.~~ The code carries the new names: `rust-dioxus/src/lib.rs` exports `analyze_diff`, `DiffAnalysis` and `DiffAnalysisOptions`, and the engine's three modules are named for them.


**Also involves**:
- DiffAnalysis (ARCH-exgfx6rwdt): Becomes DiffAnalysis
- DiffViewer (ARCH-m4dkxzw6hh): Its app-held state takes the plain Diff



<a id="decision-ARCH:dcisn-xgpr1asvyn"></a>

### The app holds the diff; the viewer renders it (accepted; draft - ARCH:dcisn-xgpr1asvyn)

**Subject**: DiffViewer (ARCH-m4dkxzw6hh)

**Context**: An app driving a mounted viewer needs to read where the changes are and which lines are hidden, which a viewer owning that state can only publish.

> **Situation**: The viewer computes the diff, the identity hash, the fold state and the row plan in its own body (`rust-dioxus/src/diff_viewer.rs:119-152`), and keeps expansions readable without any write during rendering (`fold_reset_trigger.rs:75`).
>
> **Impact**: Publishing that state outward means either writing signals during render, which loops, or an effect that leaves the app a render behind the viewer.
>
> **Vision**: One object outside the view's code path that owns the inputs and derives the answers, so nothing is published and nothing lags.

A crate hook builds the diff from the two texts, as the pair `use_diff` and `use_diff_with`, the second taking an options value that composes the engine's options with the folding choices and defaults every field. The diff owns the texts, the engine call, the identity hash, the expanded folds with their basis, and the planned rows; it answers where the changes are and which lines are hidden, and it takes expand-at-a-line, expand-everything, and reset. The viewer takes it as its one data prop and keeps the presentation props: view, theme, line-number visibility, the consumer renderers, highlighting, the click handler, and the titles. Steering names lines and never folds, so ARCH-y9545npjzg's rule holds while its reset-only trigger is retired into the diff. Chosen over publishing from an effect, and over reporting through callbacks, which pushes but cannot answer what is folded now. ~~Recorded ahead of the change: the code still carries the earlier arrangement.~~ The code carries this arrangement: ~~`rust-dioxus/src/live_diff.rs`~~ `rust-dioxus/src/diff.rs` holds the diff and the hook pair, and `rust-dioxus/src/diff_viewer.rs` takes `diff` as its one data prop.


**Also involves**:
- FoldResetTrigger (ARCH-y9545npjzg): Widened into the diff and retired as a reset-only trigger
- DiffAnalysis (ARCH-exgfx6rwdt): Computed once inside the diff rather than inside the viewer





## Design Patterns



## Files



## Collaboration Summary






## Open Questions


- [x] ***RESOLVED:*** Do the folding choices belong to the diff or to the viewer beside `view`? *(context: The diff owns the row plan, so it needs the surrounding-line count and whether folding is on at all; both read to a consumer as display choices sitting a line or two away from `view`. Raised while settling ARCH:dcisn-xgpr1asvyn.)*
  **Resolution**: The diff owns them, as fields of the options value `use_diff_with` takes. ARCH:dcisn-xgpr1asvyn gives the diff the planned rows, and the plan cannot be computed without whether folding is on and how many unchanged lines surround each change; the expanded folds' basis already turns on the same count. Two viewers over one diff therefore share one row plan and one folding configuration. The viewer keeps no folding prop.



## Discovery Notes



