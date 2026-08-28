---
feature_name: kaizen-feature-lab-init-frictions
created: "2026-08-28"
process: feature
branch_name: feat/kaizen-feature-lab-init-frictions
persist_ref: docs/features/kaizen-feature-lab-init-frictions
pbi_ref: docs/todos/pending/[KAIZEN] Init lab feature — bóveda reinyecta AGENT_RUNTIME y carrera de agentes.md
document_id: PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
uuid: "58e3c9f7-0e90-4e51-8b87-a9054a9b30fe"
execution_id: "80a3ca0d-80c5-4662-ab12-2afe757478c8"
mayeuta_verdict: ok
laudo: flag-relevo-paridad-boveda-timeout-pgid-execid-motor
---

# Objetivos — kaizen-feature-lab-init-frictions

## Misión

Que un init lab de `feature` / `bug-fix` / `refactorization` con relevo IDE sea **determinista y acotado en el tiempo**, que ningún proceso agente sobreviva a su ciclo, y que la trazabilidad documental (`execution_id`) sea emitida por el motor y verificable contra `paths.workspacesRoot`.

## Punto objetivo

> **O-LAB-INIT:** Con bóveda que define `SDDIA_AGENT_RUNTIME_COMMAND`, el init con `SDDIA_AGENT_RELAY_IDE=1` acusa JSON en <15s y las fases agente quedan `simulated`. `unset` de `COMMAND` deja de colgar el CLI. Un runtime sordo produce `agent-runtime-timeout` y muere con su grupo. Todo `execution_id` de frontmatter resuelve a workspace existente.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Flag de relevo + paridad bóveda Rust/shell | Sustituir Kalma2 agent runtime |
| Timeout motor + PGID + reentrada | PBI Aduana DLT (consumidor) |
| `execution_id` en payload/prompt/handoff + guard conflicto | Auto-review / Smart Mode Cursor |
| EM clase `daemon` + fin fail-soft + censo índice | Saneamiento retroactivo corpus Aduana |
| Gate dirty WT en init (`SDDIA_LAB_ALLOW_DIRTY`) | GC workspaces feature acumulados |
| Snapshot no barre `docs/todos/` untracked ajeno | |

## Objetivos medibles

| ID | Objetivo | Criterio (PBI) |
|----|----------|----------------|
| **O1** | Relé explícito por ambas puertas | LAB-CA1, LAB-CA2, LAB-CA3 |
| **O2** | Techo temporal + entierro | LAB-CA4, LAB-CA5 |
| **O3** | Trazabilidad no decorativa | LAB-CA6, LAB-CA7 |
| **O4** | Forja daemon ruidosa + índice honesto | LAB-CA8, LAB-CA9 |
| **O5** | Init no arrastra WIP ajeno | LAB-CA10 |
| **O6** | PBI untracked sobrevive al cierre | LAB-CA11 |

## Orden de ejecución (sello)

1. **F-AGENT-RUNTIME-NO-TIMEOUT + F-AGENT-RUNTIME-ORPHAN**
2. **F-VAULT-UNSET-REINJECT + F-VAULT-DUAL-POLICY**
3. **F-EXECUTION-ID-NO-PROPAGADO**
4. **F-DAEMON-FORGE-PORTE + F-DAEMON-INDEX-DESYNC**
5. **F-DIRTY-WT-CROSS-CHECKOUT + F-PBI-UNTRACKED-BARRIDO**

## Decisiones Mayeuta (sello)

- Relé se **pide** (`SDDIA_AGENT_RELAY_IDE=1`), no se simula por omisión.
- Una variable, una semántica, dos puertas.
- Timeout y kill viven en el motor, no en la prótesis.
- `execution_id` del acuse JSON es el único UUID válido para artefactos de este ciclo.
- Forja daemon: consolidar circuito (clase + fail-soft + censo); alcance post-forja declarado, no mutación furtiva.
- Tekton no arranca en este estímulo.

## No objetivos

- Reescribir el corpus mergeado de `kaizen-aduana-dlt-relay-supervisado`.
- Fail-hard de entregas ajenas.
- Bisturí de genoma en T0.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `CONSTITUTION_CORE` — Triaje C/A/B; Verdad Objetiva
- DA-2/DA-3: genoma vía `entity-manager` / `*-creator`; DA-4 topología activa; DA-5 fire-and-forget; DA-6 no aplica (sin CI aún)
- Rutas vía `SddIA/core/cumulo.paths.json` (`workspacesRoot`, `directories.daemons`, `env_hierarchy`, `paths.featurePath`)
- Git exclusivamente vía `skill:git-manager`
- Cierre documental en rama: PBI → `docs/todos/done/` + `validacion.md` APTO (ciclo futuro, no este estímulo)
