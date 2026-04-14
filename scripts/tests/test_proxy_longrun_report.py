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


proxy_longrun_report = load_module(
    "proxy_longrun_report",
    "proxy_longrun_report.py",
)


class ProxyLongrunReportTests(unittest.TestCase):
    def test_build_sample_preserves_recent_hot_regions_and_geo_summary(self) -> None:
        status = {
            "mode": "prod_live",
            "queue_len": 0,
            "proxy_pool_status": {
                "mode": "prod_live",
                "total": 40,
                "active": 20,
                "candidate": 10,
                "candidate_rejected": 10,
                "reported_active_ratio_percent": 50.0,
                "effective_active_ratio_percent": 50.0,
                "eligible_pool_total": 40,
                "fresh_candidate_total": 10,
                "recent_rejected_total": 10,
                "region_shortages": [],
                "hot_regions": ["us-east"],
                "recent_hot_regions": ["us-east", "eu-west", "ap-southeast"],
                "recent_hot_region_counts": {
                    "us-east": 4,
                    "eu-west": 3,
                    "ap-southeast": 2,
                },
                "hot_region_window_seconds": 600,
                "source_concentration_top1_percent": 60.0,
                "source_concentration_top3_percent": 100.0,
                "active_sources_with_min_inventory": 3,
                "active_regions_with_min_inventory": 3,
            },
            "proxy_replenish_metrics": {
                "promotion_rate": 80.0,
                "reject_rate": 20.0,
                "fallback_rate": 0.0,
                "recent_batches": 2,
            },
            "proxy_harvest_metrics": {
                "source_count": 3,
                "healthy_source_count": 3,
                "due_source_count": 0,
                "source_failures": 0,
                "source_summaries": [
                    {
                        "source_label": "provider-a",
                        "declared_geo_quality": "high",
                        "effective_geo_quality": "externally_verified",
                        "geo_coverage_percent": 95.0,
                        "active_share_percent": 60.0,
                    },
                    {
                        "source_label": "provider-b",
                        "declared_geo_quality": "medium",
                        "effective_geo_quality": "host_geo_inferred",
                        "geo_coverage_percent": 70.0,
                        "active_share_percent": 25.0,
                    },
                ],
            },
            "identity_session_metrics": {
                "active_sessions": 3,
                "reused_sessions": 2,
                "created_sessions": 1,
            },
            "proxy_site_metrics": {
                "tracked_sites": 2,
                "site_records": 4,
                "top_failing_sites": [],
            },
        }

        sample = proxy_longrun_report.build_sample(status, captured_at=1713000000)

        self.assertEqual(sample["recent_hot_regions"], ["us-east", "eu-west", "ap-southeast"])
        self.assertEqual(sample["hot_region_window_seconds"], 600)
        self.assertEqual(sample["source_concentration_top1_percent"], 60.0)
        self.assertEqual(sample["active_sources_with_min_inventory"], 3)
        self.assertEqual(
            sample["effective_geo_quality_summary"]["effective_geo_quality_counts"][
                "externally_verified"
            ],
            1,
        )

    def test_summarize_events_counts_recent_hot_and_proxy_not_found_failures(self) -> None:
        events = [
            {
                "type": "browser",
                "status": "succeeded",
                "workload": "external",
                "url": "https://example.com",
                "proxy_id": "proxy-1",
                "requested_region": "us-east",
                "hot_regions_during_request": ["us-east"],
                "recent_hot_regions_during_request": ["us-east", "eu-west"],
                "region_shortages_during_request": [],
                "identity_session_status": "auto_reused",
                "cookie_restore_count": 1,
                "cookie_persist_count": 1,
                "local_storage_restore_count": 1,
                "local_storage_persist_count": 1,
                "session_storage_restore_count": 1,
                "session_storage_persist_count": 1,
                "browser_failure_signal": None,
                "selection_reason_summary": "selected best active proxy",
                "error_message": None,
            },
            {
                "type": "browser",
                "status": "failed",
                "workload": "stateful",
                "url": "https://stateful.example.com",
                "proxy_id": None,
                "requested_region": "eu-west",
                "hot_regions_during_request": ["eu-west"],
                "recent_hot_regions_during_request": ["us-east", "eu-west", "ap-southeast"],
                "region_shortages_during_request": ["ap-southeast"],
                "identity_session_status": "auto_created",
                "cookie_restore_count": 0,
                "cookie_persist_count": 0,
                "local_storage_restore_count": 0,
                "local_storage_persist_count": 0,
                "session_storage_restore_count": 0,
                "session_storage_persist_count": 0,
                "browser_failure_signal": "proxy_selection_failure",
                "selection_reason_summary": "proxy missing after claim",
                "error_message": "proxy not found after selection claim: proxy-2",
            },
            {
                "type": "browser",
                "status": "succeeded",
                "workload": "stateful_followup",
                "url": "https://stateful.example.com",
                "proxy_id": "proxy-3",
                "requested_region": "eu-west",
                "hot_regions_during_request": ["eu-west"],
                "recent_hot_regions_during_request": ["eu-west"],
                "region_shortages_during_request": [],
                "identity_session_status": "auto_reused",
                "cookie_restore_count": 1,
                "cookie_persist_count": 1,
                "local_storage_restore_count": 1,
                "local_storage_persist_count": 1,
                "session_storage_restore_count": 1,
                "session_storage_persist_count": 1,
                "browser_failure_signal": None,
                "selection_reason_summary": "stateful followup",
                "error_message": None,
            },
        ]

        summary = proxy_longrun_report.summarize_events(events)

        self.assertEqual(summary["browser_total"], 3)
        self.assertEqual(summary["browser_proxy_not_found_failures"], 1)
        self.assertEqual(summary["browser_proxy_claim_lost_failures"], 1)
        self.assertEqual(summary["browser_recent_hot_regions_observed"]["eu-west"], 3)
        self.assertEqual(summary["browser_region_shortages_observed"]["ap-southeast"], 1)
        self.assertEqual(summary["stateful_primary_total"], 1)
        self.assertEqual(summary["stateful_primary_succeeded"], 0)
        self.assertEqual(summary["stateful_followup_total"], 1)
        self.assertEqual(summary["stateful_followup_succeeded"], 1)

    def test_build_sample_derives_source_concentration_from_source_summaries(self) -> None:
        status = {
            "proxy_pool_status": {
                "total": 672,
                "active": 362,
                "candidate": 260,
                "candidate_rejected": 50,
            },
            "proxy_replenish_metrics": {
                "promotion_rate": 0.8786,
                "reject_rate": 0.1214,
                "fallback_rate": 0.0,
                "recent_batches": 3,
            },
            "proxy_harvest_metrics": {
                "source_summaries": [
                    {"source_label": "github_speedx_http", "active_count": 318},
                    {"source_label": "github_zaeem20_http", "active_count": 21},
                    {"source_label": "github_monosans_http", "active_count": 13},
                    {"source_label": "github_mmpx12_http", "active_count": 10},
                ]
            },
        }

        sample = proxy_longrun_report.build_sample(
            status,
            captured_at=1713000000,
            fallback_mode="prod_live",
        )

        self.assertEqual(sample["mode"], "prod_live")
        self.assertAlmostEqual(sample["source_concentration_top1_percent"], 87.8453, places=4)
        self.assertAlmostEqual(sample["source_concentration_top3_percent"], 97.2376, places=4)
        self.assertEqual(sample["active_sources_with_min_inventory"], 4)

    def test_real_live_evidence_summary_contains_required_fields(self) -> None:
        summary = {
            "mode": "prod_live",
            "sample_count": 8,
            "browser_success_rate_percent": 99.5,
            "source_concentration_top1_percent": {"median": 61.0, "max": 74.0},
            "recent_hot_regions_union": ["ap-southeast", "eu-west", "cn"],
            "trend_flags": {
                "stateful_continuity_observed": True,
                "session_reuse_observed": True,
            },
            "event_summary": {
                "browser_total": 10,
                "browser_succeeded": 10,
                "browser_proxy_claim_lost_failures": 0,
                "browser_proxy_not_found_failures": 0,
                "stateful_primary_total": 3,
                "stateful_primary_succeeded": 3,
                "stateful_followup_total": 3,
                "stateful_followup_succeeded": 3,
                "browser_cookie_restore_total": 4,
                "browser_cookie_persist_total": 4,
                "browser_local_storage_restore_total": 4,
                "browser_local_storage_persist_total": 4,
                "browser_session_storage_restore_total": 4,
                "browser_session_storage_persist_total": 4,
                "browser_requested_regions": {"ap-southeast": 5, "eu-west": 3, "cn": 2},
                "browser_hot_regions_observed": {"ap-southeast": 5, "eu-west": 3},
                "browser_recent_hot_regions_observed": {
                    "ap-southeast": 5,
                    "eu-west": 3,
                    "cn": 2,
                },
            },
            "latest": {"source_concentration_top1_percent": 63.0},
        }
        session_summary = {
            "continuity_chain_observed": True,
        }
        events = [
            {
                "type": "browser",
                "status": "succeeded",
                "workload": "external",
                "url": "https://www.example.com",
                "final_url": "https://www.example.com/home",
                "proxy_id": "proxy-ext-1",
                "requested_region": "ap-southeast",
                "recent_hot_regions_during_request": ["ap-southeast"],
                "identity_session_status": "auto_created",
                "cookie_restore_count": 0,
                "cookie_persist_count": 1,
                "local_storage_restore_count": 0,
                "local_storage_persist_count": 1,
                "session_storage_restore_count": 0,
                "session_storage_persist_count": 1,
            },
            {
                "type": "browser",
                "status": "succeeded",
                "workload": "stateful",
                "url": "https://stateful.example.com",
                "final_url": "https://stateful.example.com/dashboard",
                "proxy_id": "proxy-stateful-1",
                "requested_region": "eu-west",
                "recent_hot_regions_during_request": ["eu-west"],
                "identity_session_status": "auto_created",
                "cookie_restore_count": 0,
                "cookie_persist_count": 1,
                "local_storage_restore_count": 0,
                "local_storage_persist_count": 1,
                "session_storage_restore_count": 0,
                "session_storage_persist_count": 1,
            },
            {
                "type": "browser",
                "status": "succeeded",
                "workload": "stateful_followup",
                "url": "https://stateful.example.com",
                "final_url": "https://stateful.example.com/dashboard",
                "proxy_id": "proxy-stateful-1",
                "requested_region": "eu-west",
                "recent_hot_regions_during_request": ["eu-west"],
                "identity_session_status": "auto_reused",
                "cookie_restore_count": 1,
                "cookie_persist_count": 1,
                "local_storage_restore_count": 1,
                "local_storage_persist_count": 1,
                "session_storage_restore_count": 1,
                "session_storage_persist_count": 1,
            },
        ]
        evidence = proxy_longrun_report.build_real_live_evidence_summary(
            summary, session_summary, events
        )
        self.assertTrue(evidence["sample_sufficient"])
        self.assertTrue(evidence["stateful_continuity_observed"])
        self.assertTrue(evidence["continuity_chain_observed"])
        self.assertTrue(evidence["storage_any_restore_or_persist_positive"])
        self.assertEqual(evidence["proxy_claim_lost_failures"], 0)
        self.assertEqual(
            evidence["source_concentration_top1_percent"]["max"],
            74.0,
        )
        self.assertEqual(
            evidence["unique_requested_regions"],
            ["ap-southeast", "cn", "eu-west"],
        )
        self.assertEqual(evidence["site_host_count"], 2)
        self.assertEqual(evidence["stateful_site_host_count"], 1)
        self.assertEqual(evidence["continuity_ready_site_host_count"], 1)
        self.assertEqual(
            evidence["continuity_ready_site_hosts"],
            ["stateful.example.com"],
        )
        top_host = evidence["site_host_summaries"][0]
        self.assertEqual(top_host["host"], "stateful.example.com")
        self.assertTrue(top_host["continuity_chain_observed"])

    def test_load_samples_from_input_preserves_preset_context(self) -> None:
        payload_path = ROOT_DIR / "reports" / "_tmp_proxy_longrun_input.json"
        payload_path.parent.mkdir(parents=True, exist_ok=True)
        payload_path.write_text(
            """
{
  "base_url": "http://127.0.0.1:3000",
  "mode": "prod_live",
  "preset": "stable_v1",
  "stateful_followup_count": 1,
  "sticky_stateful_region": "eu-west",
  "pool_hygiene_extra_args": ["--keep-candidate-per-source", "80"],
  "events": [],
  "errors": [],
  "status_snapshots": []
}
""".strip()
            + "\n",
            encoding="utf-8",
        )
        try:
            _, _, _, _, _, run_context = proxy_longrun_report.load_samples_from_input(
                payload_path.as_posix()
            )
        finally:
            payload_path.unlink(missing_ok=True)

        self.assertEqual(run_context["mode"], "prod_live")
        self.assertEqual(run_context["preset"], "stable_v1")
        self.assertEqual(run_context["stateful_followup_count"], 1)
        self.assertEqual(run_context["sticky_stateful_region"], "eu-west")
        self.assertEqual(
            run_context["pool_hygiene_extra_args"],
            ["--keep-candidate-per-source", "80"],
        )

    def test_load_samples_from_input_uses_driver_mode_when_status_snapshot_lacks_mode(self) -> None:
        payload_path = ROOT_DIR / "reports" / "_tmp_proxy_longrun_input_with_snapshots.json"
        payload_path.parent.mkdir(parents=True, exist_ok=True)
        payload_path.write_text(
            """
{
  "base_url": "http://127.0.0.1:3000",
  "mode": "prod_live",
  "preset": "stable_v1",
  "events": [],
  "errors": [],
  "status_snapshots": [
    {
      "captured_at": 1713000000,
      "status": {
        "queue_len": 0,
        "proxy_pool_status": {
          "total": 12,
          "active": 6,
          "candidate": 3,
          "candidate_rejected": 3
        },
        "proxy_replenish_metrics": {
          "promotion_rate": 0.8,
          "reject_rate": 0.2,
          "fallback_rate": 0.0,
          "recent_batches": 1
        }
      }
    }
  ]
}
""".strip()
            + "\n",
            encoding="utf-8",
        )
        try:
            _, samples, _, _, _, run_context = proxy_longrun_report.load_samples_from_input(
                payload_path.as_posix()
            )
        finally:
            payload_path.unlink(missing_ok=True)

        self.assertEqual(run_context["mode"], "prod_live")
        self.assertEqual(len(samples), 1)
        self.assertEqual(samples[0]["mode"], "prod_live")

    def test_source_quality_summary_derives_provider_supply_class(self) -> None:
        summary = proxy_longrun_report.build_source_quality_summary(
            [
                {
                    "status": {
                        "proxy_harvest_metrics": {
                            "source_summaries": [
                                {
                                    "source_label": "provider-a",
                                    "source_tier": "controlled",
                                    "for_prod": True,
                                    "for_demo": False,
                                    "cost_class": "internal",
                                    "effective_geo_quality": "host_geo_inferred",
                                    "declared_geo_quality": "medium",
                                },
                                {
                                    "source_label": "provider-b",
                                    "source_tier": "private_paid",
                                    "for_prod": True,
                                    "for_demo": False,
                                    "cost_class": "paid",
                                    "effective_geo_quality": "externally_verified",
                                    "declared_geo_quality": "high",
                                },
                            ]
                        }
                    }
                }
            ]
        )

        self.assertEqual(summary["private_paid_source_count"], 1)
        self.assertEqual(summary["provider_supply_class"], "private_mix")


if __name__ == "__main__":
    unittest.main()
