---




feature_name: merge-thermo-02-net-prune
process: feature
branch: feat/merge-thermo-02-net-prune
branch_name: feat/merge-thermo-02-net-prune
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[OPERATIVO] Aduana Git 02 — Poda de red y binarios del pre-push.md
pbi_document_id: PBI-MERGE-THERMO-02-NET-PRUNE
persist_ref: docs/features/merge-thermo-02-net-prune
document_id: PBI-MERGE-THERMO-02-NET-PRUNE
uuid: "84ac63fa-4369-43b1-8b09-fb3e5f963ff7"
execution_id: "bbf47a78-0bd5-41b4-9723-87e9e48ff3d7"
---
# Validación — merge-thermo-02-net-prune

| Check | Estado |
|-------|--------|
| AC-6 bus→`gh` sin red | APTO (`test-merge-thermo-02-net-prune.sh`) |
| AC-6 fetch omitido ref fresca | APTO (`resolve_base_fresh_origin_main_skips_fetch_ca6`) |
| AC-5a veto main / gate evolution material | APTO (sin regresión; suite existente) |
| R-3 F-DEP-07 `sddia-qa` | APTO (test shell) |
| R-4 timeout 180 s en hook | APTO (`resolve_timeout_secs_honors_hook_env_180`; export en `invoke_process`) |
| PBI en `docs/todos/done/` | APTO |
