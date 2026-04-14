## 2026-04-14 Status Snapshot

- Stage-closeout baseline: **93%**
- Current mainline: `lightpanda serve + CDP` + repo-owned control plane + repo-owned gateway + dual-mode proxy reporting
- Control plane: `127.0.0.1:3000`
- Repo-owned gateway: `127.0.0.1:8787`
- Live Lightpanda binary: `/usr/local/bin/lightpanda`
- Gateway acceptance branches: `no-token` safe baseline and gated `real-upstream`

## Verified Baseline Facts

- `demo_public` and `prod_live` are reported separately.
- `stable_v1` is the current repo-owned `prod_live` preset under validation.
- Short `prod_live stable_v1` verification already proved:
  - `browser_success_rate_percent = 100`
  - `browser_proxy_not_found_failures = 0`
  - `recent_hot_regions >= 3`
  - `stateful_continuity_observed = true`
- Strict `95%+` release verdict can still fail while operational verdict becomes `provider_capped` when lab-only supply is too concentrated.

## Release And Verification Entry Points

- `bash scripts/preflight_release_env.sh`
- `bash scripts/release_baseline_verify.sh --with-upstream`
- `bash scripts/release_fast_verify.sh`
- `bash scripts/gateway_verify.sh no-token`
- `bash scripts/gateway_verify.sh real-upstream`

## Current Branch Risk

- The top-level runtime baseline is aligned.
- The current dirty branch is not yet fully closed because continuity control-plane Rust follow-up still breaks `cargo test --tests`.
- That unresolved work is separate from the already-proven repo-owned runtime alignment.

