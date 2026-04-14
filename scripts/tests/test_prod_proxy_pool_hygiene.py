from __future__ import annotations

import argparse
import importlib.util
import sqlite3
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


prod_proxy_pool_hygiene = load_module(
    "prod_proxy_pool_hygiene",
    "prod_proxy_pool_hygiene.py",
)


def make_args() -> argparse.Namespace:
    return argparse.Namespace(
        promotion_threshold=0.03,
        health_threshold=45.0,
        quarantine_seconds=6 * 60 * 60,
        quarantine_min_decision_count=20,
        max_active_for_quarantine=0,
        keep_candidate_per_source=120,
        candidate_min_per_source=40,
        candidate_per_active=20,
        source_concentration_cap_percent=75.0,
        top1_source_keep_candidate_cap=40,
        underrepresented_active_threshold=5,
        underrepresented_source_keep_candidate_cap=None,
        keep_rejected_per_source=20,
        rejected_min_per_source=10,
        rejected_per_active=5,
        protect_recent_seconds=600,
    )


def build_conn() -> sqlite3.Connection:
    conn = sqlite3.connect(":memory:")
    conn.row_factory = sqlite3.Row
    conn.execute(
        """
        CREATE TABLE proxies (
            id TEXT PRIMARY KEY,
            source_label TEXT,
            status TEXT,
            score REAL,
            provider TEXT,
            region TEXT,
            country TEXT,
            last_verify_status TEXT,
            last_verify_at TEXT,
            last_seen_at TEXT,
            created_at TEXT,
            promoted_at TEXT,
            last_probe_error_category TEXT,
            last_exit_country TEXT,
            last_exit_region TEXT
        )
        """
    )
    return conn


def insert_candidate_rows(
    conn: sqlite3.Connection,
    source_label: str,
    count: int,
) -> None:
    for idx in range(count):
        conn.execute(
            """
            INSERT INTO proxies (
                id, source_label, status, score, provider, region, country,
                last_verify_status, last_verify_at, last_seen_at, created_at,
                promoted_at, last_probe_error_category, last_exit_country, last_exit_region
            ) VALUES (?, ?, 'candidate', ?, 'provider-x', 'shared', 'US', 'ok', NULL, NULL, ?, NULL, NULL, 'US', 'shared')
            """,
            (
                f"{source_label}-candidate-{idx}",
                source_label,
                0.9,
                str(1000 + idx),
            ),
        )
    conn.commit()


class ProdProxyPoolHygieneTests(unittest.TestCase):
    def test_build_actions_tightens_top1_source_and_keeps_more_for_underrepresented_source(
        self,
    ) -> None:
        conn = build_conn()
        insert_candidate_rows(conn, "source-top", 50)
        insert_candidate_rows(conn, "source-small", 50)

        sources = [
            {
                "source_label": "source-top",
                "active_count": 10,
                "candidate_count": 50,
                "rejected_count": 0,
                "decision_total": 10,
                "promotion_rate": 1.0,
                "health_score": 95.0,
                "quarantine_until": None,
            },
            {
                "source_label": "source-small",
                "active_count": 1,
                "candidate_count": 50,
                "rejected_count": 0,
                "decision_total": 1,
                "promotion_rate": 1.0,
                "health_score": 95.0,
                "quarantine_until": None,
            },
        ]

        summary = prod_proxy_pool_hygiene.summarize_sources(sources, now_ts=1000)
        self.assertEqual(summary["top1_source_label"], "source-top")
        self.assertAlmostEqual(summary["source_concentration_top1_percent"], 90.9091, places=4)

        actions = prod_proxy_pool_hygiene.build_actions(
            conn,
            sources,
            make_args(),
            now_ts=1000,
        )
        action_map = {action["source_label"]: action for action in actions}

        self.assertEqual(action_map["source-top"]["candidate_keep_limit_base"], 120)
        self.assertEqual(action_map["source-top"]["candidate_keep_limit"], 40)
        self.assertEqual(
            action_map["source-top"]["candidate_keep_adjustment"],
            "top1_source_cap",
        )
        self.assertEqual(action_map["source-small"]["candidate_keep_limit_base"], 40)
        self.assertEqual(action_map["source-small"]["candidate_keep_limit"], 120)
        self.assertEqual(
            action_map["source-small"]["candidate_keep_adjustment"],
            "underrepresented_source_bonus",
        )

    def test_apply_actions_reports_deleted_rows_by_source(self) -> None:
        conn = build_conn()
        insert_candidate_rows(conn, "source-top", 50)
        insert_candidate_rows(conn, "source-small", 50)
        sources = [
            {
                "source_label": "source-top",
                "active_count": 10,
                "candidate_count": 50,
                "rejected_count": 0,
                "decision_total": 10,
                "promotion_rate": 1.0,
                "health_score": 95.0,
                "quarantine_until": None,
            },
            {
                "source_label": "source-small",
                "active_count": 1,
                "candidate_count": 50,
                "rejected_count": 0,
                "decision_total": 1,
                "promotion_rate": 1.0,
                "health_score": 95.0,
                "quarantine_until": None,
            },
        ]

        actions = prod_proxy_pool_hygiene.build_actions(
            conn,
            sources,
            make_args(),
            now_ts=1000,
        )
        apply_result = prod_proxy_pool_hygiene.apply_actions(conn, actions, now_ts=1000)

        self.assertEqual(apply_result["deleted_proxy_rows"], 10)
        self.assertEqual(
            apply_result["deleted_proxy_rows_by_source"]["source-top"],
            10,
        )
        self.assertNotIn("source-small", apply_result["deleted_proxy_rows_by_source"])


if __name__ == "__main__":
    unittest.main()
