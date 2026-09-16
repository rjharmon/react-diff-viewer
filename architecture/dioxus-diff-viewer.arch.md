
# dioxus-diff-viewer - Architecture

> ⚠️ Automatically generated from related JSONL file; maintain using the Eidos Architect skill.

*local; DREAMED; ARCH-h2qwqte1g6*

Dioxus diff viewer crate built on a line diff engine.

**Activities**:

- Show the differences between two texts in Dioxus apps

**Responsibilities**:

- Crate identity and public vocabulary drawn from requirement terms

**Supports Requirements**: REQT-9273mztsx4, REQT-wxe0svnb22, REQT-x8e364jbqm





**Maturity**:

- DREAMED: 10/11
- draft: 1/11

## In this document

[Components and Concerns](#components-and-concerns)&nbsp;&nbsp; [Components](#components)&nbsp;&nbsp; [Interactions](#interactions)&nbsp;&nbsp; [Software Objects](#software-objects)&nbsp;&nbsp; [Decisions](#decisions)&nbsp;&nbsp; [Design Patterns](#design-patterns)&nbsp;&nbsp; [Files](#files)&nbsp;&nbsp; [Collaboration Summary](#collaboration-summary)&nbsp;&nbsp; [Open Questions](#open-questions)&nbsp;&nbsp; [Discovery Notes](#discovery-notes)

## Components and Concerns




- [DiffViewer](#diffviewer-arch-m4dkxzw6hh) (internal): The Dioxus component apps mount; placeholder until the component work elaborates it.
- [LineDiffEngine](#linediffengine-arch-jf3s5npp9s) (internal): Turns two texts and options into paired line information.
- [similar](#similar-arch-wdrt39xvh4) (external): Third-party Rust crate computing text alignment.



## Components

<a id="diffviewer-arch-m4dkxzw6hh"></a>

### Component: DiffViewer (internal; DREAMED - ARCH-m4dkxzw6hh)

The Dioxus component apps mount; placeholder until the component work elaborates it.

**Activities**:

- Render split and inline views from a LineDiff
- Fold unchanged lines around changes



**Supports Requirements**: REQT-2k7j51afde, REQT-qerexp825r, REQT-p2gkf77kjj, REQT-3ekk7hre3k, REQT-m9r3k5b1ge

**Concerns and Responsibilities**:
- **Responsibility**: Rendering engine output without diffing again
- **Responsibility**: Expanded-fold state and its reset trigger


**Interactions**: [Line diff hand-off](#interaction-ARCH-atczcqvdsz)



---

<a id="linediffengine-arch-jf3s5npp9s"></a>

### Component: LineDiffEngine (internal; DREAMED - ARCH-jf3s5npp9s)

Turns two texts and options into paired line information.

**Activities**:

- Pair old and new lines
- Mark changes, line endings, and inline changes
- Number lines on each side



**Exclusions** (boundary clarifications):
- Does NOT handle Views, folding, and styling → see ARCH-m4dkxzw6hh (DiffViewer) - The engine carries no presentation.


**Supports Requirements**: REQT-czecf8krqc, REQT-xzc8n354h1, REQT-spzdk2z1pk, REQT-hmsfnfe5wc, REQT-dqxm8fa7ts, REQT-4nz35dscrn, REQT-9tze98pt6g, REQT-rtwn1qresp, REQT-smd01rma2q

**Concerns and Responsibilities**:
- **Responsibility**: Line change marking as `similar` aligns lines
- **Responsibility**: Inline change tokens under each compare method
- **Responsibility**: Independent line numbering from the offset
- **Responsibility**: Positions of entries holding a change


**Interactions**: [Line diff hand-off](#interaction-ARCH-atczcqvdsz)



---

<a id="similar-arch-wdrt39xvh4"></a>

### Component: similar (external; DREAMED - ARCH-wdrt39xvh4)

Third-party Rust crate computing text alignment.

**Activities**:

- Align lines and tokens between two texts



**Interactions**: [Line diff hand-off](#interaction-ARCH-atczcqvdsz)



---


### Nested Components

> These components declare a `parentComponent` and should each have a separate architectural breakdown document. The summary here is a placeholder until that breakdown exists.

| Component | Parent | Summary |
|-----------|--------|---------|
| [DiffViewer](#diffviewer-arch-m4dkxzw6hh) | [dioxus-diff-viewer](#dioxus-diff-viewer-arch-h2qwqte1g6) | The Dioxus component apps mount; placeholder until the component work elaborates it. |
| [LineDiffEngine](#linediffengine-arch-jf3s5npp9s) | [dioxus-diff-viewer](#dioxus-diff-viewer-arch-h2qwqte1g6) | Turns two texts and options into paired line information. |



## Interactions



<a id="interaction-ARCH-atczcqvdsz"></a>

### Interaction: Line diff hand-off (library_call; DREAMED - ARCH-atczcqvdsz)

[DiffViewer](#diffviewer-arch-m4dkxzw6hh), [LineDiffEngine](#linediffengine-arch-jf3s5npp9s), [similar](#similar-arch-wdrt39xvh4) - Trigger: The component renders with new texts or options.

The component asks the engine for a LineDiff and renders from it; the engine splits the lines itself and asks `similar` to align their texts.

**Payload**: two texts and LineDiffOptions; LineDiff

**Error**: None on text input: every pair of texts yields a LineDiff.
**Supports Requirements**: REQT-hmsfnfe5wc



![Line diff hand-off: collaboration](./diagrams/ARCH-atczcqvdsz-collaboration.svg)


---




## Data Flows





## Software Objects


<a id="software-object-ARCH-n7wmjnmt65"></a>

### LineDiffOptions (struct; DREAMED - ARCH-n7wmjnmt65)

**Component**: LineDiffEngine (ARCH-jf3s5npp9s)

```
compare method (character, word, line); inline changes on or off; line offset
```

The engine's input choices; the app sets them through the component.

**Supports Requirements**: REQT-czecf8krqc, REQT-xzc8n354h1, REQT-spzdk2z1pk, REQT-4nz35dscrn, REQT-smd01rma2q



<a id="software-object-ARCH-exgfx6rwdt"></a>

### LineDiff (struct; DREAMED - ARCH-exgfx6rwdt)

**Component**: LineDiffEngine (ARCH-jf3s5npp9s)

```
paired line entries in display order; positions of entries holding a change, including a pair whose terminators differ
```

The engine's output and the component's only input for views and folding.

**Supports Requirements**: REQT-hmsfnfe5wc, REQT-dqxm8fa7ts, REQT-rtwn1qresp, REQT-qcnxhemvhn



<a id="software-object-ARCH-kjjykjtt5r"></a>

### PairedLineEntry (struct; DREAMED - ARCH-kjjykjtt5r)

**Component**: LineDiffEngine (ARCH-jf3s5npp9s)

```
old side and new side, each optional with text and line number; change kind (unchanged, removed, added, modified); inline change tokens on modified entries, each side's tokens marked unchanged, removed, or added and rejoining to that side's text; line ending change with each side's terminator
```

One row of the diff as either view reads it.

**Supports Requirements**: REQT-hmsfnfe5wc, REQT-dqxm8fa7ts, REQT-4nz35dscrn, REQT-rtwn1qresp, REQT-smd01rma2q



<a id="software-object-ARCH-y9545npjzg"></a>

### FoldResetTrigger (struct; draft - ARCH-y9545npjzg)

**Component**: DiffViewer (ARCH-m4dkxzw6hh)

```
an opaque copyable handle whose one action returns every expanded fold to folded
```

The optional prop through which an app resets folds, and nothing more. The crate creates it and owns the folds it acts on, so an app never names a fold; a viewer given none keeps its folds to itself.

**Supports Requirements**: REQT-869jyzdes7, REQT-v748c7mjr6






## Decisions


<a id="decision-ARCH:dcisn-wma0h4mndz"></a>

### Engine separate from component (accepted; DREAMED - ARCH:dcisn-wma0h4mndz)

**Subject**: LineDiffEngine (ARCH-jf3s5npp9s)

**Context**: Views and folding need line diff information without diffing inside rendering.

> **Situation**: The reference component calls its line computation during rendering (`src/index.tsx:479`).
>
> **Impact**: Diffing inside the component ties engine tests to a renderer and repeats work across renders.
>
> **Vision**: A presentation-free engine whose output the component renders.

The engine returns a LineDiff from two texts and LineDiffOptions; the component builds split and inline views and folding from that output alone.


**Also involves**:
- DiffViewer (ARCH-m4dkxzw6hh): Renders views and folding from LineDiff alone



<a id="decision-ARCH:dcisn-zyt966vq09"></a>

### Engine tokenizes lines, similar aligns them (accepted; DREAMED - ARCH:dcisn-zyt966vq09)

**Subject**: LineDiffEngine (ARCH-jf3s5npp9s)

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





## Design Patterns



## Files



## Collaboration Summary






## Open Questions




## Discovery Notes



