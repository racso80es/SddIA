---

feature_name: eda-redb-02-writers
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-02-writers
persist_ref: docs/features/eda-redb-02-writers
pbi_ref: docs/todos/pending/[ARQUITECTURA] EDA Redb 02 — Escritores del bus según SDDIA_EVENT_STORE.md
execution_id: "bae36c51-f0b9-4f08-b7c8-572155ae2b93"
---
# Objetivos — eda-redb-02-writers

## Misión

Envolver los escritores del bus con SDDIA_EVENT_STORE=filesystem|dual|redb. Default filesystem. dual escribe fichero y events.redb y falla cerrado si redb falla. redb no crea JSON bajo .events/. Testigos en subscriber_delivery. Progreso en progress_traces. Heartbeats y proofs siguen en disco. AC-1, AC-3, CA-04, CA-05. Laudos L-REDB-02, L-REDB-04, L-REDB-05.

## Alcance

Escritores de `execute-process` y `sddia-daemon-runtime`. El default sigue siendo `filesystem`. Heartbeats, proofs e IOTA no se mueven. El watcher sigue escaneando ficheros.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- El crate no lee `cumulo.paths.json`. La ruta de la base la resuelve el llamador: `.SddIA/runtime/events.redb`.
