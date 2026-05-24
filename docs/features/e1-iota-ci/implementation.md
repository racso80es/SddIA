---
feature_name: e1-iota-ci
created: "2026-05-24"
process: feature
items:
  - run-iota-ci-smoke.py
  - route_domain_event_core digest
  - sddia-index-qa workflow jobs
---

# Implementación — E.1 IOTA CI

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/run-iota-ci-smoke.py` | **Nuevo** — smoke CI simulate/físico |
| `SddIA/scripts/qa/route_domain_event_core.py` | Digest IOTA en respuesta route + `delivery_state` |
| `.github/workflows/sddia-index-qa.yml` | Jobs `eda-iota-smoke-simulate`, `eda-iota-physical` |
| `docs/features/e1-iota-ci/_smoke-iota-ci-merged.json` | Fixture `PullRequest_Merged` |
| `docs/features/e1-iota-ci/*.md` | Cascada documental feature |

## Decisiones de implementación

1. **Evento `PullRequest_Merged`** — un solo suscriptor IOTA; evita ejecutar `pull-request-review` en smoke.
2. **Limpieza post-smoke** — `_cleanup_smoke_artifacts` evita contaminar bus local/CI.
3. **Job físico tolerante** — sin `IOTA_WALLET_SECRET` → exit 0 + log (operador configura secret para anclaje real).
4. **Digest en route** — Kaizen mínimo para assert E1-CA4 sin parsear stdout de `npx tsx` externamente.
