---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
phases:
  - forge-entities
  - crate-jail-tests
  - action-handler
  - docs-execution
  - dcc-pr-ci-accept
branch_name: feat/tool-ephemeral-cache-purger
persist_ref: docs/features/tool-ephemeral-cache-purger
pbi_ref: docs/todos/pending/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
document_id: PBI-FEATURE-TOOL-CACHE-PURGER
pbi_uuid: "0987ac64-2c95-41c4-9ca7-d777446034cb"
pbi_version: "1.2.0"
execution_id: "7f37a724-5d10-41df-9cc9-cf22dc275951"
---

# Plan — tool-ephemeral-cache-purger

Corte diseño: clarify + objectives + spec + plan + PBI v1.2.0. **Commit planificación** antes de mutar genoma.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI Filtro A v1.2.0.

## L1 — Forja

```text
./sddia-run.sh --process entity-manager --inputs '{entity_class:tool, entity_name:ephemeral-cache-purger, lifecycle_operation:create, semantic_seed:{...}}'
./sddia-run.sh --process entity-manager --inputs '{entity_class:action, entity_name:purge-sandbox-cache, lifecycle_operation:create, semantic_seed:{...}}'
```

## L2 — Crate

`SddIA/tools/ephemeral-cache-purger/{Cargo.toml,src/jail.rs,src/main.rs}`. Tests `#[cfg(test)]` en `jail` + integración temp bajo `/tmp/cursor-sandbox-cache/<hash-test>/`.

```text
cd SddIA && cargo test -p ephemeral-cache-purger
```

## L3 — Handler acción

Módulo `purge_sandbox_cache.rs` + brazo en `actions.rs`. Tests lib `execute-process` con fixture o mock de invocación si el binario no está; mínimo: gate regex sobre JSON de dry-run.

## L4 — Docs + DCC

`implementation.md` / `execution.md` / `validacion.md` (CI `PENDIENTE-CI` hasta verde). Evolution UUID. `delivery-close-cycle`. `accept-pr` solo con checks verdes.

## Fuera

WASI. NFT DLT. Purga host root como CA de merge.
