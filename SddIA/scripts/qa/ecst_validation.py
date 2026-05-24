# -*- coding: utf-8 -*-
"""Validación ECST instancia vs Clase catalogada en SddIA/events/."""

from __future__ import annotations

import re
from pathlib import Path
from typing import Any


def _parse_payload_fields(md_body: str, section: str) -> list[str]:
    pattern = rf"### {section}\s*\n((?:- .+\n?)*)"
    match = re.search(pattern, md_body)
    if not match:
        return []
    fields: list[str] = []
    for line in match.group(1).splitlines():
        field_match = re.search(r"`([^`]+)`", line)
        if field_match and not field_match.group(1).startswith("*"):
            fields.append(field_match.group(1))
    return fields


def load_event_class_schemas(repo: Path) -> dict[str, dict[str, list[str]]]:
    events_dir = repo / "SddIA" / "events"
    index_path = events_dir / "index.md"
    if not index_path.is_file():
        return {}
    index_text = index_path.read_text(encoding="utf-8")
    schemas: dict[str, dict[str, list[str]]] = {}
    row_re = re.compile(r"\|\s*`([^`]+\.md)`\s*\|[^|]+\|[^|]+\|\s*(\S+)\s*\|")
    for row_match in row_re.finditer(index_text):
        filename, event_type = row_match.group(1), row_match.group(2)
        class_path = events_dir / filename
        if not class_path.is_file():
            continue
        body = class_path.read_text(encoding="utf-8")
        if body.startswith("---"):
            parts = body.split("---", 2)
            body = parts[2] if len(parts) >= 3 else body
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
        return False, ["event_type not cataloged in SddIA/events/index.md"]

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
