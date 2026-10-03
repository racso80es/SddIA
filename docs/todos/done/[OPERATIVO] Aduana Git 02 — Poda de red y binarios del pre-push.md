---
document_id: PBI-MERGE-THERMO-02-NET-PRUNE
uuid: "84ac63fa-4369-43b1-8b09-fb3e5f963ff7"
title: "[OPERATIVO] Aduana Git 02 — Poda de red y binarios del pre-push"
format: markdown
version: "1.0.0"
status: done
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 2
hu_order_total: 6
historia_ref: "docs/todos/historias/[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia.md"
historia_document_id: HU-MERGE-THERMODYNAMICS
cola_ejecucion: docs/todos/done/
blocked_by:
  - PBI-MERGE-THERMO-01-OBSERVABILITY
unblocks:
  - PBI-MERGE-THERMO-03-DELTA-PROFILE
  - PBI-MERGE-THERMO-06-E2E-MEASURE
baseline_decisiones:
  - "HU §2.3. Ganancia inmediata en pre-push de rama ya presentada, sin tocar el veredicto."
  - "Timeout LLM de hook = 180 s. 660 s se conserva fuera del hook."
---

# Poda de red y binarios del pre-push

HU `HU-MERGE-THERMODYNAMICS`, orden **02/06**. HU §2.3. AC-6 y la parte de `gate-evolution` de AC-5.

## 0. Filtro A

| Afirmación a evitar | Corrección |
|---|---|
| `resolve_sddia_qa` ya sigue F-DEP-07 | No. Prefiere `target/debug/sddia-qa` (1,2 GB) si existe. F-DEP-07 (`sddia_shell_lib.sh`) elige release salvo debug estrictamente más nuevo. |
| `--sync-base` debe hacer fetch siempre | `resolve_base` ya trata `origin/main` con edad ≤ `STALE_REF_AGE_SECS` (3600 s) como `synced`. El fetch es el coste evitable. |
| Bajar el timeout a 180 s en todo el motor | Solo cuando `invoke_process` corre dentro de un hook. El default 660 s de `agent_runtime.rs` no cambia. |

## 1. Intención

Quitar red y binario pesado del hilo síncrono cuando el dato local ya responde.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `should_skip_pre_push_present`: `scan_presented_for_branch` primero; `gh pr view` solo si el bus local no resuelve. |
| R-2 | `gate-evolution --sync-base` omite `git fetch` si la edad de `origin/main` ≤ `STALE_REF_AGE_SECS`. Ref ausente o vieja: fetch como hoy, presupuesto `SYNC_BUDGET_MS` (3000). |
| R-3 | `resolve_sddia_qa` aplica F-DEP-07: release salvo debug estrictamente más nuevo. |
| R-4 | `invoke_process` (hook) exporta `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS=180`. Al expirar, la fase queda `failed` y el hook sale 1 con mensaje que nombra proceso y fase. Sin fail-soft. |
| R-5 | Veto de push a `main` y bloqueo de `gate-evolution` con material en rango se mantienen (AC-5). |

## 3. Plan

1. Orden bus → `gh` en `hook_common.sh`.
2. Fetch condicionado en `gate_evolution.rs`.
3. Resolución de `sddia-qa` y timeout de hook.
4. Tests: `gh` ausente del PATH con `PullRequest_Presented` en bus; ref joven sin fetch; ref ausente con fetch; debug más nuevo que release.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-6 | Rama ya presentada resuelta por el bus no ejecuta `gh pr view`. `origin/main` fresco no dispara `git fetch` bajo `--sync-base`. | Test con `gh` fuera del PATH y ref joven: exit 0 sin red |
| AC-5a | `gate-evolution` sigue bloqueando si el rango toca material sujeto al gate. Veto de `main` intacto. | Suite `sddia-qa` + tests de `pbi-005-hito3-git-hooks` |
| AC-NET-1 | Invocación desde hook con fase que supera 180 s: exit 1 y mensaje con proceso y fase. | Test de motor con timeout forzado |
