---
document_id: merge-thermo-02-net-prune
uuid: "f3a2c1b0-9e8d-4f7a-b6c5-d4e3f2a1b0c9"
title: "Aduana Git 02 — Poda red/binarios pre-push"
format: markdown
version: "1.0.0"
status: delivered
pbi_ref: PBI-MERGE-THERMO-02-NET-PRUNE
---

# Alcance

- `hook_common.sh`: orden bus→`gh` en `should_skip_pre_push_present`; `resolve_sddia_qa` F-DEP-07; `invoke_process` exporta `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS=180` y mensaje de fallo con proceso/fase.
- `gate_evolution.rs`: `--sync-base` omite `git fetch` si `origin/main` ≤ `STALE_REF_AGE_SECS`.
- Tests: `test-merge-thermo-02-net-prune.sh`, `resolve_base_fresh_origin_main_skips_fetch_ca6`, `resolve_timeout_secs_honors_hook_env_180`.
