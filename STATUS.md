# STATUS.md

## 当前状态摘要

- **状态：** 已进入 **trust-score explainability 主链收口阶段**
- **日期：** 2026-04-01
- **当前焦点：** 把 **selection explainability / trust cache / verify 回写链** 从“功能可用”推进到 **结构稳定、测试锁死、写放大收缩**。

## 本文件用途

`STATUS.md` 只保留：
- **当前状态**
- **当前风险**
- **当前下一步**
- **本轮体检**

更完整的进展说明请看：
- `PROGRESS.md` — 已实现能力时间线
- `ROADMAP.md` — 过去 / 现在 / 未来路线图
- `EXECUTION_LOG.md` — 每轮执行记录
- `RUN_STATE.json` — 调度状态

## 当前状态

当前系统已经具备以下主线能力：

1. **执行与调度控制面**
   - DB-first queue
   - claim / reclaim / retry / cancel
   - 多 worker 并发执行
   - health / status / logs / runs 基础观测

2. **Fingerprint 控制面**
   - profile 创建 / 查询 / 绑定 / 校验
   - fake / lightpanda 统一 profile 视图
   - task/status 详情暴露 fingerprint resolution status

3. **Proxy pool / verify / trust score 主链**
   - proxy CRUD
   - provider / region / min_score / cooldown 过滤
   - sticky session 正式绑定表 `proxy_session_bindings`
   - smoke / verify / verify-batch / verify batch 查询
   - verify 结果、执行结果反哺 proxy score
   - provider / provider×region 风险快照
   - cached trust score 持久化、scan / repair / maintenance

4. **Explainability / 可观测性主链**
   - task / status / explain 接口统一暴露 `selection_reason_summary`
   - `winner_vs_runner_up_diff` 结构化输出
   - `candidate_rank_preview` 强类型化
   - `trust_score_components` 强类型化
   - `summary_artifacts` schema 标准化（source/category/severity）
   - run 级 `result_json` 持久化与 `run_id / attempt / timestamp` 溯源字段
   - `/proxies/:id/explain` 暴露 `trust_score_cached_at / explain_generated_at / explain_source`
   - explainability assembler 已从 handlers 中抽离到独立模块

5. **测试与稳定性**
   - 单测 + 集成测试持续覆盖执行、代理、verify、trust score、explainability 主链
   - 当前测试状态：**40 unit + 75 integration 全绿**

## 当前风险

1. **verify 慢路径仍偏轻。**
   当前 verify 已经有 geo / anonymity / upstream 信号，但仍属于轻量探测，距离更真实的出口真实性与匿名性校验链还有差距。

3. **高并发下的 SQL / 写放大治理还没有正式做。**
   trust cache、verify 回写、status 聚合、selection explain 已经全部进入主链，后续要正式看查询成本、索引策略与写频率。

4. **文档与路线已经追回主线，但 TODO 仍需要持续按代码节奏收敛。**
   现在文档不再明显落后，但如果继续高频迭代而不持续同步，很快还会再次漂移。

5. **Lightpanda 真实浏览器侧的更深 fingerprint 消费还没正式进入验证阶段。**
   当前 profile 注入主链是通的，但真实浏览器侧的更深能力与性能影响还没有系统评估。

## 当前下一步

### P0
1. **继续收窄 trust cache / risk snapshot 的 refresh 范围**，避免 provider 级刷新在高频 verify 下放大写压力。
2. **推进更真实的 verify 慢路径**，补匿名性 / 地区 / 出口真实性校验链。
3. **继续清生产路径里剩余的 typed/JSON 边界**，把 explainability 主链周边再收一轮。
4. **做一轮 selection / status / trust cache / verify 回写的 explain-level profiling 记录**，把热点和成本量化。

### P1
5. 设计代理质量评分系统正式形态。
6. 设计 `SessionIdentity / ExecutionIdentity`，把 `proxy + fingerprint + region + risk_level` 收到统一表达。
7. 继续压 panic 风险点、锁竞争风险点与 flaky 测试。
8. 继续完善 API / 运维 / 能力说明文档。

## 本轮体检（2026-04-01）

- **找 bug：** 这轮未再暴露主链设计级错误，更多是并行测试中的瞬时编译噪音；关键主线在全量测试里已稳定回绿。
- **性能评分：** 当前阶段 **8.4/10**。优点是 explainability 主链已经完成模块级测试锁死，verify/trust refresh 写放大也开始收窄；扣分点仍然是 verify 慢路径和更深层的 refresh 范围优化尚未完成。
- **改进建议：** 下一步最值得做的是 **refresh 范围继续收窄 + verify 慢路径增强 + 生产路径残余边界继续清理**。

## Autopilot Sync

- 当前文档已对齐到 **2026-04-01 explainability 主链收口进度**。
