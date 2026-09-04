---
feature_name: dcc-git-manager-capsule-lab
created: "2026-09-04"
process: bug-fix
version_implementation: "1.0.0"
items:
  - ignition-release-pkg-git-manager
  - ignition-debug-git-manager
  - seal-witness-no-genome
---

# Implementación — Ola 1 cápsula `git-manager`

## Cambios

| Archivo | Cambio |
|---------|--------|
| `start-sddia.sh` | `git-manager` en `release_pkgs`; debug `-p execute-process -p git-manager`; sello `write_genome:false` solo para `git-manager` |
| `SddIA/scripts/common/sddia_shell_lib.sh` | Intacta (sin helper de cápsula skill) |
| `SddIA/skills/git-manager.md` | Intacta (DA-2) |

## Detalle

El sello existente (`write_genome:true`) no lista `git-manager`. Un segundo `--seal-capsules` escribe `{elf}.sha256` sin `patch_genome_source_sha256`.
