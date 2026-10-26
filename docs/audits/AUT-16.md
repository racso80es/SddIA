---
id: llm-interaction-audit-AUT-16
type: audit
version: "1.0.0"
tracker_ref: AUT-16
process: llm-interaction-audit
created: "2026-10-26"
---

# Auditoría de interacciones LLM — AUT-16

| Timestamp (UTC) | Rol | Tipo | Resumen |
| --- | --- | --- | --- |

## Fricción — `./sddia-run.sh`

| Campo | Valor |
| --- | --- |

| Comando | `./sddia-run.sh --process bug-fix` |
| `success` | `false` |
| `error_code` | `verification-handoff-incomplete` |
| handler | `agent-runtime` |
| mensaje | handoff de Verificación sin branch o execution_id; no se escribe validacion.md |
