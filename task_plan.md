# Task Plan

## Goal
把 lightpanda-automation 当前这轮 browser/status 展示主线收成稳定、清晰、可持续演进的一段产品化基线，而不是继续停留在字段拼装阶段。

## Current Status
- Phase 1 — browser-facing contract hardening: complete
- Phase 2 — explainability + browser result summary: complete
- Phase 3 — status semantics split (`latest_tasks` / `latest_browser_tasks`): complete
- Phase 4 — browser-status ordering rules + regression protection: complete
- Phase 5 — docs sync + next-step productization: in_progress

## Phases

### Phase 1 — Browser-facing contract
- [x] 对齐 fake runner 与 lightpanda runner 的 browser-facing result contract
- [x] 给 task/run outward response 增加 `title` / `final_url` / `content_*`
- [x] 建立 task/run outward contract integration coverage

### Phase 2 — Explainability
- [x] 修复 run-level `consumption_explain` 丢失
- [x] 增加 `browser result summary` 作为 explainability artifact
- [x] 让 browser-facing 字段开始进入可读摘要层

### Phase 3 — Status semantics cleanup
- [x] 识别 `latest_tasks` 语义漂移风险
- [x] 将 `/status` 拆分为 `latest_tasks` 与 `latest_browser_tasks`
- [x] 保留通用最近任务视图，同时单独提供 browser-ready 视图

### Phase 4 — Ordering rules and tests
- [x] 为 `latest_browser_tasks` 增加排序规则
- [x] 优先 `content_ready=true`
- [x] 其后按可读性（`title` / `content_preview`）排序
- [x] 再按新鲜度排序
- [x] 用 integration / unit tests 锁住混合场景排序行为

### Phase 5 — Productization next steps
- [x] 同步 `progress.md`
- [x] 同步 `task_plan.md`
- [ ] 同步 `findings.md`
- [ ] 判断是否把 `latest_browser_tasks` 投影为更轻的 browser-summary shape
- [ ] 评估 status 面板是否还需要更强的人话层展示

## Key Decisions
- 当前主线不再是继续扩 endpoint，而是把 browser/status 展示体验收成一个稳定产品面。
- `latest_tasks` 与 `latest_browser_tasks` 必须分开，不能再混用一个字段承载两种语义。
- `latest_browser_tasks` 的排序应优先展示“更值得先看”的结果，而不是机械按时间排序。
- 在当前阶段，继续补测试和语义收口的价值，高于继续堆新字段。

## Risks
- `latest_browser_tasks` 未来若继续直接返回完整 `TaskResponse`，可能会把状态面板噪音带高。
- 排序规则目前已经合理，但如果后续引入更多 browser result 形态，可能需要重新平衡可读性和新鲜度权重。
- 如果不及时把 findings 一并追平，文档面仍会出现一部分滞后。
