---
document_id: PBI-REMOVE-ROUTE-DOMAIN-EVENT-ACTION
title: "Kaizen — Retirar acción deprecada route-domain-event"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "completado"
closed: "2026-05-22"
priority: ola-c-v3
feature_ref: docs/features/remove-route-domain-event-action
validacion_ref: docs/features/remove-route-domain-event-action/validacion.md
branch: feat/remove-route-domain-event-action
process: refactorization
parent_feature: docs/features/refactor-topologia-eventos-ola-c-v3
pr_ref: "https://github.com/racso80es/SddIA/pull/27"
merge_commit: "b6bb2c6"
---

# Kaizen — Retirar acción deprecada route-domain-event

**Estado:** ✅ Completado — deuda K3 cerrada; enrutamiento EDA exclusivo vía **proceso** `route-domain-event` (PR [#27](https://github.com/racso80es/SddIA/pull/27), merge `b6bb2c6`).

**Contexto:** Tras `refactor-topologia-eventos-ola-c-v3` (PR #25), la acción quedó `deprecated` con shim en `execute-action.py`. Este PBI elimina la dualidad.

## Entregables

| Ítem | Resultado |
|------|-----------|
| Acción `SddIA/actions/route-domain-event.md` | Eliminada |
| Shim `execute-action.py` | Retirado |
| Normativa §2.7 | `process:route-domain-event` |
| `events-contract.md` | Ciclo V3+ simétrico |
| `README.md` | Pipeline EDA documentado |

## Invocación canónica

```bash
# Watcher (automático)
python SddIA/scripts/daemons/event-watcher.py

# Manual
python SddIA/scripts/qa/execute-process.py --process route-domain-event \
  --inputs '{"event_file_path":".events/pending/<event_id>.json"}'
```
