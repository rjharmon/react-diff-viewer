# react-diff-viewer port to Rust and Dioxus

**Motivation**: Deliver the port MISSION.md describes: react-diff-viewer as a Dioxus component in the rust-dioxus/ crate, diffing with the similar crate.

**Scope**: Includes the line diff engine, the diff viewer component, and theming, one work chunk each. Excludes changes to the TypeScript package and porting the examples/ demo app.

**Success criteria**: Every mission outcome in MISSION.md is met by a delivered work chunk.

**Status**: `draft` · **Contents**: 3 (2 backlog, 1 surrogate)


**References**: [reqts](diff-viewer.reqts.jsonl)

> **JSONL is authoritative.** This document is auto-generated from `EPIC-77znwqj94w`'s `.workEpic.jsonl`. Do not hand-edit; changes will be overwritten on the next write.



## Work Units


### BACKLOG (2)


#### Diff viewer Dioxus component
<a id="work-6ds59btb76"></a>
`WORK-6ds59btb76` · state `backlog` · maturity `DREAMED`


**Problem**: Renders line diff information as a Dioxus component with the behaviors of src/index.tsx: split and inline views, line numbers, folding and expansion, fold reset, highlighted lines, line-number click, titles, and render callbacks for content and fold messages.

**Plan**: Settle the Dioxus equivalents for resetCodeBlocks and the render-function props during planning, before this chunk is work-ready. Risk signals: implementation-complexity; architectural-uncertainty until those equivalents are settled.

**Depends on**: [Line diff engine on similar](#work-p7e70a5mwf) *(surrogate → 20260915.line-diff-engine.workUnit.jsonl)*

**Blocks**: [Diff viewer theming](#work-b86cyjcn7m)



#### Diff viewer theming
<a id="work-b86cyjcn7m"></a>
`WORK-b86cyjcn7m` · state `backlog` · maturity `DREAMED`


**Problem**: The viewer ships the light and dark palettes from src/styles.ts, and consumers can override them. The reference overrides through emotion style objects, which have no Rust counterpart.

**Plan**: Settle the Rust style-override design during planning, before this chunk is work-ready. Risk signals: architectural-uncertainty until that design is settled.

**Depends on**: [Diff viewer Dioxus component](#work-6ds59btb76)





### Surrogates (1)


#### Line diff engine on similar
<a id="work-p7e70a5mwf"></a>
`WORK-p7e70a5mwf` · **surrogate** · original `WORK-p7e70a5mwf`

**Authoritative record lives elsewhere.** See `20260915.line-diff-engine.workUnit.jsonl`.

*Spun out to a standalone work unit file.*

**Blocks**: [Diff viewer Dioxus component](#work-6ds59btb76)






## Dependency Graph

- [Line diff engine on similar](#work-p7e70a5mwf) *(surrogate → 20260915.line-diff-engine.workUnit.jsonl)* → [Diff viewer Dioxus component](#work-6ds59btb76) — *component renders the engine's line diff information*
- [Diff viewer Dioxus component](#work-6ds59btb76) → [Diff viewer theming](#work-b86cyjcn7m) — *theming styles the component's markup*


