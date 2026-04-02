# STATUS.md

## 当前状态摘要

- **状态：** 已进入 **trust score 核心化 + verify 慢路径准备 + 性能治理前置阶段**
- **日期：** 2026-04-02
- **当前焦点：** 把 **selection 中仍分散存在的排序/惩罚/兜底语义** 继续统一进 **trust score / explainability 主链**，并为更真实的 verify 慢路径与高并发写放大治理收口。

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

5. **当前阶段的新收敛点：selection → trust score 核心化**
   - auto 选择主链已经明确走 `trust score` 排序
   - explainability 已能输出 `trust_score_components`、候选预览与 winner-vs-runner-up 对比
   - provider/provider×region 风险已经进入 score 组件层表达
   - 但 **explicit / sticky / filter / cooldown / min_score / no-match fallback** 仍有部分语义分散在 selection 过程，而非完全统一进入 score 体系
   - 当前真正的主任务，不再只是“explainability 收口”，而是 **继续把 selection 规则压缩为可解释、可调参、可测试的统一 score 语言**

6. **测试与稳定性**
   - 单测 + 集成测试持续覆盖执行、代理、verify、trust score、explainability 主链
   - 当前测试状态：**40 unit + 75 integration 全绿**

## 当前风险

1. **selection 语义仍未完全统一。**
   当前 auto 主链已经走 trust score，但 explicit / sticky / cooldown / min_score / no-match fallback 仍部分留在选择分支与 SQL 过滤层，后续若继续叠规则，维护成本会再次升高。

2. **verify 慢路径仍偏轻。**
   当前 verify 已经有 geo / anonymity / upstream 信号，但仍属于轻量探测，距离更真实的出口真实性与匿名性校验链还有差距。

3. **高并发下的 SQL / 写放大治理还没有正式做。**
   trust cache、verify 回写、status 聚合、selection explain 已经全部进入主链，后续要正式看查询成本、索引策略与写频率。

4. **文档已基本追回代码主线，但仍需持续同步。**
   `CURRENT_TASK.md` 与 `TODO.md` 已经基本反映真实方向，但如果 `STATUS.md` / `CURRENT_*` 不持续同步，自动推进仍可能再次围绕旧阶段动作打转。

5. **Lightpanda 真实浏览器侧的更深 fingerprint 消费还没正式进入验证阶段。**
   当前 profile 注入主链是通的，但真实浏览器侧的更深能力与性能影响还没有系统评估。

## 当前下一步

### P0
1. **继续推进 selection → trust score 核心化**，把更多分散在 selection 里的排序/惩罚/兜底语义统一进 score 表达。
2. **梳理 explicit / sticky / min_score / cooldown / no-match fallback 的统一边界**，明确哪些应保留为硬过滤，哪些应下沉为 score 惩罚或 explainability 语义。
3. **推进更真实的 verify 慢路径**，补匿名性 / 地区 / 出口真实性校验链。
4. **做一轮 selection / trust cache / verify 回写 / status 聚合的 profiling 记录**，量化热点和写放大。
5. **继续清生产路径里剩余的 typed/JSON 边界**，把 explainability 主链周边再收一轮。

### P1
6. 设计代理质量评分系统正式形态。
7. 设计 `SessionIdentity / ExecutionIdentity`，把 `proxy + fingerprint + region + risk_level` 收到统一表达。
8. 继续压 panic 风险点、锁竞争风险点与 flaky 测试。
9. 继续完善 API / 运维 / 能力说明文档。

## 本轮体检（2026-04-02）

- **找 bug：** 当前没有暴露 selection 主链级 crash，但存在一个明显“软 bug”——阶段表达略旧，容易让后续推进误以为重点仍在 explainability 尾活，而不是 trust score 核心化。
- **性能评分：** 当前阶段 **8.5/10**。优点是 trust score / explainability 主链已具备强类型结构和候选对比能力；扣分点仍然是 verify 慢路径未加深、selection 语义仍有分散、profiling 结果还未正式量化。
- **改进建议：** 下一步最值得做的是 **列出 selection 中仍未统一进 trust score 的规则清单，并优先把最影响排序语义的一批收进去**。

## Autopilot Sync

- 当前文档已对齐到 **2026-04-02 trust score 核心化 + verify 慢路径准备 + 性能治理前置阶段**。
