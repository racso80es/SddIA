---
feature_name: dcc-lab-residual-capsules
created: "2026-09-04"
process: bug-fix
branch: fix/ignition-pre-push-guard
global: APTO
pbi_archived: true
checks:
  DCC-RES-CA1: pass
  DCC-RES-CA2: pass
  DCC-RES-CA3: pass
git_changes:
  - start-sddia.sh
  - docs/fixes/dcc-lab-residual-capsules/
  - docs/todos/done/[FIX] delivery-close-cycle — residual cápsulas DCC (ca3d901fdc9a).md
---

# Validación — residual cápsulas DCC

**Veredicto global: APTO**

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| DCC-RES-CA1 | ELF PIE ambos nombres | ✅ | release `shell-executor` 685808 B; `sddia-evolution-register` 708744 B |
| DCC-RES-CA2 | gate-evolution sin cápsula ausente | ✅ | invoca `sddia-evolution-register`; `EVOL_MATERIAL_UNREGISTERED` (rango vs `origin/main`, no este PBI) |
| DCC-RES-CA3 | `--tool shell-executor` | ✅ | `gh --version` `success: true` |

Genomas `shell-executor.md` / `sddia-evolution-register.md` no mutados.

Siguiente aduana DCC (fuera): registrar evolution de `delivery_close.rs`, `sddia_shell_lib.sh`, `hook_common.sh`, `pre_push_gate.sh`.
