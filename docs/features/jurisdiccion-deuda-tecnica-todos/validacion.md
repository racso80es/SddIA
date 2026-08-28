---
feature_name: jurisdiccion-deuda-tecnica-todos
created: "2026-08-28"
process: feature
branch: feat/jurisdiccion-deuda-tecnica-todos
branch_name: feat/jurisdiccion-deuda-tecnica-todos
persist_ref: docs/features/jurisdiccion-deuda-tecnica-todos
document_id: PBI-OPER-DEUDA-TECNICA-KINTSUGI-001
uuid: 4be8aeee-896a-4d2f-b2d3-3ee0d05fbd80
execution_id: "a3050468-df71-4922-bac9-3743bef2e54d"
pbi_ref: docs/todos/done/Registro y Resolución de Deuda Técnica (Kintsugi Ontológico).md
global: APTO
pbi_archived: true
checks:
  CA1-norma-jurisdiccion: pass
  CA2-done-inalterado: pass
  CA3-reclasificacion-deuda: pass
  CA4-portador-no-fractura: pass
  CA5-evidencia-cli: pass
  CA6-sin-solape-fanout: pass
git_changes:
  - SddIA/library/norms/todos-jurisdiction.md
  - SddIA/library/norms/index.md
  - SddIA/core/eda-coverage.json
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/evolution/eb6fb73a-9ded-49a1-a2a9-314624358b4b.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/jurisdiccion-deuda-tecnica-todos/
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md
  - docs/todos/pending/[DEUDA] Escaneo lineal de docs-todos en el resolutor de fractura — umbral de indexación.md
  - docs/todos/done/Registro y Resolución de Deuda Técnica (Kintsugi Ontológico).md
  - docs/todos/tmp/ (retirado)
evolution_entry: SddIA/evolution/eb6fb73a-9ded-49a1-a2a9-314624358b4b.md
---

# Validación — jurisdicción docs/todos

**Veredicto global: APTO**

## CA1 — Norma `todos-jurisdiction` v1.0.0

Forjada vía `entity-manager` (`f0b8ce4a-2f79-4516-bee0-acfe0d25bd58`). Tabla de buckets despachable/archivable/inerte en Directriz Core. Índice `library_norms` sincronizado.

## CA2 — Done inalterado

Sin tercer estado de cierre. Patrón `features-documentation-pattern` v1.2.1 respetado.

## CA3 — Reclasificación `DeudaTecnica/`

| Documento | Laudo |
|-----------|-------|
| Paciente 0 deploy/undeploy | `process_candidate`, `dispatch: false`, en `pending/` |
| Escaneo lineal resolutor | `type: deuda` en `pending/` |
| `Optimizacion_BioIA.md` | Ausente — descarte (laudo D1) |
| Bucket `DeudaTecnica/` | Retirado |

## CA4 — Portador no-fractura

Enum `F-*` / `DT-*` documentado en norma. Habitante scan-lineal en `pending/` con `tech_debt_ids`.

## CA5 — Evidencia física CLI

```text
cd SddIA && cargo test -p execute-process extract_pbi -- --nocapture
```

```
running 4 tests
... extract_pbi_migrated_deuda_tecnica_paths ... ok
... extract_pbi_ignores_inert_bucket_paths ... ok
... extract_pbi_prefers_pending_over_inert_when_both_present ... ok
... extract_pbi_with_spaces_and_emdash ... ok
test result: ok. 4 passed
```

```text
cd SddIA && cargo test -p execute-process pending_pbi_path_accepted -- --nocapture
```

```
test ... pending_pbi_path_accepted_for_archive_gate ... ok
```

## CA6 — Sin solape fan-out

Diff sin `fracture_pbi.rs`, `materialize_fracture_pbi.rs`, `enrich_fracture_pbi_kaizen.rs`.

## Cierre documental

PBI `PBI-OPER-DEUDA-TECNICA-KINTSUGI-001` en `docs/todos/done/` en esta rama; `pbi_archived: true`.
