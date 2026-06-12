#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Auditoría EDA: correlación index.md ↔ bus ↔ artefactos .md."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
if str(SCRIPT.parent) not in sys.path:
    sys.path.insert(0, str(SCRIPT.parent))

from eda_bus_utils import find_existing_domain_event, load_eda_bus
from eda_coverage_utils import is_entity_covered, remove_entity_coverage, upsert_entity_coverage
from execute_process_core import parse_frontmatter

ENTITY_DIRS: dict[str, str] = {
    "skill": "SddIA/skills",
    "event": "SddIA/events",
    "process": "SddIA/process",
    "agent": "SddIA/agents",
    "tool": "SddIA/tools",
    "action": "SddIA/actions",
    "norm": "SddIA/library/norms",
    "codex": "SddIA/library/codexes",
    "suite": "SddIA/suites",
}


def _repo_root() -> Path:
    here = SCRIPT
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _parse_index_uuids(index_path: Path) -> dict[str, str]:
    if not index_path.is_file():
        return {}
    text = index_path.read_text(encoding="utf-8")
    out: dict[str, str] = {}
    for line in text.splitlines():
        if not line.startswith("|") or "`" not in line:
            continue
        uuids = re.findall(
            r"[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}",
            line,
            re.I,
        )
        if not uuids:
            continue
        name_match = re.search(r"`([^`]+\.md)`", line)
        if name_match:
            name = name_match.group(1).replace(".md", "")
            out[name] = uuids[0]
        else:
            cols = [c.strip() for c in line.split("|") if c.strip()]
            if cols:
                out[cols[0].replace(".md", "")] = uuids[0]
    return out


def _valid_sha256(value: Any) -> bool:
    return isinstance(value, str) and value.startswith("sha256:") and len(value) > 15


def _hash_from_artifact(repo: Path, artifact: Path) -> str:
    proc = subprocess.run(
        [
            sys.executable,
            str(repo / "SddIA" / "scripts" / "qa" / "execute-action.py"),
            "--action",
            "crypto-broker",
            "--inputs",
            json.dumps(
                {
                    "operation": "GENERATE_SHA256",
                    "target_type": "STRING",
                    "target_payload": artifact.read_text(encoding="utf-8"),
                }
            ),
        ],
        capture_output=True,
        text=True,
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if line:
        try:
            body = json.loads(line)
            digest = (body.get("data") or {}).get("result")
            if isinstance(digest, str) and digest:
                return digest if digest.startswith("sha256:") else f"sha256:{digest}"
        except json.JSONDecodeError:
            pass
    digest = hashlib.sha256(artifact.read_text(encoding="utf-8").encode("utf-8")).hexdigest()
    return f"sha256:{digest}"


def _resolve_hash_signature(repo: Path, artifact: Path, fm: dict[str, Any]) -> str:
    hs = fm.get("hash_signature")
    if _valid_sha256(hs):
        return str(hs)
    return _hash_from_artifact(repo, artifact)


def scan_orphans(repo: Path) -> dict[str, Any]:
    orphans: list[dict[str, Any]] = []
    indexed: list[dict[str, Any]] = []
    for entity_class, rel_dir in ENTITY_DIRS.items():
        base = repo / rel_dir
        index_map = _parse_index_uuids(base / "index.md")
        for name, uuid in index_map.items():
            artifact = base / f"{name}.md"
            entry: dict[str, Any] = {
                "entity_class": entity_class,
                "entity_name": name,
                "entity_uuid": uuid,
                "artifact_path": str(artifact.relative_to(repo)).replace("\\", "/"),
                "artifact_exists": artifact.is_file(),
            }
            cov = is_entity_covered(repo, uuid)
            entry["is_covered_ssot"] = cov
            if artifact.is_file() and not cov:
                fm = parse_frontmatter(artifact)
                entry["hash_signature"] = _resolve_hash_signature(repo, artifact, fm)
                orphans.append(entry)
            indexed.append(entry)
    return {
        "scanned_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "scan_source": "eda-coverage.json",
        "orphan_count": len(orphans),
        "orphans": orphans,
        "indexed_entities": len(indexed),
    }


def _merkle_root(leaves: list[str]) -> str:
    if not leaves:
        return hashlib.sha256(b"").hexdigest()
    layer = [hashlib.sha256(x.encode("utf-8")).digest() for x in sorted(leaves)]
    while len(layer) > 1:
        nxt: list[bytes] = []
        for i in range(0, len(layer), 2):
            left = layer[i]
            right = layer[i + 1] if i + 1 < len(layer) else layer[i]
            nxt.append(hashlib.sha256(left + right).digest())
        layer = nxt
    return layer[0].hex()


def anchor_merkle(repo: Path, manifest_path: Path) -> dict[str, Any]:
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    orphans = manifest.get("orphans") or []
    leaves = [
        f"{o.get('entity_uuid')}:{o.get('hash_signature', '')}"
        for o in orphans
        if o.get("entity_uuid")
    ]
    root = _merkle_root(leaves)
    acta = {
        "correlation_id": manifest.get("correlation_id"),
        "merkle_root": f"sha256:{root}",
        "entity_count": len(leaves),
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "entries": orphans,
    }
    tool_dir = repo / "SddIA" / "scripts" / "tools" / "iota-immutable-publisher"
    entry = tool_dir / "index.ts"
    digest = None
    simulate = os.environ.get("SDDIA_LAB_SIMULATE_IOTA", "").strip().lower() in ("1", "true", "yes")
    if simulate:
        digest = f"lab-simulated-{root[:16]}"
    elif entry.is_file():
        local_ts_node = tool_dir / "node_modules" / ".bin" / "ts-node"
        npx = shutil.which("npx")
        if local_ts_node.is_file():
            runner_cmd = [str(local_ts_node), str(entry)]
        elif npx:
            runner_cmd = [npx, "ts-node", str(entry)]
        else:
            runner_cmd = []
        payload = {
            "action": "publish_immutable_data",
            "network": "testnet",
            "payload": json.dumps(acta, ensure_ascii=False),
        }
        proc = (
            subprocess.run(
                runner_cmd,
                input=json.dumps(payload),
                capture_output=True,
                text=True,
                cwd=str(tool_dir),
                check=False,
            )
            if runner_cmd
            else subprocess.CompletedProcess(args=[], returncode=1, stdout="", stderr="npx not found")
        )
        if proc.returncode == 0:
            try:
                body = json.loads(proc.stdout.strip() or "{}")
                digest = (body.get("result") or {}).get("transaction_digest")
            except json.JSONDecodeError:
                pass
    out_path = manifest_path.parent / f"merkle-acta-{manifest.get('correlation_id', 'batch')}.json"
    acta["transaction_digest"] = digest
    out_path.write_text(json.dumps(acta, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    try:
        acta_rel = str(out_path.resolve().relative_to(repo.resolve())).replace("\\", "/")
    except ValueError:
        acta_rel = str(out_path)
    manifest_path.write_text(
        json.dumps(
            {**manifest, "merkle_anchored": bool(digest), "transaction_digest": digest, "merkle_acta_path": acta_rel},
            indent=2,
            ensure_ascii=False,
        )
        + "\n",
        encoding="utf-8",
    )
    return acta


def emit_orphans(
    repo: Path, report: dict[str, Any], *, skip_dlt: bool = True, correlation_id: str | None = None
) -> list[dict[str, Any]]:
    cid = correlation_id or report.get("correlation_id") or "eda-backfill"
    results: list[dict[str, Any]] = []
    cli = repo / "SddIA" / "scripts" / "qa" / "execute-action.py"
    for orphan in report.get("orphans") or []:
        artifact = repo / orphan["artifact_path"]
        if not artifact.is_file():
            continue
        fm = parse_frontmatter(artifact)
        uuid = fm.get("uuid") or orphan.get("entity_uuid")
        rel = orphan["artifact_path"].replace("\\", "/")
        origin = "local" if rel.startswith(".SddIA/") else "core"
        hash_sig = orphan.get("hash_signature") or _resolve_hash_signature(repo, artifact, fm)
        existing = find_existing_domain_event(repo, str(uuid), "create", "Domain_Entity_Created")
        if existing:
            results.append({**existing, "idempotent": True, "skip_dlt": skip_dlt})
            continue
        payload = {
            "entity_class": orphan["entity_class"],
            "entity_name": orphan["entity_name"],
            "lifecycle_operation": "create",
            "entity_uuid": uuid,
            "version": fm.get("version"),
            "hash_signature_new": hash_sig,
            "hash_signature_old": None,
            "origin_topology": origin,
            "emitter_agent": "cumulo-eda-backfill",
            "changes_summary": f"Retroactive Domain_Entity_Created — Fase C ({cid})",
        }
        proc = subprocess.run(
            [sys.executable, str(cli), "--action", "emit-domain-mutation", "--inputs", json.dumps(payload)],
            capture_output=True,
            text=True,
            cwd=str(repo),
            check=False,
        )
        line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
        body = json.loads(line) if line else {"success": False}
        data = body.get("data") or {}
        data["skip_dlt"] = skip_dlt
        results.append(data)
    return results


def backfill_coverage(repo: Path) -> dict[str, Any]:
    backfilled = 0
    skipped = 0
    indexed = 0
    for entity_class, rel_dir in ENTITY_DIRS.items():
        base = repo / rel_dir
        index_map = _parse_index_uuids(base / "index.md")
        indexed += len(index_map)
        for name, uuid in index_map.items():
            artifact = base / f"{name}.md"
            if not artifact.is_file():
                skipped += 1
                continue
            if is_entity_covered(repo, uuid):
                skipped += 1
                continue
            fm = parse_frontmatter(artifact)
            hash_sig = _resolve_hash_signature(repo, artifact, fm)
            upsert_entity_coverage(
                repo,
                uuid,
                event_type="Domain_Entity_Created",
                last_hash=hash_sig,
            )
            backfilled += 1
    return {"backfilled": backfilled, "skipped": skipped, "indexed_entities": indexed}


def main() -> int:
    parser = argparse.ArgumentParser(description="Auditoría cobertura EDA genómica")
    parser.add_argument("--scan", action="store_true", help="Escanear huérfanas")
    parser.add_argument("--json", action="store_true", help="Salida JSON")
    parser.add_argument("--emit", action="store_true", help="Emitir backfill")
    parser.add_argument("--skip-dlt", action="store_true", default=True, help="Omitir DLT por entidad (default)")
    parser.add_argument("--anchor-merkle", type=str, help="Ruta manifiesto JSON del lote")
    parser.add_argument("--correlation-id", type=str, help="ID lote auditable")
    parser.add_argument("--backfill-coverage", action="store_true", help="Poblar eda-coverage.json desde índices")
    parser.add_argument("--dry-run", action="store_true", help="Mostrar payloads sin emitir")
    args = parser.parse_args()

    repo = _repo_root()
    if args.backfill_coverage:
        result = backfill_coverage(repo)
        post = scan_orphans(repo)
        result["orphan_count_after"] = post.get("orphan_count")
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0 if post.get("orphan_count", 1) == 0 else 1

    if args.anchor_merkle:
        manifest_path = Path(args.anchor_merkle)
        if not manifest_path.is_absolute():
            manifest_path = repo / manifest_path
        acta = anchor_merkle(repo, manifest_path)
        print(json.dumps(acta, indent=2, ensure_ascii=False))
        return 0 if acta.get("transaction_digest") else 1

    report = scan_orphans(repo)
    if args.correlation_id:
        report["correlation_id"] = args.correlation_id

    if args.dry_run:
        for o in report.get("orphans") or []:
            print(json.dumps({"entity_name": o["entity_name"], "entity_uuid": o["entity_uuid"]}))
        return 0

    if args.emit:
        emits = emit_orphans(repo, report, skip_dlt=True, correlation_id=args.correlation_id)
        report["emits"] = emits
        report["emit_count"] = len(emits)
        report["emit_ok"] = sum(1 for e in emits if e.get("event_id") or e.get("idempotent"))
        post = scan_orphans(repo)
        report["orphan_count_after"] = post.get("orphan_count")
        manifest_path = repo / "docs" / "features" / "eda-domain-entities-splus" / "backfill-manifest.json"
        manifest_path.parent.mkdir(parents=True, exist_ok=True)
        manifest_path.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    if args.json or args.scan or args.emit:
        print(json.dumps(report, indent=2, ensure_ascii=False))
        if args.emit:
            return 0 if report.get("orphan_count_after", report.get("orphan_count", 1)) == 0 else 1
        return 0 if report.get("orphan_count", 0) == 0 else 1

    parser.print_help()
    return 1


if __name__ == "__main__":
    sys.exit(main())
