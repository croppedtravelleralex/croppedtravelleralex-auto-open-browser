# Findings & Decisions

## Requirements
- 项目主线遵循 **absolute fingerprint first**。
- 运行环境按 **Ubuntu 24.04 headless 服务端** 处理，不以 GUI / screenshot / 视觉链路为主线。
- 性能优化必须成立，但**不能把系统做成伪串行**。
- 当前默认规则新增：**先分析任务大小，再按大小拆分，最后执行当前最值步骤，并默认减少 token 开销**。
- 复杂项目推进时，需要保留本地 planning 控制面。

## Research Findings
- 当前项目已完成：`fingerprint_policy`、`fingerprint_consistency`、budget-aware claiming、budget visibility、concurrency-budget regression tests、`fingerprint_runtime_explain` task-result/API 聚合。
- `STATUS.md` 当前焦点仍强调 trust score 核心化、verify 慢路径并入主排序、性能治理前置。
- `TODO.md` 中与当前主线最贴近且未完成的一项是：**将 `proxy_growth` 规则接入选择链路或 explain 输出**。
- 当前项目已经存在较成熟的 `STATUS / TODO / PROGRESS` 控制面，适合继续做定向推进，而不是重新大面积扫描。

## Technical Decisions
| Decision | Rationale |
|----------|-----------|
| 采用 planning-with-files 作为当前主线本地控制面 | 先把跨会话推进稳定下来，再考虑额外执行面 |
| 当前轮次只定向锁定 `proxy_growth` 接线 | 这是未完成 P0 中最靠近主线价值的一项 |
| 不先重扫全仓 | 用户已明确要减少 token 开销 |
| 先用已有 `STATUS / TODO / PROGRESS` 收敛现状 | 这是最便宜的主线确认方式 |

## Issues Encountered
| Issue | Resolution |
|-------|------------|
| planning 初始化时脚本检查 `taskr` 配置连续出低级错误 | 暂不阻塞主线；先启用本地 planning 控制面，后续再定向检查 taskr |

## Resources
- `/root/SelfMadeprojects/lightpanda-automation/STATUS.md`
- `/root/SelfMadeprojects/lightpanda-automation/TODO.md`
- `/root/SelfMadeprojects/lightpanda-automation/PROGRESS.md`
- `/root/SelfMadeprojects/lightpanda-automation/task_plan.md`
- `/root/SelfMadeprojects/lightpanda-automation/findings.md`
- `/root/SelfMadeprojects/lightpanda-automation/progress.md`

## Visual/Browser Findings
- 当前轮没有使用浏览器/视觉检查。

---
*Update this file after every 2 view/browser/search operations*
*This prevents visual information from being lost*
