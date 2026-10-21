---

feature_name: eda-redb-02-writers
created: "2026-10-21"
items_applied:
  - gate y upsert
  - escritores execute-process
  - escritores daemon-runtime
  - tests del corte
execution_id: "bae36c51-f0b9-4f08-b7c8-572155ae2b93"
---
# Execution — eda-redb-02-writers

Rama `feat/eda-redb-02-writers` desde `main` (`4f1153c8`), execution `38364dd5-cf84-4b94-a1b0-9d9b78ea98ca`.

El primer commit de la rama guarda la HU, los PBI 02-07 y las notas locales de Argos. Este ciclo añade los escritores.

Tests en verde:

- `sddia-event-store`: 9 de store + 3 de gate
- `execute-process` `event_store_writers`: 4
- `sddia-daemon-runtime` `heartbeat_sidechannel_does_not_become_an_event_row`
- `sddia-qa verify-domain-subscription-parity`: `error_count` 0

`run-eda-e2e-lab` no se lanzó: forja una entidad en el repo. El contrato filesystem queda cubierto por `filesystem_still_writes_the_pending_json`.
