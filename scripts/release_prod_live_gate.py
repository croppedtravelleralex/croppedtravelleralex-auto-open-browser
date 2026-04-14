#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from prod_live_presets import infer_provider_supply_class_from_report


@dataclass(frozen=True)
class Thresholds:
    min_sample_count: int = 0
    min_effective_ratio_percent: float = 35.0
    min_promotion_rate_percent: float = 75.0
    min_browser_success_rate_percent: float = 98.0
    min_recent_hot_regions: int = 3
    max_source_top1_percent: float = 75.0
    min_geo_coverage_percent: float = 0.0


@dataclass(frozen=True)
class GateResult:
    strict_verdict: str
    operational_verdict: str
    provider_supply_class: str
    provider_cap_reason: str | None
    reason_code: str
    failure_scope: str
    detail: str

    def to_dict(self) -> dict[str, object]:
        return asdict(self)


def _to_float(value: object, default: float = 0.0) -> float:
    try:
        return float(value)  # type: ignore[arg-type]
    except (TypeError, ValueError):
        return default


def _to_int(value: object, default: int = 0) -> int:
    try:
        return int(value)  # type: ignore[arg-type]
    except (TypeError, ValueError):
        return default


def _normalize_percent_like(value: object) -> float:
    numeric = _to_float(value)
    if 0.0 <= numeric <= 1.0:
        return round(numeric * 100.0, 4)
    return numeric


def _normalized_str_list(value: object) -> list[str]:
    if not isinstance(value, list):
        return []
    results: list[str] = []
    for item in value:
        text = str(item or "").strip()
        if text:
            results.append(text)
    return results


def _normalized_mapping_keys(value: object) -> list[str]:
    if not isinstance(value, dict):
        return []
    results: list[str] = []
    for key in value.keys():
        text = str(key or "").strip()
        if text:
            results.append(text)
    return sorted(results)


def _evaluate_strict(
    payload: dict[str, object],
    thresholds: Thresholds,
) -> tuple[str, str, str]:
    summary = dict(payload.get("summary") or {})
    source_quality_summary = dict(payload.get("source_quality_summary") or {})
    latest = dict(summary.get("latest") or {})
    trend = dict(summary.get("trend_flags") or {})
    event_summary = dict(summary.get("event_summary") or {})
    mode = str(payload.get("mode") or summary.get("mode") or latest.get("mode") or "")
    sample_count = _to_int(summary.get("sample_count"))
    pool_active = _to_int(latest.get("pool_active"))
    stateful_continuity = bool(trend.get("stateful_continuity_observed"))
    active_pool_ever_positive = bool(trend.get("active_pool_ever_positive"))
    effective_ratio_median = _to_float(
        dict(summary.get("effective_active_ratio_percent") or {}).get("median")
    )
    promotion_rate_median = _normalize_percent_like(
        dict(summary.get("promotion_rate") or {}).get("median")
    )
    browser_success_rate_percent = _to_float(summary.get("browser_success_rate_percent"))
    proxy_claim_lost_failures = _to_int(
        event_summary.get("browser_proxy_claim_lost_failures")
    )
    proxy_not_found_failures = _to_int(
        event_summary.get("browser_proxy_not_found_failures")
    )
    recent_hot_regions_latest = _normalized_str_list(latest.get("recent_hot_regions"))
    recent_hot_regions_union = _normalized_str_list(summary.get("recent_hot_regions_union"))
    event_recent_hot_regions = _normalized_mapping_keys(
        event_summary.get("browser_recent_hot_regions_observed")
    )
    requested_regions = _normalized_mapping_keys(
        event_summary.get("browser_requested_regions")
    )
    effective_recent_hot_regions = (
        recent_hot_regions_latest
        or recent_hot_regions_union
        or event_recent_hot_regions
        or requested_regions
    )
    recent_hot_region_evidence = "latest"
    if effective_recent_hot_regions is recent_hot_regions_union:
        recent_hot_region_evidence = "summary_union"
    elif effective_recent_hot_regions is event_recent_hot_regions:
        recent_hot_region_evidence = "event_recent_hot_regions"
    elif effective_recent_hot_regions is requested_regions:
        recent_hot_region_evidence = "event_requested_regions"
    source_top1_percent_latest = _to_float(
        latest.get("source_concentration_top1_percent"),
        _to_float(
            dict(summary.get("source_concentration_top1_percent") or {}).get("median")
        ),
    )
    source_top1_percent_max = _to_float(
        dict(summary.get("source_concentration_top1_percent") or {}).get("max"),
        source_top1_percent_latest,
    )
    source_top1_percent = max(source_top1_percent_latest, source_top1_percent_max)
    effective_geo_quality_summary = dict(
        source_quality_summary.get("effective_geo_quality_summary")
        or latest.get("effective_geo_quality_summary")
        or {}
    )
    avg_geo_coverage_percent = _to_float(
        effective_geo_quality_summary.get("avg_geo_coverage_percent")
    )

    if thresholds.min_sample_count > 0 and sample_count < thresholds.min_sample_count:
        return (
            "sample_insufficient",
            "prod_live_sample",
            f"sample_count={sample_count} < {thresholds.min_sample_count}",
        )
    if mode != "prod_live":
        return (
            "preflight_failed",
            "mode_guard",
            f"prod-live report mode mismatch: {mode or '<empty>'}",
        )
    if not active_pool_ever_positive or pool_active <= 0:
        return (
            "no_active_proxy",
            "proxy_pool",
            f"prod-live active pool missing: latest.pool_active={pool_active}",
        )
    if not stateful_continuity:
        return (
            "continuity_not_observed",
            "continuity",
            "prod-live continuity chain not observed in longrun report",
        )
    if effective_ratio_median < thresholds.min_effective_ratio_percent:
        return (
            "effective_ratio_too_low",
            "proxy_pool",
            "median_effective_active_ratio_percent="
            f"{effective_ratio_median} < {thresholds.min_effective_ratio_percent}",
        )
    if promotion_rate_median < thresholds.min_promotion_rate_percent:
        return (
            "promotion_rate_too_low",
            "proxy_pool",
            f"median_promotion_rate={promotion_rate_median} < {thresholds.min_promotion_rate_percent}",
        )
    if browser_success_rate_percent < thresholds.min_browser_success_rate_percent:
        return (
            "browser_open_failed",
            "browser_verify",
            "browser_success_rate_percent="
            f"{browser_success_rate_percent} < {thresholds.min_browser_success_rate_percent}",
        )
    if proxy_claim_lost_failures > 0:
        return (
            "proxy_claim_lost",
            "proxy_pool",
            f"browser_proxy_claim_lost_failures={proxy_claim_lost_failures}",
        )
    if proxy_not_found_failures > 0:
        return (
            "proxy_claim_lost",
            "proxy_pool",
            f"browser_proxy_not_found_failures={proxy_not_found_failures}",
        )
    if len(effective_recent_hot_regions) < thresholds.min_recent_hot_regions:
        return (
            "hot_region_window_empty",
            "hot_regions",
            "effective_recent_hot_regions="
            f"{len(effective_recent_hot_regions)} "
            f"latest_recent_hot_regions={len(recent_hot_regions_latest)} "
            f"union_recent_hot_regions={len(recent_hot_regions_union)} "
            f"event_recent_hot_regions={len(event_recent_hot_regions)} "
            f"requested_regions={len(requested_regions)} "
            f"evidence={recent_hot_region_evidence} "
            f"threshold={thresholds.min_recent_hot_regions}",
        )
    if source_top1_percent > thresholds.max_source_top1_percent:
        return (
            "source_concentration_too_high",
            "proxy_pool",
            "source_concentration_top1_percent_max="
            f"{source_top1_percent} > {thresholds.max_source_top1_percent}",
        )
    if (
        thresholds.min_geo_coverage_percent > 0
        and avg_geo_coverage_percent < thresholds.min_geo_coverage_percent
    ):
        return (
            "geo_coverage_too_low",
            "proxy_pool",
            "avg_geo_coverage_percent="
            f"{avg_geo_coverage_percent} < {thresholds.min_geo_coverage_percent}",
        )
    return (
        "ok",
        "none",
        "prod-live acceptance passed: "
        f"sample_count={sample_count} "
        f"latest.pool_active={pool_active} "
        f"median_effective_active_ratio_percent={effective_ratio_median} "
        f"median_promotion_rate={promotion_rate_median} "
        f"browser_success_rate_percent={browser_success_rate_percent} "
        f"recent_hot_regions={len(effective_recent_hot_regions)} "
        f"source_concentration_top1_percent={source_top1_percent} "
        f"avg_geo_coverage_percent={avg_geo_coverage_percent}",
    )


def evaluate_report_payload(
    payload: dict[str, object],
    thresholds: Thresholds,
) -> GateResult:
    provider_supply_class = infer_provider_supply_class_from_report(payload)
    reason_code, failure_scope, detail = _evaluate_strict(payload, thresholds)
    strict_verdict = "pass" if reason_code == "ok" else "fail"
    operational_verdict = "pass" if reason_code == "ok" else "fail"
    provider_cap_reason: str | None = None

    if (
        reason_code == "source_concentration_too_high"
        and provider_supply_class == "lab_only"
    ):
        operational_verdict = "provider_capped"
        provider_cap_reason = reason_code

    return GateResult(
        strict_verdict=strict_verdict,
        operational_verdict=operational_verdict,
        provider_supply_class=provider_supply_class,
        provider_cap_reason=provider_cap_reason,
        reason_code=reason_code,
        failure_scope=failure_scope,
        detail=detail,
    )


def evaluate_report(
    payload: dict[str, object],
    thresholds: Thresholds,
) -> tuple[str, str, str]:
    result = evaluate_report_payload(payload, thresholds)
    return result.reason_code, result.failure_scope, result.detail


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Evaluate prod-live longrun report against release gate thresholds.",
    )
    parser.add_argument("report_file", help="Path to proxy_real_longrun_latest.json")
    parser.add_argument(
        "--json",
        action="store_true",
        help="Print structured gate result JSON instead of tab-delimited text",
    )
    parser.add_argument(
        "--min-sample-count",
        type=int,
        default=0,
    )
    parser.add_argument(
        "--min-effective-ratio-percent",
        type=float,
        default=35.0,
    )
    parser.add_argument(
        "--min-promotion-rate-percent",
        type=float,
        default=75.0,
    )
    parser.add_argument(
        "--min-browser-success-rate-percent",
        type=float,
        default=98.0,
    )
    parser.add_argument(
        "--min-recent-hot-regions",
        type=int,
        default=3,
    )
    parser.add_argument(
        "--max-source-top1-percent",
        type=float,
        default=75.0,
    )
    parser.add_argument(
        "--min-geo-coverage-percent",
        type=float,
        default=0.0,
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report_path = Path(args.report_file)
    if not report_path.exists():
        result = GateResult(
            strict_verdict="fail",
            operational_verdict="fail",
            provider_supply_class="lab_only",
            provider_cap_reason=None,
            reason_code="preflight_failed",
            failure_scope="prod_live_report",
            detail="prod-live report missing",
        )
        if args.json:
            print(json.dumps(result.to_dict(), ensure_ascii=False, indent=2))
        else:
            print(f"{result.reason_code}\t{result.failure_scope}\t{result.detail}")
        return 1

    payload = json.loads(report_path.read_text(encoding="utf-8"))
    result = evaluate_report_payload(
        payload,
        Thresholds(
            min_sample_count=args.min_sample_count,
            min_effective_ratio_percent=args.min_effective_ratio_percent,
            min_promotion_rate_percent=args.min_promotion_rate_percent,
            min_browser_success_rate_percent=args.min_browser_success_rate_percent,
            min_recent_hot_regions=args.min_recent_hot_regions,
            max_source_top1_percent=args.max_source_top1_percent,
            min_geo_coverage_percent=args.min_geo_coverage_percent,
        ),
    )
    if args.json:
        print(json.dumps(result.to_dict(), ensure_ascii=False, indent=2))
    else:
        print(f"{result.reason_code}\t{result.failure_scope}\t{result.detail}")
    return 0 if result.strict_verdict == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
