# CURRENT_TASK.md

## 当前任务

Task Contract / Control-Plane Visibility V1 已完成。本轮当前任务已经切换为：在不重新扩散 contract 范围的前提下，继续推进真实 Lightpanda 执行稳定化、失败分类、explainability 质量和 verify/trust 执行闭环。

---

## 任务目标

围绕 lightpanda-automation，当前阶段先完成以下三件事：

1. 继续补真实浏览器任务流样本
   - 增加 beyond-stub 的真实执行验证
   - 继续补 timeout / cancel / execution failure 的边界样本
   - 让真实引擎行为不只停留在 contract 层闭环

2. 继续清 explainability / artifact 质量
   - 继续统一 detail / runs / status 的细粒度表现
   - 继续提升 summary_artifacts 与 explain 字段可读性
   - 避免结构稳定但运营理解成本仍高

3. 推进 verify / trust score 执行闭环
   - 从选前判断继续扩展到执行前 / 执行中 / 执行后
   - 继续验证执行结果如何稳定反哺 trust score 与 proxy 质量判断

---

## 当前阶段交付物

本阶段应优先补齐：

- [x] 浏览器执行系统 V1
- [x] Fingerprint profile 注入第一版
- [x] 代理池基础能力
- [x] sticky session 正式绑定
- [x] smoke / verify / batch verify / 巡检 V1
- [x] trust score 起点与主链接入
- [x] status / explain / result 基础控制面
- [x] ExecutionIdentity V1 接口统一输出
- [x] running cancel -> cancelled 正式终态化
- [x] contract 文档同步到当前事实
- [x] /status + detail + runs 一致性测试
- [x] cancelled 契约测试
- [x] 远程 contract 验收
- [ ] 继续补真实浏览器任务流样本与失败分类
- [ ] 继续清 explainability summary / artifact 文案质量
- [ ] 推进 verify / trust score 执行闭环深化

---

## 下一步优先级

### P0
1. 补更多真实 Lightpanda 任务流样本，优先围绕超时、取消、真实执行失败边界
2. 检查并收口 detail / runs / status 在 explainability 细节上的剩余漂移
3. 推进 verify / trust score 从选前判断扩展到执行闭环

### P1
4. 继续治理高并发下的写放大、状态竞争、聚合成本
5. 设计 artifact / log 的保留、清理与归档策略
6. 再评估更深真实指纹消费与长期运行成本边界

---

## 判定标准

如果一个推进动作不能帮助回答下面任一问题，就应降低优先级：

- 它是否让真实 Lightpanda 执行链更稳定？
- 它是否让 explainability / artifact 更容易被运营面理解？
- 它是否让 verify / trust score 更接近执行闭环，而不是停留在选前判断？
- 它是否在不重新扩新 contract 范围的前提下，继续推进长期主线？
