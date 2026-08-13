---
feature_name: execute-process-phase-failure-propagation
created: "2026-08-11"
process: bug-fix
branch_name: fix/execute-process-phase-failure-propagation
persist_ref: docs/fixes/execute-process-phase-failure-propagation
pbi_ref: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
---

# Objetivos — execute-process-phase-failure-propagation

## Misión

---
document_id: 04f8f435-450b-477a-970a-4a05dd0224cb
title: execute-process — fallo de fase debe fallar ejecución global
type: bug-fix
status: pending
priority: critical
created: "2026-08-11"
suggested_branch: fix/execute-process-phase-failure-propagation
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-005
---

# execute-process — fallo de fase debe fallar ejecución global

## Problema

Una ejecución de `evolution-audit` devolvió `success:true`, `status_code:0` y `exitCode:0` aunque la fase `Persistencia oficial` terminó `failed` por `CERBERO_CONFIG_ERROR`.

## Objetivo

Garantizar que el estado global sea una agregación fiel de todas las fases y nunca certifique éxito con trabajo obligatorio fallido.

## Alcance

1. Unificar semántica terminal de fases en executor y residual runner.
2. Propagar `failed` a `success:false`, `status_code != 0` y `exitCode != 0`.
3. Distinguir `skipped/simulated/awaiting_agents` de fallo real.
4. Preservar `fail-soft` solo donde el contrato lo declare explícitamente.
5. Emitir telemetría con código de fallo y fase causal.

## Criterios de aceptación

- Cualquier fase obligatoria `failed` produce ejecución global fallida.
- Fases fail-soft declaradas no convierten silenciosamente errores no autorizados.
- La respuesta incluye `failed_phase`, código y diagnóstico original.
- Telemetría y evento de orquestación reflejan el mismo estado terminal.
- Tests cubren DI gate, Cerbero RBAC, cápsula, agente y persistencia fallidos.
- Regresión reproduce y corrige la ejecución `62b201cf-0d82-4153-8c7d-8223233cf476`.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
