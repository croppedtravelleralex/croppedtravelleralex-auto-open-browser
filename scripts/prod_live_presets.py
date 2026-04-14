#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import shlex
import sys
from dataclasses import dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
ROOT_DIR = SCRIPT_DIR.parent
DEFAULT_PRESET_NAME = "legacy"
LAB_ONLY = "lab_only"
PRIVATE_MIX = "private_mix"
PRIVATE_SOURCE_TIERS = {"private_paid", "private", "paid"}
PRIVATE_COST_CLASSES = {"private_paid", "private", "paid", "commercial"}


@dataclass(frozen=True)
class ProdLivePreset:
    name: str
    keep_candidate_per_source: int
    candidate_min_per_source: int
    candidate_per_active: int
    top1_source_keep_candidate_cap: int
    underrepresented_source_keep_candidate_cap: int
    keep_rejected_per_source: int
    rejected_min_per_source: int
    rejected_per_active: int
    stateful_followup_count: int
    auto_browser_regions_from_db: bool
    geo_enrich: bool
    pool_hygiene: bool

    def to_dict(self) -> dict[str, object]:
        payload = {
            "preset": self.name,
            "keep_candidate_per_source": self.keep_candidate_per_source,
            "candidate_min_per_source": self.candidate_min_per_source,
            "candidate_per_active": self.candidate_per_active,
            "top1_source_keep_candidate_cap": self.top1_source_keep_candidate_cap,
            "underrepresented_source_keep_candidate_cap": self.underrepresented_source_keep_candidate_cap,
            "keep_rejected_per_source": self.keep_rejected_per_source,
            "rejected_min_per_source": self.rejected_min_per_source,
            "rejected_per_active": self.rejected_per_active,
            "stateful_followup_count": self.stateful_followup_count,
            "auto_browser_regions_from_db": self.auto_browser_regions_from_db,
            "geo_enrich": self.geo_enrich,
            "pool_hygiene": self.pool_hygiene,
        }
        payload["pool_hygiene_extra_args"] = build_pool_hygiene_extra_args(payload)
        return payload


PRESETS: dict[str, ProdLivePreset] = {
    "legacy": ProdLivePreset(
        name="legacy",
        keep_candidate_per_source=120,
        candidate_min_per_source=40,
        candidate_per_active=20,
        top1_source_keep_candidate_cap=40,
        underrepresented_source_keep_candidate_cap=120,
        keep_rejected_per_source=20,
        rejected_min_per_source=10,
        rejected_per_active=5,
        stateful_followup_count=1,
        auto_browser_regions_from_db=True,
        geo_enrich=True,
        pool_hygiene=True,
    ),
    "stable_v1": ProdLivePreset(
        name="stable_v1",
        keep_candidate_per_source=80,
        candidate_min_per_source=20,
        candidate_per_active=8,
        top1_source_keep_candidate_cap=20,
        underrepresented_source_keep_candidate_cap=80,
        keep_rejected_per_source=10,
        rejected_min_per_source=5,
        rejected_per_active=2,
        stateful_followup_count=1,
        auto_browser_regions_from_db=True,
        geo_enrich=True,
        pool_hygiene=True,
    ),
}


def normalize_preset_name(raw_value: str | None) -> str:
    value = str(raw_value or DEFAULT_PRESET_NAME).strip().lower().replace("-", "_")
    if not value:
        return DEFAULT_PRESET_NAME
    if value not in PRESETS:
        available = ", ".join(sorted(PRESETS))
        raise ValueError(f"unsupported prod_live preset: {raw_value!r}; expected one of {available}")
    return value


def resolve_preset(raw_value: str | None) -> dict[str, object]:
    return PRESETS[normalize_preset_name(raw_value)].to_dict()


def build_pool_hygiene_extra_args(preset: dict[str, object]) -> list[str]:
    return [
        "--keep-candidate-per-source",
        str(int(preset.get("keep_candidate_per_source") or 0)),
        "--candidate-min-per-source",
        str(int(preset.get("candidate_min_per_source") or 0)),
        "--candidate-per-active",
        str(int(preset.get("candidate_per_active") or 0)),
        "--top1-source-keep-candidate-cap",
        str(int(preset.get("top1_source_keep_candidate_cap") or 0)),
        "--underrepresented-source-keep-candidate-cap",
        str(int(preset.get("underrepresented_source_keep_candidate_cap") or 0)),
        "--keep-rejected-per-source",
        str(int(preset.get("keep_rejected_per_source") or 0)),
        "--rejected-min-per-source",
        str(int(preset.get("rejected_min_per_source") or 0)),
        "--rejected-per-active",
        str(int(preset.get("rejected_per_active") or 0)),
    ]


def _normalize_token(value: object) -> str:
    return str(value or "").strip().lower().replace("-", "_")


def infer_provider_supply_class(source_quality_summary: dict[str, object] | None) -> str:
    summary = dict(source_quality_summary or {})
    if int(summary.get("private_paid_source_count") or 0) > 0:
        return PRIVATE_MIX
    for item in summary.get("top_sources") or []:
        if not isinstance(item, dict):
            continue
        source_tier = _normalize_token(item.get("source_tier"))
        cost_class = _normalize_token(item.get("cost_class"))
        if source_tier in PRIVATE_SOURCE_TIERS or cost_class in PRIVATE_COST_CLASSES:
            return PRIVATE_MIX
    return LAB_ONLY


def infer_provider_supply_class_from_report(payload: dict[str, object] | None) -> str:
    report = dict(payload or {})
    source_quality_summary = dict(report.get("source_quality_summary") or {})
    if "provider_supply_class" in source_quality_summary:
        value = _normalize_token(source_quality_summary.get("provider_supply_class"))
        if value in {LAB_ONLY, PRIVATE_MIX}:
            return value
    value = _normalize_token(report.get("provider_supply_class"))
    if value in {LAB_ONLY, PRIVATE_MIX}:
        return value
    return infer_provider_supply_class(source_quality_summary)


def render_shell_exports(preset: dict[str, object]) -> str:
    exports = {
        "PROD_LIVE_PRESET_NAME": str(preset.get("preset") or DEFAULT_PRESET_NAME),
        "PROD_LIVE_PRESET_STATEFUL_FOLLOWUP_COUNT": str(
            int(preset.get("stateful_followup_count") or 0)
        ),
        "PROD_LIVE_PRESET_AUTO_BROWSER_REGIONS_FROM_DB": "1"
        if bool(preset.get("auto_browser_regions_from_db"))
        else "0",
        "PROD_LIVE_PRESET_GEO_ENRICH": "1" if bool(preset.get("geo_enrich")) else "0",
        "PROD_LIVE_PRESET_POOL_HYGIENE": "1" if bool(preset.get("pool_hygiene")) else "0",
        "PROD_LIVE_PRESET_POOL_HYGIENE_EXTRA_ARGS": shlex.join(
            [str(item) for item in (preset.get("pool_hygiene_extra_args") or [])]
        ),
    }
    return "\n".join(f"{key}={shlex.quote(value)}" for key, value in exports.items()) + "\n"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Single source of truth for prod_live preset defaults.",
    )
    parser.add_argument(
        "command",
        nargs="?",
        default="print-json",
        choices=("print-json", "print-shell"),
        help="Output format",
    )
    parser.add_argument(
        "--preset",
        default=os.environ.get("PROXY_VERIFY_REAL_PRESET", DEFAULT_PRESET_NAME),
        help="Preset name: legacy or stable_v1",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    preset = resolve_preset(args.preset)
    if args.command == "print-shell":
        sys.stdout.write(render_shell_exports(preset))
        return 0
    sys.stdout.write(json.dumps(preset, ensure_ascii=False, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
