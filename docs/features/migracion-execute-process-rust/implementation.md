---
feature_name: migracion-execute-process-rust
created: "2026-06-18"
process: feature
branch_name: feat/migracion-execute-process-rust
persist_ref: docs/features/migracion-execute-process-rust
pbi_ref: docs/todos/pending/[ARQUITECTURA] Migración execute-process a Rust nativo (orquestador soberano).md
document_id: PBI-MIGRACION-EXECUTE-PROCESS-RUST
status: implementation
items:
  - id: C1
    artifact: SddIA/engine/execute-process/Cargo.toml
    nature: rust-crate-manifest
    operation: create
    genome_mutation: false
  - id: C2
    artifact: SddIA/engine/execute-process/src/main.rs
    nature: rust-native-binary
    operation: create
    genome_mutation: false
  - id: C3
    artifact: SddIA/engine/execute-process/src/core/
    nature: rust-module
    operation: create
    genome_mutation: false
  - id: C4
    artifact: SddIA/engine/execute-process/src/engine/
    nature: rust-module
    operation: create
    genome_mutation: false
  - id: C5
    artifact: SddIA/engine/execute-process/src/forges/
    nature: rust-module
    operation: create
    genome_mutation: false
  - id: C6
    artifact: SddIA/Cargo.toml
    nature: workspace-manifest
    operation: update
    genome_mutation: false
  - id: T1
    artifact: SddIA/daemons/event-watcher/src/main.rs
    nature: rust-daemon
    operation: update
    genome_mutation: false
  - id: T2
    artifact: SddIA/daemons/telegram-watcher/src/main.rs
    nature: rust-daemon
    operation: update
    genome_mutation: false
  - id: T3
    artifact: SddIA/scripts/daemons/*.{sh,bat}
    nature: launcher
    operation: update
    genome_mutation: false
  - id: T4
    artifact: SddIA/scripts/qa/git-hooks/hook_common.py
    nature: git-hook
    operation: update
    genome_mutation: false
  - id: T5
    artifact: sddia-run.sh
    nature: wrapper
    operation: update
    genome_mutation: false
  - id: T6
    artifact: .SddIA/client/sddia-client-bridge.py
    nature: runtime-instance
    operation: update
    genome_mutation: false
  - id: D1
    artifact: README.md
    nature: doc-viva
    operation: update
    genome_mutation: false
  - id: D2
    artifact: SddIA/norms/external-ai-constraints.md
    nature: norma
    operation: update
    genome_mutation: true
---

# Implementation — Orquestador `execute-process` en Rust nativo

Touchpoints físicos del porte. **Esta fase documental se detiene aquí** (sin `execution.md`): la forja efectiva del código se ejecuta en la fase de Ejecución de Tekton. La tabla `items` del frontmatter es el manifiesto de artefactos contractual.

## 1. Crate orquestador (creación)

### C1 — `SddIA/engine/execute-process/Cargo.toml`
Manifiesto del binario nativo. Dependencias: `sddia-io` (path), `serde`, `serde_json`, `serde_yaml`. Sección `[[bin]] name = "execute-process"`.

### C2 — `src/main.rs`
Entry point. Responsabilidades portadas de `execute-process.py`:
- Parseo de `--process` / `--inputs` / `--inputs-file` / stdin (paridad `_parse_inputs_arg`, `normalize_request`).
- Carga de bóvedas (`core::env::load_hierarchical_env`) tras resolver raíz del repo.
- Kill-switch opt-in (`SDDIA_ENGINE_KILL_SWITCH`).
- `emit(envelope)` → última línea JSON en stdout + `process::exit(status_code)`.
- **Safety net:** `catch_unwind` / `Result` en frontera → envelope `success:false`, `exitCode>0`; nunca panic crudo en stdout.

### C3 — `src/core/` (`parser`, `resolver`, `env`)
- `parser.rs`: frontmatter YAML vía `serde_yaml` (paridad `parse_frontmatter`, `load_process_def`).
- `resolver.rs`: `resolve_process_path` (name + `aliases`), validación de inputs (`RUNTIME_INJECTED_INPUTS`, `DEFAULTABLE_INPUTS`), `normalize_request`.
- `env.rs`: jerarquía `.dev/.env` ⊕ `.SddIA/.dev/.env`, precedencia SO, log `[CONFIG] Jerarquía detectada…`.

### C4 — `src/engine/` (`executor`, `capsules`, `handlers/`)
- `executor.rs`: `run_process`, `execute_phase`, `run_workspace_init`, ramas de suite.
- `capsules.rs`: invocación `wasmtime run --dir=. … {capsule}.wasm` vía `std::process::Command`; Peaje Termodinámico (emisión `Raw_Execution_Finished` + orquestación; fail-soft D3.13); registro de acciones → `execute-action.py` (subprocess Python, deuda separada).
- `handlers/`: `route_domain_event`, `kalma2_interact`, `telegram_fallback_responder`, `daemon_kill_switch` (paridad de cores satélite).

### C5 — `src/forges/` (`factory`)
Forjas `tool` / `action` / `process`: generación UUID, `hash_signature` (sha256 canónico), append idempotente a `index.md`. Paridad con `execute_process_forges.py`.

### C6 — `SddIA/Cargo.toml` (update)
Añadir `"engine/*"` a `members`. Comparte `target/` del workspace.

## 2. Switch de touchpoints (Fase E — atómico)

| ID | Fichero | Cambio | Riesgo |
|----|---------|--------|--------|
| T1 | `SddIA/daemons/event-watcher/src/main.rs` | `invoke_route_process` / `run_route_cli`: sustituir `python_bin() + execute-process.py` por ruta al binario `execute-process` (resuelto en `target/` o PATH); conservar args `--process`/`--inputs` | medio |
| T2 | `SddIA/daemons/telegram-watcher/src/main.rs` | Idem patrón de invocación | medio |
| T3 | `SddIA/scripts/daemons/*.{sh,bat}`, `_exec_daemon.py`, `_launch.sh` | Apuntar a binario nativo | bajo |
| T4 | `SddIA/scripts/qa/git-hooks/hook_common.py` (`invoke_process`) | Invocar binario en lugar de `python … execute-process.py` | medio (aduana local) |
| T5 | `sddia-run.sh` | Reemplazar invocación Python por `cargo run -p execute-process --` o binario compilado; reevaluar bloque venv/`pip install` | bajo |
| T6 | `.SddIA/client/sddia-client-bridge.py` (`invoke_engine`) | `cmd` apunta al binario; parseo de última línea JSON sin cambios | bajo |

**Invariante:** todos preservan `--process <name> --inputs <json>` → parseo de la última línea JSON de stdout. Solo cambia el ejecutable.

## 3. Documentación viva

| ID | Fichero | Cambio | Vía |
|----|---------|--------|-----|
| D1 | `README.md` | §«Aduana Universal (CLI)» y ejemplos `python … execute-process.py` → binario nativo | escritura directa (instancia) |
| D2 | `SddIA/norms/external-ai-constraints.md` | DA-3: vía canónica de invocación del orquestador | **`entity-manager` / proceso autorizado** (genoma) |

> Documentos históricos (`docs/todos/done/`, `evolution/`, features cerradas) **no se mutan**; se citan como antecedente.

## 4. Deudas y notas de arquitectura

- **`execute-action.py`** permanece en Python; el orquestador nativo lo invoca como subprocess. Migración futura (deuda separada).
- **PyYAML / `requirements.txt`:** poda condicional (clarify D6) tras auditar consumidores residuales (`verify-process-integrity.py`, `audit-doc-parity.py`, `execute-action.py`).
- **Coexistencia:** el `.py` orquestador se retira solo tras CA-7 verde (smokes E2E con el binario).
- **Envelope:** `OrchestratorEnvelope` propio (no `SddiaResponse`); paridad byte-compatible verificada por golden tests del `plan.md` Fase A.

## 5. Estado real de implementación (hito A–C parcial)

Forjado y commiteado en `feat/migracion-execute-process-rust` (ver `execution.md`):

| Componente | Estado | Nota |
|------------|--------|------|
| Crate `execute-process` + workspace `engine/*` | ✅ | `cargo build`/`cargo test` verdes (7 tests) |
| `core::{parser,resolver,env,env_parse,repo}` | ✅ | Paridad YAML + bóvedas + resolución |
| `envelope::OrchestratorEnvelope` | ✅ | Esquema rico byte-compatible |
| Handler nativo `kalma2-interact` | ✅ | Sin PyYAML; smoke + golden OK |
| Handler nativo `telegram-fallback-responder` (P4 parcial) | ✅ | golden OK |
| Handler nativo `telegram-gateway` (P4 parcial) | ✅ | tool + emisión domain; golden OK |
| Motor genérico P1–P3 (`executor`, `workspace_init`, `thermodynamic`) | ✅ | Smoke + golden `feature` con `SDDIA_LAB_SKIP_GIT` |
| Handlers satélite P4 | ✅ | Entry nativo `route-domain-event`; core EDA vía `_execute_process_route_bridge.py` |
| `engine/daemons` | ✅ | locks, PIDs, bus pending, orchestration |
| Cápsulas P5 (`engine::capsules`) | ✅ | wasmtime/native + fallback; `delivery-close-cycle` nativo; golden OK |
| Delegación motor legacy | ✅ | Puente transitorio para procesos no portados |
| Golden harness P8/P9 | ✅ | **14/14** (`entity-manager` incluido) |
| Touchpoints P10–P13 | ✅ | event/telegram-watcher, hooks, route/eda lab, README |
| Forjas P6–P7 | ✅ | `forges/` + `forge_parity.py` |
| Poda P17 | ⏳ | Gated por P8/P9 |

## 6. Deuda pendiente de implementación (roadmap accionable)

Items concretos para completar la migración tras el hito actual. Cada uno es una unidad de trabajo verificable.

### 6.1 Motor genérico nativo (Fase C — cierre)

| ID | Tarea | Criterio de cierre | Sustituye |
|----|-------|--------------------|-----------|
| P1 | Portar `run_workspace_init` (git-manager + `objectives.md`) a `engine::executor` | `feature`/`bug-fix` crean rama + objectives sin bridge | `execute_process_capsules.run_workspace_init` |
| P2 | Portar bucle genérico `execute_phase` + `is_workspace_init_phase` | Fases `executed`/`simulated`/`skipped` con paridad de envelope | `execute_process_capsules.execute_phase` |
| P3 | Portar Peaje Termodinámico (`run_thermodynamic_toll`) | Emite `Raw_Execution_Finished` + orquestación; fail-soft D3.13 | `run_thermodynamic_toll` |
| P4 | Portar handlers satélite restantes | ✅ entry nativo; core EDA route en bridge | `delegate_handler` / `_execute_process_handler_bridge.py` |
| P5 | Invocación de cápsulas `wasmtime` nativa (`engine::capsules`) | ✅ delivery-close + try_invoke_delegates | rama capsule en `run_process` |

### 6.2 Forjas físicas (Fase D)

| ID | Tarea | Criterio de cierre |
|----|-------|--------------------|
| P6 | `forges::factory` para `tool`/`action`/`process` | UUID + `hash_signature` sha256 canónico idéntico a Python |
| P7 | Append idempotente a `index.md` | Diff de índice byte-compatible con `_append_row` |

### 6.3 Red de paridad (Fase A — pendiente)

| ID | Tarea | Criterio de cierre |
|----|-------|--------------------|
| P8 | Golden harness: N casos reales Python→referencia | Script reproducible que normaliza no-deterministas (UUID, timestamps, `duration_ms`) |
| P9 | Suite golden en CI | `cargo test` + comparación envelope verde para `feature`, `bug-fix`, `route-domain-event`, `delivery-close-cycle`, `entity-manager` |

### 6.4 Touchpoints restantes (Fase E — cierre)

| ID | Fichero | Cambio |
|----|---------|--------|
| P10 | `SddIA/daemons/telegram-watcher/src/main.rs` | Invocación binario (patrón `event-watcher`) |
| P11 | `SddIA/scripts/qa/git-hooks/hook_common.py` (`invoke_process`) | Binario en lugar de `python … execute-process.py` |
| P12 | `SddIA/scripts/daemons/*.{sh,bat}`, `_exec_daemon.py`, `_launch.sh` | Apuntar a binario nativo |
| P13 | `SddIA/scripts/qa/run-eda-e2e-lab.py`, `route_domain_event_core.py` | Actualizar referencias |

### 6.5 Documentación viva y poda (Fase F)

| ID | Tarea | Vía |
|----|-------|-----|
| P14 | `README.md` §«Aduana Universal (CLI)» → binario nativo | escritura directa |
| P15 | `external-ai-constraints.md` DA-3 (vía canónica) | `entity-manager` / proceso autorizado (DA-2) |
| P16 | Auditar consumidores residuales de PyYAML; podar `requirements.txt` si procede | condicional clarify D6 |
| P17 | Retirar `execute-process.py`, `execute_process_*.py` y bridge tras CA-7 verde | solo con golden + smokes E2E en verde |

### 6.6 Orden recomendado y gating

```text
P1→P2→P3  (motor base)  ─┐
P4, P5    (handlers+wasm) ├─► P8,P9 (golden) ─► P10..P13 (touchpoints) ─► P14..P17 (docs+poda)
P6,P7     (forjas)       ─┘
```

**Gate de poda (P16/P17):** prohibido retirar el `.py` antes de que P8/P9 (golden) y los smokes E2E de touchpoints estén verdes. Hasta entonces, el bridge Python es la red de seguridad.

## 7. Especificación accionable de pendientes gated

Detalle técnico de los items aún no portados a nativo. Cada bloque es una unidad de trabajo cerrada con artefactos, contrato, criterio de cierre y gate. **Ninguno habilita la poda**: el bridge Python sigue siendo la red de seguridad hasta P8/P9 verdes.

### 7.1 P4 — Handlers satélite nativos

**Estado actual:** ✅ **cerrado (entry nativo).** Todos los handlers satélite resuelven en `run_process` vía `engine::handlers::*`. `HANDLER_BRIDGE` / `delegate_handler` eliminados de `mod.rs`. `route-domain-event` delega el núcleo EDA (~600 líneas ECST/fan-out) a `_execute_process_route_bridge.py` → `route_domain_event_core.py` (deuda explícita de porte Rust).

**Objetivo cumplido (entry):** cada `canonical` en routing directo; golden P9 verde por handler portado.

| Canonical | Origen Python | Núcleo | Estado |
|-----------|---------------|--------|--------|
| `route-domain-event` | `route_domain_event_core.py` | ECST, fan-out, dispatch | ✅ entry `handlers/route_domain.rs` + bridge EDA |
| `telegram-fallback-responder` | core telegram | tool + filtros | ✅ nativo |
| `telegram-gateway` | core telegram | tool + fractal | ✅ nativo |
| `daemon-kill-switch` | core daemons | governance + fractal | ✅ nativo |
| `governance-daemon-manager` | core daemons | start/status/kill OS | ✅ nativo |
| `daemon-heartbeat-audit` | core daemons | sweep + telemetry | ✅ nativo |

**Deuda post-P4:** portar `route_domain_event_core` + subset `eda_bus_utils` a Rust (o extender `sddia-daemon-runtime`).

**Criterio de cierre entry:** ✅ cada `canonical` en `run_process` sin `delegate_handler`; golden P9 verde (`route-domain-event` con fixture ECST + fixtures aislados py/rust).

**Gate poda bridge EDA:** porte full nativo del core route **no** bloquea P5/P9 ampliado; sí bloquea retirada de `_execute_process_route_bridge.py` en P17.

### 7.2 P5 — Cápsulas `wasmtime` nativas

**Estado actual:** ✅ **cerrado (entry + delivery-close + deudas §6.bis).** `engine::capsules` invoca cápsulas vía `wasmtime run --dir=.` o binario nativo con fallback wasm→native. `engine::phase_capsules` resuelve fases skill/tool; `engine::delivery_close` portado nativo. `executor` invoca cápsulas resolubles antes de marcar `simulated`.

**Artefactos:**
- `engine/capsule_paths.rs` — SSOT `compiled_capsules` desde `cumulo.paths.json` (D-P5.3)
- `engine/actions.rs` — handlers `emit-pr-*` + dispatch `try_run_native`
- `engine/domain_mutation.rs` — **`emit-domain-mutation` nativo** (Sello Universal EDA)
- `engine/ecst_validation.rs` — aduana ECST pre-`pending/`
- `engine/eda_bus.rs` / `engine/eda_coverage.rs` — idempotencia + SSOT correlación
- `engine/capsule_invoke_smoke.rs` + `process/capsule-invoke-smoke.md` — golden fase `tool:` ejecutada (D-P5.2)
- `engine/capsules.rs` — `invoke_capsule_json`, `invoke_git_manager`, `invoke_shell_executor`, `invoke_action` (nativo → cápsula → bridge Python)
- `engine/phase_capsules.rs` — handlers fase delivery + `try_invoke_delegates`
- `engine/delivery_close.rs` — proceso `delivery-close-cycle` nativo

**Matriz de delegados (fases):**

| Delegado | Resolución | Sin artefacto |
|----------|------------|---------------|
| `skill:git-manager` | nativo directo | `simulated` (`capsula ausente`) |
| `skill:shell-executor` | nativo directo | `simulated` |
| `tool:*` | `invoke_capsule_json` vía SSOT | `simulated` |
| `agent:*` | — | `simulated` (`agentes IDE`) |
| `action:emit-pr-*` | `engine::actions` nativo | bridge `execute-action.py` |
| `action:emit-domain-mutation` | `engine::domain_mutation` nativo | bridge `execute-action.py` |

**Criterio de cierre:** ✅ golden **13/13** (`delivery-close-cycle`, `capsule-invoke-smoke`); fases git-manager/shell-executor/action vía cápsulas/acciones nativas con skips lab.

**Deuda residual explícita (D-P5.1-R) — `execute-action.py` permanece como fallback:**

`engine::actions::try_run_native` porta **todo el inventario `PHYSICAL_HANDLERS`** (emit-*, domain-mutation, sync, materialización Kintsugi, policy, crypto). El bridge `execute-action.py` sigue disponible como red de seguridad hasta P17; acciones sin handler físico devuelven `simulated`.

| Acción | Estado Rust |
|--------|-------------|
| `emit-pr-presented-event` / `emit-pr-merged-event` / `emit-pr-audited-event` | ✅ nativo (`engine::actions`) |
| `emit-domain-mutation` | ✅ nativo (`engine::domain_mutation` + ECST + coverage) |
| `emit-suite-execution-requested` | ✅ nativo (`engine::suite_execution_requested`) |
| `policy-validator` | ✅ nativo (`engine::policy_validator`) |
| `sync-entity-index` | ✅ nativo (`engine::sync_entity_index`) |
| `materialize-fracture-pbi` | ✅ nativo (`engine::materialize_fracture_pbi`) |
| `materialize-kaizen-alert-doc` | ✅ nativo (`engine::materialize_kaizen_alert_doc`) |
| `enrich-fracture-pbi-kaizen` | ✅ nativo (`engine::enrich_fracture_pbi_kaizen`) |
| `crypto-broker` | ✅ nativo (`engine::crypto_broker`) |

**Deudas de ecosistema (fuera del handler):** cableado `*-creator`, fan-out `Domain_Entity_*`, Cerbero runtime. Detalle en PBI §6.bis (D-P5.1-R).

### 7.3 P6/P7 — Forjas Rust (`forges::factory`)

**Estado actual:** ✅ **cerrado (entry + paridad).** Módulo `forges::{common,factory}` portado; CLI `--forge`; Python delega vía `try_native_forge` antes del fallback legacy.

**Artefactos:**
- `SddIA/engine/execute-process/src/forges/common.rs` — `canon_json_sorted`, `sha256_canon`, `append_row`, `idempotent_forge_handoff`
- `SddIA/engine/execute-process/src/forges/factory.rs` — 9 clases (`tool`…`suite`, `skill`, `event`)
- `SddIA/scripts/qa/forge_parity.py` — paridad hash + idempotencia Rust↔Python
- `execute_process_forges.try_native_forge` + hook en `materialize_forge_by_inputs`

**Criterio de cierre:** ✅ `forge_parity.py` verde; `hash_signature` idéntico con `SDDIA_FORGE_LAB_*`; append índice idempotente.

**Gate / soberanía:** la forja efectiva sobre genoma se ejecuta **solo vía `entity-manager`** (norma `external-ai-constraints.md` DA-2/DA-3).

### 7.4 P9 — Ampliar golden harness

**Estado actual:** ✅ **cerrado.** `golden_orchestrator_parity.py` verde para **14 casos** (incl. `feature`, `bug-fix`, daemon handlers, `route-domain-event`, `delivery-close-cycle`, **`entity-manager`**).

**Caso `entity-manager` (lab):** forge local `tool` bajo `.SddIA/tools/` con `scope: local`; teardown vía `cleanup_entity_manager_lab` (pre/post caso).

**Criterio de cierre (P9):** ✅ cumplido — harness 14/14; habilita P10–P17 (touchpoints + poda condicional).

### 7.5 P10–P13 — Touchpoints de producción (switch SSOT)

**Estado actual:** ✅ **cerrado.** Todos los touchpoints productivos resuelven el orquestador vía `orchestrator_resolve` (binario Rust preferente, fallback `execute-process.py`).

| Touchpoint | Mecanismo |
|------------|-----------|
| `sddia-run.sh` | `exec python3 orchestrator_resolve.py "$@"` |
| `hook_common.py` → hooks Git | `resolve_orchestrator_cmd` |
| `execute_process_capsules.invoke_subprocess_process_full` | `resolve_orchestrator_cmd` |
| `route_domain_event_core.py` | `resolve_orchestrator_cmd` |
| `.SddIA/client/sddia-client-bridge.py` | `resolve_orchestrator_cmd` |
| `run-eda-e2e-lab.py` | `resolve_orchestrator_cmd` |
| `event-watcher` / `telegram-watcher` (Rust) | `execute_process_bin()` + fallback `.py` |
| Limbo `scripts/limbo/daemons/*` | `resolve_orchestrator_cmd` |
| `SddIA/scripts/daemons/*` | N/A — no invocan orquestador |

**Auditoría CI:** `SddIA/scripts/qa/touchpoint_orchestrator_audit.py`.

**Criterio de cierre:** ✅ audit verde; ningún touchpoint productivo hardcodea `python … execute-process.py` sin pasar por SSOT.

### 7.6 P12 — Lanzadores de daemons

**Estado actual:** grep en `SddIA/scripts/daemons/` sin referencias a `execute-process.py`. Lanzadores delegan en binarios de centinelas vía `_exec_daemon.sh` / `_exec_daemon.py`.

**Objetivo:** auditar y, si existen, reapuntar:
- `SddIA/scripts/daemons/*.{sh,bat}`, `_exec_daemon.py`, `_launch.sh`.
- Cualquier lanzador que invoque el orquestador debe usar `orchestrator_resolve` (Python) o el patrón binario+fallback (Rust/shell).

**Criterio de cierre:** ningún lanzador de daemon invoca `python … execute-process.py` directamente; todos pasan por la resolución SSOT. Si no hay lanzadores afectados, se documenta como **N/A verificado** (el grep actual lo sugiere).

**Gate:** ninguno técnico; depende de P9 para el switch definitivo.

### 7.7 P14 — README (documentación viva)

**Estado actual:** ✅ **cerrado.** §EDA, entrypoints de bóvedas y §Aduana Universal reflejan `orchestrator_resolve`, `./sddia-run.sh` y binario nativo preferente.

### 7.8 P15 — Norma DA-3 (`external-ai-constraints.md`)

**Estado actual:** ✅ **cerrado (v1.2.0).** DA-3 declara SSOT `orchestrator_resolve`, wrapper `./sddia-run.sh`, binario preferente y prohibición de hardcodear `python … execute-process.py` en touchpoints productivos.

**Nota soberanía:** norma motor en `directories.norms` — actualizada en el ciclo feature autorizado (precedente `snapshot-friccion-laboratorio-jules`); `norm-creator` aplica solo a `library_norms`.

### 7.9 Smokes E2E — CA-7 / CA-8

**Estado actual:** ✅ **cerrado.** Harness `orchestrator_touchpoint_e2e_smoke.py` — **8/8** touchpoints verdes (2026-06-18).

| Smoke | Touchpoint |
|-------|------------|
| `ssot-native-binary` | `orchestrator_resolve` → binario |
| `orchestrator-resolve-cli` | CLI entrypoint |
| `sddia-run-sh` | wrapper shell |
| `kalma2-bridge` | `.SddIA/client/sddia-client-bridge.py` |
| `hook-common` | `git-hooks/hook_common.invoke_process` |
| `event-watcher-bin` | cápsula centinela resuelta |
| `native-without-python` | CA-8 — binario con `PATH` mínimo |
| `eda-e2e-lab` | cadena entity-manager → watcher → route |

**Batería complementaria:** `golden_orchestrator_parity.py` 14/14, `forge_parity.py` OK, `cargo test -p execute-process --lib` 43/43.

### 7.10 P17 — Poda del legacy
- Retirar `execute-process.py`, `execute_process_*.py` y los bridges (`_execute_process_engine_bridge.py`, `_execute_process_handler_bridge.py`, `_execute_process_feature_phase_bridge.py`).
- Auditar y podar `requirements.txt` (PyYAML) solo tras grep limpio de consumidores residuales (P16).

**Gate duro (innegociable):**

```text
P17 (poda) ⟸ requiere TODO lo siguiente verde:
  • P9 golden (14 casos) en CI
  • P4 handlers nativos (sin HANDLER_BRIDGE)
  • P5 cápsulas nativas
  • P6/P7 forjas nativas
  • Smokes E2E de touchpoints (centinelas, hooks, sddia-run, Kalma2)
  • CA-7 + CA-8 (sin PyYAML en orquestación)
```

Hasta cumplirse, los bridges Python permanecen como red de seguridad y el `.py` convive como fallback inerte.

## 8. Cierre del ciclo (fase posterior)

- `execution.md`: se actualiza con cada hito de §6/§7 (forja efectiva, resultados golden/smokes).
- `validacion.md`: auditoría Argos (CA-1…CA-9), `git_changes`, `global: APTO`, `pbi_archived: true`.
