---
feature_name: vanguardia-soberania-local
created: "2026-05-24"
process: feature
items:
  - SddIA/scripts/qa/ecst_validation.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/process/accept-pr.md
  - SddIA/actions/emit-domain-mutation.md
  - docs/features/vanguardia-soberania-local/_smoke-*.json
---

# Implementación — Vanguardia Soberanía Local

## H0 — Módulo ECST compartido

| Artefacto | Cambio |
|-----------|--------|
| `ecst_validation.py` | **Nuevo** — `load_event_class_schemas`, `validate_ecst_instance`, `validate_domain_mutation_event` |
| `route_domain_event_core.py` | Import desde módulo; eliminada duplicación |

## H1 — Track L.1 higiene `accept-pr`

| Artefacto | Cambio |
|-----------|--------|
| `execute_process_capsules.py` | `_try_delete_branch_op`, `_delete_branch_hygiene`, `_apply_branch_hygiene_state` |
| `capsule_accept_sync_cleanup` | Delete local+remoto; payload `hygiene_failure`; sin supresión silenciosa |
| `capsule_delivery_local_hygiene` | Mismo patrón |
| `run_process` | Propaga `hygiene_failure` y `closed_branch` (incluso `null`) |
| `accept-pr.md` | § Fase 4 + output `hygiene_failure` |

## H2 — Track E.2 aduana emisor

| Artefacto | Cambio |
|-----------|--------|
| `execute-action.py` | `validate_domain_mutation_event` pre-`_write_pending_event` |
| `execute_process_capsules.py` | `emit_domain_mutation` — misma aduana |
| `emit-domain-mutation.md` | Paso 1b Aduana ECST |

## Decisiones

- Payload `delete_branch` corregido: `remote: boolean` + `force: boolean` (contrato frozen).
- Merge OK + higiene fallida → `status_code: 0` con `hygiene_failure` empírico.
- Push fallido → excepción no enmascarada (delete no ejecuta).
