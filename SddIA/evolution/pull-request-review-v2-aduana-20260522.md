---
contrato_version: "1.0.0"
id_cambio: "a7c3e891-2b4f-4d6e-9e01-pull-review-v2-20260522"
fecha: "2026-05-22T00:00:00+00:00"
autor: "feature/pull-request-review-redesign"
proyecto_origen_cambio: "SddIA"
contexto: "Aduana EDA pull-request-review v2 — reactiva a PullRequest_Presented"
descripcion_breve: "Genoma v2.0.0 (Argos/Cerbero/Cúmulo); suscripción bus; handlers lab; watcher process fan-out."
tipo_operacion: "evolucion-proceso"
cambios_realizados:
  - anterior: "pull-request-review v1.0.0 placeholder con Dedalo; sin handler ni suscripción bus"
    nuevo: "v2.0.0 — 7 fases; cableado event-subscriptions; execute_process_capsules + event-watcher"
impacto: "Alto en ciclo PR post-presentación; merge físico permanece en accept-pr."
replicacion:
  instrucciones: "Smoke: execute-process pull-request-review + emit PullRequest_Presented + event-watcher --once"
  hash_integrity: "4408f797a64d278e0107993831c72de446fb099ee5b9fc7379ec8af0961aadb3"
---

# Evolution — pull-request-review v1 → v2

## Decisiones

* **Retirada Dedalo** del escrutinio PR; consolidación en Argos + Cerbero.
* **Fase 4 TODO** reinterpretada: handoff `accept-pr`, no merge directo en aduana.
* **Suscriptor bus:** `agent: argos`, `process: pull-request-review` además de IOTA Cúmulo.

## Evidencia

`docs/features/pull-request-review-redesign/validacion.md` — E2E `62bcb6e1-f995-4edf-95d6-3745c7503303`.
