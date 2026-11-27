---
feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-26"
process: refactorization
branch_name: refactor/gate-anti-anemia-entidades-dominio
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_ref: AUT-125
---
# Plan — AUT-125

1. **T1 — Oráculo (Ola 0):** `entity_no_anemia` + `sddia-qa gate-entity-no-anemia` + tests contrafactual testigo. *(forjado)*
2. **T2 — Auditoría:** `audit-inventory.json` inventario SSOT completo. *(forjado)*
3. **T3 — Remediación DA-2:** acciones anémicas vía `entity-manager` (lote 1). *(hecho)*
4. **T4 — Peaje DCC/PPR/forge:** `phase_capsules`, `pull_request_review`, `factory::run_action_forge`. *(hecho)*
5. **T5 — Remediación resto familias + gate 0:** lotes 2 + índice acciones. *(hecho)*
6. **T6 — Cierre:** `delivery-close-cycle` → CI verde → `accept-pr` → Linear Done. *(en curso)*