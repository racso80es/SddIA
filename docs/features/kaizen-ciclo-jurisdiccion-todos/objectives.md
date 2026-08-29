---
feature_name: kaizen-ciclo-jurisdiccion-todos
created: "2026-08-29"
process: feature
branch_name: feat/kaizen-ciclo-jurisdiccion-todos
persist_ref: docs/features/kaizen-ciclo-jurisdiccion-todos
pbi_ref: docs/todos/pending/[KAIZEN] Ciclo jurisdicción todos — norm-creator parcial, huérfanos EDA y colapso DCC sin fractura.md
document_id: PBI-KAIZEN-CICLO-JURISDICCION-TODOS
uuid: 74c4e6e9-baef-4a08-aa44-4adb0ffe1dfe
execution_id: "1550128b-c2ef-4c4d-8cbb-181a15a66940"
---

# Objetivos — kaizen-ciclo-jurisdiccion-todos

## Misión

Cerrar las siete fricciones de ciclo del PBI (`F-NORM-FORGE-*`, `F-EDA-ORPHAN` residual, `F-DCC-*`, `F-EVOLUTION-CORRELACION-EDA-COVERAGE`, `F-TEKTON-BYPASS-RAW-POST-COLAPSO`) sin reabrir `PBI-OPER-DEUDA-TECNICA-KINTSUGI-001`.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| `run_norm_forge` ignora `tactical_norm_dependencies` y no emite Restricciones Duras | CA1/CA2; forge antes de re-forjar |
| `orphan_count: 0` (2026-08-28); hashes `pending-forge` vivos | CA3 = sello real, no re-backfill |
| `backfill_manifest_active` solo en Rust; nota DCC incompleta | CA3b = documentar predicado real |
| DCC `status_code: 1` no emite fractura | CA4; Kintsugi ciego |
| Motor muta `eda-coverage.json` sin correlato evolution | CA5; exención o bind automático |
| `/.tmp` anclado a raíz | CA6; `**/.tmp/` |
| Norma no cubre colapso mudo | CA7; operador emite y detiene |

## Objetivos medibles

| ID | Objetivo | Criterio PBI |
|----|----------|--------------|
| **O1** | Forge conforme a `norms-contract` v1.1.0 | CA1 |
| **O2** | `todos-jurisdiction` v1.1.0 vía EM update | CA2 |
| **O3** | Hash real en fetcher + download-remote-asset | CA3 |
| **O4** | Vía `backfill-manifest.json` documentada | CA3b |
| **O5** | Fractura en fases DCC blocked/failed | CA4 |
| **O6** | Gate evolution sin rehash manual de cobertura | CA5 |
| **O7** | Efímeros `.tmp/` ignorados en cualquier profundidad | CA6 |
| **O8** | Cláusula colapso mudo en `obediencia-procesos` | CA7 |

## Ley aplicada

- Genoma (`library_norms`, `tools`, `actions`, `process` de dominio): `entity-manager`.
- Norma motor `obediencia-procesos.md`: parche bajo DA-4 (sin creator) + evolution.
- Git: `skill:git-manager`. Rutas: `cumulo.paths.json`.
- Evidencia física por CLI (`argos.md` §2). No edición manual de `todos-jurisdiction.md`.
