---
feature_name: pr-presented-orchestration
created: "2026-05-20"
process: feature
---

# Implementación — Orquestación fractal PR presentado

## Entregables

| Capa | Artefacto | Versión |
|------|-----------|---------|
| Proceso | `SddIA/process/delivery-close-cycle.md` | 1.1.0 |
| Norma | `SddIA/norms/pull-request-orchestration.md` | §3 Presentación |
| Evento | `SddIA/events/pull-request-presented.md` | 1.1.0 (`pr_url` OPTIONAL) |
| Acción | `SddIA/actions/emit-pr-presented-event.md` | 1.1.0 |
| Laboratorio | `execute_process_capsules.py` | Handlers fases 1, 4–7 |
| Laboratorio | `execute-action.py` | Payload `pr_url` + `correlation_id` |

## Handlers laboratorio (`delivery-close-cycle`)

| Fase | Handler | Lab skip |
|------|---------|----------|
| Snapshot final | `delivery-snapshot-final` | `SDDIA_LAB_SKIP_SNAPSHOT` |
| Publicación remota | `delivery-remote-push` | `SDDIA_LAB_SKIP_GIT_PUSH` |
| Apertura en forja | `delivery-gh-pr` | `SDDIA_LAB_SIMULATE_GH_PR` o `inputs.pr_url` |
| Sello Presentación | `delivery-emit-pr-presented` | — |
| Higiene local | `delivery-local-hygiene` | `SDDIA_LAB_SKIP_HIGIENE` |

## Abortado

No se forjó `request-change-incorporation.md` ni handler asociado.
