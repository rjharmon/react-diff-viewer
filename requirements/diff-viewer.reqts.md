# About Diff Viewer

## MAINTAINERS MUST READ:
> **AUTO-GENERATED FILE - DO NOT EDIT DIRECTLY**
>
> This file is generated from the `.reqts.jsonl` source. To make changes:
> 1. Edit the JSONL source file
> 2. Run `reqts.fromJSONL <source.reqts.jsonl>` to regenerate
>
> **COMPLIANCE TRIGGER**: Before interpreting these requirements, you **MUST** read:
> `reqt-consumer.SKILL.md`
>
> **hash.notice.reqt-consumer**: 5dddc026e9370dc8

A Dioxus component that shows the differences between two texts, split or inline, with word-level detail on changed lines. *(module maturity: **DREAMED**)*


The essential technologies are **Rust, dioxus, similar**. Related technologies include tracing.



# Background

1. react-diff-viewer serves only React apps; Dioxus apps have no equivalent diff viewer.
2. Its diffing and styling rest on JavaScript libraries (jsdiff, emotion) with no Rust counterparts, so the port rebuilds both on Rust foundations.



# Must Read: Special Skills and Know-how


# Collaborators



**Expected users:** Dioxus apps that show changes between two versions of text or code.

# Functional Areas and Key Requirements

### 1. What changed
<nobr>**Key Requirements:**</nobr>
- **What changed**: Readers see which lines changed, and which words changed within them.

### 2. Where they are
<nobr>**Key Requirements:**</nobr>
- **Where they are**: Readers stay oriented in long diffs: unchanged lines fold away, and every line has a number.

### 3. How it fits the app
<nobr>**Key Requirements:**</nobr>
- **How it fits the app**: Apps style the viewer, light or dark, and render line content their own way.


# Detailed Requirements


### 1.0: What changed (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** What changed - Ensures accurate change marking. Applies when reading or changing line and word comparison, or compare methods. - <nobr>***draft** REQT-9273mztsx4*</nobr>


#### 1.1.0: Compare methods (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-rvcg21axa8*</nobr>

 - **1.1.1: Character comparison** (**IMPLEMENTED/NEEDS VERIFICATION**) - The viewer MUST compare modified lines character by character unless the consumer selects another compare method. <nobr>***DREAMED** REQT-czecf8krqc*</nobr>
 - **1.1.2: Word comparison** (**IMPLEMENTED/NEEDS VERIFICATION**) - The viewer MUST offer word comparison, with each whitespace run and each non-whitespace run as one token. <nobr>***DREAMED** REQT-xzc8n354h1*</nobr>
 - **1.1.3: Line comparison** (**IMPLEMENTED/NEEDS VERIFICATION**) - The viewer MUST offer line comparison, with each modified line as one token. <nobr>***DREAMED** REQT-spzdk2z1pk*</nobr>
 - **1.1.4: Trimmed line comparison** (**IMPLEMENTED/NEEDS VERIFICATION**) - **whole-line token ignoring leading and trailing whitespace** The viewer MUST offer trimmed line comparison, with each modified line as one token, marked unchanged when the old and new lines differ only in leading or trailing whitespace. <nobr>***DREAMED** REQT-z9r0pc53jg*</nobr>

#### 1.2.0: Line changes (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-wqffp3d26e*</nobr>

 - **1.2.1: Line marking** (**IMPLEMENTED/NEEDS VERIFICATION**) - **lines aligned as similar aligns them, each marked** The viewer MUST mark each line as unchanged, removed, or added, aligning the two texts as `similar`'s line diff aligns them and comparing lines by their text without line terminators. <nobr>***DREAMED** REQT-hmsfnfe5wc*</nobr>
 - **1.2.2: Modified lines** (**IMPLEMENTED/NEEDS VERIFICATION**) - **removed lines pair in order with the added lines after them** Where removed lines are immediately followed by added lines, the viewer MUST pair them in order as modified lines, and MUST show lines left without a partner as plain removals or additions. <nobr>***DREAMED** REQT-dqxm8fa7ts*</nobr>
 - **1.2.3: Inline changes** (**IMPLEMENTED/NEEDS VERIFICATION**) - **tokens changed within modified lines, on unless turned off** Unless the consumer turns inline changes off, the viewer MUST mark within each modified line the tokens removed from the old line and the tokens added in the new line, using the selected compare method. <nobr>***DREAMED** REQT-4nz35dscrn*</nobr>
 - **1.2.4: Trailing whitespace** (**IMPLEMENTED/NEEDS VERIFICATION**) - **trailing whitespace of either text never shows as a change** The viewer MUST ignore whitespace at the end of each text, so trailing blank lines never show as changes. <nobr>***DREAMED** REQT-9tze98pt6g*</nobr>
 - **1.2.5: Line ending changes** (**IMPLEMENTED/NEEDS VERIFICATION**) - **differing terminators on paired lines are marked** Where both lines of an unchanged or modified pair end with a line terminator and the two terminators differ, the viewer MUST mark that pair as carrying a line ending change, with each side's terminator. <nobr>***DREAMED** REQT-rtwn1qresp*</nobr>
 - **1.2.6: Leading or trailing whitespace changes** (**IMPLEMENTED/NEEDS VERIFICATION**) - **trimmed comparison still marks end-of-line whitespace edits** Under trimmed line comparison, where the old and new lines of a modified pair differ in leading or trailing whitespace, the viewer MUST mark that pair as carrying a whitespace change. <nobr>***DREAMED** REQT-hq1fzjaxg8*</nobr>

#### 1.3.0: Views (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-2k7j51afde*</nobr>

 - **1.3.1: Split view** (**IMPLEMENTED/NEEDS VERIFICATION**) - **old left, new right, unless inline is chosen** Unless the consumer selects the inline view, the viewer MUST show old lines on the left and new lines on the right, one pair per row. <nobr>***DREAMED** REQT-ys3yr5g185*</nobr>
 - **1.3.2: Inline view** (**IMPLEMENTED/NEEDS VERIFICATION**) - **one column, old text above new on modified lines** In the inline view, the viewer MUST show lines in one column, with a modified line's old text directly above its new text, and each unchanged line once with both its old and new line numbers. <nobr>***DREAMED** REQT-0xbgrj9ane*</nobr>
 - **1.3.3: Change markers** (**IMPLEMENTED/NEEDS VERIFICATION**) - The viewer MUST mark removed lines with `-` and added lines with `+` in both views. <nobr>***DREAMED** REQT-wjjyqjnjs6*</nobr>
 - **1.3.4: Line ending chips** (**IMPLEMENTED/NEEDS VERIFICATION**) - **each side's terminator shown as an escaped-text chip** In both views, the viewer MUST show each side of a line ending change as a compact chip on that side's line, reading the terminator as escaped text: `\n`, `\r\n`, or `\r`; where the inline view shows an unchanged pair as one line, that line MUST show the old side's chip, then an arrow, then the new side's chip. <nobr>***draft** REQT-4zyjfjrhd3*</nobr>
 - **1.3.5: Whitespace chips** (**IMPLEMENTED/NEEDS VERIFICATION**) - **WS chip on both sides of a whitespace change** In both views, the viewer MUST show a compact chip reading `WS` on each side's line of a pair carrying a whitespace change. <nobr>***DREAMED** REQT-27a1gxq15f*</nobr>

--------


### 2.0: Where they are (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** Where they are - Ensures readers stay oriented in long diffs. Applies when reading or changing folding, line numbering, highlighting, or line selection. - <nobr>***draft** REQT-wxe0svnb22*</nobr>


#### 2.1.0: Line numbers (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-yxkc61eq90*</nobr>

 - **2.1.1: Independent numbering** (**IMPLEMENTED/NEEDS VERIFICATION**) - **old and new lines counted separately from the offset** The viewer MUST number old-text lines and new-text lines independently, each starting at one more than the consumer's line offset, which defaults to 0. <nobr>***DREAMED** REQT-smd01rma2q*</nobr>
 - **2.1.2: Hidden line numbers** (**IMPLEMENTED/NEEDS VERIFICATION**) - The viewer MUST show line numbers unless the consumer hides them. <nobr>***DREAMED** REQT-m9r3k5b1ge*</nobr>

#### 2.2.0: Folding (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-qerexp825r*</nobr>

 - **2.2.1: Folded unchanged lines** (**IMPLEMENTED/NEEDS VERIFICATION**) - **unchanged lines beyond the surrounding count fold away** Unless the consumer turns folding off, the viewer MUST fold unchanged lines lying more than the surrounding-line count away from every change; that count defaults to 3. <nobr>***draft** REQT-qcnxhemvhn*</nobr>
 - **2.2.2: Fold rows** (**IMPLEMENTED/NEEDS VERIFICATION**) - **one row per fold, consumer may supply its content** The viewer MUST show each fold as one row reading "Expand N lines ...", where N is the number of hidden lines, unless the consumer supplies the row's content, which the viewer MUST render from the hidden-line count and the old and new line numbers of the first hidden line. <nobr>***DREAMED** REQT-1tdrfvay4q*</nobr>
 - **2.2.3: Expanding folds** (**IMPLEMENTED/NEEDS VERIFICATION**) - Activating a fold row MUST reveal the lines it hides, and they MUST stay revealed until the consumer resets folds or the compared texts or surrounding-line count change, which return every fold to folded. <nobr>***DREAMED** REQT-v748c7mjr6*</nobr>
 - **2.2.4: Fold reset** (**IMPLEMENTED/NEEDS VERIFICATION**) - **viewer keeps the state; an optional trigger resets every fold** The viewer MUST keep the expanded-fold state itself, and MUST let a consumer holding its fold reset trigger return every expanded fold to folded. The fold reset trigger MUST be optional, so that a consumer which never resets does not hold one. <nobr>***draft** REQT-869jyzdes7*</nobr>

#### 2.3.0: Line selection (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-p2gkf77kjj*</nobr>

 - **2.3.1: Highlighted lines** (**IMPLEMENTED/NEEDS VERIFICATION**) - **lines listed by id are highlighted** The viewer MUST highlight each line whose old-side or new-side line id the consumer lists, where a line id joins its side (`L` for old, `R` for new) and its line number with a hyphen, as in `L-20`. <nobr>***DREAMED** REQT-zaens35zqy*</nobr>
 - **2.3.2: Line number clicks** (**IMPLEMENTED/NEEDS VERIFICATION**) - **handler gets the line id and held modifier keys** When the consumer supplies a line-number click handler, the viewer MUST call it with the clicked line's id and the modifier keys held, each time a line number is clicked. <nobr>***DREAMED** REQT-bqm9w6v4ms*</nobr>
 - **2.3.3: Line ids from text** (**IMPLEMENTED/NEEDS VERIFICATION**) - **text form reads back into a line id** The viewer MUST let the consumer turn a line id's text form, as in `L-20` or `R-3`, back into a line id, and MUST report text in any other form as not a line id. <nobr>***DREAMED** REQT-jgkndzqkyz*</nobr>

--------


### 3.0: How it fits the app (**BACKLOG**) - **AREA:** How it fits the app - Ensures the viewer matches the app's look. Applies when reading or changing themes, style overrides, or title and line content rendering. - <nobr>***draft** REQT-x8e364jbqm*</nobr>


#### 3.1.0: Content rendering (**IMPLEMENTED/NEEDS VERIFICATION**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-3ekk7hre3k*</nobr>

 - **3.1.1: Column titles** (**IMPLEMENTED/NEEDS VERIFICATION**) - When the consumer supplies titles, the viewer MUST show the left title above the old column and the right title above the new column in the split view, and only the left title in the inline view; either title MAY be text or rendered content. <nobr>***DREAMED** REQT-vbaqm4y5zk*</nobr>
 - **3.1.2: Custom line content** (**IMPLEMENTED/NEEDS VERIFICATION**) - **consumer renderer shapes line text and inline tokens** When the consumer supplies a content renderer, the viewer MUST render line text through it, including each inline-change token on modified lines, and MUST NOT call it for a side of a split row that has no line. <nobr>***DREAMED** REQT-vxtax4x0vs*</nobr>
 - **3.1.3: Rendered content identity** (**IMPLEMENTED/NEEDS VERIFICATION**) - **rendered content stays with its line until a text changes** The viewer MUST keep content from the consumer's renderers mounted with its line while the compared texts stay the same, including as folds expand, and MUST remount all such content when either text changes. <nobr>***DREAMED** REQT-zen8fyae28*</nobr>

#### 3.2.0: Themes (**NEXT**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-3cb9k1mg2d*</nobr>

 - **3.2.1: Theme selection** (**NEXT**) - **follows the reader's color-scheme preference unless the app chooses** Unless the consumer selects light or dark, the viewer MUST follow the reader's light or dark color-scheme preference from the browser or platform. <nobr>***DREAMED** REQT-sc8expw3q8*</nobr>
 - **3.2.2: Chip and arrow color** (**NEXT**) - **chips and the arrow contrast with the line backgrounds beneath them** Each theme MUST color line ending chips, the line ending arrow, and whitespace chips so they contrast with every line background they appear on. The `react-diff-viewer` package renders none of these, so each theme names their colors rather than carrying them from the reference. <nobr>***draft** REQT-8y5fp2aarz*</nobr>
 - **3.2.3: Styling hooks** (**NEXT**) - **stable class per element kind, plus state classes and a clickable gutter** The viewer MUST give itself, the title, each row, gutter, change marker, content cell, inline-change token, line ending chip, line ending arrow, whitespace chip, and fold row a stable class name naming its kind, plus a class for its split or inline view on the viewer itself, and for its change or highlight state elsewhere, if relevant, all prefixed with `dxdiff`, so that apps and themes can style the viewer without depending on its element structure or colliding with the app's own classes. A gutter that reports its clicks MUST additionally carry a class naming that, so a hover affordance reaches only the gutters that answer a click. <nobr>***draft** REQT-3928hx46s3*</nobr>
 - **3.2.4: Theme palettes** (**NEXT**) - **one named color set per theme, carrying the reference colors** The viewer MUST color every element it renders from one named set of colors per theme, and the light and dark sets MUST carry the colors the `react-diff-viewer` package uses for the same elements. <nobr>***DREAMED** REQT-78dg0g6a2h*</nobr>
 - **3.2.5: Style delivery** (**NEXT**) - **the viewer brings its own styles; the app adds none** The viewer MUST render its own styles into the document, so that an app mounting it adds no stylesheet of its own. <nobr>***DREAMED** REQT-q356bvvv15*</nobr>
 - **3.2.6: Palette override** (**NEXT**) - **app CSS alone restyles the viewer, everywhere or one at a time** An app MUST be able to restyle the viewer, wholly or one mounted viewer at a time, using only its own CSS. <nobr>***DREAMED** REQT-kcm5ba45sp*</nobr>
     - **3.2.6.1: Named colors** (**NEXT**) - **colors read from dxdiff custom properties defaulted at the root** The viewer MUST read every theme color from a `dxdiff`-prefixed custom property, and MUST declare each property's default at the document root, so that an app's assignment on any ancestor takes effect for a viewer following the reader's color-scheme preference. A viewer given an explicit theme MAY carry that theme's values on its own element, in which case an app restyles it with rules reaching the viewer element or below. <nobr>***DREAMED** REQT-1accrb4jhp*</nobr>
     - **3.2.6.2: Yielding rules** (**NEXT**) - **the viewer's rules lose to the app's, whatever the specificity** The viewer's own style rules MUST yield to an app's rules for the same element whatever their specificity, so that an app never needs `!important` to restyle the viewer. <nobr>***DREAMED** REQT-ey9f1s27r1*</nobr>

# Files

- `rust-dioxus/src/lib.rs` - The crate root: its modules and the public names apps import.
- `rust-dioxus/src/line_diff_engine.rs` - Splits both texts into lines, aligns them through `similar`, and builds the paired entries: line marking, modified-line pairing, inline change tokens under each compare method, independent numbering, and the changed positions.
- `rust-dioxus/src/line_diff_options.rs` - The consumer's input choices: compare method, whether to mark inline changes, and the line offset.
- `rust-dioxus/src/line_diff_output.rs` - What the engine hands the component: paired line entries with each side's text and number, the change kind, inline tokens indexed into their line, and any line ending change.
- `rust-dioxus/src/diff_viewer.rs` - The component apps mount: its props, the engine call, fold state, and the viewer table with its titles.
- `rust-dioxus/src/row_rendering.rs` - The rows of both views: line rows with gutters, change markers, content, and chips, and fold rows.
- `rust-dioxus/src/fold_planning.rs` - Which rows a view shows: each paired line entry, or a fold row for a run of unchanged lines far from every change.
- `rust-dioxus/src/fold_reset_trigger.rs` - The expanded-fold state the viewer keeps, and the optional trigger an app uses to return every expanded fold to folded.
- `rust-dioxus/src/consumer_callbacks.rs` - What the viewer hands the app's renderers and click handler: line content, hidden lines, and line-number clicks with held keys.
- `rust-dioxus/src/styling_hooks.rs` - The `dxdiff` class names the viewer's markup carries for each element kind, view, and state.
- `rust-dioxus/src/line_id.rs` - Line ids: a line's side and number, written and read as `L-20` or `R-3`.

# Implementation Log

> Maintainers MUST NOT modify past entries. Append new entries only.


# Release Management Plan

See `release-management-scope.md` for version criteria and lifecycle management.
