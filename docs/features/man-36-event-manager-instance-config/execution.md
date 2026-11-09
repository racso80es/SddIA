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
status: t5-gates-ok-ppr-pending
execution_id: "4163f7d8-455e-45e3-9600-2ae355b99bc5"
---
# Ejecución — man-36-event-manager-instance-config

## Intentos orquestador

| Intento | Resultado |
|---------|-----------|
| Diseño `stop_after=design` | `4fc6eadd-…` — spec/plan v1.1.0 |
| Tekton runtime | timeout histórico @ 600s |
| Tekton IDE (esta pasada) | Delta CA-4 materializado; T5 gates OK (2026-11-09) |

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
| T5 tests + gates | done — `cargo test -p sddia-event-manager-host`; `gate-event-manager-port`; `gate-eda-unit-no-real-backend` |
| Evolution | done — `f7687d4a-…` |
| Argos + DCC | pendiente PPR foreground + DCC |

## Evidencia T5 (2026-11-09)

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p sddia-event-manager-host --lib  # 15 passed
SddIA/target/debug/sddia-qa gate-event-manager-port --json  # violation_count: 0
SddIA/target/debug/sddia-qa gate-eda-unit-no-real-backend --json  # ok
SddIA/target/debug/sddia-qa gate-evolution --json --range origin/main...HEAD  # tras commit evolution
```

## Notas

- Git vía `git-manager` MCP: working tree con diffs anteriores + delta CA-4; commit/branch no sellados en esta pasada.
- Timeout bóveda: elevar `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` si el orquestador reentra.
- Semillas Kaizen: no escritas (Tekton no escribe `docs/todos/`).
