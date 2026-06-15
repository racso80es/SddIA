#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula markdown-table-editor: mutación segura de tablas Markdown."""

from __future__ import annotations

import argparse
import json
import re
import sys
import tempfile
from pathlib import Path
from typing import Any

TOOL_NAME = "markdown-table-editor"


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _emit(envelope: dict[str, Any]) -> None:
    code = envelope.get("exitCode", 0 if envelope.get("success") else 1)
    envelope.setdefault("name", TOOL_NAME)
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _fail(message: str, *, code: int = 1) -> None:
    _emit({
        "name": TOOL_NAME,
        "success": False,
        "exitCode": code,
        "message": message,
        "error": message,
        "result": None,
    })


def _resolve_file(repo: Path, file_path: str) -> Path:
    target = (repo / file_path).resolve()
    repo_resolved = repo.resolve()
    if not str(target).startswith(str(repo_resolved)):
        raise ValueError(f"file_path fuera del workspace: {file_path}")
    return target


def _split_cells(line: str) -> list[str]:
    stripped = line.strip()
    if not stripped.startswith("|"):
        return []
    inner = stripped.strip("|")
    return [c.strip() for c in inner.split("|")]


def _is_separator_row(line: str) -> bool:
    return bool(re.match(r"^\|\s*[-:]+", line.strip()))


def _is_table_row(line: str) -> bool:
    return line.strip().startswith("|") and not _is_separator_row(line)


def _locate_tables(lines: list[str]) -> list[dict[str, Any]]:
    tables: list[dict[str, Any]] = []
    i = 0
    while i < len(lines):
        if not _is_table_row(lines[i]):
            i += 1
            continue
        header_idx = i
        headers = _split_cells(lines[i])
        i += 1
        if i < len(lines) and _is_separator_row(lines[i]):
            sep_idx = i
            i += 1
        else:
            sep_idx = None
        data_start = i
        data_rows: list[int] = []
        while i < len(lines) and _is_table_row(lines[i]):
            data_rows.append(i)
            i += 1
        tables.append(
            {
                "header_idx": header_idx,
                "separator_idx": sep_idx,
                "data_row_indices": data_rows,
                "headers": headers,
            }
        )
    return tables


def _column_index(headers: list[str], key_column: str | int | None) -> int | None:
    if key_column is None:
        return None
    if isinstance(key_column, int):
        return key_column if 0 <= key_column < len(headers) else None
    key = str(key_column).strip().lower()
    for idx, h in enumerate(headers):
        if h.lower() == key:
            return idx
    return None


def _row_matches(
    line: str,
    headers: list[str],
    key_column: str | int | None,
    row_data: dict[str, Any],
    match_token: str | None,
) -> bool:
    if not _is_table_row(line):
        return False
    cells = _split_cells(line)
    col = _column_index(headers, key_column)
    if col is not None and col < len(cells):
        key_val = row_data.get(headers[col]) if headers[col] in row_data else None
        if key_val is None:
            for v in row_data.values():
                key_val = v
                break
        if key_val is not None and str(key_val) == cells[col]:
            return True
        if key_val is not None and str(key_val) in cells[col]:
            return True
    token = match_token or row_data.get("token") or row_data.get("match_token")
    if token is not None:
        return str(token) in line
    for v in row_data.values():
        if v is not None and str(v) in line:
            return True
    return False


def _format_row(cells: list[str]) -> str:
    return "| " + " | ".join(cells) + " |\n"


def _atomic_write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        mode="w",
        encoding="utf-8",
        delete=False,
        dir=str(path.parent),
        suffix=".tmp",
    ) as tmp:
        tmp.write(content)
        tmp_path = Path(tmp.name)
    tmp_path.replace(path)


class TableSession:
    def __init__(self, path: Path) -> None:
        self.path = path
        self.text = path.read_text(encoding="utf-8")
        self.lines = self.text.splitlines(keepends=True)
        self.tables = _locate_tables(self.lines)
        self.modified = False

    def _table(self, table_index: int) -> dict[str, Any]:
        if table_index < 0 or table_index >= len(self.tables):
            raise IndexError(f"table_index {table_index} fuera de rango ({len(self.tables)} tablas)")
        return self.tables[table_index]

    def parse(self, table_index: int) -> dict[str, Any]:
        tbl = self._table(table_index)
        rows = [_split_cells(self.lines[i]) for i in tbl["data_row_indices"]]
        return {
            "headers": tbl["headers"],
            "rows": rows,
            "row_count": len(rows),
            "target_path": str(self.path),
        }

    def row_exists(
        self,
        table_index: int,
        key_column: str | int | None,
        row_data: dict[str, Any],
        match_token: str | None,
    ) -> bool:
        tbl = self._table(table_index)
        for idx in tbl["data_row_indices"]:
            if _row_matches(self.lines[idx], tbl["headers"], key_column, row_data, match_token):
                return True
        return False

    def delete_row(
        self,
        table_index: int,
        key_column: str | int | None,
        row_data: dict[str, Any],
        match_token: str | None,
    ) -> int:
        tbl = self._table(table_index)
        removed = 0
        new_lines: list[str] = []
        skip: set[int] = set()
        for idx in tbl["data_row_indices"]:
            if _row_matches(self.lines[idx], tbl["headers"], key_column, row_data, match_token):
                skip.add(idx)
                removed += 1
        if not removed:
            return 0
        for i, line in enumerate(self.lines):
            if i not in skip:
                new_lines.append(line)
        self.lines = new_lines
        self.tables = _locate_tables(self.lines)
        self.modified = True
        return removed

    def upsert_row(
        self,
        table_index: int,
        key_column: str | int | None,
        row_data: dict[str, Any],
        match_token: str | None,
    ) -> str:
        tbl = self._table(table_index)
        headers = tbl["headers"]
        if not headers:
            raise ValueError("tabla sin cabeceras")
        cells = [str(row_data.get(h, "")) for h in headers]
        if not any(cells):
            cells = [str(v) for v in row_data.values()]
            while len(cells) < len(headers):
                cells.append("")
        new_line = _format_row(cells[: len(headers)])
        for idx in tbl["data_row_indices"]:
            if _row_matches(self.lines[idx], headers, key_column, row_data, match_token):
                if self.lines[idx] != new_line:
                    self.lines[idx] = new_line
                    self.modified = True
                return "updated"
        insert_at = tbl["data_row_indices"][-1] + 1 if tbl["data_row_indices"] else (
            (tbl["separator_idx"] or tbl["header_idx"]) + 1
        )
        self.lines.insert(insert_at, new_line)
        self.tables = _locate_tables(self.lines)
        self.modified = True
        return "inserted"

    def save(self, dry_run: bool) -> None:
        if not self.modified:
            return
        if dry_run:
            return
        _atomic_write(self.path, "".join(self.lines))


def _run(payload: dict[str, Any]) -> dict[str, Any]:
    file_path = payload.get("file_path")
    operation = payload.get("operation")
    if not isinstance(file_path, str) or not file_path.strip():
        raise ValueError("file_path es obligatorio")
    if not isinstance(operation, str) or not operation.strip():
        raise ValueError("operation es obligatorio")

    repo = _repo_root()
    target = _resolve_file(repo, file_path.strip())
    if not target.is_file():
        raise FileNotFoundError(f"Archivo no encontrado: {file_path}")

    table_index = int(payload.get("table_index", 0))
    key_column = payload.get("key_column")
    row_data = payload.get("row_data") if isinstance(payload.get("row_data"), dict) else {}
    match_token = payload.get("match_token")
    if match_token is None and isinstance(row_data.get("token"), str):
        match_token = row_data["token"]
    dry_run = bool(payload.get("dry_run", False))

    session = TableSession(target)
    op = operation.strip().lower()

    if op == "parse":
        result = session.parse(table_index)
        return {
            "success": True,
            "exitCode": 0,
            "message": f"Tabla {table_index} parseada ({result['row_count']} filas).",
            "result": result,
        }

    if op == "row_exists":
        exists = session.row_exists(table_index, key_column, row_data, match_token)
        return {
            "success": True,
            "exitCode": 0,
            "message": "Fila encontrada." if exists else "Fila no encontrada.",
            "result": {"exists": exists, "target_path": str(target)},
        }

    if op == "delete_row":
        removed = session.delete_row(table_index, key_column, row_data, match_token)
        session.save(dry_run)
        return {
            "success": True,
            "exitCode": 0,
            "message": f"{removed} fila(s) eliminada(s)." if removed else "Sin filas a eliminar (idempotente).",
            "result": {
                "modified": removed > 0,
                "rows_removed": removed,
                "target_path": str(target.relative_to(repo)).replace("\\", "/"),
            },
        }

    if op == "upsert_row":
        action = session.upsert_row(table_index, key_column, row_data, match_token)
        session.save(dry_run)
        return {
            "success": True,
            "exitCode": 0,
            "message": f"Fila {action}.",
            "result": {"modified": session.modified, "action": action, "target_path": str(target)},
        }

    if op == "save":
        session.save(dry_run)
        return {
            "success": True,
            "exitCode": 0,
            "message": "Persistencia completada." if session.modified else "Sin cambios pendientes.",
            "result": {"modified": session.modified, "target_path": str(target)},
        }

    raise ValueError(f"operation no soportada: {operation}")


def main() -> None:
    parser = argparse.ArgumentParser(description=TOOL_NAME)
    parser.add_argument("--request-file", help="JSON de petición")
    args = parser.parse_args()

    try:
        if args.request_file:
            raw = Path(args.request_file).read_text(encoding="utf-8-sig")
        else:
            raw = sys.stdin.read()
        payload = json.loads(raw) if raw.strip() else {}
        if not isinstance(payload, dict):
            raise ValueError("payload debe ser objeto JSON")
        out = _run(payload)
        out["name"] = TOOL_NAME
        _emit(out)
    except Exception as e:
        _fail(str(e))


if __name__ == "__main__":
    main()
