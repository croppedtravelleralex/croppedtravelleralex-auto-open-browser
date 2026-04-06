# CURRENT_TASK.md

## 当前任务

当前任务已经从“继续推进 trust score 核心化 / verify 慢路径 / refresh 范围收口”升级到 **真实 browser 执行主线**。当前已完成执行引擎、代理池、指纹注入、sticky session、verify / trust score、status / explain / result 基础控制面的第一阶段接线；接下来主线不再是继续围绕排序细节打磨，而是把这些能力抬升为 **真实 browser 执行稳定化 + 统一执行身份 + 执行闭环质量系统 + 长期运营级控制面**。

---

## 任务目标

围绕 `lightpanda-automation`，当前阶段先完成以下五件事：

1. **稳定真实 browser 执行链**
   - 检查真实浏览器执行链上的不稳定点：任务进入、调度、运行、结果回写、失败回放
   - 优先解决真实执行路径上的稳定性和可复现性问题
   - 让系统具备持续跑真实任务而不是只在选择侧更“看起来合理”

2. **建立统一执行身份模型**
   - 设计并落地 `proxy + fingerprint + session identity` 的统一表达
   - 让网络身份、设备身份、地区一致性、会话连续性在同一条执行主线里被消费
   - 避免 verify、selection、runtime、result 对身份的定义继续分裂

3. **把 verify / trust score 扩展到执行闭环**
   - 保留 verify / trust score 的选前价值，但把重点升级为执行前、执行中、执行后全链路质量判断
   - 让 verify 结果、运行失败、执行结果、会话表现共同反哺质量模型
   - 让质量判断真正服务真实执行，而不是停留在选前门槛

4. **把 status / explain / result 升级成长期运营级控制面**
   - 让状态面不仅能看“当前选了谁”，还要能看“执行是否稳定、身份是否一致、失败集中在哪” 
   - 让 explain 面不仅解释排序，还解释身份与执行结果
   - 让 result 面成为长期运营、定位问题、回看效果的主入口之一

5. **把性能与并发治理作为配套支撑推进**
   - 继续治理写放大、claim/reclaim 抖动、状态竞争、聚合查询成本
   - 目标是支撑真实 browser 执行主线长期运行
   - 不再把性能与并发治理写成当前阶段总纲本身

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
- [ ] CURRENT_* / STATUS / TODO 文档同步到新阶段口径
- [ ] 真实 browser 执行链稳定化第一轮
- [ ] Identity Profile / SessionIdentity / ExecutionIdentity 第一版
- [ ] verify / trust score 执行闭环第一版
- [ ] status / explain / result 运营级控制面升级第一轮
- [ ] 性能与并发治理支撑项第一轮收口

---

## 下一步优先级

### P0
1. **整理四个阶段文档**，把项目主线统一切到真实 browser 执行阶段
2. **推进真实 browser 执行链稳定化**，优先定位执行路径中的稳定性与回写一致性问题
3. **设计统一执行身份模型**，把 `proxy + fingerprint + session identity` 收到统一表达
4. **定义 verify / trust score 的执行闭环口径**，明确选前、执行中、执行后分别如何贡献质量判断
5. **升级 status / explain / result 视角**，从排序解释扩展到执行稳定性、身份一致性、结果可运营
6. **继续做性能与并发治理**，但严格按支撑主线的方式推进

### P1
7. 继续补真实浏览器任务流样本与失败分类
8. 继续补 selection / verify / runtime / result 的统一 explainability
9. 继续压 panic 风险点、锁竞争风险点与 flaky 测试
10. 继续完善 API / 运维 / 能力说明文档

### P2
11. 设计策略引擎正式形态
12. 设计行为层模拟机制
13. 设计会话连续性机制
14. 设计实验记录系统
15. 评估更深真实指纹消费与长期运行成本边界

---

## 判定标准

如果一个推进动作不能帮助回答下面任一问题，就应降低优先级：

- 它是否让 **真实 browser 执行链** 更稳定？
- 它是否让 **proxy + fingerprint + session identity** 更统一？
- 它是否让 **verify / trust score** 更像执行闭环，而不是只服务选前判断？
- 它是否让 **status / explain / result** 更接近长期运营级控制面？
- 它是否以配套方式提升 **并发稳定性 / 写放大控制 / 运行成本治理**？

补充约束：当前规则仍然默认服从“真实执行优先、身份一致优先、长期可运营优先”；verify、trust score、refresh、并发治理都要服务这条主线，而不是重新变成独立总纲。
