---
feature_name: dcc-sddia-qa-lab
created: "2026-09-04"
process: bug-fix
branch_name: fix/ignition-pre-push-guard
items_applied:
  - ignition-release-pkg-sddia-qa
  - ignition-debug-sddia-qa
  - verify-ca1-elf
  - verify-ca2-gate-evolution
  - verify-ca3-dcc-aduanas
---

# Ejecución — Ola 2 `sddia-qa`

## Compilación

```bash
cd SddIA && cargo build --release -p sddia-qa && cargo build -p sddia-qa
```

Release 15m 39s. Debug 6m 08s. MIME `application/x-pie-executable`.

## DCC-QA-CA2

```bash
SddIA/target/release/sddia-qa gate-evolution --json --range --if-touched --sync-base
```

`success: true`, `reason_codes: ["EVOL_OK"]`, `skipped: if-touched`.

## DCC-QA-CA3

`delivery-close-cycle` `execution_id` `fa1e88a6-3b16-44e0-8710-8d9925f47085`. Snapshot `ab27234`.

| Fase | Status |
|------|--------|
| Aduana integridad índices | **executed** `exitCode: 0` |
| Aduana evolution | **blocked** `EVOL_CUMULO: cápsula sddia-evolution-register ausente` — no traza `sddia-qa no encontrado` |
| Apertura en forja | failed `shell-executor` ausente (fuera de Ola 2) |
