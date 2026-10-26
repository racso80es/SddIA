---
feature_name: llm-interaction-audit
created: "2026-10-26"
process: feature
tracker_ref: AUT-7
---

# Spec

## Activación

| Campo | Valores |
| --- | --- |
| `llm_audit` | `active` \| `inactive` (ausente = inactive) |

Fuente: descripción sellada del ítem (`tracker:lifecycle` / `fetch_issue`).

## API del proceso

| `operation` | Comportamiento |
| --- | --- |
| `record_interaction` | Añade fila si `llm_audit: active`; dedup `(timestamp, resumen)` |
| `record_friction` | Sección **Fricción** (no fila `code_edit`) |
| `probe_active` | Lectura sin escritura |

## Rutas

`paths.auditsPath` → `docs/audits/{tracker_ref}.md`
