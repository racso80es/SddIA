---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
phase: design
agents: dedalo
base: main
scope: sddia-index-qa-step-runtime
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
pbi_ref: docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
version_spec: "1.1.0"
status: dedalo_locked
runtime_execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
---

# Spec — kaizen-ci-step-runtime-gt-1min

## 1. Misión técnica

Erradicar calor de compile en `sddia-index-integrity` sin debilitar `verify-compiled-capsules` ni los 16 tests de memoria/LanceDB/ingesta.

## 2. Diagnóstico (PBI §2)

- 340 s = `cargo build --workspace` de ~27 bins no cubiertos por el cache parcial (aduana 29 s prueba hit).
- 361 s = `cargo test -p execute-process` recompila 366 `#[test]` para ejecutar 3.
- Key `native-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}` compartida con IOTA: first-write-wins sella `target/` de 4 crates.

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-CACHE-INTEGRITY** | Integrity: `actions/cache@v4` de `~/.cargo/{registry,git}` (sin `SddIA/target`). Key `native-integrity-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}-${{ steps.rustc.outputs.hash }}`. `restore-keys`: mismo prefijo lock (sin legado `native-*`). |
| **L-CACHE-IOTA-RO** | IOTA simulate + physical: `actions/cache/restore@v4` (misma key/paths). **Prohibido** `lookup-only` (no descarga). Jobs `wasi-*` intactos. |
| **L-SCCACHE** | `mozilla-actions/sccache-action@v0.0.9` en integrity + IOTA. `CARGO_INCREMENTAL=0`. Permiso `actions: write`. Sin mold. Desbloqueado por A3 tras hit `754c575` (restore `target/` 43 s + compile 429 s). |
| **L-ONE-WORKSPACE** | Un `cargo build --workspace` (step `Build native workspace`). Eliminar `Build QA aduana` de dos `-p`. |
| **L-GATE-IO** | `verify-compiled-capsules` solo ejecuta `sddia-qa verify-compiled-capsules`. CA1 = suma L-ONE-WORKSPACE + este step. |
| **L-INGEST-ITEST** | Mover los 3 tests de `memory_evolution_ingest_core.rs` a `tests/memory_evolution_ingest.rs`. Borrar el módulo `#[cfg(test)]` del lib. |
| **L-TEST-CMD** | `cargo test -p sddia-core-memory -p sddia-infrastructure-lancedb-thought -p sddia-infrastructure-lancedb-evolution` y `cargo test -p execute-process --test memory_evolution_ingest`. |
| **L-NO-QA** | Cero cambios en `SddIA/tools/sddia-qa`. |
| **L-NO-MOLD** | No mold/lld en este ciclo. CA1 fallback 50 % y CA5 sin fallback siguen vigentes. |

## 4. Contrato YAML

### 4.1 Cache integrity

Tras toolchain: `rustc cache id`, `sccache`, luego:

```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
    key: native-integrity-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}-${{ steps.rustc.outputs.hash }}
    restore-keys: |
      native-integrity-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}-
```

### 4.2 Cache IOTA (simulate + physical)

Mismo `path`/`key`/`restore-keys` vía `actions/cache/restore@v4`. sccache igual. No save de `actions/cache`.

### 4.3 Steps integrity (orden)

1. checkout, protoc, rust-toolchain, rustc id, sccache, cache §4.1
2. `Build native workspace` → `cd SddIA && cargo build --workspace`
3. `verify-tools-index` / `verify-process-integrity` / `evolution-register unit tests` (comandos vigentes)
4. `verify-compiled-capsules` → `SddIA/target/debug/sddia-qa verify-compiled-capsules`
5. LanceDB → § L-TEST-CMD

## 5. Contrato tests ingest

Integración usa API pública: `ingest_domain_event_file`, `lancedb_uri`, `vector_store_root`. Mismos tres nombres de test. Adapter vía crate `sddia-infrastructure-lancedb-evolution` (ya dep de `execute-process`; `use` desde el crate de integración o reexport mínimo si el compilador lo exige). `tempfile` ya es `dev-dependency`.

## 6. Techo CA1 (suelo físico)

Ola A3.0 (cerrada, NO_APTO): run 33495498463 hit de key con `SddIA/target` en el blob → Build 429 s / LanceDB 419 s / job 15 m 11 s. Causa: fingerprints Cargo vs mtimes de checkout; `lookup-only` no restaura IOTA.

Ola A3.1: sccache GHA + no cachear `target/` + `restore@v4`. Primer SHA A3.1 = miss de sccache (suelo ~340 s). CA1/CA5 se miden en el **segundo** `pull_request` con hit sccache. Techo CA1 50 % (170 s / 180 s) y CA5 < 8 min sin fallback.
