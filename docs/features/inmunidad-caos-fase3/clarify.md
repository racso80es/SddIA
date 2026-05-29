---
feature_name: inmunidad-caos-fase3
created: "2026-05-29"
purpose: Decisiones Fase 3 y herencia del gate Fase 2
---

# Clarificación — Fase 3 (Genoma de la Suite)

## Precondición (gate Fase 2)

Fase 2 cerrada con `validacion.md` APTO (AC2.1–AC2.3): tres procesos audit atómicos, handlers lab `run_chaos_audit_process`, RBAC Tekton/Argos ampliado. No se reabren procesos audit salvo hallazgo bloqueante durante forja del orquestador.

## Decisiones heredadas

| ID | Resolución | Uso en Fase 3 |
|----|------------|---------------|
| D0.2 | `suite` como 9.ª clase `entity-manager` | Forja `suite-creator`, SSOT, índice |
| D0.6 | Sub-workspace por `atomic_node` | `execution_id` nuevo por nodo en `execute-suite` |
| D0.7 | `survival-manifest.md` en workspace orquestador | Fase Argos del orquestador compila manifiesto |
| D0.9 | PBI en `pending/` | `validacion.md` con `pbi_archived: false` |
| D2.3 | Invocación tool vía handler lab subprocess | Reutilizar patrón; orquestador invoca `execute-process` hijo |
| H14–H15 | Subprocesos sin aislamiento workspace | **Resolver en Fase 3.C** — extensión runtime |

## Decisiones cerradas — Fase 3

| ID | Pregunta | Resolución |
|----|----------|------------|
| **D3.1** | ¿Scope de la familia `Suite`? | **`core`** — `SddIA/suites/` + contrato `suites-contract.md` en la misma carpeta (simetría `tools/`) |
| **D3.2** | ¿Formato del payload Suite? | Frontmatter YAML en `{suite_id}.md`: `execution_strategy` (`fail_fast` \| `run_all`), `atomic_nodes[]` con `process_name`, `expected_exit_code`, `timeout_ms` (opcional) |
| **D3.3** | ¿Hash signature sobre qué? | Canon SHA-256 sobre array `atomic_nodes` ordenado + `execution_strategy` + `version` (paridad `process` sobre `phases`) |
| **D3.4** | ¿Resolución `suite_id`? | Input `suite_id` (kebab-case sin extensión) → lectura `{directories.suites}/{suite_id}.md` vía Cúmulo / filesystem en lab |
| **D3.5** | ¿Sub-workspace por nodo? | `{orchestrator_workspace}/nodes/{node_index}-{process_name}/{execution_id}/` — `execution_id` UUID v4 nuevo por invocación hijo; propagar a stdin del subproceso |
| **D3.6** | ¿Invocación hijo? | `invoke_subprocess_process(repo, process_name, child_inputs)` con `child_inputs` incluyendo `workspace_path`, `execution_id`, `parent_execution_id` (trazabilidad) |
| **D3.7** | ¿Estrategia `fail_fast`? | Abortar orquestador al primer nodo con `exit_code != expected_exit_code`; nodos restantes no ejecutados; manifiesto parcial |
| **D3.8** | ¿Estrategia `run_all`? | Ejecutar todos los nodos secuencialmente en lab Fase 3; registrar cada resultado; manifiesto completo (concurrencia real = Kaizen) |
| **D3.9** | ¿Contenido `survival-manifest.md`? | Tabla Markdown: nodo, `process_name`, `execution_id`, sub-`workspace_path`, `expected_exit_code`, `actual_exit_code`, duración ms, veredicto Argos por nodo |
| **D3.10** | ¿Contexto RBAC `execute-suite`? | `chaos-engineering`, `quality-assurance`, `ecosystem-evolution` — orquestación caos bajo mismo contexto que procesos audit |
| **D3.11** | ¿Forja `suite-creator`? | Patrón `tool-creator` / `norm-creator`: fases Validación → Clasificación → Materialización → Indexación; handoff EDA vía `entity-manager` |
| **D3.12** | ¿Handler lab vs fan-out nativo? | Handler dedicado `run_execute_suite` en `execute_process_capsules.py` (SSOT Fase 3); suscripción ECST en Fase 4 |
| **D3.13** | ¿`core-full-stress` orden de nodos? | 1) `audit-thermodynamic-toll-failsoft` · 2) `audit-telemetry-compliance-breach` · 3) `audit-sandbox-isolation-rbac`; `execution_strategy: run_all`; `expected_exit_code: 0` cada uno |
| **D3.14** | ¿EDA coverage al indexar? | Upsert `eda-coverage.json` al crear Suite y `execute-suite`; backfill documentado si gate scan exige |
| **D3.15** | ¿Tests mínimos? | `test_execute_suite_core_full_stress.py`: smoke orquestador + asserts sub-workspaces en `execution_report.nodes[]` |

## Contrato común orquestador

| Campo / regla | Valor |
|---------------|-------|
| Input obligatorio | `suite_id` |
| Input opcional | `execution_strategy` override (default desde Suite spec) |
| `workspace_template` orquestador | `.SddIA/workspaces/{process_name}/{execution_id}/` |
| Sub-workspace hijo | Derivado de template del proceso hijo + prefijo `{orchestrator_ws}/nodes/{idx}-{process_name}/` |
| Salida Argos | Fase `Compilación manifiesto` → `{workspace_path}/survival-manifest.md` |
| `execution_report` | Incluir `nodes[]` con `execution_id`, `workspace_path`, `exit_code` por nodo (AC3.3) |

## Referencias

- Gate Fase 2: `docs/features/inmunidad-caos-fase2/validacion.md`
- Hallazgos: `docs/features/inmunidad-caos-fase0/impact-analysis.md` (H01–H05, H14–H15, H17)
- PBI: `docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md` § Fase 3
