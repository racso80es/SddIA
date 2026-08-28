---
feature_name: jurisdiccion-deuda-tecnica-todos
created: "2026-08-28"
process: feature
items:
  - norm-todos-jurisdiction
  - migrate-deuda-tecnica
  - discard-todos-tmp
  - tests-extract-pbi
---

# Implementación — jurisdicción docs/todos

## L1 — Norma `todos-jurisdiction`

| Campo | Valor |
|-------|-------|
| Ruta | `SddIA/library/norms/todos-jurisdiction.md` |
| UUID | `f0b8ce4a-2f79-4516-bee0-acfe0d25bd58` |
| Vía | `entity-manager` → `norm-creator` (`execution_id` `df71d850-d511-4b40-982f-aed0cf153c6f`) |
| Evento | `Domain_Entity_Created` `18edabc6-c2ea-4461-bb07-a973bbca583e` |

## L2 — Migración `DeudaTecnica/` → `pending/`

| Origen (retirado) | Destino |
|-------------------|---------|
| `[DEUDA] Paciente 0 — prompt y proceso de despliegue.md` | `docs/todos/pending/` + `dispatch: false` |
| `[DEUDA] Paciente 0 — prompt de teardown.md` | `docs/todos/pending/` + `companion_deploy_ref` actualizado |
| `[DEUDA] Escaneo lineal…md` | `docs/todos/pending/` (ya `dispatch: false`) |

Directorio `docs/todos/DeudaTecnica/` eliminado.

## L3 — `docs/todos/tmp/`

Cinco ficheros `status: consolidado` eliminados (`git rm`). Directorio retirado.

## L4 — Tests CA5

`task_queue_manager.rs`: `extract_pbi_migrated_deuda_tecnica_paths`, `extract_pbi_ignores_inert_bucket_paths`, `extract_pbi_prefers_pending_over_inert_when_both_present`, `pending_pbi_path_accepted_for_archive_gate`.

## Sin tocar (CA6)

`fracture_pbi.rs`, `materialize_fracture_pbi.rs`, `enrich_fracture_pbi_kaizen.rs`.
