# Progress

## 已实现

### [2026-03-26 11:23:43] 项目基础骨架已建立
- 已创建 Rust 项目基础结构。
- 已建立 `Cargo.toml`、`main.rs`、配置、状态、错误、模型、存储、调度、runner、API 等基础模块。
- 已建立 SQLite 迁移文件与 `systemd` 服务文件。

### [2026-03-26 11:23:43] 控制面基础能力已接通
- 已具备 HTTP API 骨架：`/health`、`/tasks`、`/tasks/:id`、`/tasks/:id/logs`。
- 已具备 SQLite 任务存储与日志存储。
- 已具备内存任务队列与后台 dispatcher。
- 已具备 fake runner，可用于端到端验证任务生命周期。

### [2026-03-26 11:23:43] Browser adapter 架构已建立
- 已建立 `runner/lightpanda/` 目录。
- 已建立 `BrowserAdapter` trait。
- 已建立 `BrowserRunResult` 等结果结构。
- 已建立 `LightpandaCliAdapter` 占位实现。
- 已建立 parser / models / cli 模块骨架。

### [2026-03-26 11:23:43] Browser runner 主流程已接线
- `BrowserRunner` 已不再是空占位。
- `RunnerMode::Browser` 已接到 `BrowserRunner<LightpandaCliAdapter>`。
- 真实 browser 执行路径已在主流程结构中打通，但底层 adapter 仍为占位实现。

### [2026-03-26 11:23:43] Runner mode 已下沉到配置层
- 已在 `AppConfig` 中加入 `runner_mode`。
- 已定义 `RunnerModeConfig::{Fake, Browser}`。
- dispatcher 已改为读取配置中的 runner mode，而非内部写死。

### [2026-03-26 11:23:43] 编译环境已恢复，项目当前可编译
- Cargo 代理污染问题已排除。
- 项目已恢复正常依赖下载与编译检查。
- 当前主干 `cargo check` 可通过（仍有合理 warning）。

### [2026-03-26 11:23:43] 文档基础已建立
- 已重写 `README.md`。
- 已建立 `docs/architecture.md`。
- 当前文档已覆盖项目目标、模块职责、控制面/执行面分层、接入策略与限制。

## 接下来要实现

### [2026-03-26 11:23:43] 接下来优先级
1. 为 `AppConfig` 增加环境变量读取能力，支持切换 `runner_mode=fake/browser`。
2. 为 `LightpandaCliAdapter` 增加最小 healthcheck / 命令探测逻辑。
3. 让 Browser 模式至少跑通一次结构化占位链路，而不只是静态接线。
4. 将当前进度同步补充到 `README.md` 与 `architecture.md` 的相关描述中。
5. 在后续每个功能小步完成后，持续登记到本文件，并记录精确时间。
