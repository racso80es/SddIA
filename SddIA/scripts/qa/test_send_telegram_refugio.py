# -*- coding: utf-8 -*-
"""AC7 — Táctica del Refugio send-telegram-notification."""

from __future__ import annotations

import json
import sys
import unittest
from pathlib import Path
from unittest.mock import MagicMock, patch

_QA = Path(__file__).resolve().parent
_TOOLS = _QA.parent / "tools" / "send-telegram-notification"
if str(_TOOLS) not in sys.path:
    sys.path.insert(0, str(_TOOLS))

from telegram_api import (  # noqa: E402
    escape_markdown_v2,
    is_telegram_parse_error,
    send_message_with_refugio,
)


class TestTelegramParseError(unittest.TestCase):
    def test_parse_error_400(self):
        self.assertTrue(
            is_telegram_parse_error(
                400,
                {"description": "Bad Request: can't parse entities: Character '_' is reserved"},
            )
        )

    def test_non_parse_401(self):
        self.assertFalse(is_telegram_parse_error(401, {"description": "Unauthorized"}))


class TestRefugio(unittest.TestCase):
    @patch("telegram_api._post_send_message")
    def test_degraded_plain_on_markdown_fail(self, mock_post: MagicMock) -> None:
        mock_post.side_effect = [
            (400, {"ok": False, "description": "can't parse entities"}),
            (200, {"ok": True, "result": {"message_id": 99}}),
        ]
        out = send_message_with_refugio("token", "1", "PR _broken", "MarkdownV2")
        self.assertTrue(out["success"])
        self.assertEqual(out["attempt"], 2)
        self.assertTrue(out["degraded_plain_fallback"])
        self.assertEqual(mock_post.call_count, 2)
        _args2 = mock_post.call_args_list[1][0]
        self.assertIsNone(_args2[3])

    @patch("telegram_api._post_send_message")
    def test_first_attempt_ok(self, mock_post: MagicMock) -> None:
        mock_post.return_value = (200, {"ok": True, "result": {"message_id": 1}})
        out = send_message_with_refugio("token", "1", "hola", "MarkdownV2")
        self.assertTrue(out["success"])
        self.assertEqual(out["attempt"], 1)
        self.assertFalse(out["degraded_plain_fallback"])


class TestEscape(unittest.TestCase):
    def test_escape_underscore(self):
        self.assertIn("\\_", escape_markdown_v2("a_b"))


if __name__ == "__main__":
    unittest.main()
