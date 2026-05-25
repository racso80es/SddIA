---
feature_name: route-domain-event-pr-merged-resilience
created: "2026-05-25"
process: bug-fix
branch_name: fix/route-domain-event-pr-merged-resilience
persist_ref: docs/fixes/route-domain-event-pr-merged-resilience
pbi_ref: docs/todos/pending/[Kaizen] route-domain-event — resiliencia PR mergeado sin depender solo de gh en watcher.md
related_incident:
  event_id: ce5f287e-4e27-4d18-98f6-b9201596ae00
  pr_url: https://github.com/racso80es/SddIA/pull/48
---

# Objetivos — route-domain-event PR merged resilience

## Misión

Endurecer el router EDA (`route_domain_event_core.py`) para que el suscriptor **`argos.pull-request-review`** no genere dead-letter evitable cuando un `PullRequest_Presented` se procesa **después** del merge físico y la rama remota ya fue podada, eliminando la dependencia exclusiva de `gh` en el entorno del watcher.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| PR #48 mergeado 4 min tras emisión del evento | Ventana normal en flujo single-PR |
| Watcher procesó ~6,5 h después | Rama `origin/feat/…` ya ausente tras `--prune` |
| `github_pr_merged` existe desde PR #18 | Insuficiente si `gh` falla en daemon / PATH / auth |
| Smoke aduana usa `SDDIA_LAB_SKIP_GIT_CHECKOUT` | Oculta el gap en validación feature |
| Cúmulo IOTA no necesita checkout | Solo Argos sufre el fallo → DL asimétrico |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Diagnóstico cerrado** | `spec.md` documenta cadena de resolución y estados terminales |
| **O2** | **Resolución merge multicapa** | `resolve_pull_request_lifecycle` en `eda_bus_utils.py` |
| **O3** | **Router endurecido** | `dispatch_subscriber` usa resolver antes de `execute-process` |
| **O4** | **Errores explícitos** | PR abierto + rama ausente → mensaje operador claro |
| **O5** | **Estado terminal retroactivo** | `skipped-merged-retroactive` cuenta como OK en sweep |
| **O6** | **Tests** | Casos gh ausente, rama podada, PR abierto sin ref |
| **O7** | **Regresión EDA** | `test_eda_bus_v3plus.py` + `run-eda-e2e-lab.py` verde |
| **O8** | **Cierre documental** | `validacion.md` APTO + PBI en `done/` (un PR) |

## No objetivos

- Reescribir aduana `pull-request-review` (Fase 1 checkout sigue válida para PRs abiertos).
- Higiene manual del evento `ce5f287e…` en `.events/` (operador local; fuera de scope código).
- Cambiar contrato ECST de `PullRequest_Presented`.
- Sustituir `accept-pr` / `PullRequest_Merged` como cadena soberana de merge.

## Ley aplicada

- Proceso `bug-fix` v1.4.0
- `features-documentation-pattern` v1.2.1
- `SddIA/events/events-contract.md` §4 (ciclo V3+ / testigos)
- `.cursor/rules/task-closure-documental.mdc`
