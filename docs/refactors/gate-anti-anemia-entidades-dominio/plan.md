---

feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-26"
process: refactorization
branch_name: refactor/gate-anti-anemia-entidades-dominio
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_ref: AUT-125
execution_id: "982404c8-5fc6-49c1-94ac-4a66ee48dc56"
---
# Plan — AUT-125

1. **T1 — Oráculo (Ola 0):** `entity_no_anemia` + `sddia-qa gate-entity-no-anemia` + tests contrafactual testigo. *(forjado)*
2. **T2 — Auditoría:** `audit-inventory.json` inventario pre-remediación (207 / 142 / 322). *(hecho)*
3. **T3 — Remediación DA-2:** solo testigo `switch-llm-provider` vía `entity-manager`; la remediación masiva con plantilla se revirtió. Resto → AUT-133. *(parcial)*
4. **T4 — Peaje DCC/PPR/forge:** `phase_capsules`, `pull_request_review`, `factory`, `entity_manager`. *(hecho)*
5. **T5 — AC-1 ratchet:** laudo Racso 2026-11-27. CI y PPR exigen `0` en `--if-touched`; línea base **322** congelada, sin incremento. Bajarla a 0 es AUT-133. AUT-131 y AUT-132 siguen como hijos de la auditoría. *(hecho, `refine-item` `c3a91e04-7b2d-4f58-9a16-8e0d5c4b7a21`)*
6. **T6 — Cierre:** CI verde sobre este contrato → `pull-request-review` → `accept-pr`. *(pendiente)*