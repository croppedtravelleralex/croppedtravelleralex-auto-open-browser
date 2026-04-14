## 2026-04-14 AI Maintenance Snapshot

- Stage-closeout baseline: **93%**
- Browser mainline: `lightpanda serve + CDP`
- Control plane: `127.0.0.1:3000`
- Repo-owned gateway: `127.0.0.1:8787`
- Live Lightpanda binary: `/usr/local/bin/lightpanda`
- Verification branches: `no-token` baseline plus gated `real-upstream`

## Current Facts

- The stable stage baseline is the repo-owned `serve + CDP` path, not any older fetch-based path.
- `demo_public` and `prod_live` reporting are separated.
- `stable_v1` is the current repo-owned `prod_live` preset for repeated verification.
- Strict release gating remains intentionally harder than operational reporting.
- Current operational reporting may resolve to `provider_capped` when the only failure is source concentration under lab-only supply.

## Current Limits

- No private or paid provider is connected yet.
- The dirty branch still has unresolved continuity control-plane Rust follow-up before it can reclaim a fully green test closure.
- The next safe closure step is top-level documentation and stage-entry consistency, followed by the continuity control-plane Rust refactor.

