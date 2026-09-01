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
version_spec: "1.0.0"
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
| **L-CACHE-INTEGRITY** | Integrity: key `native-integrity-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}`. `restore-keys`: `native-integrity-${{ runner.os }}-` y legado `native-${{ runner.os }}-`. |
| **L-CACHE-IOTA-RO** | `eda-iota-smoke-simulate` y `eda-iota-physical`: misma key/paths, `lookup-only: true`. No save. Jobs `wasi-*` intactos (`wasi-…`). |
| **L-ONE-WORKSPACE** | Un `cargo build --workspace` (step `Build native workspace`). Eliminar `Build QA aduana` de dos `-p`. |
| **L-GATE-IO** | `verify-compiled-capsules` solo ejecuta `sddia-qa verify-compiled-capsules`. CA1 = suma L-ONE-WORKSPACE + este step. |
| **L-INGEST-ITEST** | Mover los 3 tests de `memory_evolution_ingest_core.rs` a `tests/memory_evolution_ingest.rs`. Borrar el módulo `#[cfg(test)]` del lib. |
| **L-TEST-CMD** | `cargo test -p sddia-core-memory -p sddia-infrastructure-lancedb-thought -p sddia-infrastructure-lancedb-evolution` y `cargo test -p execute-process --test memory_evolution_ingest`. |
| **L-NO-QA** | Cero cambios en `SddIA/tools/sddia-qa`. |
| **L-NO-SCCACHE** | No sccache/mold en este ciclo. Si CA1/CA5 fallan en el PR, techo en `validacion.md` con `run_id` (CA1 fallback 50 %). CA5 sin fallback: reevaluar solo con números. |

## 4. Contrato YAML

### 4.1 Cache integrity

```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      SddIA/target
    key: native-integrity-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}
    restore-keys: |
      native-integrity-${{ runner.os }}-
      native-${{ runner.os }}-
```

### 4.2 Cache IOTA (simulate + physical)

Idéntico `path`/`key`/`restore-keys` + `lookup-only: true`.

### 4.3 Steps integrity (orden)

1. checkout, protoc, rust-toolchain, cache §4.1
2. `Build native workspace` → `cd SddIA && cargo build --workspace`
3. `verify-tools-index` / `verify-process-integrity` / `evolution-register unit tests` (comandos vigentes)
4. `verify-compiled-capsules` → `SddIA/target/debug/sddia-qa verify-compiled-capsules`
5. LanceDB → § L-TEST-CMD

## 5. Contrato tests ingest

Integración usa API pública: `ingest_domain_event_file`, `lancedb_uri`, `vector_store_root`. Mismos tres nombres de test. Adapter vía crate `sddia-infrastructure-lancedb-evolution` (ya dep de `execute-process`; `use` desde el crate de integración o reexport mínimo si el compilador lo exige). `tempfile` ya es `dev-dependency`.

## 6. Techo CA1 (suelo físico)

Si el PR de cierre es cache-miss de `native-integrity-*` (restore parcial legado), `Build native workspace` puede seguir ~340 s. Entonces CA1 usa techo < 170 s **solo** si el cronómetro del run lo permite; si el suelo de 29 bins en frío lo impide, `spec.md`/cierre documenta el suelo y CA1 queda **PENDIENTE-CI** hasta un run con hit `native-integrity-*` (el save de este PR habilita el siguiente). CA5 (< 8 min) depende de A2 (quitar ~361 s de cfg(test) orquestador) + fusión 29 s; se verifica con `run_id`.
