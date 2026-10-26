---
feature_name: eda-redb-lock
created: "2026-10-26"
process: bug-fix
branch: fix/eda-redb-lock
tracker_ref: AUT-16
execution_id: "0aab3583-5c34-40e9-ba20-d8d9a527d2b8"
---

# Ejecución — AUT-16

El ciclo `bug-fix` se detuvo en Verificación (`verification-handoff-incomplete`). El corte se cerró en directo sobre `fix/eda-redb-lock`.

| Tarea | Hecho |
| --- | --- |
| T1 retry y `StoreError::Locked` | tests `open_*` del crate `sddia-event-store` |
| T2 Presented en el store | `t_redb_ca3_scan_presented_from_store_only`, `t_adu_ca7_*` |
| T3 vitalidad por el bus | `redb_ca4_vitality_and_fracture_go_to_store` |
| T4 sonda sin falso rojo | `redb_ca5_locked_store_is_green_topology` |

El lock de `redb` es por proceso. Los tests de contención usan un `flock` en otro proceso (`python3`), no un hilo del mismo proceso.
