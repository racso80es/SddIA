# -*- coding: utf-8 -*-
"""Validación ECST instancia vs Clase catalogada en SddIA/events/ (genoma fractal)."""

from __future__ import annotations

import re
from pathlib import Path
from typing import Any


def _section_block(md_body: str, section: str) -> str:
    marker = f"### {section}"
    start = md_body.find(marker)
    if start == -1:
        return ""
    rest = md_body[start + len(marker) :]
    end = len(rest)
    for sep in ("\n### ", "\n## "):
        pos = rest.find(sep)
        if pos != -1:
            end = min(end, pos)
    return rest[:end]


def _parse_payload_fields(md_body: str, section: str) -> list[str]:
    block = _section_block(md_body, section)
    if not block:
        return []
    fields: list[str] = []
    for line in block.splitlines():
        trimmed = line.strip()
        if section == "FORBIDDEN":
            # Paridad ecst_validation.rs: solo líneas estrictas `- \`campo\``.
            if not trimmed.startswith("- `"):
                continue
            field_match = re.match(r"- `([^`]+)`", trimmed)
        else:
            field_match = re.search(r"`([^`]+)`", line)
        if field_match and not field_match.group(1).startswith("*"):
            fields.append(field_match.group(1))
    return fields


def _event_type_from_frontmatter(front: str) -> str | None:
    match = re.search(r'^event_type:\s*["\']?([^"\'\n]+)', front, re.M)
    return match.group(1).strip() if match else None


def load_event_class_schemas(repo: Path) -> dict[str, dict[str, list[str]]]:
    events_dir = repo / "SddIA" / "events"
    if not events_dir.is_dir():
        return {}
    schemas: dict[str, dict[str, list[str]]] = {}
    for class_path in sorted(events_dir.rglob("*.md")):
        if class_path.name in ("index.md", "events-contract.md"):
            continue
        text = class_path.read_text(encoding="utf-8-sig")
        if not text.startswith("---"):
            continue
        parts = text.split("---", 2)
        if len(parts) < 3:
            continue
        front, body = parts[1], parts[2]
        event_type = _event_type_from_frontmatter(front)
        if not event_type:
            continue
        schemas[event_type] = {
            "required": _parse_payload_fields(body, "REQUIRED"),
            "optional": _parse_payload_fields(body, "OPTIONAL"),
            "forbidden": _parse_payload_fields(body, "FORBIDDEN"),
        }
    return schemas


def validate_ecst_instance(
    event: dict[str, Any], schema: dict[str, list[str]] | None
) -> tuple[bool, list[str]]:
    errors: list[str] = []
    if schema is None:
        return False, ["event_type not cataloged in SddIA/events/ (genoma fractal)"]

    payload = event.get("payload")
    if not isinstance(payload, dict):
        return False, ["payload must be object"]

    for field in schema.get("required", []):
        if field not in payload or payload[field] is None:
            errors.append(f"missing required payload.{field}")

    for field in schema.get("forbidden", []):
        if field not in payload:
            continue
        value = payload[field]
        if field == "hash_signature":
            errors.append(f"forbidden payload.{field}")
        elif value is not None:
            errors.append(f"forbidden payload.{field} (must be null if present)")

    return not errors, errors


def validate_domain_mutation_event(repo: Path, event: dict[str, Any]) -> tuple[bool, list[str]]:
    event_type = event.get("event_type")
    if not isinstance(event_type, str) or not event_type.strip():
        return False, ["event_type missing or invalid"]
    schemas = load_event_class_schemas(repo)
    schema = schemas.get(event_type)
    return validate_ecst_instance(event, schema)
