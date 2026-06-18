---
feature_name: migracion-execute-process-rust
created: "2026-06-18"
process: feature
branch_name: feat/migracion-execute-process-rust
persist_ref: docs/features/migracion-execute-process-rust
pbi_ref: docs/todos/pending/[ARQUITECTURA] Migración execute-process a Rust nativo (orquestador soberano).md
document_id: PBI-MIGRACION-EXECUTE-PROCESS-RUST
status: design
related:
  - docs/features/migracion-rust-wasi/spec.md
  - docs/features/refactor-execute-process-engine/objectives.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/Cargo.toml
  - SddIA/sddia-io/src/lib.rs
---

# Objetivos — migracion-execute-process-rust

## Misión

Liquidar la última deuda técnica del intérprete Python reescribiendo el **orquestador `execute-process.py`** (CLI + `core` + `capsules` + `forges` + cores satélite) a un **binario Rust nativo desacoplado**, con **paridad funcional estricta** respecto al intérprete dinámico certificado (`refactor-execute-process-engine`, PR #9) y contrato de invocación inmutable. El nuevo binario es la **Aduana Universal de Ejecución** del ecosistema.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| `execute-process.py` fue *out of scope* en `migracion-rust-wasi` (clarify D2) | Esta feature cierra esa exclusión declarada |
| Es el último consumidor de **PyYAML** en orquestación de negocio | El porte habilita reevaluar `requirements.txt` (D3) |
| El orquestador **spawnea subprocesos** (`wasmtime`, `git`, `execute-action`) | Target = **binario nativo**, no cápsula WASI (lección D8) |
| Centinelas ya son Rust nativo e invocan `python3 … execute-process.py` | Touchpoints de invocación a actualizar, sin reescritura lógica |
| Crate `sddia-io` y workspace `SddIA/Cargo.toml` existen | Reutilización obligatoria; sin duplicar I/O ni `target/` |
| Envelope del orquestador (`data`, `execution_report`, `status_code`) ≠ `SddiaResponse` | El esquema rico debe preservarse byte-compatible para los consumidores |

## Objetivos medibles

| ID | Objetivo | Criterio de verificación |
|----|----------|--------------------------|
| **O1** | **Binario nativo en workspace** | Nuevo crate miembro de `SddIA/Cargo.toml`; `cargo build` exit 0 sin warnings lógicos |
| **O2** | **Paridad de CLI** | Acepta `--process`, `--inputs`, `--inputs-file` y stdin JSON; misma semántica que el script |
| **O3** | **Paridad de envelope** | `{success,status_code,data,error,execution_report,exitCode}` idéntico al producido por Python para los mismos inputs |
| **O4** | **Resolución de procesos** | Parsea frontmatter YAML de `SddIA/process/{name}.md`, resuelve `name`/`aliases`, valida inputs |
| **O5** | **Ejecución de fases** | Reproduce el motor de fases, invocación de cápsulas (`wasmtime`) y handlers satélite |
| **O6** | **Forjas físicas** | `tool`/`action`/`process` forjados con paridad (UUID, hash, índices) |
| **O7** | **Peaje Termodinámico** | Emite `Raw_Execution_Finished` (telemetría) + evento orquestación; fail-soft D3.13 intacto |
| **O8** | **Desacoplamiento** | Consumidores invocan el binario vía `std::process::Command`; **no** se enlaza como librería en daemons |
| **O9** | **Safety net** | Errores → JSON `success:false`/`exitCode>0`; sin panic crudo en stdout |
| **O10** | **Touchpoints actualizados** | Centinelas, hooks, `sddia-run.sh`, cliente Kalma2 operan E2E contra el binario |
| **O11** | **Documentación viva** | `README.md`, `external-ai-constraints.md` y contratos (vía proceso autorizado) reflejan el binario |
| **O12** | **Cierre documental** | Un PR: código + cascada docs + PBI en `done/` + `validacion.md` APTO |

## Alcance de esta entrega (fase actual)

1. PBI estandarizado en `docs/todos/pending/`.
2. Cascada documental bajo `persist_ref`: `objectives.md`, `clarify.md`, `spec.md`, `plan.md`, `implementation.md`.
3. **Detenerse tras `implementation.md`** — `execution.md` (forja física Rust) y `validacion.md` (Argos) quedan para la fase de Ejecución.

## Fuera de alcance

- Rediseño funcional del orquestador o de los contratos de proceso (solo paridad).
- Migración de scripts QA residuales no-orquestadores (`verify-process-integrity.py`, `audit-doc-parity.py`).
- Migración de `execute-action.py` a Rust (se sigue invocando como subprocess; deuda separada).
- Eliminación incondicional de `requirements.txt` (condicional D3 a auditoría de consumidores residuales).
- Enlace del orquestador como librería dentro de los daemons (prohibido por Ceguera Espacial).

## Ley aplicada

- **Sustrato:** binario nativo Rust (subprocess spawning); WASI descartado por D8.
- **Soberanía de rutas:** `SddIA/core/cumulo.paths.json` — no inferir paths.
- **Genoma protegido (DA-2):** mutaciones en `process/`, `skills/`, `norms/` vía `entity-manager`/proceso autorizado.
- **Git** exclusivamente vía `skill:git-manager` en inicialización.
- **Jerarquía:** Acción → Agente → Skill → Tools; EDA vía bus.
