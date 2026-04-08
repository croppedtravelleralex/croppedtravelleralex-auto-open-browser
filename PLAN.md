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

Task Contract / Control-Plane Visibility V1 已完成。

当前阶段已从 contract 收口切换为：

> 回到真实 Lightpanda 执行稳定化 + explainability 质量深化 + verify/trust 执行闭环推进。

这意味着当前阶段首先要做到：
- 继续补真实任务流样本
- 继续清 explainability / artifact 质量
- 继续推进 verify / trust score 执行闭环
- 不重新扩散已完成的 contract 范围

---

## 3. 当前优先级

### P0：主线推进
1. 补更多真实浏览器任务流样本，优先围绕 timeout / cancel / execution failure 边界
2. 继续清理 detail / runs / status 的 explainability / artifact 细节表现
3. 推进 verify / trust score 从选前判断扩展到执行前 / 执行中 / 执行后闭环
4. 继续做远程真实链路验证，确认当前主线不是只在测试里成立

### P1：下一阶段预留
1. 继续治理高并发下的写放大、状态竞争、聚合成本
2. 设计 artifact / log 的保留、清理与归档策略
3. 继续评估更深真实指纹消费边界与长期运行成本

### P2：中期能力铺垫
1. 代理池 / 代理抓取 / 清洗 / 轮换 / 自生长策略设计
2. 磁盘使用控制、artifact/log 保留与归档策略
3. 高并发下性能优化与写放大控制策略

---

## 4. 当前已知阻塞 / 风险

- beyond-stub 的真实 Lightpanda 深层验证仍不足
- explainability / artifact 文案质量还有继续清理空间
- verify / trust score 还未完全收成执行闭环
- 当前工作树仍有未跟踪目录 `data/`，提交时需继续避免误入库

---

## 5. 当前执行原则

1. 一次只聚焦一个主任务
2. 文档描述必须与代码能力对齐
3. contract 已完成，本轮不再重复扩写同一主线
4. 若出现跨视图细节漂移，优先最小修补现有装配路径，不新增平行抽象

---

## 6. 建议的接手动作顺序

1. 先跑真实 Lightpanda 任务流样本，确认当前 beyond-stub 能力边界
2. 再读 status/detail/runs 的剩余 explainability 表现，找最值钱的细节缺口
3. 再决定是先补失败分类样本，还是先推进 verify/trust 执行闭环

---

## 7. 本计划书与旧文档关系

- TODO.md：保留为细粒度待办池
- ROADMAP.md：保留为滚动路线图
- CURRENT_TASK.md / CURRENT_DIRECTION.md：保留为阶段性方向文件
- PLAN.md：只做统一收口与当前优先级定义
