---
feature_name: husky-pre-push-blocking-route
process: feature
created: 2026-06-09T00:00:00Z
branch: feat-husky-pre-push-blocking-route-8716941346700891712
---

# Especificación

## Contratos afectados

| Artefacto | Rol |
|-----------|-----|
| `.husky/pre-push` | Despertador inerte — delega a `pre_push_gate.py`; prohibida toda lógica de dominio |
| `SddIA/scripts/qa/route_domain_event_core.py` `__main__` | Modo `--blocking`: escribe evento pending, activa `SDDIA_LAB_ROUTE_SYNC=1`, llama `route_domain_event()` |
| `SddIA/core/event-domain-subscriptions.json` | Entrada `Local_QA_Requested` → agente `argos` → proceso `pull-request-review` |
| `SddIA/events/orchestration/local-qa-requested.md` | Clase ECST canónica del evento |

## Invariantes arquitectónicas

- **Ceguera Espacial:** `.husky/pre-push` no conoce eventos, suscriptores ni rutas de ejecución.
- **SSOT de enrutamiento:** toda decisión de dispatch reside en `event-domain-subscriptions.json` + `dispatch_subscriber`.
- **Bloqueo síncrono:** implementado exclusivamente mediante `os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"` antes de llamar a `route_domain_event()`.
- **shell=False:** todo `subprocess.run` en ruta de producción usa `shell=False`.

## Payload `Local_QA_Requested`

| Campo | Tipo | Requerido |
|-------|------|-----------|
| `branch` | string | REQUIRED |
| `emitter_context` | string | OPTIONAL |
