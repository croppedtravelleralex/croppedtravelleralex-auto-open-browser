# Findings

## Current mainline
- 当前最值主线已经从“继续扩 browser endpoint”切到“把已有 browser-facing result contract、task/run outward response、fingerprint runtime explainability 收成稳定基线”。
- fake runner 与 lightpanda runner 的契约差距，已经被显著缩小；这对开发态验证和 API 一致性很重要。

## Browser-facing contract
- fake runner 现在已经补齐关键字段：
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
- 这意味着 fake runner 不再只是“能跑”，而是已经能承担 browser-facing contract 的开发态基线角色。

## Task/Run outward response
- `TaskResponse` 与 `RunResponse` 已新增 browser-facing outward fields：
  - `title`
  - `final_url`
  - `content_preview`
  - `content_length`
  - `content_truncated`
  - `content_kind`
  - `content_source_action`
  - `content_ready`
- handlers 现在会从 `result_json` 中回填这些字段到 `/tasks/:id`、`/tasks/:id/runs` 与 `status.latest_tasks`。

## Explainability
- run-level `fingerprint_runtime_explain.consumption_explain` 之前存在丢失问题。
- 已在 `src/runner/engine.rs` 中补上兜底逻辑：当 payload 中没有完整 `fingerprint_runtime.consumption_explain` 时，会基于 fingerprint profile 自动生成 consumption explain。
- 该修复已让原本失败的 `task_runs_expose_run_level_trace_metadata_and_standardized_artifacts` 恢复通过。

## Testing decisions
- `get_title` 比 `open_page` 更适合作为 outward contract 的 run/task 集成测试样例：
  - 可以稳定断言 `title`
  - 也能稳定断言 `final_url`
  - 同时避免把 `content_kind` 强行写成不稳定预期
- 当前测试方向应优先锁定“task 与 run outward fields 一致”而不是继续堆更多 endpoint 创建测试。

## Cleanup direction
- 当前 `task_plan.md / findings.md / progress.md` 之前被 OpenHands/Aider 编排内容串线污染，已经不适合作为 lightpanda-automation 本线控制面继续使用。
- 本轮需要把 planning 文件正式切回 lightpanda-automation 当前真实主线，并围绕 browser contract / explainability / commit baseline 继续推进。
