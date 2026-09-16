# react-diff-viewer port to Rust and Dioxus

**Motivation**: Deliver the port MISSION.md describes: react-diff-viewer as a Dioxus component in the rust-dioxus/ crate, diffing with the similar crate.

**Scope**: Includes the line diff engine, the diff viewer component, theming, and programmatic control of the viewer beyond the reference's surface, one work chunk each. Excludes changes to the TypeScript package and porting the examples/ demo app.

**Success criteria**: Every mission outcome in MISSION.md is met by a delivered work chunk.

**Status**: `draft` · **Contents**: 4 (1 backlog, 3 surrogate)


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





### Surrogates (3)


#### Diff viewer Dioxus component
<a id="work-6ds59btb76"></a>
`WORK-6ds59btb76` · **surrogate** · original `WORK-6ds59btb76`

**Authoritative record lives elsewhere.** See `20260915.diff-viewer-component.workUnit.jsonl`.

*Spun out to a standalone work unit file.*

Risk signals: implementation-complexity; architectural-uncertainty. Kept as one work unit by stakeholder decision.

**Depends on**: [Line diff engine on similar](#work-p7e70a5mwf) *(surrogate → 20260915.line-diff-engine.workUnit.jsonl)*

**Blocks**: [Diff viewer theming](#work-b86cyjcn7m), [Programmatic control of the diff viewer](#work-98bxwcd28f) *(surrogate → 20260916.diff-viewer-programmatic-control.workUnit.jsonl)*



#### Programmatic control of the diff viewer
<a id="work-98bxwcd28f"></a>
`WORK-98bxwcd28f` · **surrogate** · original `WORK-98bxwcd28f`

**Authoritative record lives elsewhere.** See `20260916.diff-viewer-programmatic-control.workUnit.jsonl`.

*Spun out to a standalone work unit file.*

Risk signals: architectural-uncertainty.

**Depends on**: [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)*



#### Line diff engine on similar
<a id="work-p7e70a5mwf"></a>
`WORK-p7e70a5mwf` · **surrogate** · original `WORK-p7e70a5mwf`

**Authoritative record lives elsewhere.** See `20260915.line-diff-engine.workUnit.jsonl`.

*Spun out to a standalone work unit file.*

Risk signals: implementation-complexity.

**Blocks**: [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)*






## Dependency Graph

- [Line diff engine on similar](#work-p7e70a5mwf) *(surrogate → 20260915.line-diff-engine.workUnit.jsonl)* → [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)* — *component renders the engine's line diff information*
- [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)* → [Programmatic control of the diff viewer](#work-98bxwcd28f) *(surrogate → 20260916.diff-viewer-programmatic-control.workUnit.jsonl)* — *control widens the component's fold reset trigger*
- [Diff viewer Dioxus component](#work-6ds59btb76) *(surrogate → 20260915.diff-viewer-component.workUnit.jsonl)* → [Diff viewer theming](#work-b86cyjcn7m) — *theming styles the component's markup*


