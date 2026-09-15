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
 - **1.1.4: jsdiff methods without a counterpart** (**BACKLOG**) - The viewer MUST NOT offer sentence, CSS, or trimmed-line comparison, since `similar` has no tokenizer for them; word comparison covers jsdiff's words-with-space method. <nobr>***DREAMED** REQT-h70r7qp9g0*</nobr>

--------


### 2.0: Where they are (**BACKLOG**) - **AREA:** Where they are - Ensures readers stay oriented in long diffs. Applies when reading or changing folding, line numbering, highlighting, or line selection. - <nobr>***draft** REQT-wxe0svnb22*</nobr>


--------


### 3.0: How it fits the app (**BACKLOG**) - **AREA:** How it fits the app - Ensures the viewer matches the app's look. Applies when reading or changing themes, style overrides, or title and line content rendering. - <nobr>***draft** REQT-x8e364jbqm*</nobr>


# Files


# Implementation Log

> Maintainers MUST NOT modify past entries. Append new entries only.


# Release Management Plan

See `release-management-scope.md` for version criteria and lifecycle management.
