---
feature_name: sddia-installer-v3-ux-executables
created: "2026-09-26"
process: feature
persist_ref: docs/features/sddia-installer-v3-ux-executables
---

# Implementación

| Artefacto | Rol |
|-----------|-----|
| `SddIA/scripts/installer/sddia-installer-ui.sh` | Presentador TTY + confirmación ELIMINAR |
| `SddIA/scripts/installer/shortcuts/*` | Plantillas atajos |
| `SddIA/scripts/sddia-installer.sh` | Comando `shortcuts`, tracking `units` |
| `installer_io.py` | `static-envelope` |
| Norma `1.3.0` | Presentador, hold, atajos |
