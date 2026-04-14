#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

import release_prod_live_gate


def _load_optional_json(path_str: str | None) -> dict[str, Any]:
    if not path_str:
        return {}
    path = Path(path_str)
    if not path.exists():
        return {}
    return json.loads(path.read_text(encoding="utf-8"))


def _path_text(path_str: str | None) -> str:
    if not path_str:
        return ""
    return str(Path(path_str))


def _to_bool(value: Any) -> bool:
    if isinstance(value, bool):
        return value
    if isinstance(value, (int, float)):
        return value != 0
    if isinstance(value, str):
        return value.strip().lower() in {"1", "true", "yes", "on"}
    return False


def _to_float(value: Any, default: float = 0.0) -> float:
    try:
        return float(value)
    except (TypeError, ValueError):
        return default


def _to_int(value: Any, default: int = 0) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def summarize_release_payload(
    payload: dict[str, Any],
    thresholds: release_prod_live_gate.Thresholds,
    report_json_path: str | None = None,
    source_summary_json_path: str | None = None,
    session_summary_json_path: str | None = None,
    real_live_evidence_json_path: str | None = None,
    source_summary: dict[str, Any] | None = None,
    session_summary: dict[str, Any] | None = None,
    real_live_evidence_summary: dict[str, Any] | None = None,
) -> dict[str, Any]:
    source_summary = dict(source_summary or payload.get("source_quality_summary") or {})
    session_summary = dict(
        session_summary or payload.get("session_continuity_summary") or {}
    )
    real_live_evidence_summary = dict(
        real_live_evidence_summary or payload.get("real_live_evidence_summary") or {}
    )
    summary = dict(payload.get("summary") or {})
    latest = dict(summary.get("latest") or {})
    trend_flags = dict(summary.get("trend_flags") or {})
    if "stateful_continuity_observed" not in trend_flags and session_summary:
        trend_flags["stateful_continuity_observed"] = _to_bool(
            session_summary.get("stateful_continuity_observed")
        )
    if "session_reuse_observed" not in trend_flags and session_summary:
        trend_flags["session_reuse_observed"] = _to_bool(
            session_summary.get("session_reuse_observed")
        )
    enriched_summary = dict(summary)
    enriched_summary["trend_flags"] = trend_flags
    enriched_payload = dict(payload)
    enriched_payload["summary"] = enriched_summary

    gate_result = release_prod_live_gate.evaluate_report_payload(
        enriched_payload, thresholds
    )

    effective_geo_quality_summary = dict(
        source_summary.get("effective_geo_quality_summary") or {}
    )
    source_top1_summary = dict(summary.get("source_concentration_top1_percent") or {})

    return {
        "report_json_path": _path_text(report_json_path),
        "source_quality_summary_json_path": _path_text(source_summary_json_path),
        "session_continuity_summary_json_path": _path_text(session_summary_json_path),
        "real_live_evidence_summary_json_path": _path_text(real_live_evidence_json_path),
        "summary_mode": str(summary.get("mode") or latest.get("mode") or ""),
        "summary_sample_count": _to_int(summary.get("sample_count")),
        "prod_live_preset": str(payload.get("preset") or summary.get("preset") or ""),
        "prod_live_gate_reason_code": gate_result.reason_code,
        "prod_live_gate_failure_scope": gate_result.failure_scope,
        "prod_live_gate_detail": gate_result.detail,
        "prod_live_provider_supply_class": gate_result.provider_supply_class,
        "prod_live_strict_verdict": gate_result.strict_verdict,
        "prod_live_operational_verdict": gate_result.operational_verdict,
        "prod_live_provider_cap_reason": gate_result.provider_cap_reason or "",
        "prod_live_effective_active_ratio_percent_median": _to_float(
            dict(summary.get("effective_active_ratio_percent") or {}).get("median")
        ),
        "prod_live_promotion_rate_median": _to_float(
            dict(summary.get("promotion_rate") or {}).get("median")
        ),
        "prod_live_browser_success_rate_percent": _to_float(
            summary.get("browser_success_rate_percent")
        ),
        "prod_live_recent_hot_regions": latest.get("recent_hot_regions")
        or summary.get("recent_hot_regions_union")
        or [],
        "prod_live_source_concentration_top1_percent": _to_float(
            latest.get("source_concentration_top1_percent"),
            _to_float(source_top1_summary.get("median")),
        ),
        "prod_live_source_concentration_top1_percent_max": _to_float(
            source_top1_summary.get("max")
        ),
        "prod_live_browser_proxy_claim_lost_failures": _to_int(
            dict(summary.get("event_summary") or {}).get("browser_proxy_claim_lost_failures")
        ),
        "prod_live_browser_proxy_not_found_failures": _to_int(
            dict(summary.get("event_summary") or {}).get("browser_proxy_not_found_failures")
        ),
        "source_quality_source_count": _to_int(source_summary.get("source_count")),
        "source_quality_healthy_source_count": _to_int(
            source_summary.get("healthy_source_count")
        ),
        "source_quality_controlled_source_count": _to_int(
            source_summary.get("controlled_source_count")
        ),
        "source_quality_effective_geo_quality_counts": dict(
            source_summary.get("effective_geo_quality_counts") or {}
        ),
        "source_quality_avg_geo_coverage_percent": _to_float(
            effective_geo_quality_summary.get("avg_geo_coverage_percent")
        ),
        "session_continuity_level": str(session_summary.get("continuity_level") or ""),
        "session_stateful_continuity_observed": _to_bool(
            session_summary.get("stateful_continuity_observed")
        ),
        "session_continuity_chain_observed": _to_bool(
            session_summary.get("continuity_chain_observed")
        ),
        "session_storage_restored_observed": _to_bool(
            session_summary.get("storage_restored_observed")
        ),
        "session_storage_persisted_observed": _to_bool(
            session_summary.get("storage_persisted_observed")
        ),
        "real_live_evidence_ready": _to_bool(
            real_live_evidence_summary.get("evidence_ready")
        ),
        "real_live_sample_sufficient": _to_bool(
            real_live_evidence_summary.get("sample_sufficient")
        ),
        "real_live_browser_total": _to_int(real_live_evidence_summary.get("browser_total")),
        "real_live_browser_success_rate_percent": _to_float(
            real_live_evidence_summary.get("browser_success_rate_percent")
        ),
        "real_live_continuity_chain_observed": _to_bool(
            real_live_evidence_summary.get("continuity_chain_observed")
        ),
        "real_live_site_host_count": _to_int(
            real_live_evidence_summary.get("site_host_count")
        ),
        "real_live_stateful_site_host_count": _to_int(
            real_live_evidence_summary.get("stateful_site_host_count")
        ),
        "real_live_continuity_ready_site_host_count": _to_int(
            real_live_evidence_summary.get("continuity_ready_site_host_count")
        ),
        "real_live_continuity_ready_site_hosts": list(
            real_live_evidence_summary.get("continuity_ready_site_hosts") or []
        ),
        "real_live_evidence_reasons": list(
            real_live_evidence_summary.get("evidence_readiness_reasons") or []
        ),
    }


def emit_summary_lines(summary: dict[str, Any]) -> str:
    lines: list[str] = []
    for key, value in summary.items():
        if isinstance(value, (dict, list)):
            rendered = json.dumps(value, ensure_ascii=False)
        else:
            rendered = str(value)
        lines.append(f"{key}={rendered}")
    return "\n".join(lines) + "\n"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Render release summary fields from longrun + sidecar artifacts."
    )
    parser.add_argument("--report-json", required=True)
    parser.add_argument("--source-summary-json")
    parser.add_argument("--session-summary-json")
    parser.add_argument("--real-live-evidence-json")
    parser.add_argument("--prod-live-min-sample-count", type=int, default=6)
    parser.add_argument("--prod-live-min-effective-ratio-percent", type=float, default=35.0)
    parser.add_argument("--prod-live-min-promotion-rate-percent", type=float, default=75.0)
    parser.add_argument(
        "--prod-live-min-browser-success-rate-percent", type=float, default=98.0
    )
    parser.add_argument("--prod-live-min-recent-hot-regions", type=int, default=3)
    parser.add_argument("--prod-live-max-source-top1-percent", type=float, default=75.0)
    parser.add_argument("--prod-live-min-geo-coverage-percent", type=float, default=0.0)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report_path = Path(args.report_json)
    if not report_path.exists():
        raise SystemExit("report_json missing")

    payload = json.loads(report_path.read_text(encoding="utf-8"))
    source_summary = _load_optional_json(args.source_summary_json)
    session_summary = _load_optional_json(args.session_summary_json)
    evidence_summary = _load_optional_json(args.real_live_evidence_json)
    summary = summarize_release_payload(
        payload,
        release_prod_live_gate.Thresholds(
            min_sample_count=max(args.prod_live_min_sample_count, 1),
            min_effective_ratio_percent=args.prod_live_min_effective_ratio_percent,
            min_promotion_rate_percent=args.prod_live_min_promotion_rate_percent,
            min_browser_success_rate_percent=args.prod_live_min_browser_success_rate_percent,
            min_recent_hot_regions=max(args.prod_live_min_recent_hot_regions, 0),
            max_source_top1_percent=args.prod_live_max_source_top1_percent,
            min_geo_coverage_percent=args.prod_live_min_geo_coverage_percent,
        ),
        report_json_path=args.report_json,
        source_summary_json_path=args.source_summary_json,
        session_summary_json_path=args.session_summary_json,
        real_live_evidence_json_path=args.real_live_evidence_json,
        source_summary=source_summary,
        session_summary=session_summary,
        real_live_evidence_summary=evidence_summary,
    )
    print(emit_summary_lines(summary), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
