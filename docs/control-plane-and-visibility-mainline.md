# Control-Plane and Visibility Mainline (2026-04-02)

## Goal

After closing the refresh-scope mainline, shift project momentum from refresh expansion to **control-plane clarity** and **visibility quality**.

## Mainline intent

This mainline is not about adding more refresh semantics.
It is about making the current project state easier to operate, easier to reason about, and harder to accidentally reopen in the wrong places.

## What this mainline should do

1. **clarify current stage ownership**
   - what is completed
   - what is stable enough to freeze
   - what remains intentionally deferred

2. **improve operator visibility quality**
   - keep current explain / trust / phase outputs understandable
   - make stage boundaries explicit in docs and status surfaces

3. **prevent accidental re-expansion**
   - stop refresh-scope work from quietly expanding back into providerRegion / selection redesign / broad trust semantics changes

## Deliverables

- a current-stage control summary
- a deferred-work freeze list
- a reopen-condition list for future stages

## Recommendation

> Treat the next mainline as a **project-control mainline**, not a hidden continuation of refresh-scope implementation.
