---
feature_name: aut-131-forjas-preservar-contratos-rollback
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-131-forjas-preservar-contratos-rollback
persist_ref: docs/refactors/aut-131-forjas-preservar-contratos-rollback
tracker_ref: AUT-131
---
# Plan — AUT-131

1. **T1 — Rama:** `refactor/aut-131-forjas-preservar-contratos-rollback` desde `main` post-merge AUT-125 (#480).
2. **T2 — Código:** Confirmar paridad forjas/herencia main; renombrar módulos de test `aut131_forge_*` (trazabilidad PBI).
3. **T3 — Regresiones:** KA-FORGE-1..5 ya cubiertos en `common.rs` / `factory.rs` tests.
4. **T4 — Evolución:** `SddIA/evolution/7862a7dc-7624-4eff-bc4d-cf68db4ea714.md` + fila `Evolution_log.md`.
5. **T5 — Verificación:** `cargo test -p execute-process forges` + registro en `validacion.md`.
6. **T6 — Cierre:** PR único → `pull-request-review` → merge.