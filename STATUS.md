# STATUS.md

## 当前状态摘要

- **状态：** 已进入 **Task Contract / Control-Plane Visibility V1 收口阶段**
- **日期：** 2026-04-07
- **当前焦点：** 把已经完成的 ExecutionIdentity V1 + cancelled API 闭环 从代码事实升级成 文档契约 + 接口一致性测试 + 远程验收口径

## 本文件用途

STATUS.md 只保留：
- 当前状态
- 当前风险
- 当前下一步
- 本轮体检

更完整的进展说明请看：
- PROGRESS.md — 已实现能力时间线
- ROADMAP.md — 过去 / 现在 / 未来路线图
- EXECUTION_LOG.md — 每轮执行记录
- RUN_STATE.json — 调度状态

## 当前状态

当前系统已经具备以下主线能力：

1. 执行与调度控制面
   - DB-first queue
   - claim / reclaim / retry / cancel
   - 多 worker 并发执行
   - health / status / logs / runs 基础观测

2. Fingerprint 控制面
   - profile 创建 / 查询 / 绑定 / 校验
   - fake / lightpanda 统一 profile 视图
   - task/status 详情暴露 fingerprint resolution status

3. Proxy / verify / trust score 基础质量链
   - proxy CRUD
   - provider / region / min_score / cooldown 过滤
   - sticky session 正式绑定表 proxy_session_bindings
   - smoke / verify / verify-batch / verify batch 查询
   - verify 结果、执行结果反哺 proxy score
   - provider / provider×region 风险快照
   - cached trust score 持久化、scan / repair / maintenance

4. Explain / status / result 基础控制面
   - task / status / explain 接口统一暴露 selection_reason_summary
   - selection_explain 结构化输出
   - winner_vs_runner_up_diff 结构化输出
   - candidate_rank_preview 强类型化
   - trust_score_components 强类型化
   - summary_artifacts schema 标准化（source/category/severity）
   - run 级 result_json 持久化与 run_id / attempt / timestamp 溯源字段
   - /proxies/:id/explain 暴露 trust_score_cached_at / explain_generated_at / explain_source
   - explainability assembler 已从 handlers 中抽离到独立模块

5. ExecutionIdentity V1 已闭环
   - execution_identity 已在 task detail / runs / status 三个主要消费面统一对外可见
   - proxy / fingerprint / selection / trust score 不再需要由下游手工拼装统一身份视图
   - identity_network_explain 已围绕统一执行身份结构对外输出

6. running cancel 已完成正式终态化
   - running task 取消后写回 cancelled 正式终态
   - error_kind=runner_cancelled
   - failure_scope=runner_cancelled
   - detail / runs / status 可围绕同一 cancelled 语义消费结果

7. 当前阶段主线已切换到 contract 收口
   - 这一轮不再继续扩新功能
   - 当前任务是把已完成的 identity / cancelled 成果升级成稳定的对外 contract
   - 主线目标是让文档、接口、测试、远程验收四者给出同一口径

8. 测试与稳定性
   - 单测 + 集成测试持续覆盖执行、代理、verify、trust score、explainability 主链
   - integration_lightpanda_runner 已覆盖 running cancel 闭环样本
   - 当前新增重点是 API contract 一致性验证

## 当前风险

1. 文档仍可能落后于接口事实。
   如果 docs/api-ops.md、docs/lightpanda-api-task-structure.md、docs/control-plane-and-visibility-mainline.md 不同步，后续控制面消费口径会继续分裂。

2. status / detail / runs 虽已接线，但还缺正式 contract 测试钉子。
   如果没有最小一致性测试，后续改动容易把统一 identity 或 cancelled 语义悄悄打散。

3. cancelled 语义刚完成闭环，仍需继续防止退回 generic failure。
   如果后续实现把 cancelled 与 failed / timed_out 混写，运营面会失去正式终态区分。

4. 真实 Lightpanda 仍未完成 beyond-stub 的更深验证。
   当前 contract 主线已经成立，但真实浏览器执行稳定性主线还会在下一阶段继续推进。

## 当前下一步

### P0
1. 对接主文档，明确 execution_identity、cancelled、/status vs detail/runs 的稳定口径。
2. 补 API contract 一致性测试，验证同一任务在 /status、detail、runs 中的 identity / failure 语义一致。
3. 补 cancelled 契约验证，用测试固定 runner_cancelled 的正式终态表达。
4. 完成远程验收，确认文档里的 create / inspect / cancel 流程与接口事实一致。

### P1
5. 继续补更多真实任务流样本与失败分类。
6. 继续把 status / explain / result 推向更高层运营级控制面。
7. 在 contract 收口完成后，再继续推进真实 Lightpanda 执行稳定化第二轮。

## 本轮体检（2026-04-07）

- 找 bug：本轮核心不是继续扩能力，而是避免已完成的 identity / cancelled 成果只存在于代码里。
- 阶段判断：当前最重要的是把 ExecutionIdentity V1 与 cancelled 正式终态写成稳定 contract，而不是马上开启新一轮功能膨胀。
- 改进建议：下一步最值钱的是文档收口、最小一致性测试、远程验收三件事一起完成。

## Autopilot Sync

- 当前文档已对齐到 2026-04-07 Task Contract / Control-Plane Visibility V1 收口阶段。
- 阶段冻结边界：本轮默认不扩新能力；只收口 identity / cancelled / status-detail-runs 一致性。
