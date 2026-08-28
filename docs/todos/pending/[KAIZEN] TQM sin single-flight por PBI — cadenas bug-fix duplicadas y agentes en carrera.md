---
document_id: PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI
uuid: "86eabaf5-3fa2-4321-96ad-88d1b5485aa2"
title: "[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera"
format: markdown
version: "1.0.0"
created: "2026-08-28"
status: "pendiente"
priority: "alta"
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-tqm-single-flight-pbi
incident_ref: "Sesión Tekton 2026-08-28 07:31 — dos cadenas route-domain→TQM→bug-fix simultáneas sobre docs/todos/pending/[FIX] x.md"
friction_ids:
  - F-TQM-NO-DEDUP-PBI
  - F-AGENT-RACE-SAME-PERSIST-REF
  - F-WORKTREE-CROSS-WRITE-DURANTE-CICLO
depends_on:
  - PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
related:
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - docs/features/kaizen-feature-lab-init-frictions/execution.md
source_audit: "Tabla de procesos capturada en vivo durante el cierre del PR #209; dos cursor-agent concurrentes escribiendo docs/fixes/x/"
---

# [KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera

## 1. Falla Estructural y Contexto

Durante el cierre de `PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS` se observó que
`docs/fixes/x/` se ensuciaba de forma repetida pese a revertirlo varias veces. La
causa no era residuo: había un productor activo.

El `event-watcher` despachó **dos cadenas completas e independientes** para el mismo
PBI, con ocho segundos de separación y `correlation_id` distintos. Cada cadena levantó
su propio runtime de agente y su propio `cursor-agent`, ambos escribiendo sobre el
mismo `persist_ref`.

### 1.1. Cronología verificada

Capturada con `ps -eo pid,ppid,etime,lstart,cmd` a las 07:32 (hora local +0200).

| Hora | PID | Hecho |
|------|-----|-------|
| 07:31:04 | 752859 | `route-domain` sobre `.events/domain/eaa5fb5b-….json` |
| 07:31:04 | 752860 | `task-queue-manager` — `pbi_ref: docs/todos/pending/[FIX] x.md` |
| 07:31:04 | 752861 | `bug-fix` — `correlation_id: eaa5fb5b-fdc6-4911-9782-8518c6bf0801` |
| 07:31:12 | 753455 | `route-domain` sobre `.events/domain/cc6d6e2c-….json` |
| 07:31:12 | 753456 | `task-queue-manager` — **mismo** `pbi_ref` |
| 07:31:12 | 753457 | `bug-fix` — `correlation_id: cc6d6e2c-b84b-40f9-ac01-acff25ed252e` |
| 07:31:56 | 757948 | `kalma2-agent-runtime-cursor.py` de la cadena 1 → `cursor-agent --print --trust` |
| 07:32:21 | 758489 | `kalma2-agent-runtime-cursor.py` de la cadena 2 → `cursor-agent --print --trust` |

Ambas cadenas materializaron artefactos en `docs/fixes/x/`. El `_agent_handoff.md`
resultante quedó sellado con el `execution_id` de la última en escribir
(`92716387-568c-42c9-895d-2bf2aa186659`), perdiendo la traza de la otra.

## 2. Impacto

- **Coste económico directo**: dos invocaciones de `cursor-agent` de pago por cada PBI
  despachado por duplicado.
- **Corrupción de traza**: el handoff conserva un solo `execution_id`; la escritura
  perdedora queda sin evidencia y sin forma de auditarla.
- **Escritura cruzada en el worktree**: cualquier ciclo humano o de IA que trabaje en
  la misma copia ve aparecer cambios ajenos a mitad de operación, lo que obliga a
  aislarlos a mano antes de cerrar un PR (se hizo vía `git stash` acotado en el PR #209).
- **Riesgo de sobrescritura**: dos agentes sobre el mismo `persist_ref` sin bloqueo
  pueden pisarse en cualquier orden.

## 3. Hipótesis de causa raíz

`task_queue_manager.rs` implementa un `SingleFlightGuard` con `single_flight_dir`,
pero la clave de exclusión **no incluye el `pbi_ref`**. Dos eventos de dominio con
`correlation_id` distintos que apuntan al mismo PBI obtienen cada uno su guard y
avanzan en paralelo.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| TQM-CA1 | Dos eventos con distinto `correlation_id` y el mismo `pbi_ref` producen una sola cadena `bug-fix`; la segunda retorna descartada con causa explícita |
| TQM-CA2 | El guard se libera correctamente al terminar o abortar la cadena, sin dejar cerrojos huérfanos |
| TQM-CA3 | El descarte queda registrado con `correlation_id` de ambas cadenas para poder auditarlo |
| TQM-CA4 | Test unitario que simula dos despachos concurrentes sobre el mismo `pbi_ref` |
| TQM-CA5 | Smoke: dos eventos de dominio consecutivos sobre el mismo PBI levantan un único `cursor-agent` |

## 5. Notas de implementación

Considerar como clave del guard el `pbi_ref` normalizado en lugar del `correlation_id`,
o combinar ambos con precedencia del primero. Revisar también si el ciclo debería
rechazar el despacho cuando ya existe un `persist_ref` con un `execution_id` vivo
distinto: el guard `persist-execution-id-conflict` de `agent_runtime` ya cubre ese caso
a nivel de fase, pero actúa demasiado tarde, cuando el agente ya se ha invocado.
