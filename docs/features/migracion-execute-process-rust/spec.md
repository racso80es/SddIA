---
feature_name: migracion-execute-process-rust
created: "2026-06-18"
process: feature
branch_name: feat/migracion-execute-process-rust
persist_ref: docs/features/migracion-execute-process-rust
---

# Especificación Técnica — Orquestador `execute-process` en Rust nativo

## 1. Propósito

Definir el contrato técnico del binario Rust nativo que reemplaza al intérprete `SddIA/scripts/qa/execute-process.py` con paridad funcional estricta, preservando el contrato de invocación CLI y el esquema de envelope JSON consumido por centinelas, hooks y el puente Kalma2.

## 2. Fronteras

| In scope | Out of scope |
|----------|--------------|
| Crate Rust nativo: CLI + core + engine + forges + handlers | Rediseño de contratos de proceso |
| Parser frontmatter YAML de `SddIA/process/*.md` | Migración de `execute-action.py` (sigue como subprocess) |
| Motor de fases + invocación cápsulas (`wasmtime`) | Reescritura de cápsulas skills/tools (ya migradas) |
| Peaje Termodinámico (telemetría + orquestación) | Poda incondicional de `requirements.txt` (condicional D6) |
| Carga jerárquica de bóvedas (`env_loader` → `core::env`) | Enlace del orquestador como librería en daemons |
| Actualización de touchpoints de invocación | Migración de scripts QA no-orquestadores |
| `README.md` + norma DA-3 (vía proceso autorizado) | Adapters LanceDB / memoria vectorial |

## 3. Arquitectura de ejecución

```text
                   ┌──────────────────────────────────────────┐
   invocación CLI  │  binario nativo `execute-process`         │
   (Command/shell) │                                            │
  ────────────────►│  main.rs ──► core::{parser,resolver,env}   │
                   │                │                           │
                   │                ▼                           │
                   │        engine::executor (fases)            │
                   │         ├─ engine::capsules ──► wasmtime run│──► *.wasm (skills/tools)
                   │         ├─ engine::handlers::* (satélites)  │
                   │         ├─ forges::factory ──► {tool,action,process}
                   │         └─ Peaje Termodinámico ──► ./.events/{telemetry,orchestration}
                   │                │                           │
                   │                ▼                           │
                   │     envelope JSON (última línea stdout)    │
                   └──────────────────────────────────────────┘
```

Subprocesos invocados por el binario (vía `std::process::Command`): `wasmtime`, `git` (vía `git-manager.wasm` o ruta lab), `execute-action.py`.

## 4. Contrato de interfaz (paridad obligatoria)

### 4.1 CLI

| Flag | Semántica | Paridad |
|------|-----------|---------|
| `--process <name>` | Nombre canónico del proceso | Idéntica |
| `--inputs <json>` | `process_inputs` inline | Idéntica |
| `--inputs-file <path>` | Ruta JSON (acepta BOM `utf-8-sig`) | Idéntica |
| *(stdin)* | JSON `{process, inputs}` o request normalizable si no hay `--process` | Idéntica (`normalize_request`) |

### 4.2 Envelope de salida (stdout, última línea)

```json
{
  "success": true,
  "status_code": 0,
  "data": { "...": "payload del proceso (handoff, workspace_path, thermodynamic_toll, ...)" },
  "error": null,
  "execution_report": { "process_name": "...", "phases": [ { "phase_name": "...", "status": "executed|simulated|skipped|failed", "handler": "..." } ] },
  "exitCode": 0
}
```

**Invariante de paridad:** mismos campos, tipos y semántica que el Python actual. Campos no deterministas excluidos de la comparación golden: `uuid`/`*_id`, `created`/timestamps, `duration_ms`, rutas de evento con UUID.

### 4.3 Reutilización de `sddia-io`

- `read_stdin_json()` para lectura de stdin.
- Semántica de `process::exit(exit_code)`.
- El envelope rico del orquestador se define como struct propia (`OrchestratorEnvelope`) por no encajar en `SddiaResponse`; comparten primitivas, no esquema.

## 5. Módulos del crate (paridad por componente)

| Módulo | Origen Python | Funciones clave a portar |
|--------|---------------|--------------------------|
| `main` | `execute-process.py` | parseo de args, carga bóvedas, kill-switch opt-in, `emit` |
| `core::parser` | `execute_process_core.py` | `parse_frontmatter` (YAML), `load_process_def` |
| `core::resolver` | `execute_process_core.py` | `resolve_process_path` (name + aliases), `normalize_request`, validación de inputs (`RUNTIME_INJECTED_INPUTS`, `DEFAULTABLE_INPUTS`) |
| `core::env` | `env_loader.py` | jerarquía `.dev/.env` ⊕ `.SddIA/.dev/.env`; log `[CONFIG] Jerarquía detectada…` |
| `engine::executor` | `execute_process_capsules.py` | `run_process`, `execute_phase`, `run_workspace_init`, estrategias de suite |
| `engine::capsules` | `execute_process_capsules.py` | invocación `wasmtime`, Peaje Termodinámico, registro de acciones |
| `engine::handlers::*` | cores satélite | `route-domain-event`, `kalma2-interact`, `telegram-fallback-responder`, kill-switch |
| `forges::factory` | `execute_process_forges.py` | `run_tool_forge`, `run_action_forge`, `run_process_forge`, hashes/índices |

## 6. Criterios de aceptación (S+ Grade)

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA-1 | Binario miembro del workspace; `cargo build` exit 0, sin warnings lógicos | CI + local |
| CA-2 | Paridad de envelope para procesos núcleo | Golden tests (Python vs Rust, normalizando no-deterministas) |
| CA-3 | Resolución de procesos por `name` y `aliases` con frontmatter YAML | Tests unitarios `core::resolver` |
| CA-4 | Invocación de cápsulas `wasmtime` y Peaje Termodinámico operativos | Smoke `run-eda-e2e-lab`, eventos en `./.events/` |
| CA-5 | Forjas `tool`/`action`/`process` con paridad (UUID, hash, índice) | Tests `forges` + diff de índices |
| CA-6 | Errores → JSON `success:false`/`exitCode>0`; sin panic en stdout | Inyección de fallos + revisión |
| CA-7 | Touchpoints (centinelas, hooks, `sddia-run.sh`, Kalma2) operan E2E | Smokes E2E |
| CA-8 | Ningún flujo de orquestación requiere PyYAML | Grep + ejecución sin venv yaml |
| CA-9 | Ciclo documental completo + PBI archivado | Argos / `validacion.md` APTO |

## 7. Dependencias

- Workspace: `SddIA/Cargo.toml`, crate `SddIA/sddia-io`, `SddIA/sddia-daemon-runtime`.
- Runtime cápsulas: `wasmtime`, target `wasm32-wasip1` (ya operativo por `migracion-rust-wasi`).
- Contrato I/O: `SddIA/norms/capsule-json-io.md`.
- Antecedente lógico: `docs/features/refactor-execute-process-engine/` (intérprete dinámico certificado).

## 8. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Deriva de paridad en envelope (campos sutiles) | Golden tests automatizados contra el Python actual antes de la poda |
| `execute-action.py` sigue en Python (cadena mixta) | Documentar como deuda; el orquestador lo invoca como subprocess estable |
| Parser YAML Rust (serde_yaml) difiere de PyYAML en casos límite | Cubrir frontmatters reales del genoma con tests de fixtures |
| Coexistencia temporal Python/Rust durante migración | Mantener el `.py` hasta CA-7 verde; cambio de touchpoints atómico por PR |
