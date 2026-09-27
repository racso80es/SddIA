---
feature_name: sddia-installer-v3-ui-tty-expect
---

# Spec

| ID | Verificación |
|----|----------------|
| AC-T1 | `script` + deploy dry-run: línea `k/N —`, resumen, `SDDIA_INSTALLER_HOLD=0` |
| AC-T2 | `expect`: entrada ≠ ELIMINAR → exit 3, sin líneas de paso |
| AC-T3 | `script` + teardown `--yes` dry-run → exit 0, resumen OK |
| AC-T4 | Caso 14 smoke sin TTY (sin cambios) |
