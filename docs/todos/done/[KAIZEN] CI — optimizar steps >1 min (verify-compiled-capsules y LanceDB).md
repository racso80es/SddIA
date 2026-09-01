---
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
title: "[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB)"
format: markdown
version: "1.1.0"
created: "2026-09-01"
updated: "2026-09-01"
status: done
refinement_status: implemented
priority: alta
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-ci-step-runtime-gt-1min
persist_ref_suggested: docs/features/kaizen-ci-step-runtime-gt-1min
baseline_run: "33477170741"
baseline_job: "99758755444"
baseline_event: pull_request
baseline_head_sha: "58e0802968fb1e37e03ddd72799450c45f252a0a"
depends_on: []
related:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/tools/sddia-qa/src/verify_compiled_capsules.rs
  - SddIA/core/memory/Cargo.toml
  - SddIA/infrastructure/adapters/lancedb_thought_repo/Cargo.toml
  - SddIA/infrastructure/adapters/lancedb_evolution_repo/Cargo.toml
  - SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs
  - docs/todos/done/PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION.md
  - docs/todos/pending/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
---

# [KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB)

## Mandato

Comprimir los **steps >60 s** del workflow `.github/workflows/sddia-index-qa.yml` **sin recortar cobertura**.

Prioridad absoluta: `verify-compiled-capsules` y `LanceDB memory integration tests` del job `sddia-index-integrity`.

Inventario A0 del run baseline (PR, cinco jobs): **solo esos dos steps más el restore de `actions/cache@v4` superan 60 s**. El resto de steps del workflow está por debajo del umbral en este `headSha` (no en un cold-cache hipotético).

No reabre `PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION` (segregación `push` vs `pull_request`). No solapa `PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY`.

## 0. Evidencia empírica (baseline)

| Campo | Valor |
| :--- | :--- |
| Run | [33477170741](https://github.com/racso80es/SddIA/actions/runs/33477170741) |
| Job | [sddia-index-integrity / 99758755444](https://github.com/racso80es/SddIA/actions/runs/33477170741/job/99758755444) |
| Evento | `pull_request` (`feat/latido-ontologico-vitalidad-organos`) |
| `headSha` | `58e0802968fb1e37e03ddd72799450c45f252a0a` |
| Fecha | 2026-09-01 |
| Wall-clock job | **13 m 36 s** (816 s) |

### 0.1 Steps del job `sddia-index-integrity`

| Step | Duración | ¿>60 s? |
| :--- | ---: | :---: |
| Set up job | 1 s | no |
| actions/checkout@v4 | 3 s | no |
| Install protoc (lance-encoding) | 13 s | no |
| dtolnay/rust-toolchain@stable | 1 s | no |
| **actions/cache@v4 (restore)** | **1 m 2 s** (62 s) | **sí** |
| Build QA aduana (execute-process + sddia-qa) | 29 s | no |
| verify-tools-index | 0 s | no |
| verify-process-integrity | 0 s | no |
| evolution-register unit tests | 1 s | no |
| **verify-compiled-capsules** | **5 m 40 s** (340 s) | **sí** |
| **LanceDB memory integration tests** | **6 m 1 s** (361 s) | **sí** |
| Post cache | 0 s | no |
| Post checkout | 1 s | no |
| Complete job | 0 s | no |

Suma de los **tres** steps >60 s: **62 + 340 + 361 = 763 s = 12 m 43 s ≈ 93 %** del job (816 s).  
Suma de los **dos** steps de compile/test: **340 + 361 = 701 s = 11 m 41 s ≈ 86 %** del job.

### 0.2 Inventario A0 — mismos `headSha` / run (steps >60 s)

Ningún step de los jobs hermanos supera 60 s en este run. Wall-clock de job >60 s ≠ step >60 s.

| Job | Job wall-clock | Steps >60 s | Nota |
| :--- | ---: | :--- | :--- |
| `sddia-index-integrity` | 816 s | cache 62 s, verify-compiled-capsules 340 s, LanceDB 361 s | objeto de este PBI |
| `eda-iota-smoke-simulate` | 86 s | ninguno (máx. build 35 s, cache 30 s) | verde |
| `wasi-runtime-smoke` | 109 s | ninguno (máx. cache 43 s, WASI build 11 s) | verde |
| `eda-bus-e2e-smoke` | 105 s | ninguno (máx. cache 47 s, nativos 26 s) | verde |
| `eda-iota-physical` | 72 s | ninguno (máx. cache 32 s) | verde; `run-iota-ci-smoke (physical)` = **0 s** → omitido por `IOTA_WALLET_SECRET` vacío (`exit 0` del YAML), no anclaje físico ejecutado |

`build-wasi-capsules.sh` **no** es cuello de botella en este run (11 s / 10 s). No se ataca en el ciclo salvo que un run cold-cache lo eleve >60 s.

## 1. Superficie de impacto (genoma verificado)

- Workflow: `.github/workflows/sddia-index-qa.yml`, job `sddia-index-integrity`.
- Gate: `SddIA/tools/sddia-qa/src/verify_compiled_capsules.rs` — **no compila**. Recorre un nivel bajo `SCAN_ROOTS` (`SddIA/{engine,skills,tools,daemons,interfaces}`) buscando `src/main.rs` y comprueba el binario en `compiled_capsules.native_root`. Hoy: **29** crates. `mandatory_bins` = `{execute-process, sddia-qa}`.
- Tests del step LanceDB (YAML vigente):
  - `sddia-core-memory` — 3 unit tests; deps serde/sha2; **sin** `lancedb`.
  - `sddia-infrastructure-lancedb-thought` — `lancedb = "=0.37.1"`; 5 tests. **Ningún binario del workspace depende de este crate** (solo miembro de `SddIA/Cargo.toml`).
  - `sddia-infrastructure-lancedb-evolution` — mismo grafo `lancedb`; 5 tests. Sí es dep de `execute-process`.
  - `cargo test -p execute-process -- memory_evolution_ingest json_fallback` — **3** tests en `memory_evolution_ingest_core.rs` (`memory_evolution_ingest_persists_to_lancedb`, `ingests_telemetry_captured_to_vector_store`, `json_fallback_is_not_used`). El filtro libtest es substring OR sobre la ruta del test; el módulo `memory_evolution_ingest_core` hace coincidir los tres. `json_fallback` es redundante. Total del step: **16** tests. El crate `execute-process` declara **366** `#[test]`; el filtro solo afecta **runtime**, no el grafo `cfg(test)`.

## 2. Diagnóstico causal

### 2.1 `verify-compiled-capsules` (340 s)

El step **no** es el gate. YAML vigente:

```yaml
- name: verify-compiled-capsules
  run: |
    cd SddIA
    cargo build --workspace
    target/debug/sddia-qa verify-compiled-capsules
```

El cronómetro de GitHub **no separa** `cargo build` del binario QA. Inferencia (no medición aislada): el gate es un walk de filesystem (~29 crates); el calor es el build.

Hecho empírico del mismo job: el step previo compiló `-p execute-process` y `-p sddia-qa` en **29 s** tras restore de cache. Eso **refuta** la tesis v1.0.0 de «fingerprints inválidos → recompile completo del grafo». Hay **hit parcial**: `execute-process` ya arrastra `lancedb-evolution` + Arrow. Los 340 s son el resto del workspace — **sobre todo los otros ~27 binarios nativos** que el gate exige presentes, no «librerías huérfanas».

`--workspace` vs lista de los 29 `main.rs`: la diferencia material es `sddia-infrastructure-lancedb-thought` (y libs ya deps de bins). **Sustituir `--workspace` por `-p` de `SCAN_ROOTS` no elimina los 340 s** si el gate sigue exigiendo los 29 bins.

Prohibido satisfacer CA1 **renombrando** el compile a otro step y dejando el gate en <60 s de I/O: el calor no desaparece.

### 2.2 `LanceDB memory integration tests` (361 s)

Cuatro `cargo test` **secuenciales**. Tras el `--workspace` del step anterior, las deps `lancedb`/Arrow **ya están compiladas** (al menos vía `execute-process`). El coste dominante no es «volver a compilar LanceDB desde cero»; es:

1. `cargo test -p execute-process`: recompila el orquestador entero bajo `cfg(test)` (**366** tests) para **ejecutar 3**.
2. `cfg(test)` de los dos adapters (thought no era dep de ningún bin; su lib puede no estar tan caliente como evolution).
3. Cuatro procesos Cargo (startup + grafo no compartido entre invocaciones).

`sddia-core-memory` es barato; no justifica un proceso aislado, tampoco es LanceDB.

**Reubicar** los 3 tests de ingesta al crate `lancedb-evolution` es **inviable sin ciclo de deps**: `execute-process` ya depende del adapter; el adapter no puede depender del orquestador. `ingest_domain_event_file` es `pub` en el lib (`src/lib.rs` + `engine::memory_evolution_ingest_core`). Palanca real: test de integración `tests/*.rs` (o crate de test) que enlace el **lib ya compilado sin `cfg(test)` del crate completo**), o extraer la ingesta a crate propio (alcance mayor, Forja si toca genoma indexado).

### 2.3 `actions/cache@v4` restore (62 s)

Cachea `~/.cargo/registry`, `~/.cargo/git` y `SddIA/target`. Key `native-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}` **compartida** con `eda-iota-smoke-simulate` y `eda-iota-physical` (jobs `wasi-*` usan key `wasi-…`).

Hipótesis de alta confianza (semántica documentada de `actions/cache@v4`: la key es **inmutable**; el primer `save` gana): los jobs IOTA (~72–86 s) corren en paralelo, terminan antes que integrity (816 s) y, tras cambio de `Cargo.lock`, pueden **sellar un `target/` parcial** (4 crates). Integrity restaura ese parcial → aduana 29 s + workspace 340 s. No se ha inspeccionado el blob de cache; el patrón 29 s vs 340 s + first-write-wins es la explicación operativa a verificar en A3 (no dogma de «incremental siempre inválido entre runners»).

## 3. Estrategia

Enfoque: **cortar calor de compile real**. Prohibido `skip` de tests, `continue-on-error` sobre aduanas, bajar el umbral del gate, o mover compile de step para maquillar CA1.

### Ola A0 — Inventario (cerrada para este baseline)

Tabla §0.2 = SSOT. Decisión: atacar los 3 steps de integrity; diferir wasi/e2e/iota salvo regresión >60 s en step. Reconfirmar A0 solo si el PR de cierre muestra un step nuevo >60 s (p. ej. cold cache).

### Ola A1 — `verify-compiled-capsules`

- El gate sigue exigiendo los **29** bins `main.rs`. El compile asociado permanece en el **mismo presupuesto de wall-clock** que CA1 (step nombrado **o** suma compile+verify si se separan).
- Palanca real: cache de `target/` **completo** de integrity (A3), no recortar el conjunto de bins.
- Fusión «Build QA aduana» + workspace: ahorra ~29 s de doble invocación; no es el 340 s.
- Mutación de `SddIA/tools/sddia-qa/` **solo si** cambia el gate, y solo vía `entity-manager` (DA-2). YAML `.github/` no es genoma. `engine/` e `infrastructure/` tampoco están en la tabla DA-2.

### Ola A2 — Tests LanceDB

- Una invocación `cargo test` con varios `-p` (grafo compartido).
- **No** mover unit tests de ingesta al adapter (ciclo de deps).
- Evitar `cargo test -p execute-process` como vehículo de 3 aserciones: integración sobre el lib público, o crate extraído. Invariante: los 3 tests (o equivalentes nombrados) siguen en verde, más los 3+5+5 de memory/thought/evolution.
- `sddia-core-memory` en el mismo invocador.

### Ola A3 — Cache

Medir, no dogma:

| Hipótesis | Experimento |
| :--- | :--- |
| First-write-wins de key `native-*` deja `target/` parcial | Keys por job, o iota **restore-only** (sin save de `SddIA/target`), o save solo desde integrity |
| Restore de `SddIA/target` (62 s) no paga si el hit es parcial | Registry/git vs registry/git+target; wall-clock del job |
| `CARGO_INCREMENTAL=0` | Solo con números |
| sccache / mold\|lld | Solo si A1+A2+key de cache no bastan |

Prohibido partir LanceDB a job paralelo si Σ minutos-runner crece más que el ahorro de wall-clock del workflow (Filtro C).

### Ola A4 — Cierre documental

`SddIA/evolution/` anclado a `530039c9-100b-413a-b3d5-ca632d83acc6`. Baseline vs post-parche en `validacion.md` (mismo tipo de evento: `pull_request`). PBI → `docs/todos/done/` en la rama del PR.

## 4. Criterios de aceptación

- [ ] **CA1 (Umbral, anti-maquillaje):** en un run `pull_request` post-parche, `verify-compiled-capsules` y `LanceDB memory integration tests` duran **< 60 s** cada uno **con el compile que hoy vive en esos steps aún atribuido a ese presupuesto**. Si se separa compile vs verify, la **suma** compile-de-bins-del-gate + verify, y la **suma** compile-cfg(test)-adapters + tests LanceDB, deben cumplir el mismo umbral. **O** `spec.md` fija techo justificado con cronómetro **< 50 % del baseline de ese step** (340 s / 361 s) porque el suelo de compile lo impide.
- [ ] **CA2 (Cobertura):** el gate sigue exigiendo el mismo conjunto de bins `main.rs` (hoy 29 + `mandatory_bins`). Los 16 tests actuales (o superconjunto / mapeo explícito si hay integración) se ejecutan.
- [ ] **CA3 (Inventario):** A0 del run `33477170741` queda como SSOT en este PBI. El PR de cierre adjunta tabla de steps del mismo workflow; si aparece un step nuevo >60 s, decisión atacar / diferir / techo.
- [ ] **CA4 (Cache):** decisión empírica sobre key `native-*` y sobre cachear `SddIA/target` (mantener / partir por job / restore-only en iota / eliminar target) en `implementation.md` con números de un run.
- [ ] **CA5 (Job):** wall-clock de `sddia-index-integrity` **< 8 min** en `pull_request` (baseline 13 m 36 s), sin `continue-on-error` ni skip de aduana.
- [ ] **CA6 (Hermandad):** `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-smoke-simulate` verdes en el PR de cierre. `eda-iota-physical` con `conclusion: success`; si el secret está ausente, el skip `exit 0` no cuenta como prueba de anclaje (fuera de alcance de este PBI).
- [ ] **CA7 (Forja):** si se muta `SddIA/tools/`, vía `./sddia-run.sh --process entity-manager`. Workflow `.github/` en el mismo PR. Evolution anclado a `530039c9-100b-413a-b3d5-ca632d83acc6`.

## 5. Fuera de alcance

- Segregación `push`/`pull_request` (PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION, done).
- Puente telemetría CI → bus local (PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY, pending).
- Semántica LanceDB de producción, MiniLM, umbrales de heartbeat.
- Debilitar el gate a solo `mandatory_bins`, bins stub, omitir `protoc`, `SDDIA_SKIP_HOOKS`.
- Optimizar `build-wasi-capsules.sh` salvo que A0 de cierre muestre step >60 s.
- Polling de GitHub Actions (DA-6).

## 6. Semilla de ciclo

```text
feature_name: kaizen-ci-step-runtime-gt-1min
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
pbi_ref: docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
```

## 7. Correcciones v1.0.0 → v1.1.0

| v1.0.0 | Hecho |
| :--- | :--- |
| «Tres steps = 11 m 41 s ≈ 85 %» | 11 m 41 s = solo verify+LanceDB. Tres steps = **12 m 43 s ≈ 93 %**. |
| Run sin `run_id` / evento | PR [33477170741](https://github.com/racso80es/SddIA/actions/runs/33477170741), job 99758755444. |
| Hermanos «fuera del cronómetro»; wasi/e2e candidatos por `build-wasi-capsules.sh` | A0 del mismo run: **ningún step hermano >60 s**; WASI build 11 s. |
| `--workspace` arrastra libs; sustituir por `SCAN_ROOTS` recorta el calor | El gate exige **29 bins**; esa palanca no mata 340 s. |
| Restore no impide rebuild (fingerprints) | Aduana 29 s prueba hit parcial. |
| «2–3 tests» / «~13 tests» | **3** ingest + **16** en el step. Filtro no silencia `ingests_telemetry_captured_to_vector_store`. |
| Reubicar tests al adapter evolution | Ciclo de deps. `ingest_domain_event_file` es `pub` en el lib. |
| Coste LanceDB = compilar lancedb/Arrow de nuevo | Deps ya compiladas vía execute-process; calor = `cfg(test)` del orquestador (**366** tests compilados / 3 ejecutados). |
| CA1 medible por nombre de step | Renombrar/partir compile maquilla el umbral; CA1 ahora suma presupuestos. |
| `SddIA/scripts/qa/build-wasi-capsules.sh` en related como superficie caliente | Fuera de alcance salvo regresión A0. |
