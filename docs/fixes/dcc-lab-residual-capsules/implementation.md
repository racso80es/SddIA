---
feature_name: dcc-lab-residual-capsules
created: "2026-09-04"
process: bug-fix
version_implementation: "1.0.0"
items:
  - ignition-release-pkg-shell-executor
  - ignition-release-pkg-sddia-evolution-register
  - ignition-debug-both
  - seal-witness-no-genome
---

# Implementación — residual cápsulas DCC

## Cambios

| Archivo | Cambio |
|---------|--------|
| `start-sddia.sh` | `shell-executor` y `sddia-evolution-register` en `release_pkgs` y lote debug; sello `write_genome:false` con `git-manager` |
| `SddIA/skills/*.md` | Intactos (DA-2) |
