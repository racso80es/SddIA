---
feature_name: eda-redb-05-dlt-frontier
created: "2026-10-21"
phases:
  - despacho
  - suscripciones
  - fichas
  - pruebas
---

# Plan — eda-redb-05-dlt-frontier

1. Cortar el publisher en el despacho salvo las dos clases externas, y sellar solo esas cuando el store escribe.
2. Retirar el objeto IOTA de las claves locales en `event-domain-subscriptions.json` y en el espejo `event-subscriptions.json`.
3. Declarar el alcance en las clases, en `events-contract` 1.2.0 y en `pull-request-orchestration` 1.3.0 vía `entity-manager`.
4. Probar AC-5, AC-6 y la paridad con filtros, sin la suite completa de `execute-process`.
