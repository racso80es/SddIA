#!/usr/bin/env python3
"""Fase 0 — rescate Merkle corpus dead-letter (L-RESCUE). Sin route_domain_event ni pending/."""
from __future__ import annotations

import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
EVENTS = REPO / ".events"
DL_SUB = EVENTS / "dead-letter" / "subscribers"
PROCESSED_SUB = EVENTS / "processed" / "subscribers"
PROOFS = REPO / ".SddIA" / "proofs"
ACTA_NAME = "merkle-acta-dlt-backfill-20260827.json"
WINDOW_START = "2026-08-25"
WINDOW_END = "2026-08-27"
TRACE_MARKERS = ("batch-missing-merkle-anchor", "batch-anchor-failed:")


def load_vault_env() -> None:
    """Jerarquía .dev/.env → .SddIA/.dev/.env (instancia pisa global)."""
    merged: dict[str, str] = {}
    for rel in (".dev/.env", ".SddIA/.dev/.env"):
        p = REPO / rel
        if not p.is_file():
            continue
        for line in p.read_text(encoding="utf-8").splitlines():
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, _, val = line.partition("=")
            key = key.strip()
            val = val.strip().strip('"').strip("'")
            if key:
                merged[key] = val
    for key, val in merged.items():
        os.environ[key] = val


def find_event_path(uuid: str) -> Path | None:
    for root in [
        EVENTS / "processed",
        EVENTS / "dead-letter",
        EVENTS / "processing",
        EVENTS / "pending",
    ]:
        p = root / f"{uuid}.json"
        if p.is_file():
            return p
    return None


def inventory() -> list[tuple[Path, dict, Path]]:
    rows: list[tuple[Path, dict, Path]] = []
    if not DL_SUB.is_dir():
        return rows
    for dl in sorted(DL_SUB.glob("*.cumulo.iota-immutable-publisher.json")):
        try:
            meta = json.loads(dl.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            continue
        trace = str(meta.get("error_trace") or "")
        if not any(m in trace for m in TRACE_MARKERS):
            continue
        failed = str(meta.get("failed_at") or "")
        if failed and failed[:10] < WINDOW_START:
            continue
        if failed and failed[:10] > WINDOW_END:
            continue
        uuid = meta.get("event_uuid") or dl.name.split(".")[0]
        ev_path = find_event_path(uuid)
        if not ev_path:
            print(f"[WARN] sin envelope para {uuid}", file=sys.stderr)
            continue
        try:
            event = json.loads(ev_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            continue
        rows.append((dl, event, ev_path))
    return rows


def compact_payload(payload: object) -> str:
    return json.dumps(payload, ensure_ascii=False, separators=(",", ":"), sort_keys=True)


def invoke_publisher(payloads: list[str]) -> dict:
    req = {
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": payloads,
    }
    bin_path = REPO / "SddIA" / "target" / "debug" / "iota-immutable-publisher"
    if not bin_path.is_file():
        bin_path = REPO / "SddIA" / "target" / "release" / "iota-immutable-publisher"
    env = os.environ.copy()
    proc = subprocess.run(
        [str(bin_path)],
        input=json.dumps(req),
        capture_output=True,
        text=True,
        cwd=str(REPO),
        env=env,
        timeout=120,
    )
    if proc.returncode != 0:
        raise RuntimeError(f"publisher exit {proc.returncode}: {proc.stderr or proc.stdout}")
    line = proc.stdout.strip().splitlines()[-1]
    return json.loads(line)


def main() -> int:
    load_vault_env()
    rows = inventory()
    if not rows:
        print("Censo vacío — nada que rescatar en ventana.")
        return 0
    print(f"Inventario: {len(rows)} eventos subscriber DL")

    payloads = []
    uuids = []
    paths = []
    for _dl, event, ev_path in rows:
        uuid = event.get("event_id")
        ds = event.get("delivery_state") or {}
        if ds.get("merkle_anchored") and ds.get("transaction_digest"):
            dig = str(ds.get("transaction_digest"))
            if dig and dig != "batched-digest":
                print(f"[SKIP] ya anclado {uuid}")
                continue
        payloads.append(compact_payload(event.get("payload") or {}))
        uuids.append(uuid)
        paths.append(ev_path)

    if not payloads:
        print("Todos ya anclados.")
        return 0

    print(f"Re-anclaje Merkle único: {len(payloads)} payloads")
    resp = invoke_publisher(payloads)
    if not resp.get("success"):
        err = resp.get("error") or resp.get("feedback") or resp
        print(f"[ERROR] lote falló: {err}", file=sys.stderr)
        return 1

    result = resp.get("result") or {}
    digest = result.get("transaction_digest")
    merkle_root = result.get("merkle_root")
    proofs = result.get("merkle_proofs") or []
    anchored_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    PROOFS.mkdir(parents=True, exist_ok=True)
    PROCESSED_SUB.mkdir(parents=True, exist_ok=True)

    for i, (uuid, ev_path) in enumerate(zip(uuids, paths)):
        event = json.loads(ev_path.read_text(encoding="utf-8"))
        ds = event.setdefault("delivery_state", {})
        ds["cumulo"] = "success"
        ds["merkle_anchored"] = True
        ds["transaction_digest"] = digest
        if merkle_root:
            ds["merkle_root"] = merkle_root
        ds["anchored_retroactively"] = True
        ds["anchored_at"] = anchored_at
        ds.pop("last_batch_anchor_error", None)
        ev_path.write_text(json.dumps(event, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        if i < len(proofs):
            (PROOFS / f"{uuid}.json").write_text(
                json.dumps(proofs[i], ensure_ascii=False, indent=2) + "\n",
                encoding="utf-8",
            )
        dl = DL_SUB / f"{uuid}.cumulo.iota-immutable-publisher.json"
        if dl.is_file():
            dest = PROCESSED_SUB / dl.name
            dest.write_text(dl.read_text(encoding="utf-8"), encoding="utf-8")
            dl.unlink()

    acta = {
        "acta": ACTA_NAME,
        "window": {"start": WINDOW_START, "end": WINDOW_END},
        "retroactive": True,
        "anchored_at": anchored_at,
        "transaction_digest": digest,
        "merkle_root": merkle_root,
        "event_uuids": uuids,
        "count": len(uuids),
    }
    (PROOFS / ACTA_NAME).write_text(
        json.dumps(acta, ensure_ascii=False, indent=2) + "\n", encoding="utf-8",
    )
    print(f"Acta: {PROOFS / ACTA_NAME}")
    print(f"Digest: {digest}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
