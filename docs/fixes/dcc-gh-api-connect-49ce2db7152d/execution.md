---
feature_name: dcc-gh-api-connect-49ce2db7152d
created: "2026-09-08"
process: bug-fix
branch_name: fix/dcc-gh-api-connect-49ce2db7152d
persist_ref: docs/fixes/dcc-gh-api-connect-49ce2db7152d
execution_id: "0165ad09-f245-4465-aa03-280770a4ac93"
items_applied:
  - f4c-gh-api-tokens
  - stamp-ok-failed-forge
  - unit-tests
---

# Ejecución — fractura `49ce2db7152d`

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-49ce2db7152d-init.json
```

`execution_id`: `0165ad09-f245-4465-aa03-280770a4ac93`. Diseño `simulated`.

## Tests (CA-1–CA-5)

```text
cd SddIA && cargo test -p execute-process --lib -- dcc_transient dcc_fracture stamp_dcc_network
# 15 passed; 0 failed
```

## Evolution

`sddia-qa evolution-register` → `0b3ef02b-debe-4532-91da-c265ed59942b` (`EVOL_OK`, `alta`).
