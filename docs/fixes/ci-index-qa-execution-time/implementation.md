---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
tracker_ref: AUT-13
---
# Implementación — AUT-13

| Área | Cambio |
|------|--------|
| `linear_*_e2e_lab.rs` | `--lib` en `cargo test -p execute-process` |
| `prep-installer-lab-release.sh` | Release acotado a paquetes obligatorios AC-4 skip-build (sin `--workspace`) |
| `classify-sddia-index-qa-changes.sh` | Flags `lance_memory`, `linear_e2e`, `installer` |
| `sddia-index-qa.yml` | Job `changes`; poda integrity/E2E; staging WASI; env debuginfo |
| `sddia-installer-qa.yml` | Smoke instalador fuera del workflow índice |
