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
- Added `browser result summary` into explainability summary artifacts so browser-facing fields now surface as readable summaries instead of only raw fields.
- Split `/status` response semantics into both `latest_tasks` and `latest_browser_tasks`, instead of overloading one field with two meanings.
- Added ordering logic for `latest_browser_tasks` so it now prefers `content_ready=true`, then stronger readability (`title` / `content_preview`), then freshness.
- Hardened that browser-status ordering with integration coverage for mixed scenarios.
- Landed the status/browser display-line commits:
  - `f29844f` — `feat: harden browser contract and explainability`
  - `d6eab84` — `feat: split latest browser tasks in status`
  - `148a520` — `feat: prioritize browser-ready status results`
  - `3b5e79f` — `test: harden browser status ordering rules`

## Current Focus
- Sync planning/docs with the new status/browser display semantics.
- Decide whether `latest_browser_tasks` should later project to a lighter browser-summary shape.
- Treat the current status/browser line as largely stabilized and shift future effort toward higher-value product-facing next steps.
