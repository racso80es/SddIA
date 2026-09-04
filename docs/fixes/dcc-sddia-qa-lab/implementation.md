---
feature_name: dcc-sddia-qa-lab
created: "2026-09-04"
process: bug-fix
version_implementation: "1.0.0"
items:
  - ignition-release-pkg-sddia-qa
  - ignition-debug-sddia-qa
---

# Implementación — Ola 2 `sddia-qa`

## Cambios

| Archivo | Cambio |
|---------|--------|
| `start-sddia.sh` | `sddia-qa` en `release_pkgs`; debug `-p sddia-qa` |
| `SddIA/scripts/qa/git-hooks/hook_common.sh` | Intacta (SSOT de paths) |
