# Task Plan: lightpanda-automation 主线推进

## Goal
在 **Ubuntu headless / absolute fingerprint first** 约束下，持续推进 `lightpanda-automation` 主线：优先完成真实指纹消费、指纹-代理-地区一致性、性能/并发预算与 explainability 闭环，并避免系统退化成伪串行。

## Task Size Analysis
- **任务大小：大任务**
- **原因：** 跨模块、跨阶段、跨会话，且包含代码、验证、性能、文档同步四条线。
- **默认策略：** 先收边界，再拆阶段；每轮只执行当前最值步骤；默认减少 token 开销，只做定向检查与必要输出。

## Current Phase
Phase 1

## Phases

### Phase 1: 主线校准与收边界
- [x] 确认当前项目主线与环境约束
- [x] 确认当前默认规则：先判任务大小、按大小拆分、最后执行最值步骤、减少 token 开销
- [x] 确认当前项目已启用本地 planning 控制面
- **Status:** complete

### Phase 2: 指纹优先主线梳理
- [x] 收口当前已实现能力
- [x] 明确当前 P0 主线与冻结边界
- [x] 明确当前最值下一步
- **Status:** complete

### Phase 3: 当前最值步骤执行
- [x] 优先推进 `proxy_growth` 接入选择链路或 explain 输出
- [x] 同步检查 explainability summary / artifact 文案质量
- [x] 保持“heavy 限量但不堵死 light/medium”的并发红线
- **Status:** complete

### Phase 4: 验证与回归
- [x] 跑定向测试或关键链路验证
- [x] 记录本轮结果、风险与是否进入下一步
- [x] 如发现问题，先修主线阻塞项
- **Status:** complete

### Phase 5: 文档与控制面同步
- [x] 同步 `STATUS / TODO / PROGRESS`
- [x] 若阶段切换，再补充 planning 文件
- [x] 稳定后再 commit
- **Status:** complete

## Key Questions
1. 当前最值动作是否仍是把 `proxy_growth` 真正接入 selection/explain 主链？
2. 接线后是否会引入新的排序语义漂移或 explain 文案噪音？
3. 如何在继续增强真实能力的同时保持 headless + 低 token + 非伪串行约束？

## Decisions Made
| Decision | Rationale |
|----------|-----------|
| 本项目按“大任务”处理 | 跨模块、跨阶段、跨会话，不能用小任务方式硬顶 |
| 默认先做最值步骤，不全量重扫 | 降低 token 开销，减少无效排查 |
| 当前最值步骤先选 `proxy_growth` 接线 | TODO 中仍未完成，且直接关系高级代理体系最小闭环 |
| planning-with-files 作为本地控制面 | 便于跨会话续推进，不丢主线 |

## Errors Encountered
| Error | Attempt | Resolution |
|-------|---------|------------|
| planning 初始化阶段脚本 heredoc / grep 低级错误 | 1 | 改为更稳的直接文件操作与定向检查；未影响项目主线文件 |

## Notes
- 默认先判任务大小，再决定拆分深度与执行方式
- 默认减少 token 开销：少读大文件、少做全量扫描、少重复总结
- 大任务每轮只推进一个当前最值步骤，避免支线膨胀
