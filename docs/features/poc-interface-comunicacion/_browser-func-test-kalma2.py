#!/usr/bin/env python3
"""Prueba funcional Kalma2 vía navegador (Playwright)."""
from __future__ import annotations

import json
import sys
from pathlib import Path

PROMPT = (
    "Confirma el cierre de la feature poc-interface-comunicacion: "
    "commit en rama feat/poc-interface-comunicacion, PR #94 y merge a main."
)
BASE_URL = "http://127.0.0.1:8765"


def main() -> int:
    try:
        from playwright.sync_api import sync_playwright
    except ImportError:
        print("ERROR: pip install playwright && playwright install chromium", file=sys.stderr)
        return 2

    report: dict = {"prompt": PROMPT, "steps": []}
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto(BASE_URL, wait_until="networkidle")
        report["steps"].append({"goto": BASE_URL, "title": page.title()})

        page.fill("#prompt", PROMPT)
        page.click("#send")
        page.wait_for_function(
            "() => { const o = document.getElementById('output'); return o && o.value && !o.value.includes('procesando'); }",
            timeout=120_000,
        )
        output = page.locator("#output").input_value()
        report["output"] = output
        report["success"] = "Tormentosa" in output or "Aiúa" in output
        browser.close()

    out_path = Path(__file__).resolve().parent / "_browser-func-test-result.json"
    out_path.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0 if report.get("success") else 1


if __name__ == "__main__":
    raise SystemExit(main())
