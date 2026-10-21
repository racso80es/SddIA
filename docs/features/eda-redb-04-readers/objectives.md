---

feature_name: eda-redb-04-readers
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-04-readers
persist_ref: docs/features/eda-redb-04-readers
pbi_ref: docs/todos/done/[ARQUITECTURA] EDA Redb 04 — Lectores, SSE y CLI de inspección.md
document_id: PBI-EDA-REDB-04-READERS
execution_id: "06f21693-9776-4537-a9fe-00cc76dda132"
---
# Objetivos — eda-redb-04-readers

## Misión

Los lectores de `kalma2-bridge`, `event-bus-audit` y `sddia-qa` siguen el directorio en `filesystem`. En `dual` o `redb` leen `progress_traces`, `events`, `family_index`, `subscriber_delivery` y `dead_letter_index`. El frame SSE `event: progress` no cambia de formato. `start-sddia.sh` confirma heartbeats desde `.SddIA/daemons/state/heartbeats/` y no desde `.events/telemetry/`. Los flags `--query-events`, `--inspect-dead-letter` y `--replay-dead-letter` devuelven un envelope JSON. AC-8, AC-9, CA-07. Laudos L-REDB-04, L-REDB-05.

## Alcance

Lectores del puente, la auditoría del bus, el teardown del laboratorio y el CLI de `execute-process`. El default del store sigue siendo `filesystem`. Proofs e inbox cognitivo permanecen en disco. El sweeper no se retira.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- El crate no lee `cumulo.paths.json`. La base vive en `.SddIA/runtime/events.redb`.
