# STATUS.md

## 当前状态摘要

- **状态：** 已进入 **真实 browser 执行主线升级阶段**
- **日期：** 2026-04-06
- **当前焦点：** 把项目从“代理排序 / trust score / verify / refresh 收口”上提到 **真实 browser 执行链稳定化 + 统一执行身份模型 + 执行闭环质量系统 + 长期运营级控制面**

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

3. **Proxy / verify / trust score 基础质量链**
   - proxy CRUD
   - provider / region / min_score / cooldown 过滤
   - sticky session 正式绑定表 `proxy_session_bindings`
   - smoke / verify / verify-batch / verify batch 查询
   - verify 结果、执行结果反哺 proxy score
   - provider / provider×region 风险快照
   - cached trust score 持久化、scan / repair / maintenance

4. **Explain / status / result 基础控制面**
   - task / status / explain 接口统一暴露 `selection_reason_summary`
   - `selection_explain` 结构化输出
   - `winner_vs_runner_up_diff` 结构化输出
   - `candidate_rank_preview` 强类型化
   - `trust_score_components` 强类型化
   - `summary_artifacts` schema 标准化（source/category/severity）
   - run 级 `result_json` 持久化与 `run_id / attempt / timestamp` 溯源字段
   - `/proxies/:id/explain` 暴露 `trust_score_cached_at / explain_generated_at / explain_source`
   - explainability assembler 已从 handlers 中抽离到独立模块

5. **真实 browser 执行主线的阶段判断**
   - 代理排序、verify 慢路径、trust score、refresh 范围治理已经完成第一阶段接线
   - 这些能力现在更适合作为真实执行主线的支撑层，而不是继续单独充当项目总纲
   - 当前阶段的核心问题已变成：真实执行是否稳定、身份是否统一、质量判断是否覆盖执行闭环、控制面是否能支撑长期运营

6. **统一执行身份模型仍待正式落地**
   - 当前已有 proxy 绑定、fingerprint 注入、sticky session 等基础
   - 但 `proxy + fingerprint + session identity` 仍未形成单一执行身份模型
   - selection、verify、runtime、result 之间仍存在语义分散问题

7. **性能与并发治理进入支撑位**
   - trust cache、verify 回写、status 聚合、selection explain 已经全部进入主链
   - 当前需要持续治理写放大、状态竞争、聚合成本与高并发抖动
   - 但这些工作现在的定位是支撑真实 browser 执行长期运行，而不是单独定义阶段目标

8. **测试与稳定性**
   - 单测 + 集成测试持续覆盖执行、代理、verify、trust score、explainability 主链
   - 当前测试状态：**41 unit + 84 integration 全绿**

## 当前风险

1. **真实 browser 执行主线还没有被明确写成第一优先级。**
   如果继续沿用旧口径，后续动作容易继续围绕排序细节和 refresh 收口打转，而不是解决真实执行稳定性。

2. **执行身份语义仍然分散。**
   当前 proxy、fingerprint、sticky session、地区一致性已经分别存在，但还没有统一成可被 selection / runtime / result / explain 共同消费的执行身份模型。

3. **verify / trust score 仍偏“选前判断”视角。**
   当前 verify 慢路径已经进入部分排序输入，但执行中与执行后的质量反馈仍未完整进入闭环。

4. **status / explain / result 还偏调试视角。**
   现在能解释排序和部分运行信息，但还不足以作为长期运营级控制面持续回答“执行是否稳定、身份是否一致、失败集中在哪”。

5. **高并发下的写侧压力与状态竞争仍需持续治理。**
   写放大、refresh 范围、回写频率、状态聚合成本依然是真实 browser 执行主线规模化后的主要支撑风险。

6. **文档刚完成阶段切换，后续仍需持续同步。**
   如果 `STATUS / TODO / CURRENT_*` 不继续跟进，自动推进仍可能回到旧主线。

## 当前下一步

### P0
1. **统一阶段文档口径**，把当前主线明确为真实 browser 执行升级阶段。
2. **推进真实 browser 执行链稳定化**，优先定位执行路径上的稳定性、回写一致性、失败可复现问题。
3. **设计 `SessionIdentity / ExecutionIdentity`**，把 `proxy + fingerprint + session identity` 收到统一执行身份表达。
4. **定义 verify / trust score 的执行闭环角色**，明确它们在执行前、执行中、执行后分别如何产出与消费质量信号。
5. **升级 status / explain / result 控制面**，让它们开始面向长期运营而不是只服务调试。

### P1
6. 继续补真实任务流样本与失败分类。
7. 继续补 selection / verify / runtime / result 的统一 explainability。
8. 继续压 panic 风险点、锁竞争风险点与 flaky 测试。
9. 继续完善 API / 运维 / 能力说明文档。

## 本轮体检（2026-04-06）

- **找 bug：** 本轮重点不是新增代码修复，而是完成阶段口径切换，避免项目继续围绕旧主线做小修小补。
- **阶段判断：** 当前最重要的不是再证明 trust score / verify / refresh 有多完整，而是明确它们已经应当退到支撑位，主线升级为真实 browser 执行系统。
- **改进建议：** 下一步最值得做的是把 `proxy + fingerprint + session identity` 和 `status / explain / result` 拉到同一执行闭环视角下。

## Autopilot Sync

- 当前文档已对齐到 **2026-04-06 真实 browser 执行主线升级阶段**。
- **阶段冻结边界：** 性能与并发治理继续推进，但按支撑项处理；不再把代理排序 / trust score / refresh 收口单独写成阶段总纲。
