---
feature_name: ppr-cosecha-kaizen-20260904
created: "2026-09-04"
process: bug-fix
base: main
scope: documentary-harvest
uuid: "04af1cd2-c9ee-4cc0-8b6b-af8d6b533ae0"
branch_name: docs/ppr-cosecha-kaizen-20260904
persist_ref: docs/ppr-cosecha-kaizen-20260904
pbi_ref: docs/todos/done/[FIX] delivery-close-cycle — residual cápsulas DCC (ca3d901fdc9a).md
---

# Especificación — cosecha Kaizen PPR residual

## Diagnóstico

PPR Cosecha Kaizen de `#251` materializó handoff, `validacion.md` y seeds en sinks de ignición/DCC residual. Esa carga contaminaba la ola centinelas (`#252`). Se aisló en `docs/ppr-cosecha-kaizen-20260904`.

## Alcance

- Handoff + `validacion` + `_kaizen_seed_ppr_revoked` en `docs/fixes/ignition-pre-push-guard/` y `docs/fixes/dcc-lab-residual-capsules/`.
- Sighting `PBI-RESTORE-…-PPR-REVOKED-REGISTRY` (pending) y affirm `#186` (done).
- Sink propio `docs/ppr-cosecha-kaizen-20260904/` (rama `docs/*` → persist_ref 1:1).
- Sync `origin/main` (merge `#252`) antes de DCC.

## Fuera

- PBI fractura `1479509cab7d` (DCC Apertura en forja; incidente distinto).
- Mutación de genoma. Rehab `revoked_entities`.
