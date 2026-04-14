from __future__ import annotations

import importlib.util
import sys
import unittest
from pathlib import Path


ROOT_DIR = Path(__file__).resolve().parents[2]


def load_module(module_name: str, file_name: str):
    path = ROOT_DIR / "scripts" / file_name
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"failed to load module from {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)
    return module


release_prod_live_gate = load_module(
    "release_prod_live_gate",
    "release_prod_live_gate.py",
)


def base_payload() -> dict[str, object]:
    return {
        "summary": {
            "mode": "prod_live",
            "sample_count": 8,
            "effective_active_ratio_percent": {"median": 40.0},
            "promotion_rate": {"median": 80.0},
            "browser_success_rate_percent": 99.0,
            "recent_hot_regions_union": ["us-east", "eu-west", "ap-southeast"],
            "source_concentration_top1_percent": {"median": 60.0, "max": 60.0},
            "trend_flags": {
                "stateful_continuity_observed": True,
                "active_pool_ever_positive": True,
            },
            "event_summary": {
                "browser_proxy_claim_lost_failures": 0,
                "browser_proxy_not_found_failures": 0,
            },
            "latest": {
                "mode": "prod_live",
                "pool_active": 20,
                "recent_hot_regions": ["us-east", "eu-west", "ap-southeast"],
                "source_concentration_top1_percent": 60.0,
            },
        },
        "source_quality_summary": {
            "effective_geo_quality_summary": {
                "avg_geo_coverage_percent": 70.0,
            }
        },
    }


class ReleaseProdLiveGateTests(unittest.TestCase):
    def test_accepts_report_that_meets_thresholds(self) -> None:
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            base_payload(),
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "ok")
        self.assertEqual(failure_scope, "none")
        self.assertIn("prod-live acceptance passed", detail)

    def test_rejects_proxy_not_found_failures(self) -> None:
        payload = base_payload()
        payload["summary"]["event_summary"]["browser_proxy_not_found_failures"] = 1
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "proxy_claim_lost")
        self.assertEqual(failure_scope, "proxy_pool")
        self.assertIn("browser_proxy_not_found_failures=1", detail)

    def test_rejects_proxy_claim_lost_failures(self) -> None:
        payload = base_payload()
        payload["summary"]["event_summary"]["browser_proxy_claim_lost_failures"] = 2
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "proxy_claim_lost")
        self.assertEqual(failure_scope, "proxy_pool")
        self.assertIn("browser_proxy_claim_lost_failures=2", detail)

    def test_rejects_when_sample_count_is_insufficient(self) -> None:
        payload = base_payload()
        payload["summary"]["sample_count"] = 2
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(min_sample_count=6),
        )
        self.assertEqual(reason_code, "sample_insufficient")
        self.assertEqual(failure_scope, "prod_live_sample")
        self.assertIn("sample_count=2", detail)

    def test_rejects_missing_recent_hot_regions(self) -> None:
        payload = base_payload()
        payload["summary"]["recent_hot_regions_union"] = ["us-east"]
        payload["summary"]["latest"]["recent_hot_regions"] = ["us-east"]
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "hot_region_window_empty")
        self.assertEqual(failure_scope, "hot_regions")
        self.assertIn("latest_recent_hot_regions=1", detail)

    def test_rejects_source_concentration_over_threshold(self) -> None:
        payload = base_payload()
        payload["summary"]["latest"]["source_concentration_top1_percent"] = 90.0
        payload["summary"]["source_concentration_top1_percent"]["max"] = 90.0
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "source_concentration_too_high")
        self.assertEqual(failure_scope, "proxy_pool")
        self.assertIn("source_concentration_top1_percent_max=90.0", detail)

    def test_accepts_fractional_promotion_rate_median(self) -> None:
        payload = base_payload()
        payload["summary"]["promotion_rate"]["median"] = 0.7802
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "ok")
        self.assertEqual(failure_scope, "none")
        self.assertIn("median_promotion_rate=78.02", detail)

    def test_uses_top_level_mode_when_summary_mode_is_missing(self) -> None:
        payload = base_payload()
        payload["mode"] = "prod_live"
        payload["summary"]["mode"] = ""
        payload["summary"]["latest"]["mode"] = ""
        reason_code, failure_scope, _ = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "ok")
        self.assertEqual(failure_scope, "none")

    def test_uses_event_requested_regions_when_recent_hot_regions_are_missing(self) -> None:
        payload = base_payload()
        payload["summary"]["recent_hot_regions_union"] = []
        payload["summary"]["latest"]["recent_hot_regions"] = []
        payload["summary"]["event_summary"]["browser_requested_regions"] = {
            "us-east": 2,
            "eu-west": 2,
            "ap-southeast": 2,
        }
        reason_code, failure_scope, detail = release_prod_live_gate.evaluate_report(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(reason_code, "ok")
        self.assertEqual(failure_scope, "none")
        self.assertIn("recent_hot_regions=3", detail)

    def test_lab_only_source_concentration_failure_becomes_provider_capped(self) -> None:
        payload = base_payload()
        payload["summary"]["latest"]["source_concentration_top1_percent"] = 90.0
        result = release_prod_live_gate.evaluate_report_payload(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(result.reason_code, "source_concentration_too_high")
        self.assertEqual(result.strict_verdict, "fail")
        self.assertEqual(result.operational_verdict, "provider_capped")
        self.assertEqual(result.provider_supply_class, "lab_only")
        self.assertEqual(result.provider_cap_reason, "source_concentration_too_high")

    def test_non_concentration_failure_does_not_become_provider_capped(self) -> None:
        payload = base_payload()
        payload["summary"]["event_summary"]["browser_proxy_not_found_failures"] = 1
        result = release_prod_live_gate.evaluate_report_payload(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(result.reason_code, "proxy_claim_lost")
        self.assertEqual(result.strict_verdict, "fail")
        self.assertEqual(result.operational_verdict, "fail")
        self.assertIsNone(result.provider_cap_reason)

    def test_private_mix_concentration_failure_stays_fail(self) -> None:
        payload = base_payload()
        payload["summary"]["latest"]["source_concentration_top1_percent"] = 90.0
        payload["source_quality_summary"]["private_paid_source_count"] = 1
        payload["source_quality_summary"]["provider_supply_class"] = "private_mix"
        result = release_prod_live_gate.evaluate_report_payload(
            payload,
            release_prod_live_gate.Thresholds(),
        )
        self.assertEqual(result.provider_supply_class, "private_mix")
        self.assertEqual(result.strict_verdict, "fail")
        self.assertEqual(result.operational_verdict, "fail")
        self.assertIsNone(result.provider_cap_reason)


if __name__ == "__main__":
    unittest.main()
