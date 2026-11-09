---
feature_name: man-37-eda-test-pyramid
created: "2026-11-09"
process: refactorization
branch_name: refactor/man-37-eda-test-pyramid
persist_ref: docs/features/man-37-eda-test-pyramid
tracker_ref: MAN-37
execution_id: "47b9678f-7ee1-4080-bd3e-12cf0d366069"
---
# Ejecución — man-37-eda-test-pyramid

## Comandos de verificación

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process test_harness --lib
cd SddIA && CARGO_TARGET_DIR=target cargo build -p sddia-qa
SddIA/target/debug/sddia-qa gate-eda-unit-no-real-backend --json
SddIA/target/debug/sddia-qa gate-event-manager-port --json
```

## Inventario T1 (resumen)

| Path | Decisión |
|------|----------|
| `eda_port.rs` (test_harness) | migrate (T0) |
| `domain_mutation`, `ecst_input_loader`, `tracker_pbi_meta` tests | migrate (T2) |
| `route_domain_core`, `eda_bus_*`, handlers diversos | reclassify integration (allowlist T3) |
| `event-bus-audit`, `repo-retention-purger`, `kalma2-bridge` tests src | reclassify e2e (allowlist T3) |

## Orquestador

| Intento | Resultado |
|---------|-----------|
| `refactorization` + `SDDIA_TQM_STOP_AFTER=design` | Diseño sellado (plan/spec) |
| `refactorization` continuo | Tekton timeout 600s; código parcial aplicado en working tree |
| Cierre local | Harness + gate + CI + docs forja (este commit) |
