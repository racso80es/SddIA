# -*- coding: utf-8 -*-
"""Plantillas mensaje EDA → Telegram."""

from __future__ import annotations

import unittest

from telegram_notify_core import build_telegram_message_from_event


class TestBuildMessage(unittest.TestCase):
    def test_pr_presented(self):
        msg = build_telegram_message_from_event(
            {
                "event_type": "PullRequest_Presented",
                "payload": {"branch": "feat/x", "pr_url": "https://github.com/o/r/pull/1"},
            }
        )
        self.assertIn("feat/x", msg or "")
        self.assertIn("pull/1", msg or "")

    def test_fracture(self):
        msg = build_telegram_message_from_event(
            {
                "event_type": "System_Fracture_Detected",
                "payload": {
                    "process_name": "feature",
                    "trace_hash": "abc123",
                },
            }
        )
        self.assertIn("feature", msg or "")
        self.assertIn("abc123", msg or "")


if __name__ == "__main__":
    unittest.main()
