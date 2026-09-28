---
feature_name: dcc-push-non-fast-forward-4c01d65b972f
created: "2026-09-28"
process: bug-fix
branch_name: fix/dcc-push-non-fast-forward-4c01d65b972f
persist_ref: docs/fixes/dcc-push-non-fast-forward-4c01d65b972f
execution_id: "55be0f6b-b30f-47b4-a8df-5853e2b5d0cf"
items_applied:
  - delivery_close_non_ff
  - fracture_catalog_suppress
---

# Ejecución — fractura `4c01d65b972f`

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-4c01d65b972f-init.json
```

`execution_id`: `55be0f6b-b30f-47b4-a8df-5853e2b5d0cf`. Diseño: commit `c43fef0`.

## Verificación local

```bash
cd SddIA && cargo test -p execute-process --lib -- dcc_non_ff dcc_fracture_suppressed_on_push_non_fast fracture_corpus_regression dcc_push_blocked_skips
./SddIA/target/debug/sddia-qa gate-evolution --json --range --sync-base
./SddIA/target/debug/sddia-qa verify-process-integrity
./SddIA/target/debug/sddia-qa verify-tools-index
```

## entity-manager

```bash
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-dcc-non-ff.json
```

## delivery-close-cycle

`execution_id`: `9a08a08c-36ab-4461-b15b-fa355e166891`. PR: https://github.com/racso80es/SddIA/pull/313

Aduana evolution bloqueó en intento 1 (ruido snapshot); commit `3c50a89` revierte systemd/daemons/tools colaterales. Pre-push local `EVOL_OK` tras push.

## Pendiente

- CI verde en PR #313 (`head` `3c50a89`) → `accept-pr` + cierre.
