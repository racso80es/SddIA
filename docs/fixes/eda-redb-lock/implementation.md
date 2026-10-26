---
feature_name: eda-redb-lock
created: "2026-10-26"
process: bug-fix
branch: fix/eda-redb-lock
tracker_ref: AUT-16
---

# Implementación — AUT-16

`EventStore::open` reintenta solo `DatabaseAlreadyOpen`, con backoff y jitter, hasta `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` (2000 ms). Al agotar el tiempo devuelve `StoreError::Locked`.

`scan_presented_for_branch` consulta `presented_in_store` cuando el modo escribe en el store, y mantiene el barrido de `.events/` para el modo fichero.

`emit_vitality_event` y `emit_probe_fracture` pasan por `write_fractal_event`. En modo `redb` no crean JSON bajo `.events/`. Si `probe_bus_topology` recibe `Locked`, la sonda queda verde con causa `store ocupado`.
