# Progress

## 2026-04-08 Session
- 已完成 Task Contract / Control-Plane Visibility V1 主线收口。
- 已确认 execution_identity 在 task detail / runs / status 三面稳定对外可见。
- 已确认 running cancel 的正式终态语义稳定：
  - status=cancelled
  - error_kind=runner_cancelled
  - failure_scope=runner_cancelled
- 已完成 contract 主文档同步：
  - docs/api-ops.md
  - docs/lightpanda-api-task-structure.md
  - docs/control-plane-and-visibility-mainline.md
- 已完成 integration_api contract 测试钉住：
  - status_detail_and_runs_share_execution_identity_contract
  - cancelled_contract_is_visible_across_status_detail_and_runs
- 已完成远程 create -> inspect -> cancel -> inspect 验收闭环。
- 已在远程 Ubuntu 上完成真实 timeout / cancel 样本验收，并确认 /tasks/:id、/tasks/:id/runs、/status 三面字段与 artifact 语义一致。
- 已复跑真实执行质量相关测试并全部通过：
- 已补 execution_stage 阶段化失败证据，并确认 timeout / process exit / cancelled 在 runner result 中可稳定落出最小阶段语义：
  - timeout -> execution_stage=navigate
  - process exit -> execution_stage=action
  - cancelled -> execution_stage=action
- 已完成 explainability 三面投影收口，确认 task detail / runs / status 的 browser failure summary 统一暴露 execution_stage。
- 已新增 proxy health 边界回归测试，确认 runner_cancelled 不会误伤 trust feedback；当前执行后健康回写边界稳定为 success 加分、failed/timeout 扣分、cancelled 不处罚。
  - lightpanda_runner_timeout_marks_timed_out_and_cleans_state
  - lightpanda_runner_non_zero_exit_marks_failed
  - task_and_run_views_expose_browser_failure_signal_fields
  - status_latest_execution_summaries_include_browser_failure_artifact
  - proxy_health_is_updated_after_success_and_timeout
- 已尝试补 browser execution 真实失败样本，但当前环境下 DNS / TLS / 非标准端口目标仍统一收敛到 runner_timeout，说明下一步更适合补可稳定复现 browser_failure_signal 的样本，而不是继续盲打目标站点。
- 已将当前阶段状态从“contract 收口中”推进为“contract 主线已完成，进入下一阶段”。

## Current Focus
- 回到真实 Lightpanda 执行稳定化主线。
- 补可稳定触发 browser_failure_signal 的真实或半真实样本。
- 推进 verify / trust score 从选前判断扩展到执行闭环。

## Next Step
1. 先补可稳定复现 browser_failure_signal 的 browser execution 样本，优先避免继续用只会收敛成 timeout 的目标站点。
2. 再推进 verify / trust score 执行前 / 执行中 / 执行后闭环，并评估是否需要对“有明确阶段证据的失败”做更细粒度负反馈。
3. 再补一组 succeeded / timeout staged / browser execution / cancelled 的长期回归样本集，作为后续真实验收基线。
