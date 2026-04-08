# CURRENT_TASK.md

## 当前任务

当前任务已经从真实 browser 执行主线升级阶段进一步收口到 Task Contract / Control-Plane Visibility V1。当前代码层面已经完成 ExecutionIdentity V1 输出接线与 running cancel -> cancelled 正式终态化；接下来这轮不再扩新功能，而是把这些成果升级成 文档契约 + 接口一致性测试 + 远程验收依据。

---

## 任务目标

围绕 lightpanda-automation，当前阶段先完成以下四件事：

1. 统一对外 contract 文档
   - 把 execution_identity 正式写成统一执行身份视图
   - 把 cancelled 写成正式终态而不是临时状态
   - 明确 /status、/tasks/:id、/tasks/:id/runs 的职责边界

2. 补最小 API 一致性测试
   - 验证同一任务在 /status、detail、runs 中的 execution_identity 口径一致
   - 验证 failure_scope、browser_failure_signal、summary_artifacts 等主张不会在不同视图漂移

3. 补 cancelled 契约验证
   - 固化 runner_cancelled 语义
   - 确认 task / run / status 对 cancelled 的对外口径一致
   - 避免后续回退成 generic failure / timeout 混写

4. 完成远程验收闭环
   - 按文档走 create -> inspect -> cancel -> inspect 流程
   - 确认文档写的字段与实际接口返回字段一致

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
- [ ] contract 文档同步到当前事实
- [ ] /status + detail + runs 一致性测试
- [ ] cancelled 契约测试
- [ ] 远程 contract 验收

---

## 下一步优先级

### P0
1. 更新 contract 文档，统一 execution_identity / cancelled / status-detail-runs 口径
2. 补 integration_api 最小验证，钉住三面一致性与 cancelled 契约
3. 必要时最小修 handler/explainability，只修字段漂移，不扩新能力
4. 跑远程测试与 curl 验收，确认文档、接口、测试三者一致

### P1
5. 继续补真实浏览器任务流样本与失败分类
6. 继续补 selection / verify / runtime / result 的统一 explainability
7. 继续把 status / explain / result 推向长期运营级控制面

### P2
8. 在 contract 收口完成后，再重回真实 Lightpanda 执行稳定化下一轮
9. 再评估更深真实指纹消费与长期运行成本边界

---

## 判定标准

如果一个推进动作不能帮助回答下面任一问题，就应降低优先级：

- 它是否让 execution_identity 的对外口径更稳定？
- 它是否让 cancelled 正式终态更不容易被后续改坏？
- 它是否让 /status、detail、runs 更像同一控制面的不同层级，而不是三个分裂视图？
- 它是否让文档、接口、测试、远程验收指向同一事实？

补充约束：当前规则仍然默认服从 contract 稳定优先、控制面一致优先、最小改动优先；本轮默认不扩新功能。
