---
feature_name: gate-evolution-evol-stem-not-v4
created: "2026-10-27"
process: bug-fix
branch_name: fix/gate-evolution-evol-stem-not-v4
persist_ref: docs/fixes/gate-evolution-evol-stem-not-v4
tracker_ref: AUT-9
document_id: PBI-e90c2357
global: APTO
pbi_archived: false
branch: fix/gate-evolution-evol-stem-not-v4
---

# Validación — gate-evolution-evol-stem-not-v4

| Check | Resultado |
|-------|-----------|
| `cargo test -p sddia-evolution-register --lib` | 26/26 OK |
| AC AUT-9 (cápsula) | `EVOL_ID_NOT_UUID_V4` + precedencia sobre `EVOL_MATERIAL_UNREGISTERED` |
| Contrato §8 | Fila `EVOL_ID_NOT_UUID_V4` |

`pbi_archived: false` — cierre en Linear vía PR merge (provider `linear`).
