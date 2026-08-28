---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
execution_id: "13161205-2a2a-4320-9953-554e18a1f7c5"
---

# Execution — anclaje de ejecución

## L5 — Sellado genoma + testigo

Entrada oficial:

```bash
export CARGO_TARGET_DIR="$REPO_ROOT/SddIA/target"
./SddIA/target/debug/execute-process --seal-capsules \
  --inputs '{"profile":"release","write_genome":true,"write_witness":true,"names":["iota-immutable-publisher"]}'
```

Vía `entity-manager` (un artefacto):

```bash
./sddia-run.sh --process entity-manager --inputs '{
  "entity_class":"tool",
  "entity_name":"iota-immutable-publisher",
  "lifecycle_operation":"seal-anchor",
  "semantic_seed":{"profile":"release"}
}'
```

| Cápsula | `source_sha256` (release) | testigo |
|---------|---------------------------|---------|
| `iota-immutable-publisher` | `sha256:30027ec820e44a0c80d2665c6bdf92edd6d2ae53dd91d194ba847f1bb05a80fa` | `SddIA/target/release/iota-immutable-publisher.sha256` |
| `event-watcher` | sellado | sí |
| `event-sweeper` | sellado | sí |

`execute-process` / `kalma2-bridge`: sin genoma `{name}.md` en tools/skills/daemons — testigo/genoma pendiente de entidad indexada.

## L7 — Rebuild + ancla

```bash
export CARGO_TARGET_DIR="$REPO_ROOT/SddIA/target"
cd SddIA && cargo build --release -p iota-immutable-publisher -p event-watcher -p event-sweeper
cd SddIA && cargo build -p execute-process --bin execute-process
# re-sellar tras cambio de fuentes
```

Activación runtime: `SDDIA_CAPSULE_ANCHOR=1` (default en `start-sddia.sh` tras sellado; override `=0` para legacy).

## L8 — DLT reanchor-queue

10 entradas con `Campo obligatorio ausente o inválido: payload` — eventos `pending/` ya ausentes → cola huérfana purgada (instancia).

Repro post-fix: `cargo test -p iota-immutable-publisher` — `merkle_array_payload_returns_root_and_proofs` OK.

## Digest

Política bundle (path-deps + workspace `Cargo.toml`/`Cargo.lock`) en `capsule_digest::compute_bundle_source_digest` — paridad `build-release-bundle.sh` `_sddia_source_digest`.
