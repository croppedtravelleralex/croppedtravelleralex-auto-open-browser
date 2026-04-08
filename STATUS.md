# STATUS.md

## 当前状态摘要

- **状态：** Task Contract / Control-Plane Visibility V1 已完成收口
- **日期：** 2026-04-08
- **当前焦点：** 从 contract 收口切到下一阶段：真实 Lightpanda 执行稳定化 + explainability 质量继续清理 + verify/trust 执行闭环深化

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

7. Task Contract / Control-Plane Visibility V1 已完成
   - 文档已对齐 execution_identity / cancelled / status-detail-runs 职责边界
   - integration_api 已补三面一致性与 cancelled contract 测试
   - 远程 create -> inspect -> cancel -> inspect 验收已跑通
   - 当前主线不再是 contract 收口，而是进入下一轮真实执行稳定化

## 当前风险

1. beyond-stub 的真实 Lightpanda 深层验证仍不足。
   当前 contract 已稳定，但真实浏览器执行成功率、失败分类与真实引擎行为仍需继续压实。

2. explainability / artifact 文案质量仍有继续清理空间。
   当前结构已经稳定，但部分字段在 detail / runs / status 的细粒度表现还可以继续打磨。

3. verify / trust score 还没有完全扩展成执行前 / 执行中 / 执行后闭环。
   当前已有基础质量链，但长期运营级闭环还没完全收口。

## 当前下一步

### P0
1. 继续补真实浏览器任务流样本与失败分类。
2. 继续清 explainability summary / artifact 文案质量与跨视图细节一致性。
3. 推进 verify / trust score 从选前判断扩展到执行前 / 执行中 / 执行后闭环。

### P1
4. 继续治理高并发下的写放大、状态竞争、聚合成本。
5. 设计 artifact / log 的保留、清理与归档策略。
6. 在真实 Lightpanda 运行链上做更深一轮稳定性验证。

## 本轮体检（2026-04-08）

- 找 bug：本轮已确认当前主线不再是 contract 缺口，而是 contract 完成后的下一阶段推进。
- 阶段判断：Task Contract / Control-Plane Visibility V1 已具备“文档 + 接口 + 测试 + 远程验收”四证合一。
- 改进建议：下一步最值钱的是回到真实执行质量，而不是继续重复改 contract 文案。

## Autopilot Sync

- 当前文档已对齐到 2026-04-08：Task Contract / Control-Plane Visibility V1 已完成。
- 下一阶段默认主线：真实 Lightpanda 执行稳定化、explainability 质量继续清理、verify/trust 执行闭环深化。
