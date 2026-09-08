---
feature_name: route-domain-event-enetunreach-41717b4bb229
created: "2026-09-08"
process: bug-fix
branch_name: fix/route-domain-event-enetunreach-41717b4bb229
persist_ref: docs/fixes/route-domain-event-enetunreach-41717b4bb229
execution_id: "7065b7fe-c55c-43cd-876c-78dfca1f99b1"
items_applied:
  - dlt-transient-predicate
  - mayeuta-dlt-bucket
  - unit-tests
  - e2e-capsule-simulate0
---

# Ejecución — fractura `41717b4bb229`

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-41717b4bb229-init.json
```

`execution_id`: `7065b7fe-c55c-43cd-876c-78dfca1f99b1`. Diseño `simulated`. Commit planificación `7d07fcd`.

## Tests (CA1–CA3)

```text
cd SddIA && cargo test -p execute-process --lib -- dlt_transient emit_dlt_batch_fracture analyze_fracture_kaizen enqueue_dlt_reanchor
# 19 passed; 0 failed
```

## E2E CA4 (`SIMULATE=0`)

```bash
./sddia-run.sh --tool iota-immutable-publisher --prefer-native --inputs-file .tmp/iota-publish-ca4-41717.json
```

Unit systemd `sddia-iota-publish-relay@home-racso-Proyectos-SddIA` **active**. Cápsula (no curl POST):

```text
success: false
exitCode: 1
error: iota-relay-publish-error: status=500 fetch failed | cause: ENETUNREACH
```

Sello **reproducido**. F1 (conectividad host → Testnet) sigue fuera de código. Sin `transaction_digest`. `/health` del relay (systemd active) ≠ publish 2xx (`A-HEALTH-NO-ES-PUBLISH`).

Laudo: CA4 anclaje on-chain **PENDIENTE_INSTANCIA**. El predicado CA1 cubre exactamente esta traza. No simular IOTA. No bypass raw.

## Evolution

`sddia-qa evolution-register` → `57e1dcd8-c04c-4818-818c-fb04ecf046e2` (`EVOL_OK`, `alta`).
