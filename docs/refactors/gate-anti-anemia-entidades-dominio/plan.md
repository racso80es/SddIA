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
2. **T2 — Auditoría:** `audit-inventory.json` inventario pre-remediación (207 / 142 / 322). *(hecho)*
3. **T3 — Remediación DA-2:** solo testigo `switch-llm-provider` vía `entity-manager`; la remediación masiva con plantilla se revirtió. Resto → AUT-133. *(parcial)*
4. **T4 — Peaje DCC/PPR/forge:** `phase_capsules`, `pull_request_review`, `factory`, `entity_manager`. *(hecho)*
5. **T5 — Gate global 0:** bloqueado por AUT-133; CI en ratchet `--if-touched`. *(abierto)*
6. **T6 — Cierre:** nueva pasada `pull-request-review` sobre el HEAD corregido → CI → laudo humano sobre AC-1. *(pendiente)*