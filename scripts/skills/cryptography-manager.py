#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula cryptography-manager: I/O JSON estricto por stdin/stdout."""

from __future__ import annotations

import hashlib
import json
import sys
import uuid
from pathlib import Path

OPS = frozenset({"GENERATE_SHA256", "VALIDATE_HASH", "GENERATE_UUID"})
TYPES = frozenset({"STRING", "FILE_PATH"})


def _fail(msg: str) -> None:
    out = {"success": False, "exitCode": 1, "error": msg}
    sys.stdout.write(json.dumps(out, ensure_ascii=False))
    sys.exit(1)


def _ok(result) -> None:
    out = {"success": True, "exitCode": 0, "data": {"result": result}}
    sys.stdout.write(json.dumps(out, ensure_ascii=False))
    sys.exit(0)


def _sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _safe_read_file(rel: str) -> bytes:
    p = Path(rel)
    if p.is_absolute():
        _fail("FILE_PATH must be relative to workspace")
    cwd = Path.cwd().resolve()
    try:
        resolved = (cwd / p).resolve()
        resolved.relative_to(cwd)
    except (OSError, ValueError):
        _fail("path escapes workspace or is invalid")
    if not resolved.is_file():
        _fail("FILE_PATH does not exist or is not a file")
    return resolved.read_bytes()


def main() -> None:
    try:
        raw = sys.stdin.read()
        data = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _fail(f"invalid JSON stdin: {e}")

    op = data.get("operation")
    tt = data.get("target_type")
    payload = data.get("target_payload")

    if op not in OPS:
        _fail(f"operation must be one of {sorted(OPS)}")
    if op != "GENERATE_UUID" and tt not in TYPES:
        _fail(f"target_type must be one of {sorted(TYPES)}")

    if op == "GENERATE_UUID":
        _ok(str(uuid.uuid4()))

    if op == "GENERATE_SHA256":
        if tt == "STRING":
            if not isinstance(payload, str):
                _fail("target_payload must be a string for GENERATE_SHA256+STRING")
            digest = _sha256_bytes(payload.encode("utf-8"))
            _ok(digest)
        if tt == "FILE_PATH":
            if not isinstance(payload, str):
                _fail("target_payload must be a string path for GENERATE_SHA256+FILE_PATH")
            b = _safe_read_file(payload)
            _ok(_sha256_bytes(b))

    if op == "VALIDATE_HASH":
        if not isinstance(payload, dict):
            _fail("target_payload must be a JSON object for VALIDATE_HASH")
        expected = payload.get("expected_sha256")
        if not isinstance(expected, str) or len(expected) != 64:
            _fail("expected_sha256 must be a 64-char hex string")
        try:
            bytes.fromhex(expected)
        except ValueError:
            _fail("expected_sha256 is not valid hex")
        expected = expected.lower()

        if tt == "STRING":
            subj = payload.get("subject")
            if not isinstance(subj, str):
                _fail("subject must be a string for STRING validation")
            actual = _sha256_bytes(subj.encode("utf-8"))
        else:
            path = payload.get("path")
            if not isinstance(path, str):
                _fail("path must be a string for FILE_PATH validation")
            actual = _sha256_bytes(_safe_read_file(path))

        _ok(actual == expected.lower())

    _fail("unsupported operation")


if __name__ == "__main__":
    main()
