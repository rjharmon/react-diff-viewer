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


### 1.0: What changed (**BACKLOG**) - **AREA:** What changed - Ensures accurate change marking. Applies when reading or changing line and word comparison, or compare methods. - <nobr>***draft** REQT-9273mztsx4*</nobr>


#### 1.1.0: Compare methods (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-rvcg21axa8*</nobr>

 - **1.1.1: Character comparison** (**BACKLOG**) - The viewer MUST compare modified lines character by character unless the consumer selects another compare method. <nobr>***DREAMED** REQT-czecf8krqc*</nobr>
 - **1.1.2: Word comparison** (**BACKLOG**) - The viewer MUST offer word comparison, with each whitespace run and each non-whitespace run as one token. <nobr>***DREAMED** REQT-xzc8n354h1*</nobr>
 - **1.1.3: Line comparison** (**BACKLOG**) - The viewer MUST offer line comparison, with each modified line as one token. <nobr>***DREAMED** REQT-spzdk2z1pk*</nobr>

#### 1.2.0: Line changes (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-wqffp3d26e*</nobr>

 - **1.2.1: Line marking** (**BACKLOG**) - **lines aligned as similar aligns them, each marked** The viewer MUST mark each line as unchanged, removed, or added, aligning the two texts as `similar`'s line diff aligns them. <nobr>***DREAMED** REQT-hmsfnfe5wc*</nobr>
 - **1.2.2: Modified lines** (**BACKLOG**) - **removed lines pair in order with the added lines after them** Where removed lines are immediately followed by added lines, the viewer MUST pair them in order as modified lines, and MUST show lines left without a partner as plain removals or additions. <nobr>***DREAMED** REQT-dqxm8fa7ts*</nobr>
 - **1.2.3: Inline changes** (**BACKLOG**) - **tokens changed within modified lines, on unless turned off** Unless the consumer turns inline changes off, the viewer MUST mark within each modified line the tokens removed from the old line and the tokens added in the new line, using the selected compare method. <nobr>***DREAMED** REQT-4nz35dscrn*</nobr>
 - **1.2.4: Trailing whitespace** (**BACKLOG**) - **trailing whitespace of either text never shows as a change** The viewer MUST ignore whitespace at the end of each text, so trailing blank lines never show as changes. <nobr>***DREAMED** REQT-9tze98pt6g*</nobr>

#### 1.3.0: Views (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-2k7j51afde*</nobr>

 - **1.3.1: Split view** (**BACKLOG**) - **old left, new right, unless inline is chosen** Unless the consumer selects the inline view, the viewer MUST show old lines on the left and new lines on the right, one pair per row. <nobr>***DREAMED** REQT-ys3yr5g185*</nobr>
 - **1.3.2: Inline view** (**BACKLOG**) - **one column, old text above new on modified lines** In the inline view, the viewer MUST show lines in one column, with a modified line's old text directly above its new text, and each unchanged line once with both its old and new line numbers. <nobr>***DREAMED** REQT-0xbgrj9ane*</nobr>
 - **1.3.3: Change markers** (**BACKLOG**) - The viewer MUST mark removed lines with `-` and added lines with `+` in both views. <nobr>***DREAMED** REQT-wjjyqjnjs6*</nobr>

--------


### 2.0: Where they are (**BACKLOG**) - **AREA:** Where they are - Ensures readers stay oriented in long diffs. Applies when reading or changing folding, line numbering, highlighting, or line selection. - <nobr>***draft** REQT-wxe0svnb22*</nobr>


#### 2.1.0: Line numbers (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-yxkc61eq90*</nobr>

 - **2.1.1: Independent numbering** (**BACKLOG**) - **old and new lines counted separately from the offset** The viewer MUST number old-text lines and new-text lines independently, each starting at one more than the consumer's line offset, which defaults to 0. <nobr>***DREAMED** REQT-smd01rma2q*</nobr>
 - **2.1.2: Hidden line numbers** (**BACKLOG**) - The viewer MUST show line numbers unless the consumer hides them. <nobr>***DREAMED** REQT-m9r3k5b1ge*</nobr>

#### 2.2.0: Folding (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-qerexp825r*</nobr>

 - **2.2.1: Folded unchanged lines** (**BACKLOG**) - **unchanged lines beyond the surrounding count fold away** Unless the consumer turns folding off, the viewer MUST fold unchanged lines lying more than the surrounding-line count away from every change; that count defaults to 3 and treats negative values as 0. <nobr>***DREAMED** REQT-qcnxhemvhn*</nobr>
 - **2.2.2: Fold rows** (**BACKLOG**) - **one row per fold, consumer may supply its content** The viewer MUST show each fold as one row reading "Expand N lines ...", where N is the number of hidden lines, unless the consumer supplies the row's content, which the viewer MUST render from the hidden-line count and the old and new line numbers of the first hidden line. <nobr>***DREAMED** REQT-1tdrfvay4q*</nobr>
 - **2.2.3: Expanding folds** (**BACKLOG**) - Activating a fold row MUST reveal the lines it hides, and they MUST stay revealed until the consumer resets folds. <nobr>***DREAMED** REQT-v748c7mjr6*</nobr>
 - **2.2.4: Fold reset** (**BACKLOG**) - The viewer MUST let the consumer return every expanded fold to folded. <nobr>***DREAMED** REQT-869jyzdes7*</nobr>

#### 2.3.0: Line selection (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-p2gkf77kjj*</nobr>

 - **2.3.1: Highlighted lines** (**BACKLOG**) - **lines listed by id are highlighted** The viewer MUST highlight each line whose old-side or new-side line id the consumer lists, where a line id joins its side (`L` for old, `R` for new) and its line number with a hyphen, as in `L-20`. <nobr>***DREAMED** REQT-zaens35zqy*</nobr>
 - **2.3.2: Line number clicks** (**BACKLOG**) - When the consumer supplies a line-number click handler, the viewer MUST call it with the clicked line's id each time a line number is clicked. <nobr>***DREAMED** REQT-bqm9w6v4ms*</nobr>

--------


### 3.0: How it fits the app (**BACKLOG**) - **AREA:** How it fits the app - Ensures the viewer matches the app's look. Applies when reading or changing themes, style overrides, or title and line content rendering. - <nobr>***draft** REQT-x8e364jbqm*</nobr>


#### 3.1.0: Content rendering (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-3ekk7hre3k*</nobr>

 - **3.1.1: Column titles** (**BACKLOG**) - When the consumer supplies titles, the viewer MUST show the left title above the old column and the right title above the new column in the split view, and only the left title in the inline view; either title MAY be text or rendered content. <nobr>***DREAMED** REQT-vbaqm4y5zk*</nobr>
 - **3.1.2: Custom line content** (**BACKLOG**) - **consumer renderer shapes line text and inline tokens** When the consumer supplies a content renderer, the viewer MUST render line text through it, including each inline-change token on modified lines. <nobr>***DREAMED** REQT-vxtax4x0vs*</nobr>

#### 3.2.0: Themes (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-3cb9k1mg2d*</nobr>

 - **3.2.1: Theme selection** (**BACKLOG**) - **follows the reader's color-scheme preference unless the app chooses** Unless the consumer selects light or dark, the viewer MUST follow the reader's light or dark color-scheme preference from the browser or platform. <nobr>***DREAMED** REQT-sc8expw3q8*</nobr>

# Files


# Implementation Log

> Maintainers MUST NOT modify past entries. Append new entries only.


# Release Management Plan

See `release-management-scope.md` for version criteria and lifecycle management.
