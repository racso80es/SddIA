---
feature_name: centinelas-kalma2-fracture-ola-20260722
created: "2026-07-22"
process: bug-fix
---

# Implementation

| Archivo | Cambio |
|---------|--------|
| `start-sddia.sh` | Source `sddia_shell_lib` + `_sddia_load_vault`; cleanup `rm` locks; `_wait_required_heartbeats` |
| `start-sddia.md` | v1.2.0 — bóveda, gate heartbeat, apagado con retiro de locks |
| `docs/todos/done/` | 5 PBI fractura archivados + PBI paraguas de la ola |

Sin mutación de genoma protegido (`daemons/`, `skills/`, `process/`). Remedio en artefacto de instancia (`start-sddia.sh`).
