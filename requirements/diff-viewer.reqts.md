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

--------


### 2.0: Where they are (**BACKLOG**) - **AREA:** Where they are - Ensures readers stay oriented in long diffs. Applies when reading or changing folding, line numbering, highlighting, or line selection. - <nobr>***draft** REQT-wxe0svnb22*</nobr>


#### 2.1.0: Line numbers (**BACKLOG**) - **AREA:** ‹no-area› - <nobr>***DREAMED** REQT-yxkc61eq90*</nobr>

 - **2.1.1: Independent numbering** (**BACKLOG**) - **old and new lines counted separately from the offset** The viewer MUST number old-text lines and new-text lines independently, each starting at one more than the consumer's line offset, which defaults to 0. <nobr>***DREAMED** REQT-smd01rma2q*</nobr>

--------


### 3.0: How it fits the app (**BACKLOG**) - **AREA:** How it fits the app - Ensures the viewer matches the app's look. Applies when reading or changing themes, style overrides, or title and line content rendering. - <nobr>***draft** REQT-x8e364jbqm*</nobr>


# Files


# Implementation Log

> Maintainers MUST NOT modify past entries. Append new entries only.


# Release Management Plan

See `release-management-scope.md` for version criteria and lifecycle management.
