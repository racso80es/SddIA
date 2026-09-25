---
feature_name: paciente0-redeploy-ola10-codex-extraction
created: "2026-09-25"
process: feature
branch_name: feat/paciente0-redeploy-ola10-codex-extraction
persist_ref: docs/features/paciente0-redeploy-ola10-codex-extraction
pbi_document_id: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
execution_id: "3098fa27-fb71-48e7-8ee2-aab2bee2c468"
items_applied:
  - l0-diseno
  - l1-vault
  - l2-bundle
  - l3-creator
  - l4-ignicion
  - l5-gates
  - l6-audit-deuda
---

# Ejecución

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 ./sddia-run.sh --process feature` → `3098fa27-fb71-48e7-8ee2-aab2bee2c468`. Commit planificación `c2ccbbe`.

## Runtime

- Bundle `created_at=20260925T114055Z` 13 ELF + `user-preference-store`. `[bundle] OK`.
- Creator `b4ac5748-dc6c-4a14-9277-bfc8a497b228`: `llm_registry_materialized` + `domain_profile_materialized`.
- `start-sddia.sh` exit 128 (`fatal: no en un directorio git`). Fallback enable 5 unidades. WUI `:8766` HTTP 200.
- G3 pre-overlay: `genoma ausente …/aiua_core.md`. Absorción `faa18af8` + overlay. G3 post: `success:true` `vivo` 12.3s.
- `test-build-release-bundle-filtro-c.sh` OK.

## Entrega

Audit `docs/audits/paciente0-deploy-20260925T114629Z.md`. DEUDA v1.7.0. DCC en L7.
