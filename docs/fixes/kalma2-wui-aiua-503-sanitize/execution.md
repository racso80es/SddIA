---
feature_name: kalma2-wui-aiua-503-sanitize
created: "2026-09-09"
process: bug-fix
branch: fix/kalma2-wui-aiua-503-sanitize
execution_id: "8abff97c-9ce3-4d83-bdd8-4b08ac0d75d0"
items_applied:
  - classify-sanitize-503
  - hermetic-tests
  - intact-appjs-gemini
---

# Execution — kalma2-wui-aiua-503-sanitize

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-kalma2-wui-aiua-503-sanitize-init.json
```

`execution_id`: `8abff97c-9ce3-4d83-bdd8-4b08ac0d75d0`. workspace-init **executed**. Diseño `simulated`. Planificación commit `3ff239d`.

## Código

`sanitize_bridge_message` clasifica 503 antes del recorte 240. Prefijo `http-status-503` suficiente. Canónico 91 chars. Cero `sleep`/retry añadidos (los `thread::sleep` del crate son preexistentes: lock/SSE, no combustión Gemini).

## CA5 tests

```text
cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge -- --test-threads=1
# 41 passed; 0 failed
```

Incluye `flatten_aiua_wui_sanitizes_incident_503_blob`, `flatten_aiua_wui_sanitizes_http_status_503_prefix_alone`, `flatten_aiua_wui_preserves_gemini_model_unavailable`, `sanitize_maps_google_json_503_without_tool_prefix`, `sanitize_does_not_remap_timeout_or_join_literals`, `flatten_aiua_wui_rejects_business_failure`.

`sddia-qa evolution-register` → `24061402-d129-49b1-8a35-ee2388ee4816` (`EVOL_OK`, `alta`).

## Archivo

PBI `pending/` → `done/`. `status: cerrado`. `fix_ref: docs/fixes/kalma2-wui-aiua-503-sanitize`. `document_id` conservado.

## DCC / PR / CI

`delivery-close-cycle` `execution_id` `a94e5fec-0ccc-4b40-867f-7297be16c1a3`. Snapshot `dae9949`. PR https://github.com/racso80es/SddIA/pull/282. `PullRequest_Presented` `4dca2e3a-a5a8-456b-94fc-607242dfaf62`.

Run [34380881579](https://github.com/racso80es/SddIA/actions/runs/34380881579) sobre `dae9949`: success. Evento `push` `34380878572`: skip e2e/physical (no fallo).

## Fuera del diff

`interfaces/kalma2/app.js`, `SddIA/tools/gemini-http-infer`, `.SddIA/observability/ecosystem-health.json`, PBI Telegram kitchen.
