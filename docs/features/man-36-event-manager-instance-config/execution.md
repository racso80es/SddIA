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
status: implementation-closed-pending-verify
---

# Ejecución — man-36-event-manager-instance-config

## Intentos orquestador

| Intento | Resultado |
|---------|-----------|
| Diseño `stop_after=design` | `4fc6eadd-…` — spec/plan sellados |
| Ciclo completo | Tekton `agent-runtime-timeout` @ 600s tras Mayeuta+Dedalo OK |
| Cierre Tekton (IDE) | Compilación y tests host; parches `event-bus-audit` / `github_bridge`; docs forja |

## Evidencia local

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p sddia-event-manager-host --lib
cd SddIA && CARGO_TARGET_DIR=target cargo build -p execute-process -p event-bus-audit -p sddia-daemon-runtime
SddIA/target/debug/sddia-qa gate-event-manager-port --json
SddIA/target/debug/sddia-qa gate-eda-unit-no-real-backend --json
```

## Tareas plan.md

| Tajo | Estado |
|------|--------|
| T0–T4 | done |
| T5 tests + forja docs | done (esta pasada) |
| T6 Argos + DCC | pendiente orquestador |

## Notas

- Timeout Tekton: elevar `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` / `_EJECUCION` en bóveda antes del siguiente `refactorization` completo.
