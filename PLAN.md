# PLAN.md

lightpanda-automation / AutoOpenBrowser 项目统一计划书。

---

## 1. 当前总目标

把当前已经跑起来的真实 browser 自动化后端，继续推进为一个：

- 具备稳定任务生命周期管理能力
- 具备统一执行身份模型（proxy + fingerprint + session identity）
- 具备执行前 / 执行中 / 执行后闭环质量判断
- 具备长期运营级 status / explain / result 控制面
- 可在真实 Lightpanda 执行链上持续稳定运行

---

## 2. 当前阶段

当前阶段不是继续扩新执行能力，而是：

> 把已经完成的 ExecutionIdentity V1 + cancelled API 闭环 升级成 Task Contract / Control-Plane Visibility V1。

这意味着当前阶段首先要做到：
- 文档口径一致
- status / detail / runs 口径一致
- cancelled 终态口径一致
- 测试能直接钉住这些事实

---

## 3. 当前优先级

### P0：主线推进
1. 更新 docs/lightpanda-api-task-structure.md、docs/api-ops.md、docs/control-plane-and-visibility-mainline.md，明确当前稳定 contract
2. 同步 STATUS.md / TODO.md / CURRENT_TASK.md / progress.md / PLAN.md 到 contract 收口阶段
3. 在 tests/integration_api.rs 增加三面一致性测试，验证 /status、detail、runs 对同一执行的 identity / explain / failure 口径一致
4. 在 tests/integration_api.rs 增加 cancelled 契约测试，固定 runner_cancelled 的正式终态表达
5. 如测试暴露轻微字段漂移，仅最小修补 src/api/handlers.rs / src/api/explainability.rs
6. 完成远程测试与 curl 验收

### P1：下一阶段预留
1. 继续补更多真实任务流样本与失败分类
2. 继续把 status / explain / result 推向更高层运营级控制面
3. 在 contract 收口后，再回到真实 Lightpanda 执行稳定化下一轮

### P2：中期能力铺垫
1. 补更深真实指纹消费边界
2. 代理池 / 代理抓取 / 清洗 / 轮换 / 自生长策略设计
3. 磁盘使用控制、artifact/log 保留与归档策略
4. 高并发下性能优化与写放大控制策略

---

## 4. 当前已知阻塞 / 风险

- 文档可能仍落后于接口现实，需要先完成同步
- tests/integration_api.rs 还缺针对统一 identity / cancelled contract 的最小钉子
- 当前工作树已有未提交改动，实施时需严格最小改动
- beyond-stub 的真实 Lightpanda 深层验证仍是后续阶段任务，不应在本轮重新扩面

---

## 5. 当前执行原则

1. 一次只聚焦一个主任务
2. 文档描述必须与代码能力对齐
3. 本轮默认不扩新能力，只收口 contract
4. 若测试暴露问题，优先最小修补现有接口装配，不新增新字段和新行为

---

## 6. 建议的接手动作顺序

1. 先读 STATUS.md 与 progress.md，确认当前 contract 收口状态
2. 再读 docs/api-ops.md / docs/lightpanda-api-task-structure.md / docs/control-plane-and-visibility-mainline.md
3. 补 tests/integration_api.rs 的一致性与 cancelled 契约测试
4. 再决定是否需要最小修补 handler / explainability

---

## 7. 本计划书与旧文档关系

- TODO.md：保留为细粒度待办池
- ROADMAP.md：保留为滚动路线图
- CURRENT_TASK.md / CURRENT_DIRECTION.md：保留为阶段性方向文件
- PLAN.md：只做统一收口与当前优先级定义
