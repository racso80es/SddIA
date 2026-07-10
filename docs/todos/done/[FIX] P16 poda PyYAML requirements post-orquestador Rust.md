---
document_id: PBI-FIX-P16-PYYAML-PODA
title: "[FIX] P16 poda PyYAML requirements post-orquestador Rust"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "cerrado"
priority: baja
process: bug-fix
related:
  - docs/features/migracion-execute-process-rust/implementation.md
  - docs/fixes/p16-pyyaml-poda/execution.md
  - docs/fixes/capsules-bridge-rust-port/execution.md
---

# [FIX] P16 — Poda PyYAML en `requirements.txt`

## Contexto

CA-8 cumple para el **entrypoint** orquestador (binario Rust, sin PyYAML en CLI). Tras PR #102, capsules bridge eliminado; tras este fix, route bridge wrapper eliminado y scripts QA migrados a parser Rust (`--parse-frontmatter`).

## Objetivo

Auditar consumidores de PyYAML en `SddIA/scripts/qa/`; podar `requirements.txt` solo cuando `grep` no reporte importaciones activas en paths productivos.

## Criterio de cierre

- `grep -r PyYAML|yaml` limpio en touchpoints productivos.
- `requirements.txt` sin PyYAML o con justificación documentada en execution.md.
- CI verde.

## Gate

Condicional P16 (clarify D6):

| Gate | Estado |
|------|--------|
| Capsules bridge (`_execute_process_capsules_bridge.py`) | ✅ cerrado — PR #102 |
| Route bridge (`_execute_process_route_bridge.py`) | ✅ cerrado — `python_core::invoke_route_domain_event` |
| `grep` limpio en touchpoints productivos QA | ✅ 0 importaciones directas |

Poda de `requirements.txt` aplicada (ver `docs/fixes/p16-pyyaml-poda/execution.md`).
