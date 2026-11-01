#!/usr/bin/env python3
"""AUT-21 T7 — reset subscriber_delivery + route-domain-event (lab operativo)."""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
EXEC = REPO / "SddIA/target/debug/execute-process"
SDDIA_RUN = REPO / "sddia-run.sh"
SUBSCRIBER = "tekton.tracker-stamp"

# Casos documentados en AUT-21 (añadir más UUIDs si aplica).
DEFAULT_EVENT_IDS = [
    "4581b7f6-cc3b-4486-b2b2-efa9a12491cb",
]


def run_json(cmd: list[str]) -> dict:
    proc = subprocess.run(
        cmd,
        cwd=REPO,
        capture_output=True,
        text=True,
        check=False,
    )
    raw = (proc.stdout or proc.stderr or "").strip()
    if not raw:
        raise RuntimeError(f"sin stdout: {' '.join(cmd)} exit={proc.returncode}")
    try:
        body = json.loads(raw.splitlines()[-1])
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"JSON ilegible: {raw!r}") from exc
    if proc.returncode != 0 or not body.get("success", body.get("exitCode") == 0):
        err = body.get("error") or body
        raise RuntimeError(f"fallo {' '.join(cmd)}: {err}")
    return body


def reset_delivery(event_id: str) -> None:
    run_json(
        [
            str(EXEC),
            "--reset-subscriber-delivery",
            "--inputs",
            json.dumps(
                {
                    "event_id": event_id,
                    "subscriber_id": SUBSCRIBER,
                    "reason": "AUT-21-T7",
                }
            ),
        ]
    )


def route_event(event_id: str) -> dict:
    rel = f".events/pending/{event_id}.json"
    proc = subprocess.run(
        [str(SDDIA_RUN), "--process", "route-domain-event", "--inputs", json.dumps({"event_file_path": rel, "blocking": True})],
        cwd=REPO,
        capture_output=True,
        text=True,
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(f"route sin acuse: {proc.stderr}")
    return json.loads(line)


def main() -> int:
    if not EXEC.is_file():
        print(f"compilar: cd SddIA && cargo build -p execute-process", file=sys.stderr)
        return 1
    ids = [a for a in sys.argv[1:] if a.strip()] or DEFAULT_EVENT_IDS
    report = []
    for eid in ids:
        entry = {"event_id": eid}
        try:
            reset_delivery(eid)
            entry["reset"] = "ok"
            acuse = route_event(eid)
            entry["route"] = acuse.get("data") or acuse
        except Exception as exc:  # noqa: BLE001 — script operativo
            entry["error"] = str(exc)
        report.append(entry)
    print(json.dumps({"replay": report}, indent=2))
    return 0 if all("error" not in r for r in report) else 2


if __name__ == "__main__":
    raise SystemExit(main())
