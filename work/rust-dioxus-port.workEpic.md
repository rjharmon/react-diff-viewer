# react-diff-viewer port to Rust and Dioxus

**Motivation**: Deliver the port MISSION.md describes: react-diff-viewer as a Dioxus component in the rust-dioxus/ crate, diffing with the similar crate.

**Scope**: Includes the line diff engine, the diff viewer component, and theming, one work chunk each. Excludes changes to the TypeScript package and porting the examples/ demo app.

**Success criteria**: Every mission outcome in MISSION.md is met by a delivered work chunk.

**Status**: `draft` · **Contents**: 3 (1 backlog, 2 surrogate)


**References**: [reqts](diff-viewer.reqts.jsonl)

> **JSONL is authoritative.** This document is auto-generated from `EPIC-77znwqj94w`'s `.workEpic.jsonl`. Do not hand-edit; changes will be overwritten on the next write.



## Work Units


### BACKLOG (1)


#### Diff viewer theming
<a id="work-b86cyjcn7m"></a>
`WORK-b86cyjcn7m` · state `backlog` · maturity `DREAMED`


**Problem**: The viewer ships the light and dark palettes from src/styles.ts, and consumers can override them. The reference overrides through emotion style objects, which have no Rust counterpart.

**Plan**: Settle the Rust style-override design during planning, before this chunk is work-ready. Risk signals: architectural-uncertainty until that design is settled.

**Depends on**: [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)*





### Surrogates (2)


#### Diff viewer Dioxus component
<a id="work-6ds59btb76"></a>
`WORK-6ds59btb76` · **surrogate** · original `WORK-6ds59btb76`

**Authoritative record lives elsewhere.** See `20260915.diff-viewer-component.workUnit.jsonl`.

*Spun out to a standalone work unit file.*

Risk signals: implementation-complexity; architectural-uncertainty. Kept as one work unit by stakeholder decision.

**Depends on**: [Line diff engine on similar](#work-p7e70a5mwf) *(surrogate → 20260915.line-diff-engine.workUnit.jsonl)*

**Blocks**: [Diff viewer theming](#work-b86cyjcn7m)



#### Line diff engine on similar
<a id="work-p7e70a5mwf"></a>
`WORK-p7e70a5mwf` · **surrogate** · original `WORK-p7e70a5mwf`

**Authoritative record lives elsewhere.** See `20260915.line-diff-engine.workUnit.jsonl`.

*Spun out to a standalone work unit file.*

Risk signals: implementation-complexity.

**Blocks**: [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)*






## Dependency Graph

- [Line diff engine on similar](#work-p7e70a5mwf) *(surrogate → 20260915.line-diff-engine.workUnit.jsonl)* → [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)* — *component renders the engine's line diff information*
- [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)* → [Diff viewer theming](#work-b86cyjcn7m) — *theming styles the component's markup*


