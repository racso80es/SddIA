#!/usr/bin/env python3
"""Reconstruye el catálogo de SddIA/actions/index.md (celdas de una línea)."""
from __future__ import annotations

import re
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
INDEX = REPO / "SddIA/actions/index.md"
ACTIONS = REPO / "SddIA/actions"


def split_md(text: str) -> tuple[str, str]:
    parts = text.split("---")
    if len(parts) < 3:
        return "", text
    return parts[1].strip(), "---".join(parts[2:]).lstrip("\n")


def sanitize_description(orchestration: str) -> str:
    text = orchestration.strip()
    chunks = text.split("##")
    purpose = text
    for ch in chunks:
        t = ch.strip().lower()
        if t.startswith("propósito") or t.startswith("proposito") or t.startswith("1. propósito"):
            purpose = ch
            break
    lines = [ln.strip() for ln in purpose.splitlines() if ln.strip() and not ln.strip().startswith("#")]
    if lines and lines[0].lower().startswith(("propósito", "proposito", "1.")):
        lines = lines[1:]
    para = " ".join(lines[:3]) if lines else "Acción del Core SddIA."
    para = " ".join(para.split()).replace("|", "/")
    return para if len(para) <= 220 else para[:217] + "..."


def caps_yaml_list(raw_yaml: str) -> str:
    caps = []
    in_caps = False
    for line in raw_yaml.splitlines():
        if line.strip().startswith("capabilities:"):
            in_caps = True
            continue
        if in_caps:
            if re.match(r"^\s*-\s+", line):
                m = re.search(r'["\']?([^"\']+)["\']?', line.split("-", 1)[-1])
                if m:
                    caps.append(f"`{m.group(1).strip()}`")
            elif line and not line.startswith(" "):
                break
    return ", ".join(caps)


def parse_action(path: Path) -> tuple[str, str, str, str, str, str] | None:
    raw = path.read_text(encoding="utf-8", errors="replace")
    ym, body = split_md(raw)
    name_m = re.search(r'^name:\s*["\']?([^"\'\n]+)', ym, re.M)
    uuid_m = re.search(r'^uuid:\s*["\']?([^"\'\n]+)', ym, re.M)
    ver_m = re.search(r'^version:\s*["\']?([^"\'\n]+)', ym, re.M)
    ctx_m = re.search(r'^context:\s*["\']?([^"\'\n]+)', ym, re.M)
    if not all([name_m, uuid_m, ver_m, ctx_m]):
        return None
    name = name_m.group(1).strip().strip('"')
    return (
        name,
        uuid_m.group(1).strip().strip('"'),
        ver_m.group(1).strip().strip('"'),
        ctx_m.group(1).strip().strip('"'),
        sanitize_description(body),
        caps_yaml_list(ym),
    )


def main() -> int:
    rows = []
    for path in sorted(ACTIONS.glob("*.md")):
        if path.name == "index.md" or path.name.endswith("-contract.md"):
            continue
        row = parse_action(path)
        if row:
            n, u, v, c, d, caps = row
            rows.append(f"| {n} | {u} | {v} | {c} | {d} | {caps} |")

    text = INDEX.read_text(encoding="utf-8")
    start = text.find("## Catálogo de definiciones")
    if start < 0:
        print("catalog section not found", flush=True)
        return 1
    end = text.find("\n## ", start + 1)
    if end < 0:
        end = len(text)
    catalog = (
        "## Catálogo de definiciones (`{name}.md`)\n\n"
        "| Name | UUID | Versión | Context | Descripción | Capabilities |\n"
        "|------|------|---------|---------|-------------|--------------|\n"
        + "\n".join(rows)
        + "\n"
    )
    new_text = text[:start] + catalog + text[end:]
    INDEX.write_text(new_text, encoding="utf-8")
    print(f"rebuilt {len(rows)} rows", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())