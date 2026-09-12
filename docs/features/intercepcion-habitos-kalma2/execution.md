---
feature_name: intercepcion-habitos-kalma2
created: "2026-09-12"
process: feature
branch_name: feat/intercepcion-habitos-kalma2
persist_ref: docs/features/intercepcion-habitos-kalma2
execution_id: "54b02c30-b579-4d0b-a7e0-272d3c3c6af0"
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
items_applied:
  - forge-action-em
  - genome-conscience
  - handler-tendon-dispatch
  - destilar-hint-hash
  - triage-key-candidates
  - tests-lab
  - evolution-register
---

# Ejecución — intercepcion-habitos-kalma2

## Init

`SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `54b02c30-b579-4d0b-a7e0-272d3c3c6af0`. Commit planificación `a3272a0`.

## Genoma

| Entidad | Vía | Sello |
|---------|-----|-------|
| action `dispatch-aiua-intent` 1.1.0 | `entity-manager` update `cf5993bc-9ee4-4210-b078-a471af88ee5d` | Domain_Entity_Updated `9e5501ee-c817-4a60-a1af-db811caae9c2` UUID `a1086194-e19c-49b1-88e1-bd828211b3a8` hash `sha256:9b717e070717cfbe31b580fcd1509fbfdf80548fcbdae1d08710674ee1ee6015` |
| `aiua_core.md` 1.3.0 | edición conscience post-topología | fila `delegar_habito` |

## Tests locales

```text
TMPDIR=.tmp-tests cargo test --offline -p user-preference-core --lib
# 12 passed
TMPDIR=.tmp-tests cargo test --offline -p execute-process --lib -- aiua_intent email_triage user_preference aiua_stimulus
# filtros verdes; overlay hábito CA-9; hint computrabajo → P-MUTE-SENDER
```

Mutex `LAB_ENV` serializa overlays `SDDIA_LAB_MOCK_*` (carrera preexistente agravada por el segundo overlay).

## Evolution

`sddia-qa evolution-register` → `89d3ce2b-4bde-49af-8945-8aeb9c4e84cf`.
