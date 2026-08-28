---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
updated: "2026-08-28T19:15:00Z"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
pbi_ref: docs/todos/done/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
document_id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
uuid: e7edb590-3193-4709-a1a2-e863a79842e4
fracture_hash: 6a49e0ad310e
regression_of: PBI-FIX-FRACTURE-6a49e0ad310e
global: APTO
pbi_archived: true
branch: fix/capsula-binario-fosil-release-stale
approval_status: aprobado
verdict: aprobado
delivery_state: success
resolution: DONE
checks:
  CA1-CA2_PORCELAIN: APTO
  CA3_FRACTURE_EMIT: APTO
  CA4-CA7_GENOME_WITNESS: APTO
  CA5-CA6_ANCHOR_ADUANA: APTO
  CA8_PUBLISHER_BUNDLE: APTO
  CA9_DIGEST_BUNDLE: APTO
  CA12-CA13_TRAZA: APTO
  CA14_REBUILD_INVENTORY: APTO
  CA15-CA16_DLT_DRAIN: APTO
  CARGO_TEST_CAPSULE: APTO
  IOTA_PUBLISHER_ARRAY_PAYLOAD: APTO
  PBI_ARCHIVED: APTO
  CASCADE_SPEC: APTO
  CASCADE_IMPLEMENTATION: APTO
  CASCADE_EXECUTION: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/capsule_seal.rs
  - SddIA/engine/execute-process/src/engine/capsule_digest.rs
  - SddIA/engine/execute-process/src/engine/capsule_paths.rs
  - SddIA/engine/execute-process/src/engine/entity_manager.rs
  - SddIA/engine/execute-process/src/forges/common.rs
  - SddIA/engine/execute-process/src/main.rs
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/daemons/event-watcher.md
  - SddIA/daemons/event-sweeper.md
  - start-sddia.sh
  - docs/fixes/capsula-binario-fosil-release-stale/
  - docs/todos/done/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
---

# Validación — anclaje de ejecución (Argos)

## Veredicto

**APTO** — Causa raíz física cerrada: `resolve_capsule_native` ya no sirve ELF release fósil de `iota-immutable-publisher` sin verificar identidad. Patrón de Anclaje operativo (genoma `source_sha256` + testigo `elf_sha256` + aduana `SDDIA_CAPSULE_ANCHOR`).

## Evidencia

| Check | Estado | Notas |
|-------|--------|-------|
| Porcelain UTF-8 (`pbi_ref` en scope) | APTO | `git_porcelain.rs` + test unitario |
| Fractura `F-DIRTY-WORKTREE` en precondición | APTO | `workspace_init.rs` |
| Publisher release acepta `payload` array | APTO | `cargo test -p iota-immutable-publisher` — `merkle_array_payload_returns_root_and_proofs` |
| Sellado publisher | APTO | `sha256:30027ec8…` en genoma + testigo release |
| Cola reanchor | APTO | 10 entradas huérfanas purgadas (sin `pending/` asociado) |
| Tests aduana | APTO | `capsule_paths` (3), `capsule_digest` (2) |

## Criterios PBI R1

- [x] Ciclo abre sin `SDDIA_LAB_ALLOW_DIRTY` (porcelain corregido)
- [x] Rebuild release + inventario documentado (`execution.md`)
- [x] Anclaje genoma → testigo → ELF
- [x] Cola DLT drenada (instancia)
- [x] `validacion.md` APTO, `pbi_archived: true`
- [x] PBI en `docs/todos/done/`

## Deuda residual (no bloqueante)

- Backfill batch del resto de tools indexados (`--seal-capsules` sin filtro `names`).
- Digest política A estricta (`cargo metadata` + lockfile recortado) — follow-up.
- Genomas ausentes para `execute-process` / `kalma2-bridge` (sin `{name}.md` en tools/skills/daemons).
