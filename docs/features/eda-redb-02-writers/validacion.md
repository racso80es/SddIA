---
feature_name: eda-redb-02-writers
branch: feat/eda-redb-02-writers
global: APTO
pbi_archived: false
checks:
  - id: AC-1
    result: APTO
    note: "SDDIA_EVENT_STORE=redb materializa Domain_Entity_Created en events y no crea JSON bajo .events/"
  - id: AC-3
    result: APTO
    note: "filesystem escribe el JSON y no abre events.redb. Paridad de suscripciones error_count 0. eda_e2e_lab no ejecutado."
  - id: CA-04
    result: APTO
    note: "dual deja el mismo event_id en fichero y en events; argos y cumulo son dos filas."
  - id: CA-05
    result: APTO
    note: "progress_traces con seq 0 y 1. El side-channel de heartbeats no es una fila."
git_changes:
  - SddIA/sddia-event-store/
  - SddIA/engine/execute-process/
  - SddIA/sddia-daemon-runtime/
  - SddIA/Cargo.lock
  - docs/features/eda-redb-02-writers/
---

# Validación — eda-redb-02-writers

El PBI sigue en `docs/todos/pending/`. `pbi_archived` queda en false hasta el cierre documental de la rama.
