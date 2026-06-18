#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Paridad P6/P7: forja Rust vs Python (hash_signature + cuerpo normalizado)."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
QA = SCRIPT.parent
if str(QA) not in sys.path:
    sys.path.insert(0, str(QA))

from execute_process_forges import run_tool_forge, try_native_forge  # noqa: E402
from lab_teardown import cleanup_lab_entity_forge  # noqa: E402

UUID_RE = re.compile(
    r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",
    re.I,
)
LAB_UUID = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"
LAB_SHA = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
TOOL_NAME = "forge-parity-lab"


def repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("repo root not found")


def normalize_artifact(text: str) -> str:
    text = UUID_RE.sub("<UUID>", text)
    text = re.sub(r"sha256:[0-9a-f]{64}", "sha256:<HEX>", text, flags=re.I)
    return text


def main() -> int:
    repo = repo_root()
    os.environ["SDDIA_FORGE_LAB_UUID"] = LAB_UUID
    os.environ["SDDIA_FORGE_LAB_SHA256"] = LAB_SHA
    os.environ.pop("SDDIA_DISABLE_NATIVE_FORGES", None)

    cleanup_lab_entity_forge(
        repo, entity_class="tool", entity_name=TOOL_NAME, event_id=None
    )

    inputs: dict[str, Any] = {
        "entity_class": "tool",
        "tool_name": TOOL_NAME,
        "scope": "local",
        "execution_logic": "P6/P7 parity smoke",
        "lifecycle_operation": "create",
    }

    try:
        py_out = run_tool_forge(repo, inputs)
        rust_out = try_native_forge(repo, inputs)
        if rust_out is None:
            print("ERR native forge unavailable")
            return 1

        py_hash = py_out.get("handoff_hash_signature_new")
        rust_hash = rust_out.get("handoff_hash_signature_new")
        if py_hash != rust_hash:
            print(f"FAIL hash py={py_hash} rust={rust_hash}")
            return 1

        artifact = repo / ".SddIA" / "tools" / f"{TOOL_NAME}.md"
        if not artifact.is_file():
            print("ERR artifact missing")
            return 1

        rust_idem = try_native_forge(repo, inputs)
        if not rust_idem or not rust_idem.get("idempotent"):
            print("FAIL idempotent native forge", rust_idem)
            return 1

        body = normalize_artifact(artifact.read_text(encoding="utf-8"))
        if TOOL_NAME not in body or "P6/P7 parity smoke" not in body:
            print("FAIL artifact content")
            return 1

        print("OK  forge-parity tool local hash + idempotent")
        return 0
    finally:
        cleanup_lab_entity_forge(
            repo, entity_class="tool", entity_name=TOOL_NAME, event_id=None
        )


if __name__ == "__main__":
    raise SystemExit(main())
