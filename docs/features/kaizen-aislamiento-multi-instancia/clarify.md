---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA
branch_name: feat/kaizen-aislamiento-multi-instancia
persist_ref: docs/features/kaizen-aislamiento-multi-instancia
pbi_ref: docs/todos/pending/[KAIZEN] aislamiento multi-instancia centinelas.md
document_id: PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA
uuid: "b5d19318-a0fd-440b-9aac-8c6d93f775ed"
execution_id: "3b40b62c-d048-4896-b8c1-1ee267ca7704"
mayeuta_verdict: ok
laudo: execstart-percent-f-launcher-cwd-no-pkill
---

# Clarificación — kaizen-aislamiento-multi-instancia

Transcript Mayeuta (2026-08-26). Semilla PBI v1.0.0 + audit `docs/audits/paciente0-centinelas-email-sordo-20260826.md`. Filtro A contra genoma vigente (Kaizen ignición soberana PR #191: `@%f` + WD; ExecStart aún absoluto).

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `kaizen-aislamiento-multi-instancia` |
| Rama | `feat/kaizen-aislamiento-multi-instancia` |
| `execution_id` | `3b40b62c-d048-4896-b8c1-1ee267ca7704` |
| Init lab | skips archive/delivery + pin release |

## D1 — Laudos (PBI §0bis absorbidos)

| ID | Laudo |
|----|-------|
| `%f` vs `%i` | `enable` usa especificador `%i`; `%f` = path reconstruido. `WorkingDirectory=%f` ya correcto. |
| Un ExecStart para todos | **No.** Email = `daemons/email-watcher.sh`; fábrica = `scripts/daemons/@@DAEMON_NAME@@.sh`. |
| `$PWD` único oráculo | **No.** Jerarquía env → cwd instancia → fallback SCRIPT_DIR. |
| Independencia = no lab | **Fuera.** R-07 es ensayo, no plantilla. |

## Residual genoma indexado

`SddIA/process/instance-creator.md` y `sddia-distribution-protocol.md` aún mencionan `@@SDDIA_CORE_ROOT@@` = instance_root. Mutación de esos `.md` = `entity-manager`. Runtime (plantillas + handler) es `%f` sin hornear.
