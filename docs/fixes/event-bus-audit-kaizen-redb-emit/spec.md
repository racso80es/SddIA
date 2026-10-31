---
fix_name: event-bus-audit-kaizen-redb-emit
created: "2026-10-30"
process: bug-fix
branch_name: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/fixes/event-bus-audit-kaizen-redb-emit
---

# Spec — Kaizen emit alineado a `SDDIA_EVENT_STORE`

## Cambio

`write_pending_kaizen` delega persistencia en `stamp_published_event` + `record_ecst` cuando `writes_store`; escribe JSON en `paths.pending` solo si `writes_filesystem`.

## AC

| ID | Criterio |
|----|----------|
| AC-1 | `redb`: sin fichero `.events/pending/{id}.json`; evento en store |
| AC-2 | `filesystem`: fichero pending |
| AC-5 | Tests `needs_kaizen_*` sin regresión |

## Archivos

- `SddIA/tools/event-bus-audit/src/main.rs`
- `SddIA/tools/event-bus-audit/Cargo.toml`
- `SddIA/tools/event-bus-audit.md` v1.2.1
- `SddIA/process/event-bus-audit.md`
