# Progress

## 2026-04-05 Session
- Reconfirmed the current mainline is browser-facing contract hardening, not more endpoint sprawl.
- Upgraded fake runner so it now exposes browser-facing fields aligned much more closely with lightpanda runner output.
- Added and passed targeted fake-runner tests for:
  - extract_text content contract
  - title/final_url metadata exposure
  - timeout status behavior
- Extended `TaskResponse` and `RunResponse` with outward browser-facing fields:
  - `title`
  - `final_url`
  - `content_preview`
  - `content_length`
  - `content_truncated`
  - `content_kind`
  - `content_source_action`
  - `content_ready`
- Wired handlers to lift those outward fields from `result_json` into task/run/status responses.
- Found and fixed the run-level `fingerprint_runtime_explain.consumption_explain` gap.
- Added a fallback builder in `runner/engine.rs` so runtime consumption explainability can still be produced from fingerprint profile data when payload-side runtime details are incomplete.
- Recovered the previously failing integration test `task_runs_expose_run_level_trace_metadata_and_standardized_artifacts`.
- Reframed the integration test to use a browser-facing task shape that better matches the new outward contract assertions.
- Rewrote planning files so they now reflect the actual lightpanda-automation mainline again.

## Current Focus
- Finish locking task/run outward fields with tighter integration assertions.
- Run the small targeted regression pack for fake runner + browser task + run trace.
- Land a clean baseline commit for this contract/explainability round.
