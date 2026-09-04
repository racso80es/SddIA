---
feature_name: dcc-lab-residual-capsules
created: "2026-09-04"
process: bug-fix
branch_name: fix/ignition-pre-push-guard
items_applied:
  - ignition-release-pkg-shell-executor
  - ignition-release-pkg-sddia-evolution-register
  - ignition-debug-both
  - seal-witness-no-genome
  - verify-ca1-elf
  - verify-ca2-gate-evolution
  - verify-ca3-tool-shell-executor
---

# Ejecución — residual cápsulas DCC

## Compilación

```bash
cd SddIA && cargo build --release -p shell-executor -p sddia-evolution-register
cd SddIA && cargo build -p shell-executor -p sddia-evolution-register
execute-process --seal-capsules --inputs '{"profile":"release","write_genome":false,"write_witness":true,"names":["git-manager","shell-executor","sddia-evolution-register"]}'
```

Release 5.35s. Debug 3.85s. MIME ELF PIE. Genomas `{name}.md` sin `source_sha256`. Testigos bajo `SddIA/target/release/*.sha256`.

## DCC-RES-CA2

```bash
unset SDDIA_CAPSULE_ANCHOR
SddIA/target/release/sddia-qa gate-evolution --json --range --sync-base
```

Cápsula invocada (`entityId: sddia-evolution-register`). Sin literal `cápsula sddia-evolution-register ausente`. Veredicto de rango: `EVOL_MATERIAL_UNREGISTERED` (fuera de este residual).

## DCC-RES-CA3

```bash
./sddia-run.sh --tool shell-executor --inputs '{"executable":"gh","arguments":["--version"],"working_directory":"/home/racso/Proyectos/SddIA"}'
```

`success: true`. Sin `no encontrada bajo SddIA/target`.
