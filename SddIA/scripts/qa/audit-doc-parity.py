#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Sensor DIA: cruza diff del PR contra spec.md (impacts_doc). Alerta no bloqueante — sin agentes."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

from frontmatter_rust import parse_frontmatter_text

SCRIPT = Path(__file__).resolve()
DIA_HEADING = "### Impacto en Documentación"
DEFAULT_MONITORED = (
    "SddIA/core/",
    "SddIA/process/",
    "SddIA/scripts/qa/",
    "README.md",
)
PLACEHOLDER_LINES = frozenset({"- (ninguno)", "- (none)", "- ninguno", "- none"})


def _repo_root(explicit: str | None) -> Path:
    if explicit:
        return Path(explicit).resolve()
    if (SCRIPT.parents[2] / "tools").is_dir():
        return SCRIPT.parents[3]
    return SCRIPT.parents[2]


def _parse_frontmatter(text: str) -> dict[str, Any]:
    return parse_frontmatter_text(text)


def _dia_section_nonempty(body: str) -> bool:
    idx = body.find(DIA_HEADING)
    if idx < 0:
        return False
    rest = body[idx + len(DIA_HEADING) :]
    next_h = re.search(r"\n### ", rest)
    section = rest[: next_h.start()] if next_h else rest
    for line in section.splitlines():
        s = line.strip()
        if not s or s.startswith("<!--"):
            continue
        if s in PLACEHOLDER_LINES:
            continue
        return True
    return False


def _resolve_impacts_doc(fm: dict[str, Any]) -> bool | None:
    if "impacts_doc" not in fm:
        return None
    val = fm.get("impacts_doc")
    if isinstance(val, bool):
        return val
    if isinstance(val, str):
        low = val.strip().lower()
        if low in ("true", "yes", "1"):
            return True
        if low in ("false", "no", "0"):
            return False
    return None


def _git_diff_names(repo: Path, base_ref: str, head_ref: str) -> list[str]:
    for ref_spec in (f"origin/{base_ref}...{head_ref}", f"{base_ref}...{head_ref}"):
        proc = subprocess.run(
            ["git", "diff", "--name-only", ref_spec],
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            cwd=str(repo),
            check=False,
        )
        if proc.returncode == 0 and proc.stdout.strip():
            return [ln.strip().replace("\\", "/") for ln in proc.stdout.splitlines() if ln.strip()]
    proc = subprocess.run(
        ["git", "diff", "--name-only", "HEAD"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    if proc.returncode == 0:
        return [ln.strip().replace("\\", "/") for ln in proc.stdout.splitlines() if ln.strip()]
    return []


def _monitored_hits(
    diff_paths: list[str],
    prefixes: tuple[str, ...],
    persist_ref: str,
) -> list[str]:
    persist_prefix = persist_ref.strip().replace("\\", "/").rstrip("/") + "/"
    hits: list[str] = []
    for path in diff_paths:
        if path.startswith(persist_prefix):
            continue
        for prefix in prefixes:
            if prefix.endswith(".md"):
                if path == prefix or path.endswith("/" + prefix):
                    hits.append(path)
                    break
            elif path.startswith(prefix) or path == prefix.rstrip("/"):
                hits.append(path)
                break
    return sorted(set(hits))


def _evaluate(
    persist_ref: str,
    monitored_hits: list[str],
    impacts_doc: bool | None,
    dia_nonempty: bool,
) -> tuple[bool, str]:
    if not monitored_hits:
        return False, "no_monitored_diff"
    if impacts_doc is not True:
        return True, "impacts_doc_false_with_core_mutation"
    if not dia_nonempty:
        return True, "impacts_doc_true_empty_section"
    return False, "dia_declared_ok"


def audit(
    repo: Path,
    persist_ref: str,
    base_ref: str,
    head_ref: str,
    monitored_paths: tuple[str, ...],
    correlation_hint: str | None,
) -> dict[str, Any]:
    spec_path = repo / persist_ref.strip().replace("\\", "/") / "spec.md"
    if not spec_path.is_file():
        return {
            "success": False,
            "alert_required": False,
            "reason": "spec_missing",
            "error": f"spec.md no encontrado: {spec_path.relative_to(repo).as_posix()}",
            "persist_ref": persist_ref,
        }
    try:
        spec_text = spec_path.read_text(encoding="utf-8")
    except OSError as e:
        return {
            "success": False,
            "alert_required": False,
            "reason": "spec_read_error",
            "error": str(e),
            "persist_ref": persist_ref,
        }
    fm = _parse_frontmatter(spec_text)
    impacts_doc = _resolve_impacts_doc(fm)
    dia_nonempty = _dia_section_nonempty(spec_text)
    diff_paths = _git_diff_names(repo, base_ref, head_ref)
    hits = _monitored_hits(diff_paths, monitored_paths, persist_ref)
    alert_required, reason = _evaluate(persist_ref, hits, impacts_doc, dia_nonempty)
    payload: dict[str, Any] = {
        "success": True,
        "alert_required": alert_required,
        "reason": reason,
        "persist_ref": persist_ref,
        "impacts_doc": impacts_doc,
        "dia_section_nonempty": dia_nonempty,
        "monitored_hits": hits,
    }
    if correlation_hint:
        payload["correlation_hint"] = correlation_hint
    return payload


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default="", help="Raíz del repositorio")
    parser.add_argument("--persist-ref", required=True, help="Ruta docs/features/…")
    parser.add_argument("--base-ref", default="main", help="Ref base del diff")
    parser.add_argument("--head-ref", default="HEAD", help="Ref head del diff")
    parser.add_argument(
        "--monitored-paths",
        default=",".join(DEFAULT_MONITORED),
        help="Prefijos CSV a monitorizar",
    )
    parser.add_argument("--alert-file", default="", help="Ruta JSON alerta (opcional, .tmp/)")
    parser.add_argument("--json", action="store_true", help="Emitir JSON en stdout")
    parser.add_argument(
        "--correlation-hint",
        default="",
        help="ID correlación (opcional, no invoca agentes)",
    )
    args = parser.parse_args()

    repo = _repo_root(args.repo_root or None)
    if not (repo / "SddIA").is_dir():
        err = {"success": False, "error": f"repo inválido: {repo}", "alert_required": False}
        print(json.dumps(err, ensure_ascii=False))
        return 2

    prefixes = tuple(p.strip().replace("\\", "/") for p in args.monitored_paths.split(",") if p.strip())
    payload = audit(
        repo,
        args.persist_ref.strip(),
        args.base_ref.strip(),
        args.head_ref.strip(),
        prefixes or DEFAULT_MONITORED,
        args.correlation_hint.strip() or None,
    )

    if not payload.get("success"):
        print(json.dumps(payload, ensure_ascii=False, indent=2 if args.json else None))
        return 2

    text = json.dumps(payload, ensure_ascii=False, indent=2)
    print(text)
    if args.alert_file.strip():
        alert_path = Path(args.alert_file.strip())
        alert_path.parent.mkdir(parents=True, exist_ok=True)
        alert_path.write_text(text + "\n", encoding="utf-8")

    return 0


if __name__ == "__main__":
    sys.exit(main())
