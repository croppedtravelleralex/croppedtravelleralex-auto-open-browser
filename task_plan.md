# Task Plan

## Goal
把当前 lightpanda-automation 这一轮 browser-facing contract / explainability / runner 对齐改动收成一条干净主线，并建立可回归的测试与提交基线。

## Current Status
- Phase 1 — fake runner browser contract alignment: complete
- Phase 2 — run/task content field exposure: in_progress
- Phase 3 — fingerprint runtime explainability backfill: complete
- Phase 4 — planning cleanup + commit baseline: pending

## Phases

### Phase 1 — Fake runner contract alignment
- [x] 对齐 `requested_action` / `action` / `supported_actions`
- [x] 对齐 `title` / `final_url`
- [x] 对齐 `content_preview` / `content_length` / `content_kind`
- [x] 补充 fake runner 定向测试

### Phase 2 — Task/Run API outward contract
- [x] 给 `TaskResponse` / `RunResponse` 增加 browser-facing content fields
- [x] 在 handlers 中从 `result_json` 回填字段
- [ ] 补更系统的 integration consistency assertions
- [ ] 再跑一轮定向回归

### Phase 3 — Fingerprint runtime explainability
- [x] 修复 run-level `consumption_explain` 丢失
- [x] 为 `engine` 增加基于 fingerprint profile 的兜底 explainability 生成
- [ ] 补一条更贴近 browser-facing task 的 explainability 回归

### Phase 4 — Cleanup and baseline commit
- [ ] 清理串线的 planning 文件
- [ ] 更新 findings / progress 为当前真实主线
- [ ] 跑通过定向测试集合
- [ ] 提交一轮稳定 commit

## Key Decisions
- 当前最值主线不是继续盲扩 endpoint，而是把已有 browser contract、task/run outward response、fingerprint runtime explainability 收成稳定基线。
- `get_title` 更适合作为 run/task outward contract 测试样例，因为能稳定暴露 `title/final_url`，同时不引入 `content_kind` 的额外变量。
- planning 文件必须回到 lightpanda-automation 本线，不能继续保留 OpenHands/Aider 串线内容。

## Risks
- run/task outward fields 增加后，旧测试和辅助构造器容易继续漏字段。
- content field 的外露能力已进入 API 契约层，后续再改字段名会影响面更大。
- 当前工作树改动较多，若不及时收口 commit，后续继续推进会提高返工风险。
