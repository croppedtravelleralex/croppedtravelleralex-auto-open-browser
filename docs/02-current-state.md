# 02 褰撳墠鐘舵€?

## 2026-04-14 continuity scoring delta

- `/status.identity_session_metrics` no longer depends on repeated `tasks.result_json` `json_extract` aggregation.
  - task terminal write path now stores explicit continuity metric columns
  - `/status` aggregates numeric task columns directly
  - historical/manual rows keep one-shot backfill compatibility
- continuity observability now exposes real timing splits:
  - `restore_ms`
  - `persist_db_ms`
  - `snapshot_ms`
  - `total_overhead_ms`
- new fields are now visible in:
  - task / run `execution_identity.continuity_timing`
  - `/status.identity_session_metrics.continuity_timing`
- local verification passed:
  - `cargo test --tests`
  - `cargo build --release`

## 2026-04-11 remote verification delta

- 远端 target/debug/AutoOpenBrowser 已重新拉起，/status 现在真实输出 mode 与 effective ratio 新口径。
- public-smoke preflight 已通过。
- public-smoke release_fast_verify 已通过。
- scripts/lightpanda_verify.sh 已修复 create body 传递与 json_get 原始 JSON 解析两个脚本问题。
- 远端 cargo test -q 仍未全绿，当前 blocker 已前移到远端脏分支中的 handlers/core.rs、runner/engine.rs、db/schema.rs 编译漂移。

## 2026-04-12 prod-live verification delta

- prod-live preflight 已通过。
- prod-live release_fast_verify 已通过，当前稳定报告为 `reports/release_prod_live_latest.txt`。
- 这次 prod-live 长跑窗口使用 180 秒快速验收，报告中已经满足：
  - `mode == prod_live`
  - `active_pool_ever_positive == true`
  - `stateful_continuity_observed == true`
- 最新 prod-live 报告显示 continuity 链已真实跑亮：
  - `auto_created -> auto_reused -> storage restored` 已被观测到
  - cookie / localStorage / sessionStorage restore + persist 均为正
- `scripts/proxy_longrun_report.py` 现已固定输出两个独立 artifact：
  - `reports/source_quality_summary_latest.txt|json`
  - `reports/session_continuity_summary_latest.txt|json`
- 当前 prod-live 快照仍暴露代理质量短板：
  - `effective_active_ratio_percent ≈ 0.52`
  - `promotion_rate ≈ 0.0604`
  - `reject_rate ≈ 0.9396`
  - hot region 仍为空，说明 release / continuity 已过，但代理质量分还未到目标线

## 2026-04-12 prod-live pool hygiene delta

- 已新增 `scripts/prod_proxy_pool_hygiene.py`，用于：
  - 从受控 prod config 读取 source 集合
  - 回填 `proxy_harvest_sources` 的 `for_prod/source_tier/validation_mode` 元数据
  - 对 `candidate/candidate_rejected` 做按 source 的压缩
  - 对 `zero-active + low-promotion` source 写入 `quarantine_until`
- 远端本轮已实跑：
  - `python3 scripts/prod_proxy_pool_hygiene.py --db data/auto_open_browser.db --mode prod_live --config /root/prod_proxy_sources.controlled.json --apply`
- 本轮实际落地结果：
  - 删除低价值 `candidate/candidate_rejected` 共 `16457` 条
  - 同步 prod source metadata `4` 条
  - `github_monosans_http` 已进入 quarantine
  - 为让运行中的内存池与新 DB 对齐，已重启**同一份**远端 `target/debug/AutoOpenBrowser`，未重编 binary
- 重启后的 `/status` 实际快照已经明显改善：
  - `proxy_pool_status.total = 481`
  - `active = 147`
  - `candidate = 271`
  - `candidate_rejected = 63`
  - `effective_active_ratio_percent = 30.5613`
  - `promotion_rate = 0.6774`
  - `reject_rate = 0.3226`
- 当前代理质量分已不再卡在“分母失真 + 低质源污染主池”这一层，prod-live 主池已经进入 80+ 验收区间。
- 当前剩余缺口已缩窄为：
  - `hot_regions` 仍为空，region 维度覆盖还不够
  - active 仍高度集中在 `github_speedx_http`
  - `expected_geo_quality` 仍以 `unknown` 为主，geo 真实性还没有进一步拉高
  - 这轮尚未补新的 30 分钟 prod-live 长跑，当前代理质量结论以实时 `/status` 与 DB 快照为主

## 2026-04-12 prod-live sustained hygiene delta

- 已继续把 pool hygiene 接入 `scripts/proxy_real_longrun_driver.py`：
  - `prod_live` 默认开启周期性 hygiene
  - 每个窗口调用 `scripts/prod_proxy_pool_hygiene.py`
  - `scripts/proxy_longrun_report.py` 已补 `hygiene_total / hygiene_deleted_proxy_rows_total / hygiene_ratio_before_last / hygiene_ratio_after_last`
- 已在远端完成一轮 240 秒验证：
  - `reports/proxy_real_longrun_hygiene_latest.txt|json`
  - 配置：`prod_live + controlled config + fingerprint_profile_id + stateful continuity`
- 这轮验证中已观测到：
  - harvest 曾把池从 `491` 短时拉到 `3264`
  - driver 自动执行 hygiene 后又把池压回 `481`
  - latest `effective_active_ratio_percent = 32.4324`
  - median `effective_active_ratio_percent = 31.7719`
  - median `promotion_rate = 0.7156`
  - median `reject_rate = 0.2844`
  - `hygiene_total = 6`，`hygiene_succeeded = 6`
  - `hygiene_deleted_proxy_rows_total = 6066`
  - `browser total = 12` 且 `browser_succeeded = 12`
  - `stateful_continuity_observed = true`
- 这说明当前提分已经从“一次性手工整治”进入“长跑过程中自动维持分母”的阶段。
- 同时已补上错误模式门禁：
  - `scripts/proxy_real_longrun_driver.py` 在 `prod_live` 下强制拒绝 repo demo config，即使显式传入 `--allow-demo-config` 也会失败
  - `scripts/proxy_mainline_verify.sh` 现在会在进入 real-live driver 前拒绝 `prod_live + PROXY_VERIFY_REAL_ALLOW_DEMO=1`
- 当前剩余代理质量缺口进一步收缩为：
  - `hot_regions` 仍为空
  - source concentration 仍偏高，`github_speedx_http` 占比过大
  - geo 质量仍主要停留在 `unknown/global`

## 2026-04-12 prod-live geo enrichment + regional demand delta

- 已新增 `scripts/prod_proxy_geo_enrich.py`，用于在**不重编 Rust binary** 的前提下，给 `prod_live` 代理池补 host-IP geo 元数据：
  - 回填 `country`
  - 回填 `region`
  - 回填 `last_exit_country`
  - 回填 `last_exit_region`
  - 回填 `last_exit_ip`
  - 追加 `last_verify_source=...+geoip_host_enrich`
- 远端已实跑：
  - `python3 scripts/prod_proxy_geo_enrich.py --db data/auto_open_browser.db --mode prod_live --config /root/prod_proxy_sources.controlled.json --only-status active --limit 200 --apply`
- 这轮实跑结果：
  - `updated_proxy_rows = 176`
  - `updated_source_rows = 3`
  - `lookup_succeeded = 176`
  - active region 从 **0 个已知 region** 提升到 **13 个已知 region**
  - top active regions 现已能直接分布到：
    - `ap-southeast = 62`
    - `eu-west = 40`
    - `cn = 24`
    - `jp = 16`
    - `us-east = 10`
    - `us-west = 9`
- 已继续把 geo enrichment 接入 `scripts/proxy_real_longrun_driver.py`：
  - `prod_live` 默认可周期性执行 geo enrich
  - 支持 `--auto-browser-regions-from-db`
  - 会按 active region top-N 自动轮转 browser demand，不再只打一条无 region 任务
- 同时已补一层 metadata 保护：
  - `scripts/prod_proxy_pool_hygiene.py` 在 sync source metadata 时，若 config 仍是 `unknown`，不再把已落地的 `host_geo_inferred` 覆盖回去
- 远端 120 秒区域 demand 验证结果：
  - `reports/proxy_real_longrun_geo_hot_latest.txt|json`
  - `effective_active_ratio_percent median = 33.0357`
  - `promotion_rate median = 0.749`
  - `reject_rate median = 0.251`
  - browser `8/8` 成功
  - `stateful_continuity_observed = true`
  - `browser requested_regions = ap-southeast=4, eu-west=2, cn=2`
  - `browser hot_regions_observed = ap-southeast=4, eu-west=2, cn=2`
  - `geo_enrich_active_regions_after_last = ap-southeast, eu-west, cn, jp, us-east`
- 需要注意：
  - `/status.latest.proxy_pool_status.hot_regions` 仍可能显示 `none`
  - 这不是 region demand 没打到，而是当前 control-plane 只统计**queued/running** 时刻；任务完成后 latest snapshot 会重新归零
  - 现在 longrun report 已补 `browser hot_regions_observed`，能够直接证明 in-flight region demand 已真实出现
鏇存柊鏃堕棿锛?026-04-11

## 宸插畬鎴愮殑鍏抽敭鏀归€?
### 1. 鍏变韩鎸囩汗娑堣垂鍙ｅ緞宸茶惤鍦?
宸叉柊澧炲苟鎺ュ叆锛?
- `src/network_identity/fingerprint_consumption.rs`
- validator / budget classifier / runner runtime explain 鍏变韩鍚屼竴 canonical schema
- `device_memory` 宸插綊涓€鍒?`device_memory_gb`
- `headers.accept_language` 宸插綊涓€鍒?`accept_language`

缁熶竴杈撳嚭锛?
- `declared_fields`
- `resolved_fields`
- `applied_fields`
- `ignored_fields`
- `consumption_status`
- `consumption_version`

### 2. explainability 涓诲瓧娈靛凡鎵╁睍

DTO / explainability 宸叉墦閫氫互涓嬪瓧娈碉細

- `mode`
- `source_tier`
- `verification_path`
- `continuity_level`
- `consumption_source_of_truth`
- `reported_active_ratio_percent`
- `effective_active_ratio_percent`
- `eligible_pool_total`
- `fresh_candidate_total`
- `recent_rejected_total`

### 3. proxy source metadata 宸茶繘鍏ヤ富閾捐矾

`proxy_harvest_sources` 鍙婅繍琛屾椂鍏冩暟鎹凡鏀寔锛?
- `source_tier`
- `for_demo`
- `for_prod`
- `validation_mode`
- `expected_geo_quality`
- `cost_class`
- `quarantine_until`

`engine` 涓殑 auto selection 涓庢樉寮?/ sticky 璺緞锛屽凡缁忓紑濮嬫寜 mode + source eligibility 杩囨护銆?
### 4. `/status` 涓诲彛寰勫凡鎷嗗嚭 effective ratio

褰撳墠 `/status.proxy_pool_status` 宸叉敮鎸侊細

- `reported_active_ratio_percent`
- `effective_active_ratio_percent`
- `eligible_pool_total`
- `fresh_candidate_total`
- `recent_rejected_total`

骞朵笖鎸夎繍琛?mode 杩囨护姹犲瓙鍙ｅ緞銆?
### 5. longrun 椹卞姩 / 鎶ュ憡宸茶繘鍏?mode 鍖?
鑴氭湰渚у凡琛ュ埌锛?
- `scripts/proxy_real_longrun_driver.py` 鏀寔 `--mode`
- `prod_live` 寮哄埗瑕佹眰 `fingerprint_profile_id`
- raw 杈撳嚭浼氬啓鍏?`mode`
- `scripts/proxy_longrun_report.py` 浼氳緭鍑?`mode`銆乪ffective/reported/derived ratio 鍖哄垎

## 姝ｅ湪鎺ㄨ繘 / 灏氭湭褰诲簳鏀跺彛

### 1. release / preflight profile 已完成 public-smoke + prod-live 收口

当前状态：

- `public-smoke` 远端已通过
- `prod-live` 远端已通过
- `gateway-upstream` 仍需补独立稳定验收

剩余工作已从“能否跑通”转为“结果是否足够强”和“失败分面是否足够稳定”。
### 2. verify batch 鐨?mode 杩囨护鍒氳ˉ鍏?
`verify_batch_proxies` 宸插紑濮嬫寜 `AUTO_OPEN_BROWSER_PROXY_MODE` + source eligibility 杩囨护 active proxy锛屼絾杩橀渶瑕佽繙绔疄璺戦獙璇併€?
### 3. 代理质量仍是当前主短板

这条短板在 2026-04-12 晚些时候已完成第一轮收口：

- 低质 source 已可通过 `prod_proxy_pool_hygiene.py` 自动 quarantine
- prod-live 主池分母已压缩到有效窗口，`effective_active_ratio_percent` 已提升到 `30.5613`
- promotion / reject 指标已经进入健康区间

当前未收口部分只剩：

- hot region 仍未建立稳定覆盖
- source 仍偏单一，尚未形成多 provider / 多 region 的均衡主池
## 褰撳墠涓昏闃诲

杩滅鏁村簱 `cargo test -q` 鐩墠琚?*浠撳簱鏃㈡湁闂**闃诲锛屼笉鏄繖杞敼閫犻鍏堝紩鍏ワ細

- `src/api/routes.rs` 寮曠敤浜嗗缁勫苟涓嶅瓨鍦ㄧ殑 handler

鍥犳褰撳墠楠岃瘉绛栫暐闇€瑕佹媶寮€锛?
- 鍏堝仛鑴氭湰 / Python / 灞€閮ㄨ娉曟鏌?- 鍐嶅仛杩滅 profile 瀹炶窇
- 鏈€鍚庢妸鏁村簱 compile blocker 浣滀负鐙珛椋庨櫓璁板綍


## 2026-04-12 continuity control-plane implementation delta

- ???????? visibility / profile ???????????????????????????????????????
  - `heartbeat` ???????
  - `manual gate` 6 ?????
  - continuity snapshot ???`current_stable / latest_attempt / quarantine_broken`
  - `StorePlatformOverride.admin_origin / entry_origin / entry_paths_json`
  - Telegram ?? + 30 ????
- `/status.proxy_pool_status` ??? **mode-aware** ???
  - `total / active / candidate / candidate_rejected / effective_active_ratio_percent`
  - `eligible_pool_total` ????? mode ?????????
- `verify_batch` ??? **mode-aware source eligibility**?
  - `prod_live` ??? `demo-only` source ????
  - legacy ? source ???????????????????????
  - `verify_batches.filters_json` ?? `mode`
- runtime mode ?? `AppState` ???????????????????????????????
- explainability ????????????
  - unresolved task ???????? proxy_id??????? execution identity
  - Lightpanda runner ?????? `fingerprint_runtime` ? control-plane explain????? runner runtime ??
- Windows ? Lightpanda ??? `.sh` ????????
  - ? timeout / non-zero-exit / cancel stub ? PowerShell ????
  - running cancel ?? PID ??????? Windows ?? `taskkill`
- ?????????
  - `cargo test --test integration_continuity_control_plane` ?
  - `cargo test --tests` ?
- ????????????????????????
  - `src/api/handlers.rs` ? `region_anchor` / `persona_status` ?? dead-code warning
  - `src/network_identity/proxy_harvest.rs` ? `HarvestSourceRow` ??????????????

## 2026-04-13 local warning-debt cleanup delta

- ?????? continuity / proxy-harvest ?????? warning debt??????????
  - `src/api/handlers.rs`
    - ?? `ResolvedNetworkPolicyModel.region_anchor`
    - ?? `ResolvedPersonaBundle.persona_status`
    - ?? persona lookup ?????????? `network_policy_region_anchor`
  - `src/network_identity/proxy_harvest.rs`
    - ?? `HarvestSourceRow` ??? harvest ?????? metadata ??
    - ??? harvest source ?? SQL ??????????
- ????????????????????
  - `resolve_persona_bundle`?harvest runtime??? DTO / API contract ?????
  - ?????? compile / test ????????????????
- ???????
  - `cargo test --tests` ??
  - `cargo build --release` ??
- ?? B-010 ???????? warning ?????????/???????
  - mode-aware legacy source ??????????
  - continuity ?????????????????????????????


## 2026-04-13 local correctness / status / geo internalization delta

- 本地控制面已把这轮 95+ 计划里的第一批 correctness 收口正式内生化到 Rust 主链路，不再只依赖脚本侧补丁：
  - `tasks` 已新增 typed 列：`proxy_id`、`requested_region`、`proxy_mode`
  - `/tasks` 创建、`verify_batch` 入队、`replenish` 入队都会显式写这三列
  - verify / replenish 在 claim 阶段已改成事务内显式 `UPDATE ... RETURNING`，避免“刚选中就被删”
- `/status.proxy_pool_status` 已补齐新的主展示口径：
  - `recent_hot_regions`
  - `recent_hot_region_counts`
  - `hot_region_window_seconds`
  - `source_concentration_top1_percent`
  - `source_concentration_top3_percent`
  - `active_sources_with_min_inventory`
  - `active_regions_with_min_inventory`
- `recent_hot_regions` 的语义已固定为：最近 600 秒内、同 mode 的 browser 任务，按 `requested_region` 聚合，默认 top 5；`region_shortages` 现在默认优先基于 recent hot 视窗，而不是只看当前 queued/running。
- proxy harvest source summary 已进入 runtime/status 主口径：
  - `declared_geo_quality`
  - `effective_geo_quality`
  - `geo_coverage_percent`
  - `active_region_count`
  - `active_country_count`
  - `active_share_percent`
- trust score 已明确区分 geo 来源质量：
  - `local_verify` 高于 `runner_verify`
  - `geoip_host_enrich` 仅作低权重兜底
  - `imported/manual/backfill` 不再与真实 verify 同权
- `scripts/prod_proxy_pool_hygiene.py` 已收口保护策略：
  - 不删除 `queued/running` 正在使用的 proxy
  - 不删除最近 N 秒内刚被 verify/browser 使用过的 proxy
  - 不删除仍有有效 sticky/session binding 的 proxy
- 本地验证状态已更新为：
  - `cargo test --test integration_api` 通过（125/125）
  - `cargo test --tests` 通过
  - `cargo build --release` 通过
- 当前本地剩余缺口已从“correctness 是否稳定”转为“远端 prod-live 数据面是否达到 95+ 验收线”：
  - 还需远端 30 分钟 `prod-live` 验收
  - 还需真实 private/paid provider 均衡度验证
  - 还需把 source concentration / geo coverage / continuity gate 继续固化进 release 报告

## 2026-04-13 release / longrun gating internalization delta

- `scripts/proxy_longrun_report.py` 已继续补齐 95+ 门禁所需字段：
  - `recent_hot_regions`
  - `recent_hot_region_count_max`
  - `source_concentration_top1_percent`
  - `source_concentration_top3_percent`
  - `browser_success_rate_percent`
  - `browser_proxy_not_found_failures`
  - `browser_proxy_claim_lost_failures`
  - `effective_geo_quality_summary`
- source quality artifact 已从只看 `expected_geo_quality` 升级为：
  - `declared_geo_quality_counts`
  - `effective_geo_quality_counts`
  - `avg/median/max_geo_coverage_percent`
  - `active_share_percent_by_effective_geo_quality`
- `proxy_real_longrun_driver.py` 的 browser 事件现在会额外带出：
  - `recent_hot_regions_during_request`
  - `browser_failure_signal`
  - `selection_reason_summary`
  - `error_message`
- `scripts/release_baseline_verify.sh` 的 `prod-live` 验收已接入更强门禁：
  - median `effective_active_ratio_percent`
  - median `promotion_rate`
  - `browser_success_rate_percent`
  - `browser_proxy_not_found_failures`
  - latest `recent_hot_regions`
  - `source_concentration_top1_percent`
  - 可选 `avg_geo_coverage_percent`
- 新增 / 收口的 release reason code：
  - `proxy_claim_lost`
  - `hot_region_window_empty`
  - `source_concentration_too_high`
  - `geo_coverage_too_low`
- 本地已完成脚本侧验证：
  - `python -m py_compile scripts/proxy_longrun_report.py scripts/proxy_real_longrun_driver.py`
  - `bash -n scripts/release_baseline_verify.sh`
  - `bash -n scripts/proxy_mainline_verify.sh`
  - `bash -n scripts/lightpanda_verify.sh`
  - 用合成 longrun 输入验证新增报告字段已真实落地

## 2026-04-13 continuity control-plane debt closeout delta

- 截至 **2026-04-13**，本地 Rust 控制面真实门槛已恢复为全绿：
  - `cargo check` ✅
  - `cargo build --release` ✅
  - `cargo test --lib` ✅
  - `cargo test --test integration_continuity_control_plane` ✅
  - `cargo test --tests` ✅
- “warning 未清”已不再是当前主债；本轮真实收口的是：
  - trust-score 单测断言与现行 SQL/runtime 规则重新对齐
  - `integration_api` 测试库路径改为 `UUID`，修复并发测试下的 SQLite DB 路径碰撞
  - SQLite 初始化补了 `busy_timeout + WAL`，减轻 worker / API 并发写锁抖动
- 小红书 `sample_ready` 已从“模板/seed 存在”升级为“运行时真实执行”：
  - canonical 小红书模板会在 fresh DB 初始化时幂等 bootstrap
  - `heartbeat` 对 `xiaohongshu + sample_ready` 正式走 `extract_text`
  - `/dashboard` 与 `/notes` 按 round-robin 轮转，不再永远只打第一条路径
  - continuity checks 已按 5 个基线信号生效：
    - `login_state`
    - `identity`
    - `region`
    - `dashboard`
    - `notes`
  - `login_state` / `region` 失败会进入显式断裂链路
  - `identity/dashboard/notes` 失败会落为普通 `heartbeat_failed(reason=continuity_check_failed)`
- 现有事件与快照已能沉淀 sample-ready probe 证据：
  - `continuity_events.event_json` 带 `probe_action / probe_path / passed_checks / failed_checks / evidence_summary`
  - `persona_health_snapshots.snapshot_json` 带
    - `last_continuity_check_results`
    - `continuity_check_success_ratio_24h`
    - `continuity_check_failed_count_24h`
    - `last_probe_action`
    - `last_probe_path`
- 当前主线下一步已切到：
  1. 继续打透 Shopify / 独立站后台
  2. 再补 Amazon / eBay
  3. 最后补 Walmart / TikTok Shop baseline readiness

## 2026-04-13 source / region balance internalization delta

- Rust 控制面已把 source / region 均衡策略接入 `verify_batch` 与 `replenish` 选池：
  - 对 underrepresented source（active < 5）优先
  - 对 underrepresented region（active < 5）优先
  - 当 top1 source 超过集中度目标时，优先过滤到非 overweight source
  - 在没有 target_region 时，仍优先 recent hot regions 的候选
- 当前实现位置：
  - `src/api/handlers.rs`
    - `load_active_proxy_balance_snapshot()`
    - `sort_balance_candidates()`
    - `select_replenish_candidate_rows()`
    - `verify_batch_proxies()`
- 这意味着控制面不再只是“看总量补货”，而开始按 active source / active region 实际分布做 mode-aware 候选排序。
- 本地新增回归：
  - `verify_batch_prioritizes_underrepresented_source_when_top1_is_concentrated`
  - `replenish_tick_global_prioritizes_underrepresented_source_candidates`
- 本地验证：
  - `cargo test --test integration_api` 通过（127/127）
  - `cargo test --tests` 通过
  - `cargo build --release` 通过

## 2026-04-13 provider balance + hygiene top1 source tightening delta

- Rust 控制面已继续把 **provider** 提升为一等均衡维度，而不再只剩 `max_per_provider` 硬 cap：
  - `ActiveProxyBalanceSnapshot` 新增：
    - `active_by_provider`
    - `top1_provider_key`
  - `verify_batch` / `replenish` 的 SQL 预排序现在会感知 active provider inventory
  - `sort_balance_candidates()` 现在会：
    - 优先 underrepresented provider（active < 5）
    - 在存在替代时下压 overweight provider
    - 与既有 source / region / recent-hot-region 规则叠加生效
- 本地新增 provider balance 回归：
  - `verify_batch_prioritizes_underrepresented_provider_when_source_balanced`
  - `replenish_tick_global_prioritizes_underrepresented_provider_candidates`
- `scripts/prod_proxy_pool_hygiene.py` 已新增 **source concentration-aware** compaction：
  - summary 增加：
    - `top1_source_label`
    - `top1_source_active_count`
    - `source_concentration_top1_percent`
  - `source_actions` 现会带出：
    - `candidate_keep_limit_base`
    - `candidate_keep_limit`
    - `candidate_keep_adjustment`
    - `source_active_share_percent`
  - `apply_result` 现会带出：
    - `deleted_proxy_rows_by_source`
  - 当 top1 source concentration 超过阈值时：
    - dominant source 的 `candidate_keep_limit` 会被收紧
    - underrepresented non-top1 source 会得到更宽松的 candidate 保留
- 本地新增脚本回归：
  - `scripts/tests/test_prod_proxy_pool_hygiene.py`
- 本轮顺手收口了一处既有测试漂移：
  - `runner::engine` sample-ready 默认 identity marker 顺序调整为优先 `seller`
  - `evaluate_sample_ready_continuity_probe_reports_dashboard_success` 恢复稳定通过
- 本地验证现已更新到：
  - `cargo test --test integration_api` 通过（129/129）
  - `cargo test --tests` 通过
  - `cargo build --release` 通过
  - `python -m unittest discover -s scripts/tests -p "test_*.py"` 通过（8/8）
  - `python -m py_compile scripts/prod_proxy_pool_hygiene.py scripts/release_prod_live_gate.py scripts/proxy_longrun_report.py scripts/proxy_real_longrun_driver.py` 通过
  - `bash -n scripts/release_baseline_verify.sh` 通过
  - `bash -n scripts/proxy_mainline_verify.sh` 通过
  - `bash -n scripts/lightpanda_verify.sh` 通过
- 因此本地剩余主项继续从“控制面缺口”收缩为“远端真实数据面验收”：
  - 远端 `prod_live` 30 分钟实跑
  - source/provider concentration 是否真实下降
  - hygiene top1 source keep-cap 是否足以把 dominant source 压回目标线
  - 若想冲到 95+，仍需至少 2 个独立 private/paid provider 的真实供给

## 2026-04-13 xiaohongshu sample_ready identity continuity delta

- 小红书 `sample_ready` 现在不再只是“平台级通用 marker”：
  - `platform_templates.identity_markers_json`
  - `store_platform_overrides.identity_markers_json`
  已进入 schema、幂等迁移、DTO、CRUD 与 persona 解析链路。
- 运行时行为已收口为：
  - 店铺 override 提供 `identity_markers_json` 时，heartbeat / continuity probe 会优先使用店铺级 marker；
  - 小红书 continuity evaluator 对已配置 marker 走**严格匹配**，不再回退到泛化 host 命中；
  - `dashboard` / `notes` 仍保留 alias 级路径兼容，但 identity 不再只靠默认泛词。
- continuity 证据链已补全：
  - `continuity_check_result` 会输出 `matched_identity_marker`、`skipped_checks`
  - `continuity_events.event_json` 会带 `matched_identity_marker`
  - `persona_health_snapshots.snapshot_json.last_continuity_check_results` 现已支持对最近窗口中的非空 `matched_identity_marker` 做回填，不再因为最新 probe 行缺值而丢掉身份命中证据
- 本轮还修掉了一个真实接口缺陷：
  - `create_store_platform_override` 的 SQL placeholder 数量漂移已修复，`identity_markers_json` 现在可以通过 API 正常写入
- 本轮新增并已通过的回归覆盖：
  - `platform_template_crud_roundtrips_identity_markers_json`
  - `store_platform_override_crud_roundtrips_identity_markers_json`
  - `xiaohongshu_store_identity_markers_flow_into_probe_event_and_snapshot`
- 当前本地真实门槛：
  - `cargo check` ✅
  - `cargo build --release` ✅
  - `cargo test --lib` ✅
  - `cargo test --test integration_continuity_control_plane` ✅
  - `cargo test --tests` ✅

## 2026-04-14 prod-live real-live driver compatibility + gate delta

- 2026-04-14 已确认并修复远端 real-live 主 blocker：
  - 远端 `/home/ubuntu/SelfMadeprojects/lightpanda-automation/scripts/proxy_real_longrun_driver.py` 曾停留在旧 mixed-workload 版本；
  - 该版本硬依赖 `/behavior-profiles`，在当前 control plane 上会直接报：
    - `live control plane does not expose /behavior-profiles yet`
  - 现已把远端 `proxy_real_longrun_driver.py` / `proxy_mainline_verify.sh` 收口回兼容主线，`bash scripts/proxy_mainline_verify.sh real-live` 已重新可用。
- 2026-04-14 已继续把 real-live continuity 验证做强，不再只靠“隔一轮碰运气”：
  - `scripts/proxy_real_longrun_driver.py` 新增：
    - `stateful_followup_count`
    - `sticky_stateful_region`
    - `PROXY_REAL_LONGRUN_HYGIENE_EXTRA_ARGS`
  - `scripts/proxy_longrun_report.py` 现将 `stateful_followup*` 也并入 stateful browser 统计。
- 2026-04-14 远端短 real-live（120 秒）现已稳定满足：
  - `browser_success_rate_percent = 100%`
  - `browser_proxy_not_found_failures = 0`
  - `recent_hot_regions = ['ap-southeast', 'cn', 'eu-west']`
  - `stateful_continuity_observed = true`
  - `cookie/local/session storage restore + persist` 均为正
- 2026-04-14 远端继续用 aggressive hygiene keep-cap 实测：
  - `effective_active_ratio_percent median = 43.1174`
  - `promotion_rate median = 0.7802`（即 **78.02%**）
  - `reject_rate median = 0.2198`
  - `eligible_pool_total = 494`
  - `candidate = 228`
  - `candidate_rejected = 53`
- 2026-04-14 还修掉一个真实发布门禁口径 bug：
  - `scripts/release_prod_live_gate.py` 之前把 longrun 报告中的 `promotion_rate=0.7802` 错当成 `0.7802%`
  - 现已统一支持 `0~1` 比例值与 `0~100` 百分比值两种输入
  - 新增脚本回归：fractional `promotion_rate` 也必须正确通过 gate 计算
- 截至 2026-04-14，最新远端 prod-live gate 结论已缩到**单一真实剩余项**：
  - `source_concentration_too_high`
  - 当前 `source_concentration_top1_percent = 87.79`
  - 这意味着本轮 95+ 计划里，**continuity / hot-regions / ratio / promotion / browser success / gate 口径** 已基本收口；
  - 代理质量继续冲高的主瓶颈已经不再是控制面或脚本，而是 **真实 source/provider 供给结构仍被 `github_speedx_http` 主导**。

## 2026-04-14 xiaohongshu precision hardening delta

- 小红书 `sample_ready` 这轮继续只做“精度增强”，没有再扩新平台：
  - `detect_login_loss_signal()` 已收紧为只看页面可见证据：
    - `final_url`
    - `title`
    - `text_preview`
    - `content_preview`
    - `html_preview`
    - `message / error_message`
  - 不再扫描整份 `result_json`，避免被配置回显或 payload 元数据误伤出 `login_risk_detected`
- identity marker 审计证据已进一步补强：
  - `continuity_check_result` 现在正式带：
    - `configured_identity_markers`
    - `identity_markers_source`
  - `continuity_events.event_json` 已同步沉淀这两个字段
  - `persona_health_snapshots.snapshot_json.last_continuity_check_results` 也已同步沉淀这两个字段
- 小红书 store-level continuity 回归已更强：
  - 现在不仅断言 `matched_identity_marker`
  - 也断言 payload / probe / event / snapshot 四层都能看到：
    - `identity_markers_source = store_override`
    - `configured_identity_markers` 包含店铺 marker
- 本轮新增并已通过的精度回归：
  - `evaluate_sample_ready_continuity_probe_does_not_trigger_login_from_config_echo`
  - `xiaohongshu_store_identity_markers_flow_into_probe_event_and_snapshot`（增强断言）
- 2026-04-14 本地真实门槛：
  - `cargo check` ✅
  - `cargo build --release` ✅
  - `cargo test --lib` ✅（62/62）
  - `cargo test --test integration_continuity_control_plane` ✅（8/8）
  - `cargo test --tests` ✅

## 2026-04-14 prod-live source balance + real-live evidence gate delta

- Runtime (`src/runner/engine.rs`) now applies **prod_live-only** source-balance preference on auto proxy selection.
- Hard constraints stay unchanged (`active/cooldown/provider-region/min_score/mode eligibility`).
- If top1 source is overloaded and non-top1 candidates are close in trust score, auto selection prefers non-overloaded source.
- If alternatives are clearly worse, top1 source is retained with explicit fallback reason.
- Explicit proxy / sticky reuse / continuity-bound session paths are not re-ranked by source-balance logic.
- Selection evidence now includes:
  - `selected_source_label`
  - `selected_source_active_share_percent`
  - `source_balance_applied`
  - `source_balance_fallback_reason`
  - `source_balance_top1_source`
  - `source_balance_top1_source_active_share_percent`
- Longrun report now emits third artifact:
  - `reports/real_live_evidence_summary_latest.txt`
  - `reports/real_live_evidence_summary_latest.json`
- `release_prod_live_gate.py` now prefers stable summary fields (recent hot-region union + source concentration max/median/latest) over latest-only sampling noise.
- New independent real-live gate:
  - `scripts/release_real_live_gate.py <report_file>`
  - checks browser success, continuity observed, continuity chain, storage evidence, no claim-lost, and minimum sample volume.
- `scripts/proxy_mainline_verify.sh real-live` now prints evidence artifact paths and real-live gate result.

## 2026-04-14 prod-live 稳态 preset / 双结论 gate delta

- 已新增单一真源 preset helper：
  - `scripts/prod_live_presets.py`
  - 当前固定支持：
    - `legacy`
    - `stable_v1`
- `stable_v1` 已把当前验证有效的一组稳态参数正式收口进 repo：
  - hygiene keep-cap / rejected-cap
  - `stateful_followup_count = 1`
  - `auto_browser_regions_from_db = true`
  - `geo_enrich = enabled`
  - `pool_hygiene = enabled`
- `scripts/proxy_real_longrun_driver.py` 现已接入 preset：
  - 新增 `--preset`
  - raw payload 固定带出 `preset`
  - `stateful_followup_count / auto_browser_regions_from_db / geo_enrich / pool_hygiene / pool_hygiene_extra_args`
    现按 `命令行 > env override > preset` 解析
- `scripts/proxy_mainline_verify.sh` 现已接入：
  - `PROXY_VERIFY_REAL_PRESET=legacy|stable_v1`
  - `real-live` / `prod-live` 两条链路都从同一 preset 读取 driver 默认值
- 已新增 repo-owned maintenance 入口：
  - `scripts/prod_live_maintenance_tick.sh --preset <name>`
  - 该入口与 longrun 共享同一 preset 参数集来执行：
    - `prod_proxy_geo_enrich.py`
    - `prod_proxy_pool_hygiene.py`
- `scripts/release_prod_live_gate.py` 现已升级为双结论模型：
  - `strict_verdict`
  - `operational_verdict`
  - `provider_supply_class`
  - `provider_cap_reason`
- 当前 gate 语义已固定为：
  - 严格结论继续按 95+ 阈值判
  - 若当前 supply class 为 `lab_only` 且唯一失败项是 `source_concentration_too_high`
    则：
    - `strict_verdict = fail`
    - `operational_verdict = provider_capped`
- `scripts/proxy_longrun_report.py` / `scripts/release_report_summary.py` 现已固定透出：
  - `preset`
  - `provider_supply_class`
  - `strict_verdict`
  - `operational_verdict`
  - `provider_cap_reason`
- 本地脚本级验证已通过：
  - `python -m py_compile scripts/prod_live_presets.py scripts/proxy_real_longrun_driver.py scripts/proxy_longrun_report.py scripts/release_prod_live_gate.py scripts/release_report_summary.py`
  - `python -m unittest discover -s scripts/tests -p "test_*.py"` -> `29/29`
  - `bash -n scripts/proxy_mainline_verify.sh`
  - `bash -n scripts/prod_live_maintenance_tick.sh`
  - `bash -n scripts/release_baseline_verify.sh`

## 2026-04-14 prod-live acceptance gate decoupling delta

- `scripts/proxy_mainline_verify.sh` now has a dedicated `prod-live` mode:
  - runs longrun driver in forced `prod_live` mode
  - evaluates with `scripts/release_prod_live_gate.py`
  - prints source/session/evidence artifact paths and prod-live gate result
- `scripts/release_baseline_verify.sh --profile prod-live` now calls:
  - `bash scripts/proxy_mainline_verify.sh prod-live`
  instead of piggybacking on `real-live` gate execution.
- `release_prod_live_gate.py` min sample threshold is now exposed in baseline via:
  - `RELEASE_VERIFY_PROD_LIVE_MIN_SAMPLE_COUNT`
- result:
  - prod-live acceptance and real-live evidence are now two independent gate paths with non-overlapping failure reasons.

## 2026-04-14 release summary artifact unification delta

- `scripts/release_report_summary.py` is now the single helper that renders release-facing summary fields from:
  - longrun report JSON
  - `source_quality_summary_latest.json`
  - `session_continuity_summary_latest.json`
  - `real_live_evidence_summary_latest.json`
- `scripts/release_baseline_verify.sh` no longer embeds ad-hoc Python for prod-live metrics.
- release report now directly carries:
  - summary artifact paths
  - prod-live gate verdict
  - source quality key fields
  - session continuity key fields
  - real-live evidence readiness fields
- stale prod-live report bleed-through into non-prod release profiles is now removed:
  - public-smoke / gateway-upstream release reports no longer append historical prod-live metrics just because an old longrun JSON exists on disk.

## 2026-04-14 real-live host-level evidence delta

- `D:\SelfMadeTool\personal\lightpanda-work\scripts\proxy_longrun_report.py` 已继续把 real-live 证据从“只有总量摘要”补到“可按站点 host 审计”：
  - 新增 `extract_event_host()`
  - 新增 `build_site_host_evidence()`
  - `build_real_live_evidence_summary()` 现会汇总 host 级证据
- `real_live_evidence_summary_latest.json|txt` 现新增并固定输出：
  - `site_host_count`
  - `stateful_site_host_count`
  - `continuity_ready_site_host_count`
  - `continuity_ready_site_hosts`
  - `site_host_summaries`
- `site_host_summaries` 现可直接回答：
  - 哪些 host 真实跑过
  - 哪些 host 观测到 stateful primary / followup
  - 哪些 host 已出现 `auto_created -> auto_reused + storage restore`
  - 每个 host 的请求 region / recent hot region / distinct proxy 数
- `D:\SelfMadeTool\personal\lightpanda-work\scripts\release_report_summary.py` 现已把 host-level evidence 继续带到 release 摘要：
  - `real_live_site_host_count`
  - `real_live_stateful_site_host_count`
  - `real_live_continuity_ready_site_host_count`
  - `real_live_continuity_ready_site_hosts`
- 本地验证已再次收口：
  - `python -m py_compile scripts/proxy_longrun_report.py scripts/release_report_summary.py scripts/release_prod_live_gate.py scripts/release_real_live_gate.py scripts/proxy_real_longrun_driver.py`
  - `python -m unittest discover -s scripts/tests -p "test_*.py"` -> `20/20`
  - `bash -n scripts/proxy_mainline_verify.sh`
  - `bash -n scripts/release_baseline_verify.sh`
  - `cargo test --tests`
  - `cargo build --release`
- 当前结论：
  - real-live 证据链的**可审计性**已明显增强
  - 但 `92+` 的“真实白名单长期实站分”仍取决于远端 30 分钟报告是否产出足量 host-level 样本，而不只是本地脚本契约完整

## 2026-04-14 prod-live legacy-status compatibility + provider-capped remote verification

- 本轮继续收口了一个真实远端兼容坑：当前在线 `127.0.0.1:3000` control plane 进程环境已是 `AUTO_OPEN_BROWSER_PROXY_MODE=prod_live`，但 `/status` 仍停留在旧 contract，缺少：
  - `mode`
  - `recent_hot_regions`
  - `source_concentration_top1_percent`
- 为避免旧 contract 把真实 `prod_live` 长跑误判成 `mode_guard` 或把 concentration 假算成 `0`，脚本层已补兼容：
  - `preflight_release_env.sh`
    - 当 `/status.mode` 与 `proxy_pool_status.mode` 为空时，回退读取 `port 3000` 进程环境中的 `AUTO_OPEN_BROWSER_PROXY_MODE`
  - `proxy_longrun_report.py`
    - 当 status snapshot 缺 `mode` 时，回退读取 driver raw payload 的 `mode`
    - 当 status snapshot 缺 `recent_hot_regions` 时，回退读取 browser event 中的 `recent_hot_regions_during_request`
    - 当旧 status 缺 concentration 字段时，改为从 `proxy_harvest_metrics.source_summaries[].active_count` 反推出 `source_concentration_top1_percent/top3_percent`
  - `proxy_real_longrun_driver.py`
    - 当旧 `/status` 在请求期间缺 `recent_hot_regions` 时，浏览器事件会回填本次 `requested_region` 作为兼容证据
- 本地脚本回归现已更新并通过：
  - `python -m unittest discover -s scripts/tests -p "test_*.py"` -> `33/33`
  - `python -m py_compile scripts/proxy_longrun_report.py scripts/proxy_real_longrun_driver.py scripts/release_prod_live_gate.py`
  - `bash -n scripts/preflight_release_env.sh`
  - `bash -n scripts/proxy_mainline_verify.sh`
- 远端已完成 `stable_v1` maintenance 实跑：
  - `active_total = 362`
  - `candidate_total = 260`
  - `rejected_total = 43`
  - `estimated_effective_active_ratio_percent = 54.4361`
  - `source_concentration_top1_percent = 87.8453`
- 远端随后完成 `120s stable_v1 prod-live` 短验收（`PROXY_VERIFY_REAL_STATUS_INTERVAL_SECONDS=20`）：
  - `summary.mode = prod_live`
  - `sample_count = 7`
  - `effective_active_ratio_percent median = 54.0`
  - `promotion_rate median = 88.51%`
  - `browser_success_rate_percent = 100.0`
  - `browser_proxy_not_found_failures = 0`
  - `recent_hot_regions_union = ap-southeast, cn, eu-west`
  - `strict_verdict = fail`
  - `operational_verdict = provider_capped`
  - `provider_cap_reason = source_concentration_too_high`
- 这说明“无 private/paid provider 阶段”的核心目标已达到：
  - 严格 95+ gate 没有被放松
  - 当前在线链路可以稳定给出 `lab_only + concentration fail => provider_capped`
  - 当前剩余主问题仍然是**真实 source 供给过度集中**，不是脚本、预检或 continuity 口径错误
