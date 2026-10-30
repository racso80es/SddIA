#!/usr/bin/env python3
"""Lab: entity-manager updates for doc-drift P1 (actions). Not genoma — invoca forja oficial."""
from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
SDDIA_RUN = REPO / "sddia-run.sh"

DESCRIPTIONS = {
    "emit-pr-presented-event": "Emite PullRequest_Presented; persistencia según SDDIA_EVENT_STORE (events-contract §4).",
    "emit-pr-audited-event": "Emite PullRequest_Audited; persistencia según SDDIA_EVENT_STORE (events-contract §4).",
    "emit-pr-resealed-event": "Emite PullRequest_Resealed; persistencia según SDDIA_EVENT_STORE (events-contract §4).",
    "emit-pr-merged-event": "Emite PullRequest_Merged en bus; medio store/FS según SDDIA_EVENT_STORE (cumulo.paths).",
}

PERSIST_STEP = """### Paso 4 — Persistencia

Según `SDDIA_EVENT_STORE` (`SddIA/events/events-contract.md` §4): en `redb`/`dual` el runtime persiste vía `write_pending_event` en `.SddIA/runtime/events.redb` (sin fichero bajo `.events/pending/` en `redb` puro). En `filesystem`, JSON en `{eda_bus.pending}/<event_id>.json` (resoluble vía `cumulo.paths.json`). El `target_path` devuelto puede ser la ruta lógica `.events/pending/<event_id>.json` aunque el fichero no exista en `redb`."""

PERSIST_MERGED = """### Paso 4 — Persistencia

Según `SDDIA_EVENT_STORE` (`SddIA/events/events-contract.md` §4): en `redb`/`dual` el runtime persiste vía `write_pending_event` en `.SddIA/runtime/events.redb` (sin fichero bajo `.events/pending/` en `redb` puro). En `filesystem`, invocar `skill:filesystem-manager` con `WRITE_FILE` a `{eda_bus.pending}/<event_id>.json` (crear directorio vía `CREATE_DIR` si falta). El `target_path` devuelto puede ser la ruta lógica `.events/pending/<event_id>.json` aunque el fichero no exista en `redb`."""

PURPOSE_SUB = (
    "persiste el JSON en pending",
    "registra el ECST (medio según `events-contract.md` §4)",
)


def split_frontmatter(text: str) -> tuple[str, str]:
    if not text.startswith("---"):
        raise ValueError("no frontmatter")
    end = text.index("---", 3)
    return text[3:end].strip(), text[end + 3 :].lstrip("\n")


def yaml_list_to_json(yaml_block: str, key: str):
    lines = yaml_block.splitlines()
    out = []
    in_key = False
    for line in lines:
        if line.startswith(f"{key}:"):
            in_key = True
            continue
        if in_key:
            if re.match(r"^[a-z_]+:", line) and not line.startswith("  "):
                break
            m = re.match(r'^\s+-\s+"([^"]+)":\s*(.*)$', line)
            if m:
                out.append({m.group(1): m.group(2).strip().strip('"')})
    return out


def patch_body(name: str, body: str) -> str:
    body = body.replace(PURPOSE_SUB[0], PURPOSE_SUB[1])
    if name == "emit-pr-merged-event":
        body = re.sub(
            r"### Paso 4 — Persistencia.*?### Paso 5",
            PERSIST_MERGED + "\n\n### Paso 5",
            body,
            flags=re.DOTALL,
        )
    else:
        body = re.sub(
            r"### Paso 4 — Persistencia.*?\n\nEscribir.*?\.\n",
            PERSIST_STEP + "\n",
            body,
            flags=re.DOTALL,
        )
        body = re.sub(
            r"### Paso 4 — Persistencia \(`skill:filesystem-manager`\)\n\nEscribir.*?\.\n",
            PERSIST_STEP + "\n",
            body,
        )
    return body


def run_em(entity_name: str, seed: dict) -> None:
    payload = {
        "entity_class": "action",
        "entity_name": entity_name,
        "lifecycle_operation": "update",
        "semantic_seed": seed,
    }
    proc = subprocess.run(
        [str(SDDIA_RUN), "--process", "entity-manager", "--inputs", json.dumps(payload)],
        cwd=REPO,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(proc.stdout, proc.stderr, file=sys.stderr)
        raise SystemExit(f"entity-manager failed for {entity_name}: {proc.returncode}")
    print(f"OK {entity_name}")


def main() -> None:
    actions = [
        "emit-pr-presented-event",
        "emit-pr-audited-event",
        "emit-pr-resealed-event",
        "emit-pr-merged-event",
    ]
    for name in actions:
        path = REPO / "SddIA/actions" / f"{name}.md"
        yaml_block, body = split_frontmatter(path.read_text())
        new_body = patch_body(name, body)
        if new_body == body and name in path.read_text():
            new_body = body  # idempotent re-run: keep body
        inputs = yaml_list_to_json(yaml_block, "inputs")
        outputs = yaml_list_to_json(yaml_block, "outputs")
        caps = []
        in_caps = False
        for line in yaml_block.splitlines():
            if line.startswith("capabilities:"):
                in_caps = True
                continue
            if in_caps:
                if line.startswith("inputs:"):
                    break
                m = re.match(r'^\s+-\s+"([^"]+)"\s*$', line)
                if m:
                    caps.append(m.group(1))
        seed = {
            "action_name": name,
            "action_body": new_body,
            "action_inputs": inputs,
            "action_outputs": outputs,
            "action_capabilities": caps,
            "orchestration_logic": DESCRIPTIONS.get(name, f"Acción {name}"),
        }
        run_em(name, seed)


if __name__ == "__main__":
    main()
