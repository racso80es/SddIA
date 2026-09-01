---
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
title: "[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB)"
format: markdown
version: "1.0.0"
created: "2026-09-01"
updated: "2026-09-01"
status: pending
refinement_status: proposed
priority: alta
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-ci-step-runtime-gt-1min
persist_ref_suggested: docs/features/kaizen-ci-step-runtime-gt-1min
depends_on: []
related:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/tools/sddia-qa/src/verify_compiled_capsules.rs
  - SddIA/scripts/qa/build-wasi-capsules.sh
  - SddIA/core/memory/Cargo.toml
  - SddIA/infrastructure/adapters/lancedb_thought_repo/Cargo.toml
  - SddIA/infrastructure/adapters/lancedb_evolution_repo/Cargo.toml
  - SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs
  - docs/todos/done/PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION.md
  - docs/todos/pending/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
---

# [KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB)

## Mandato

Erradicar el calor residual de la aduana GitHub Actions: todo **step** de `.github/workflows/sddia-index-qa.yml` cuya duración empírica supere **60 s** debe ser diagnosticado y comprimido **sin recortar cobertura**.

Prioridad absoluta: `verify-compiled-capsules` y `LanceDB memory integration tests` del job `sddia-index-integrity`. El resto de steps/jobs >1 min del mismo workflow entra en inventario (Ola A0) y se ataca si el diagnóstico lo justifica.

No reabre `PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION` (segregación `push` vs `pull_request`). No solapa `PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY` (captura de colapsos).

## 0. Evidencia empírica (baseline)

Fuente: run GitHub Actions del workflow `sddia-index-qa`, job **`sddia-index-integrity`**, **2026-09-01**. Duración total del job: **13 m 36 s**. Jobs hermanos (`eda-iota-smoke-simulate`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical`) en verde — fuera de este baseline de cronómetro, no del inventario A0.

| Step | Duración | ¿>60 s? |
| :--- | ---: | :---: |
| Set up job | 1 s | no |
| actions/checkout@v4 | 3 s | no |
| Install protoc (lance-encoding) | 13 s | no |
| dtolnay/rust-toolchain@stable | 1 s | no |
| **actions/cache@v4 (restore)** | **1 m 2 s** | **sí** |
| Build QA aduana (execute-process + sddia-qa) | 29 s | no |
| verify-tools-index | 0 s | no |
| verify-process-integrity | 0 s | no |
| evolution-register unit tests | 1 s | no |
| **verify-compiled-capsules** | **5 m 40 s** | **sí** |
| **LanceDB memory integration tests** | **6 m 1 s** | **sí** |
| Post cache / checkout / Complete | ≤1 s | no |

Suma de los tres steps >1 min: **11 m 41 s ≈ 85 %** del job.

## 1. Superficie de impacto (genoma verificado)

- Workflow: `.github/workflows/sddia-index-qa.yml` (job `sddia-index-integrity`, steps L55–66).
- Gate: `SddIA/tools/sddia-qa/src/verify_compiled_capsules.rs` — **no compila**; recorre crates con `src/main.rs` bajo `engine/`, `skills/`, `tools/`, `daemons/`, `interfaces/` y comprueba presencia del binario en `compiled_capsules.native_root`.
- Tests LanceDB:
  - `sddia-core-memory` (`SddIA/core/memory/`) — crate ligero (serde/sha2); 3 unit tests; **sin** dep `lancedb`.
  - `sddia-infrastructure-lancedb-thought` — `lancedb = "=0.37.1"` + `protoc`; 5 tests.
  - `sddia-infrastructure-lancedb-evolution` — mismo grafo pesado; 5 tests.
  - `execute-process` filtro `memory_evolution_ingest json_fallback` — 2–3 tests en `memory_evolution_ingest_core.rs`; **compila el orquestador entero** bajo `cfg(test)`.

## 2. Diagnóstico causal (no hipótesis de producto)

### 2.1 `verify-compiled-capsules` (5 m 40 s)

El step **no** es el gate. YAML vigente:

```yaml
- name: verify-compiled-capsules
  run: |
    cd SddIA
    cargo build --workspace
    target/debug/sddia-qa verify-compiled-capsules
```

El comando `sddia-qa` es I/O de filesystem (fracción de segundo). Los 5 m 40 s son `cargo build --workspace` **después** de un step previo que ya compiló `-p execute-process` y `-p sddia-qa` (29 s). El gate solo exige binarios con `main.rs`; `--workspace` arrastra librerías sin binario (`sddia-core`, adapters, etc.).

### 2.2 `LanceDB memory integration tests` (6 m 1 s)

Cuatro invocaciones **secuenciales** de `cargo test`, cada una con grafo de compile propio:

```yaml
cargo test -p sddia-core-memory
cargo test -p sddia-infrastructure-lancedb-thought
cargo test -p sddia-infrastructure-lancedb-evolution
cargo test -p execute-process -- memory_evolution_ingest json_fallback
```

El coste dominante es **compilar** `lancedb`/Arrow/`execute-process` con `cfg(test)`, no ejecutar ~13 tests. El primer paquete (`sddia-core-memory`) no toca LanceDB; paga startup de Cargo. El cuarto paga el crate más pesado del workspace para dos aserciones de ingesta.

### 2.3 `actions/cache@v4` restore (1 m 2 s)

Cachea `~/.cargo/registry`, `~/.cargo/git` **y** `SddIA/target` con clave `native-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}` (compartida con `eda-iota-smoke-simulate` y `eda-iota-physical`). Restaurar `target/` ~1 min **no impide** el rebuild posterior: fingerprints de rustc/incremental suelen invalidarse entre runners. Se paga restore + compile completo.

## 3. Estrategia (línea de montaje)

Enfoque: **medir → cortar calor de compile → preservar cobertura**. Prohibido `skip` de tests, `continue-on-error` sobre aduanas, o bajar el umbral del gate de binarios.

### Ola A0 — Inventario termodinámico

Cronometrar (un run representativo, mismo `headSha`) **todos** los steps de los cinco jobs del workflow. Tabla SSOT en `objectives.md`/`spec.md` del ciclo: step, duración, job, causa (compile / restore / runtime / red). Cualquier step >60 s entra en alcance de parche si el ROI de compile lo justifica (`wasi-runtime-smoke` / `eda-bus-e2e-smoke` son candidatos por `build-wasi-capsules.sh` + nativos).

### Ola A1 — `verify-compiled-capsules`

- Separar **compile de binarios exigidos** vs **verificación de presencia**.
- Sustituir `cargo build --workspace` por el conjunto mínimo que el propio gate descubre (crates con `main.rs` en `SCAN_ROOTS`), o fusionar con el step «Build QA aduana» para no compilar dos veces.
- El binario `sddia-qa verify-compiled-capsules` permanece; no se debilita `mandatory_bins` ni el descubrimiento.
- Mutación de `SddIA/tools/sddia-qa/` solo vía `entity-manager` (DA-2). El YAML no es genoma.

### Ola A2 — Tests LanceDB

- Una sola invocación `cargo test` con varios `-p` (grafo compartido) en lugar de cuatro procesos Cargo.
- Extraer o reubicar los tests de `memory_evolution_ingest_core.rs` al adapter `lancedb-evolution` si el único motivo de `-p execute-process` es esas 2–3 funciones: el orquestador no debe recompilarse entero para aserciones de persistencia.
- `sddia-core-memory` puede quedar en el mismo invocador; no justifica un `cargo test` aislado.
- Invariante: mismos tests (o superconjunto) en verde. Filtro `memory_evolution_ingest json_fallback` no debe silenciar `ingests_telemetry_captured_to_vector_store` si hoy corre.

### Ola A3 — Cache y linker

Medir, no dogma:

| Hipótesis | Experimento |
| :--- | :--- |
| `SddIA/target` en cache es antieconómico | Restore-only registry/git vs cache actual; comparar wall-clock del job |
| `CARGO_INCREMENTAL=0` en CI | Cold/restore build vs incremental inválido |
| sccache / mold|lld | Solo si A1+A2 no bajan los dos steps <60 s y el delta es reproducible |

Prohibido hinchar minutos-runner: partir LanceDB a job paralelo **solo** si el wall-clock del workflow baja **y** Σ minutos de runner no crece más que el ahorro (Filtro C).

### Ola A4 — Cierre documental

Registro en `SddIA/evolution/` ligado a este UUID. Baseline vs post-parche en `validacion.md` (tabla de duraciones del mismo job, mismo tipo de evento). PBI a `docs/todos/done/` en la rama del PR (cierre documental un PR).

## 4. Criterios de aceptación

- [ ] **CA1 (Umbral):** en un run `pull_request` o `push` post-parche, `verify-compiled-capsules` y `LanceDB memory integration tests` duran **< 60 s** cada uno, **o** el `spec.md` documenta con cronómetro por qué el suelo físico (compile LanceDB/Arrow) impide el umbral y fija un techo justificado **< 50 % del baseline** (hoy 5 m 40 s / 6 m 1 s).
- [ ] **CA2 (Cobertura):** `sddia-qa verify-compiled-capsules` sigue exigiendo el mismo conjunto de bins `main.rs`; los tests de los cuatro paquetes listados en el YAML vigente siguen ejecutándose (nombres estables o mapeo explícito si se reubican).
- [ ] **CA3 (Inventario):** A0 publicado: lista de steps >60 s del workflow completo, con decisión atacar / diferir / techo.
- [ ] **CA4 (Cache):** decisión empírica sobre cachear `SddIA/target` (mantener / partir / eliminar) escrita en `implementation.md` con números de un run.
- [ ] **CA5 (Job):** wall-clock de `sddia-index-integrity` **< 8 min** en el mismo tipo de evento que el baseline (13 m 36 s), sin `continue-on-error` ni skip de aduana.
- [ ] **CA6 (No regresión de hermandad):** `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-smoke-simulate`, `eda-iota-physical` permanecen verdes en el PR de cierre.
- [ ] **CA7 (Forja):** cambios en `SddIA/tools/` vía `./sddia-run.sh --process entity-manager`; workflow `.github/` en el mismo PR; evolution anclado a `530039c9-100b-413a-b3d5-ca632d83acc6`.

## 5. Fuera de alcance

- Segregación de jobs por `push`/`pull_request` (PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION, done).
- Puente de telemetría CI → bus local (PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY, pending).
- Cambiar semántica de LanceDB en producción, MiniLM, o umbrales de heartbeat.
- `SDDIA_SKIP_HOOKS`, omitir `protoc`, o falsear `verify-compiled-capsules` con bins stub.
- Polling de GitHub Actions (DA-6).

## 6. Semilla de ciclo

```text
feature_name: kaizen-ci-step-runtime-gt-1min
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
pbi_ref: docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
```
