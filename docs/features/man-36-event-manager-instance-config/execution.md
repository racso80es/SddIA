---
feature_name: man-36-event-manager-instance-config
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
phase: ejecucion
branch_name: refactor/man-36-event-manager-instance-config
persist_ref: docs/features/man-36-event-manager-instance-config
pbi_ref: MAN-36
tracker_ref: MAN-36
document_id: PBI-0604fb3c
agent: tekton
execution_id: "4fc6eadd-78b9-4022-8991-ed39d911e9af"
status: code-delta-ca4-pending-verify
---

# Ejecución — man-36-event-manager-instance-config

## Intentos orquestador

| Intento | Resultado |
|---------|-----------|
| Diseño `stop_after=design` | `4fc6eadd-…` — spec/plan v1.1.0 |
| Tekton runtime | timeout histórico @ 600s |
| Tekton IDE (esta pasada) | Delta CA-4 materializado vía MCP; T5 gates **no** corridos |

## Delta código (T2)

| Path relativo piloto | Cambio |
|----------------------|--------|
| `SddIA/engine/execute-process/src/engine/eda_port.rs` | Sin fallback `DEFAULT_REDB_REL`; helpers host |
| `…/route_domain_core.rs` | `path_is_resolved_redb` / `store_present` |
| `…/fractal.rs` | Token lógico `target_path` |
| `…/eda_bus.rs` | Token lógico `target_path` |
| `…/handlers/system_vitality.rs` | Sonda solo handshake |
| `SddIA/tools/event-bus-audit/src/main.rs` | Sin fallback cableado |

## Tareas plan.md

| Tajo | Estado |
|------|--------|
| T0–T1 auditoría | done (árbol vivo) |
| T2 delta CA-4 | done (código) |
| T3 Cúmulo | done (confirmado v1.12.0) |
| T4 docs residuales | parcial — Constitución/env OK; README/gates a Argos |
| T5 tests + gates | **blocked** — `run_check` CERBERO_DENY (sin `mcp.allowed_executables` en `.SddIA/project.md`) |
| Argos + DCC | pendiente orquestador |

## Evidencia pendiente (no inventar APTO)

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p sddia-event-manager-host --lib
cd SddIA && CARGO_TARGET_DIR=target cargo build -p execute-process -p event-bus-audit
SddIA/target/debug/sddia-qa gate-event-manager-port --json
SddIA/target/debug/sddia-qa gate-eda-unit-no-real-backend --json
```

## Notas

- Git vía `git-manager` MCP: working tree con diffs anteriores + delta CA-4; commit/branch no sellados en esta pasada.
- Timeout bóveda: elevar `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` si el orquestador reentra.
- Semillas Kaizen: no escritas (Tekton no escribe `docs/todos/`).
