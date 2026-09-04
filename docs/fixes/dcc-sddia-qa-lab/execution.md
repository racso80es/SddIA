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
