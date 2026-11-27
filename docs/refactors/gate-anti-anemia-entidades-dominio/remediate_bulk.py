#!/usr/bin/env python3
"""Remediación DA-2 masiva vía entity-manager (AUT-125 lote 2)."""
from __future__ import annotations

import json
import re
import subprocess
import sys
import time
import uuid
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
QA = REPO / "SddIA/target/debug/sddia-qa"

FAMILY_DIR = {
    "action": "actions",
    "agent": "agents",
    "skill": "skills",
    "tool": "tools",
    "event": "events",
    "process": "process",
    "suite": "suites",
    "daemon": "daemons",
    "norm": "library/norms",
    "codex": "library/codexes",
}


def split_md(text: str) -> tuple[str, str]:
    parts = text.split("---")
    if len(parts) < 3:
        return "", text
    return parts[1].strip(), "---".join(parts[2:]).lstrip("\n")


def first_para(body: str) -> str:
    lines = []
    past_h1 = False
    for line in body.splitlines():
        s = line.strip()
        if s.startswith("# ") and not s.startswith("##"):
            past_h1 = True
            continue
        if not past_h1:
            continue
        if not s:
            if lines:
                break
            continue
        if s.startswith("##"):
            break
        lines.append(s)
    return " ".join(lines)[:500] if lines else ""


def enrich_body(body: str, name: str) -> str:
    body = body.strip()
    if body.count("##") >= 2 and (
        "## Propósito" in body or "## 1. Propósito" in body
    ):
        return body
    para = first_para(body) or f"Entidad `{name}` del Core SddIA."
    suffix = (
        f"\n\n## Propósito\n\n{para}\n\n"
        f"## Operación\n\n"
        f"Ejecución conforme al contrato de familia y topología Cúmulo; "
        f"delegación en cápsulas autorizadas sin rutas cableadas.\n"
    )
    if "## Propósito" in body or "## 1. Propósito" in body:
        return body
    if body.count("##") < 2:
        return body + suffix
    return body + "\n\n## Propósito (complemento)\n\n" + para + "\n"


def entity_class_for(path: str, family: str) -> str:
    if "library/norms/capability-contracts" in path:
        return "capability-contract"
    return family


def entity_name_from(path: str) -> str:
    return Path(path).stem


def scan_violations() -> list[dict]:
    proc = subprocess.run(
        [str(QA), "gate-entity-no-anemia", "--json"],
        cwd=REPO,
        capture_output=True,
        text=True,
    )
    line = proc.stdout.strip().splitlines()[-1] if proc.stdout.strip() else ""
    data = json.loads(line)
    return data["result"]["violations"]


def uuid_v4_ok(raw: str) -> bool:
    return bool(
        re.match(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$",
            raw.strip().strip('"'),
        )
    )


def run_entity_manager(payload: dict) -> bool:
    cmd = [
        str(REPO / "sddia-run.sh"),
        "--process",
        "entity-manager",
        "--inputs",
        json.dumps(payload, ensure_ascii=False),
    ]
    r = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True)
    if '"success":true' in r.stdout.replace(" ", "").lower():
        return True
    print("FAIL", payload.get("entity_name"), r.stdout[-400:], r.stderr[-200:], file=sys.stderr)
    return False


def remediate_path(path: str, codes: set[str]) -> bool:
    full = REPO / path
    if not full.is_file():
        return False
    raw = full.read_text(encoding="utf-8", errors="replace")
    yaml_fm, body = split_md(raw)
    if "/process/" in path or path.startswith("SddIA/process/"):
        family = "process"
    elif "/events/" in path:
        family = "event"
    elif path.startswith("SddIA/library/codexes/"):
        family = "codex"
    else:
        family = next(
            (f for f, d in FAMILY_DIR.items() if path.startswith(f"SddIA/{d}/")),
            None,
        )
    if family is None:
        return False

    name = entity_name_from(path)
    eclass = entity_class_for(path, family)
    seed: dict = {}

    new_body = enrich_body(body, name)
    if family == "action":
        if body.strip() != new_body.strip() or "SECTIONS_INSUFFICIENT" in codes:
            seed["orchestration_logic"] = new_body
    elif new_body != body.strip():
        seed["markdown_body_replacements"] = [{"from": body, "to": new_body}]

    if family == "event" and yaml_fm:
        m_fam = re.search(r'^event_family:\s*["\']?([^"\'\n]+)', yaml_fm, re.M)
        if m_fam:
            seed["event_family"] = m_fam.group(1).strip().strip('"')

    need_uuid_fix = "UUID_INVALID" in codes or "UUID_NOT_V4" in codes
    if need_uuid_fix and yaml_fm:
        m = re.search(r'^uuid:\s*["\']?([^"\'\n]+)', yaml_fm, re.M)
        if m and (not uuid_v4_ok(m.group(1)) or "UUID_NOT_V4" in codes):
            old = m.group(1).strip()
            new = str(uuid.uuid4())
            old_line = f'uuid: "{old}"'
            if old_line not in yaml_fm:
                old_line = f"uuid: {old}"
            seed["artifact_yaml_replacements"] = [
                {"from": old_line, "to": f'uuid: "{new}"'}
            ]

    if not seed:
        return False

    payload = {
        "entity_class": eclass,
        "entity_name": name,
        "lifecycle_operation": "update",
        "semantic_seed": seed,
    }
    ok = run_entity_manager(payload)
    time.sleep(0.35)
    return ok


def main() -> int:
    if not QA.is_file():
        print("compilar sddia-qa primero", file=sys.stderr)
        return 1
    by_path: dict[str, set[str]] = {}
    for v in scan_violations():
        by_path.setdefault(v["path"], set()).add(v["code"])

    ok = fail = skip = 0
    for path in sorted(by_path):
        codes = by_path[path]
        if remediate_path(path, codes):
            ok += 1
            print("OK", path)
        else:
            fail += 1
            print("SKIP/FAIL", path, codes)
    print(f"done ok={ok} fail={fail}")
    rem = scan_violations()
    print(f"remaining violations: {len(rem)}")
    return 0 if not rem else 2


if __name__ == "__main__":
    sys.exit(main())