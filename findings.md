# Findings

## Current mainline
- 当前 `lightpanda-automation` 最值主线已经从“继续扩 browser endpoint”切到“把 browser-facing contract、explainability、status 展示语义与排序规则收成稳定产品面”。
- 这条线现在已经不是底层拼装，而是进入“展示层产品化”的后半段。

## Browser-facing contract
- fake runner 已经补齐关键 browser-facing 字段，能承担开发态基线角色：
  - `requested_action`
  - `action`
  - `supported_actions`
  - `title`
  - `final_url`
  - `html_preview/html_length/html_truncated`
  - `text_preview/text_length/text_truncated`
  - `content_preview/content_length/content_truncated`
  - `content_kind`
  - `content_source_action`
  - `content_ready`
- `TaskResponse` 与 `RunResponse` 也已外露 browser-facing 字段，使 task/run 视图与 runner 结果更一致。

## Explainability
- run-level `fingerprint_runtime_explain.consumption_explain` 丢失问题已修复。
- `src/api/explainability.rs` 里新增了 `browser result summary`，浏览器结果开始进入 summary artifact 体系，不再只是散落字段。
- explainability 现在已能同时覆盖：
  - selection decision
  - identity/network summary
  - proxy growth assessment
  - browser result summary

## Status semantics
- `/status` 曾出现语义漂移风险：`latest_tasks` 一度被拿来承载“最近 browser-ready 任务”，名字和真实含义不一致。
- 该问题已通过拆分字段解决：
  - `latest_tasks`：保留通用最近任务视图
  - `latest_browser_tasks`：单独承载 browser-ready 展示视图
- 这一拆分显著降低了后续状态页扩展时的语义冲突风险。

## Ordering rules
- `latest_browser_tasks` 现在不是简单过滤，而是有明确排序策略：
  1. `content_ready=true` 优先
  2. 可读性更强（`title` / `content_preview` 更完整）优先
  3. 更新更近优先
- 该排序规则已通过 unit + integration coverage 锁住，包括混合场景：
  - content-ready vs readable-title
  - readable-title vs only-final-url

## Commits landed on this line
- `f29844f` — `feat: harden browser contract and explainability`
- `d6eab84` — `feat: split latest browser tasks in status`
- `148a520` — `feat: prioritize browser-ready status results`
- `3b5e79f` — `test: harden browser status ordering rules`

## Current product judgment
- 这条 browser/status 展示主线已经接近稳定完成，不应再在低收益的细枝末节上过度打磨。
- 当前更值的后续方向有两个：
  1. 把 `latest_browser_tasks` 投影成更轻、更像产品结果卡片的 shape
  2. 进入下一条更有价值的新主线，而不是继续停留在排序细节上
