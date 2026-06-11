---
feature_name: husky-pre-push-blocking-route
process: feature
created: 2026-06-09T00:00:00Z
branch: feat-husky-pre-push-blocking-route-8716941346700891712
---

# Plan

1. Documentación en `docs/features/husky-pre-push-blocking-route/`.
2. Clase ECST `local-qa-requested.md` + actualización `orchestration/index.md`.
3. Registro SSOT en `event-domain-subscriptions.json`.
4. Corrección `.husky/pre-push` → despertador inerte (canon `git-hooks/pre-push` Ola B).
5. Reescritura `__main__` en `route_domain_event_core.py`: eliminar lookup manual, shell=True, y resolución de suscriptores. Implementar con `SDDIA_LAB_ROUTE_SYNC=1` + `route_domain_event()`.
