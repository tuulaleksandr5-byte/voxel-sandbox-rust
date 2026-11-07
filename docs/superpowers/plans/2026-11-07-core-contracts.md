# Core Contracts Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement task-by-task.

**Goal:** Implement Issue #1 and enable independent subsystem work.
**Architecture:** Bevy-independent game_core value types and small traits.
**Tech Stack:** Rust edition 2024; existing Bevy =0.20.1 client; no new dependencies.
**Spec:** ../specs/2026-11-07-core-contracts-design.md

## Global Constraints
- Y=-64..320 inclusive; sections 16x16x16; horizontal chunks 16x16.
- No world generation, graphics, mobs, mechanical networks or other gameplay.
- Work on feature/core-contracts; create PR into main; do not merge.

## Review Focus
- Negative multiples and i32 extremes must reconstruct without wrapping.
- Partial top section must never expose or mutate Y>320.
- Unloaded world data must not silently become air.
- Stable IDs must survive raw round trips, independent of ECS/registry order.
- Independent consumers must not need Bevy or concrete future subsystems.

## Task 1 — Coordinates and section data (coordinate agent)
Files: src/coords.rs, src/section.rs, tests/coordinates.rs, tests/sections.rs.
- [x] Write boundary, index, reconstruction and partial-section tests.
- [x] Run tests and record missing API failures.
- [x] Implement BlockPos/ChunkPos/SectionPos/LocalBlockPos and Section.
- [x] Run tests; retain original scalar helpers as compatible reexports.

## Task 2 — IDs and subsystem contracts (primary agent)
Files: src/ids.rs, inventory.rs, simulation.rs, world.rs, render.rs, player.rs,
persistence.rs, lib.rs; tests/identifiers.rs, inventory.rs, contracts.rs.
- [x] Write stable-ID roundtrip and zero-stack rejection tests; observe RED.
- [x] Implement IDs and invariant-bearing values; verify GREEN.
- [x] Write independent consumer integration tests; observe missing trait RED.
- [x] Implement traits and extensible command/event payloads; verify GREEN.

## Task 3 — Documentation, verification, review and PR
Files: docs/CORE_CONTRACTS.md, ARCHITECTURE.md, PARALLEL_WORK.md, README.md.
- [x] Document signatures, authority, persistence limits and parallel boundaries.
- [x] Run fmt, check, clippy all-targets -D warnings, test on workspace.
- [x] Obtain independent review; address substantive findings with regression tests.
- [ ] Commit and push only feature/core-contracts, open PR into main.
- [ ] Inspect GitHub CI when API access is available; report blockers accurately.

## Execution record
Baseline: cargo test --workspace passed, 3 tests. Existing cloud checkout is
isolated; branch feature/core-contracts created from cffba68 (remote main).
Issue #1 supplied verbatim by user because API network access was blocked.

Current validation: all four requested workspace commands passed (exit 0),
19 runtime tests and 2 compile-fail doctests. No new dependencies; main unchanged.
Independent code review: no Critical/Important findings. Plan status corrected
before delivery; no deferred code findings. GitHub API blocked by enforced network policy;
api.github.com added to configuration draft but not applied to runtime yet.
