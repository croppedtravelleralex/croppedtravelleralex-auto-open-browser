# 03 路线图

## 主线说明

当前路线图以“长期身份连续性控制面”为主线，不再把工作重心放在单纯的 visibility 或临时跑通能力上。

冻结目标：

- `persona`：店铺级、平台独立
- 终端：桌面优先
- 运行模型：低频心跳 + 任务驱动
- 恢复边界：仅温和恢复
- 地区一致性：国家和地区都严格
- 人工接管：Telegram + API
- 冻结范围：仅冻结当前 `persona`

## 已完成：Phase 1 基线

已完成并通过远端 targeted 验证：

- 控制面 schema / API / 路由接通
- `persona_id` 进入 task/browser 契约
- `/status` 可按 persona 展示健康摘要与 continuity 指标
- manual gate 与 continuity event 时间线基线
- runner 的状态 restore / persist 基础闭环

## P0：收口 Phase 2 状态闭环

目标：

- 把当前 restore / persist 基础能力升级为可长期运行的正式闭环
- 补齐 heartbeat、snapshot 生命周期和断裂保护

当前重点：

1. 已完成最小 heartbeat tick / 入队 / 去重 / 主进程 loop
2. 补 heartbeat 失败治理、证据与观测
3. 快照版本治理、校验、清理、归档
4. login loss / region drift / snapshot restore fail 的统一断裂判定
5. `/status` 与 health snapshot 聚合口径稳定

## P1：小红书先打透

目标：

- 在统一 schema 下，把小红书做成首个完整样板平台

当前重点：

1. 完整小红书平台模板
2. 完整 continuity checks / login loss signals / recovery steps
3. manual gate 分类细化
4. Telegram + API 接管链路收口
5. 形成 30 天 persona 连续性验收口径

## P1：其余 6 平台补齐 baseline readiness

覆盖平台：

- Amazon
- eBay
- Shopify
- Walmart
- TikTok Shop
- 独立站后台
- 小红书

除小红书外，其余平台当前目标不是一次性打透，而是先达到 baseline：

1. persona 绑定
2. 模板可挂载
3. warm / revisit / stateful 路径可定义
4. login loss / region drift 可判定
5. 高风险路径可进入 manual gate

## P2：长稳运营化

目标：

- 从“能跑”升级到“可长期运营”

当前规划：

1. persona 健康度快照与 30/90 天趋势
2. 批量店铺级调度与容量治理
3. 事件时间线分页查询与归档策略
4. `10-50` 店铺级身份的长期并行运行

## 配套子线

以下工作继续保留，但降级为主线配套子任务：

- proxy / fingerprint explainability
- `/status` 观测口径细化
- release / preflight / longrun 验证脚本
- targeted integration tests 与回归门禁
