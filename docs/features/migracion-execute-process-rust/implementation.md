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
| Motor genérico P1–P3 (`executor`, `workspace_init`, `thermodynamic`) | ✅ | Smoke `feature` 7 fases nativo |
| Handlers satélite P4 | 🔶 | Bridge `_execute_process_handler_bridge.py` |
| Delegación motor legacy | ✅ | Puente transitorio para procesos no portados |
| Golden harness P8 | 🔶 | `golden_orchestrator_parity.py` (`kalma2-interact`) |
| Touchpoints P10–P13 | 🔶 | event/telegram-watcher, hooks, route/eda lab, README |
| Forjas P6–P7 | ⏳ | Pendiente |
| Poda P17 | ⏳ | Gated por P8/P9 |

## 6. Deuda pendiente de implementación (roadmap accionable)

Items concretos para completar la migración tras el hito actual. Cada uno es una unidad de trabajo verificable.

### 6.1 Motor genérico nativo (Fase C — cierre)

| ID | Tarea | Criterio de cierre | Sustituye |
|----|-------|--------------------|-----------|
| P1 | Portar `run_workspace_init` (git-manager + `objectives.md`) a `engine::executor` | `feature`/`bug-fix` crean rama + objectives sin bridge | `execute_process_capsules.run_workspace_init` |
| P2 | Portar bucle genérico `execute_phase` + `is_workspace_init_phase` | Fases `executed`/`simulated`/`skipped` con paridad de envelope | `execute_process_capsules.execute_phase` |
| P3 | Portar Peaje Termodinámico (`run_thermodynamic_toll`) | Emite `Raw_Execution_Finished` + orquestación; fail-soft D3.13 | `run_thermodynamic_toll` |
| P4 | Portar handlers satélite restantes (`route-domain-event`, `telegram-fallback-responder`, kill-switch, `governance-daemon-manager`, `daemon-heartbeat-audit`) | Cada `canonical` resuelto nativo; bridge solo para no portados | dispatch en `run_process` |
| P5 | Invocación de cápsulas `wasmtime` nativa (`engine::capsules`) | `wasmtime run --dir=.` vía `std::process::Command` con paridad de I/O | rama capsule en `run_process` |

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

## 7. Cierre del ciclo (fase posterior)

- `execution.md`: se actualiza con cada hito de §6 (forja efectiva, resultados golden/smokes).
- `validacion.md`: auditoría Argos (CA-1…CA-9), `git_changes`, `global: APTO`, `pbi_archived: true`.
