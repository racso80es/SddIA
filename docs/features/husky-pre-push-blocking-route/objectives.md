---
feature_name: husky-pre-push-blocking-route
process: feature
created: 2026-06-09T00:00:00Z
branch: feat-husky-pre-push-blocking-route-8716941346700891712
---

# Objetivos

1. Materializar la Aduana Física local (Barrera Táctil) descrita en `docs/todos/done/Barrera Táctil Local Interceptación QA Síncrona Bloqueante.md`: interceptar `git push` y bloquear si QA falla.
2. Registrar el evento `Local_QA_Requested` como Clase ECST canónica (familia orchestration) con suscriptor `argos → pull-request-review` en `event-domain-subscriptions.json`.
3. Implementar el modo `--blocking` en `route_domain_event_core.py` **exclusivamente** vía `SDDIA_LAB_ROUTE_SYNC=1` + `route_domain_event()`, sin bifurcación de código ni lógica de dominio en `__main__`.
4. Mantener el hook `.husky/pre-push` como despertador inerte: delega íntegramente a `SddIA/scripts/qa/git-hooks/pre_push_gate.py` (canon Ola B).
