---
feature_name: evolution-history-normalization
created: "2026-08-14"
process: refactorization
branch_name: refactor/evolution-history-normalization
persist_ref: docs/features/evolution-history-normalization
pbi_ref: docs/todos/pending/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
document_id: 7bb37ff1-decd-4ec5-968b-344a5334f9eb
correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
execution_id: 63062872-e707-496e-b1b3-1ea736e256f0
phase: implementation
agents: tekton
---

# Implementation — evolution-history-normalization

## Touchpoints

| Artefacto | Acción |
|-----------|--------|
| `SddIA/tools/sddia-qa/src/migrate_evolution_history.rs` | Alta — `manifest` / `apply` / `verify` / `reindex`; hash vía `sddia-evolution-register::canonical_hash` |
| `SddIA/tools/sddia-qa/src/validate_evolution_contract.rs` | Extensión `--universe official` + `--manifest` (excluye L4) |
| `SddIA/tools/sddia-qa/src/main.rs` | Wire CLI |
| `SddIA/tools/sddia-qa/Cargo.toml` | Dep crate `sddia-evolution-register` |
| `docs/features/evolution-history-normalization/migration-manifest.json` | Manifiesto congelado (`frozen_at: 2026-08-14T10:08:05Z`, 0 blocked, universo 64) |
| `SddIA/evolution/*.md` | L1–L3 normalización FM + renombres UUID v4 |
| `docs/audits/evolution/drafts/` | L4 extracción de 2 `*-analisis-temp.md` |
| `SddIA/evolution/Evolution_log.md` | Índice 64 CANONICO + fila hito `63062872-…` |
| `SddIA/evolution/evolution_contract.md` | §3 universo oficial CANONICO; §6 borradores fuera del directorio |
| `SddIA/evolution/63062872-e707-496e-b1b3-1ea736e256f0.md` | Alta ciclo vía `evolution-register` |

## Lotes (manifiesto)

| Lote | Acción | Resultado |
|------|--------|-----------|
| L1 | `normalize_fm` INV-A | Frontmatter v1.1.1 + `hash_integrity` |
| L2 | `normalize_fm` INV-L | Contrato 1.1.1 + enum `tipo_operacion` |
| L3 | `normalize_and_rename` | Stems UUID v4; `origen:` / `origen_migracion` |
| L4 | `extract` | Fuera de `directories.evolution` |
| SKIP | `0bceeb41-…` | Sin reescritura (L-SKIP-CANON) |

## Alias L3 (extracto)

| old_path | new_path |
|----------|----------|
| `capsules-bridge-rust-port.md` | `a7b3c4d5-e6f7-4890-a1b2-c3d4e5f67890.md` |
| `event-bus-audit-process.md` | `8d577a50-055a-40b9-b7e2-93e2d2415796.md` |
| `git-hooks-ca3-ola-b-contract.md` | `c032d392-a586-4b8c-baaf-6cb831ebb943.md` |
| `migracion-execute-process-rust-p14-p15.md` | `2021b663-ecf6-4ea9-82dc-06aa52e44413.md` |
| `pull-request-automation-dlt-oraculo-20260523.md` | `05d3d2f9-8b67-4e51-a215-03a15c4efd06.md` |
| `pull-request-review-v2-aduana-20260522.md` | `cfeb3de1-b708-4a7b-9bce-92553d98fe85.md` |
| `pull-request-review-v2.1-dia-20260525.md` | `7a396904-dd3a-4e44-ba82-8df2c59430b6.md` |
| `pull-request-review-v2.2-kaizen-alert-eda-v2-20260525.md` | `cfeac8b3-24ec-4405-bcff-cc4526698257.md` |
| `e1f2a3b4-c5d6-4789-e012-3456789abcde.md` | `e8f967f5-99c3-484b-b876-3c6d0f6d29b1.md` |

Mapa completo: `migration-manifest.json`.

## No tocado

- `cumulo.paths.json`
- Reason-codes / fail-hard del gate (`70f78d23-…`)
- Cuerpos Markdown (salvo cabecera YAML)
- Corte histórico `docs/audits/evolution/2026-08-11.md`
- PBI Kalma2 (`1de0bdd1-…`) — ciclo distinto
