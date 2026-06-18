---
document_id: PBI-FIX-P16-PYYAML-PODA
title: "[FIX] P16 poda PyYAML requirements post-orquestador Rust"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "abierto"
priority: baja
process: bug-fix
related:
  - docs/features/migracion-execute-process-rust/implementation.md
  - SddIA/scripts/qa/requirements.txt
---

# [FIX] P16 — Poda PyYAML en `requirements.txt`

## Contexto

CA-8 cumple para el **entrypoint** orquestador (binario Rust, sin PyYAML en CLI). Scripts QA residuales (`execute_process_capsules.py`, audit, route core) pueden seguir consumiendo PyYAML hasta cerrar bridges P17+.

## Objetivo

Auditar consumidores de PyYAML en `SddIA/scripts/qa/`; podar `requirements.txt` solo cuando `grep` no reporte importaciones activas en paths productivos.

## Criterio de cierre

- `grep -r PyYAML|yaml` limpio en touchpoints productivos.
- `requirements.txt` sin PyYAML o con justificación documentada en execution.md.
- CI verde.

## Gate

Condicional P16 (clarify D6); ejecutar tras cierre route bridge + capsules bridge.
