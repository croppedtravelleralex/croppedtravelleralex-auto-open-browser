# Control-Plane and Visibility Mainline (2026-04-07)

## Goal

After closing the execution-identity wiring and running-cancel closure work, shift this stage from code-only completion to contract-level stability across docs, API views, and tests.

## Mainline intent

This mainline is not about adding more new execution features.
It is about making the current project state easier to operate, easier to explain, and safer to consume as a stable control plane.

## What this mainline should do

1. formalize the outward contract
   - define what /status, task detail, and runs each mean
   - define which fields are now stable enough for operators and downstream readers

2. stabilize execution identity visibility
   - keep execution_identity as the unified outward identity snapshot
   - stop downstream readers from reconstructing identity from fragmented fields

3. formalize cancelled terminal semantics
   - keep cancelled as an explicit terminal state
   - keep runner_cancelled separate from generic execution failure and timeout

4. pin the contract with tests
   - verify status/detail/runs consistency for the same task
   - verify the cancelled contract in the same outward vocabulary used by docs

## Stable field vocabulary for this stage

Current stable control-plane vocabulary:
- execution_identity
- failure_scope
- browser_failure_signal
- summary_artifacts
- selection_reason_summary
- selection_explain
- fingerprint_runtime_explain
- identity_network_explain
- browser result summary fields

Current view split:
- /status = top-level system snapshot
- /tasks/:id = canonical task snapshot
- /tasks/:id/runs = per-attempt drill-down snapshot

## Deliverables

- documentation aligned to current interface facts
- integration tests that pin the shared contract
- remote validation flow that demonstrates create -> inspect -> cancel -> inspect

## Recommendation

> Treat the current mainline as Task Contract / Control-Plane Visibility V1.
> Do not expand feature scope until docs, interface outputs, and tests describe the same reality.
