---
feature_name: pr-presented-orchestration
created: "2026-05-20"
process: feature
branch: feat/pr-presented-orchestration
global: laboratorio
---

# Validación — pr-presented-orchestration

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| Genoma `delivery-close-cycle` v1.1 sin `emit-pr-merged-event` | ✅ | `SddIA/process/delivery-close-cycle.md` |
| Norma presentación vía proceso | ✅ | `pull-request-orchestration.md` §3 |
| Evento `pr_url` OPTIONAL | ✅ | `pull-request-presented.md` v1.1 |
| Acción `emit-pr-presented-event` v1.1 | ✅ | inputs `pr_url`, `correlation_id` |
| Handler proceso fases 4–6 | ✅ | `execute_process_capsules.py` |
| Handler acción payload `pr_url` | ✅ | `execute-action.py` |
| Smoke delivery-close-cycle (lab) | ✅ | `pr_url` + `event_id` en envelope; fases 4–6 executed |
| Payload ECST con `pr_url` | ✅ | `emitter_agent: delivery-close-cycle` |
| Watcher → processed + IOTA | ⏳ | `event-watcher.py --once` tras evento en pending |

## Comando de verificación

Ver `execution.md`.
