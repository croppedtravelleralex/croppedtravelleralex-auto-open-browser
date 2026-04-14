# lightpanda-automation

## Current Stage Snapshot

- Stage-closeout baseline: **93%**
- Browser mainline: `lightpanda serve + CDP`
- Control plane: `127.0.0.1:3000`
- Repo-owned gateway: `127.0.0.1:8787`
- Live Lightpanda binary: `/usr/local/bin/lightpanda`
- Verification entrypoints:
  - `bash scripts/preflight_release_env.sh`
  - `bash scripts/release_baseline_verify.sh --with-upstream`
  - `bash scripts/release_fast_verify.sh`
  - `bash scripts/gateway_verify.sh no-token`
  - `bash scripts/gateway_verify.sh real-upstream`

## Current Operating Truth

- `demo_public` and `prod_live` are now intentionally separated.
- `stable_v1` is the repo-owned `prod_live` preset under repeated validation.
- Without private or paid providers, strict `95%+` release gating can still fail on source concentration while the operational conclusion is `provider_capped`.
- The current dirty branch still contains unresolved continuity control-plane Rust follow-up, so these top-level docs describe the stable stage baseline instead of every in-flight refactor in the worktree.

## Current Acceptance Shape

- Short `prod_live stable_v1` verification already proved:
  - `browser_success_rate_percent = 100`
  - `browser_proxy_not_found_failures = 0`
  - `recent_hot_regions >= 3`
  - `stateful_continuity_observed = true`
- The remaining strict gap is supply structure, especially `source_concentration_top1_percent`.

## Fast Verify

1. `bash scripts/preflight_release_env.sh`
2. `bash scripts/release_baseline_verify.sh --with-upstream`
3. `bash scripts/release_fast_verify.sh`
4. `bash scripts/gateway_verify.sh no-token`
5. `bash scripts/gateway_verify.sh real-upstream`

