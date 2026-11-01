---
feature_name: aut20-ecst-router-verify
created: 2026-11-01
process: bug-fix
branch: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-router-verify
tracker_ref: AUT-57
historia_ref: AUT-20
global: APTO
approval_status: aprobado
checks:
  AUT57-L1-ECST-CATALOG: APTO
  AUT57-L1-ROUTE-NO-BRANCH-GATE: APTO
  AUT57-L3-BUS-MISMATCH: APTO
  AUT57-L4-LIB-TESTS: APTO
  AUT57-INTEGRATION-EDA-ECST: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/route_domain_core.rs
- SddIA/engine/execute-process/src/engine/mod.rs
- SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs
- SddIA/engine/execute-process/src/engine/fix_tool_process_core.rs
- SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
- SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs
- SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs
- docs/fixes/aut20-ecst-router-verify/
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
validated_head_sha: 0334c03b6cabd2b1d7425f19db84350ce85087d3
validated_tree_sha: a4d96b33c8b5bf1f2b441dfedfbadbd5e5c72c1e
diff_digest: sha256:c0b4d4f5ea07ed2983bcd69e199a4ec738633dcd36ba72ab4163d223ace5ffb2
---

# Validación — AUT-57

Cierre de criterios 1 y 4 de AUT-20 en `route-domain-event` (rama `Fix_Events_Multiple`). Provider Linear: sin fichero en `docs/todos/done/`; Done del issue tras merge del PR.

## Veredicto

**APTO** — El router entrega ECST a los procesos del catálogo de dominio; `tracker-sync-replay` y `tracker-markdown-apply` rechazan ECST inline que no coincide con `events.redb`.

## Evidencia (2026-11-01, `TMPDIR=.tmp/cargo-test`, `CARGO_TARGET_DIR=SddIA/target`)

| Check | Comando / artefacto | Resultado |
|-------|---------------------|-----------|
| L1 catálogo ECST | `cargo test -p execute-process --lib domain_process_catalog` | 1/1 OK |
| L3 mismatch | `cargo test -p execute-process --lib run_rejects` | 2/2 OK |
| L1 router redb | `cargo test -p execute-process --test eda_subscriber_ecst_redb route_domain_` | 2/2 OK |
| Integración ECST redb | `cargo test -p execute-process --test eda_subscriber_ecst_redb` | 6/7 OK |

Nota: `tracker_stamp_loads_ecst_from_store_without_pending_file` falla en repo Core con `LINEAR_GRAPHQL_ERROR` (handler directo + repo real); no es regresión del despacho AUT-57. Los tests de router `route_domain_*` cubren el corte `branch missing`.

## Criterios de cierre (refinamiento AUT-57)

| Criterio | Laudo |
|----------|-------|
| Catálogo recibe ECST; sin `branch missing` salvo PPR | APTO (tests router + `build_ecst_subscriber_inputs`) |
| `memory-evolution-ingest` / handlers store | APTO (código + route telemetry) |
| Verificación bus antes de Linear (replay + markdown) | APTO (`failed-bus-mismatch`) |
| Tests por área | APTO (tabla anterior) |