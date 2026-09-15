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
- **What changed**: Readers see at a glance which lines changed, and exactly which words changed within them.

### 2. Where they are
<nobr>**Key Requirements:**</nobr>
- **Where they are**: Readers keep their bearings: unchanged stretches fold out of the way, and every line has a number they can point to.

### 3. How it fits the app
<nobr>**Key Requirements:**</nobr>
- **How it fits the app**: Apps give the viewer their own look, light or dark, down to how each line's content is drawn.


# Detailed Requirements


### 1.0: What changed (**BACKLOG**) - **AREA:** What changed - Ensures readers can trust what the viewer marks as changed. Applied when reading or reviewing how changes are found and shown within lines, or when adding a way to compare text. - <nobr>***DREAMED** REQT-9273mztsx4*</nobr>


--------


### 2.0: Where they are (**BACKLOG**) - **AREA:** Where they are - Ensures readers stay oriented in long diffs. Applied when reading or changing how unchanged lines fold and open, or how lines are numbered, highlighted, and selected. - <nobr>***DREAMED** REQT-wxe0svnb22*</nobr>


--------


### 3.0: How it fits the app (**BACKLOG**) - **AREA:** How it fits the app - Ensures the viewer can look native in any app. Applied when reviewing or changing themes, style overrides, or how an app draws line content and titles. - <nobr>***DREAMED** REQT-x8e364jbqm*</nobr>


# Files


# Implementation Log

> Maintainers MUST NOT modify past entries. Append new entries only.


# Release Management Plan

See `release-management-scope.md` for version criteria and lifecycle management.
