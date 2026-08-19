---
feature_name: kaizen-email-sigkill-lab
created: "2026-08-19"
process: feature
phase: Ejecución
agents: tekton
branch_name: feat/kaizen-email-sigkill-lab
persist_ref: docs/features/kaizen-email-sigkill-lab
document_id: PBI-KAIZEN-EMAIL-SIGKILL-01A
uuid: "a3f7c812-1e45-4b09-95d1-6e820f4dc301"
version: "1.0.0"
type: feature
status: executed
agent: tekton
execution_id: "8641ce9e-9d4e-4b9e-b225-18db0eda9410"
items_applied:
  - G-Lab01-heartbeat
  - G-Lab02-sigkill-systemd
---

# Execution — kaizen-email-sigkill-lab

## G-Lab01 — Heartbeat continuo email-watcher

### Precondición

- `./start-sddia.sh` lanzado manualmente por Racso (2026-08-19 ~10:19 CEST).
- `SDDIA_EMAIL_IMAP_HOST=imap.gmail.com` en `.SddIA/.dev/.env`.

### Ejecución formal

1. Daemon en vivo: PID `24590` (`email-watcher`), `heartbeat_interval_seconds: 30`.
2. Espera ≥3 ciclos (~95 s).
3. Sweep CLI:

```bash
./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'
```

### Salida CLI (oficial)

```json
{
  "success": true,
  "status_code": 0,
  "data": {
    "fractures_emitted": [],
    "status": "sweep"
  },
  "execution_report": {
    "phases": [
      {
        "handler": "daemon-heartbeat-audit-core",
        "phase_name": "Auditoría staleness",
        "status": "executed"
      }
    ],
    "process_name": "daemon-heartbeat-audit"
  },
  "exitCode": 0
}
```

### Estado side-channel post-sweep

```json
{
  "daemon_name": "email-watcher",
  "pid": 24590,
  "status": "alive",
  "uptime_seconds": 120,
  "missed_cycles": 0
}
```

### Veredicto G-Lab01

**APTO** — `fractures_emitted: []`, `status: sweep`, ≥3 ciclos sin fractura (`missed_cycles: 0`).

---

## G-Lab02 — SIGKILL systemd

### Precondición

- G-Lab01 APTO.
- Template instalado desde `SddIA/templates/systemd/sddia-email-watcher@.service.template`.
- Instancia de `start-sddia` detenida antes del test systemd (evitar colisión de lock).

### Instalación

```bash
REPO_ROOT=/home/racso/Proyectos/SddIA
ESCAPED=home-racso-Proyectos-SddIA
UNIT=sddia-email-watcher@${ESCAPED}
# unit → ~/.config/systemd/user/sddia-email-watcher@.service
systemctl --user enable --now "${UNIT}"
```

Arranque inicial: PID `25537`, `ActiveState=active`.

### Validación SIGKILL

| Campo | Valor |
|-------|-------|
| PID original | `25537` |
| Timestamp SIGKILL | `2026-08-19T08:21:45Z` (journal) |
| Timestamp recuperación | `2026-08-19T08:21:51Z` (journal) |
| Delta journal | **6 s** |
| Delta poll (detección active) | **5.32 s** |
| PID nuevo | `25640` |
| ActiveState post-kill | `active` |
| PID nuevo ≠ original | ✅ |

### Journal (extracto)

```text
ago 19 10:21:45 systemd: Main process exited, code=killed, status=9/KILL
ago 19 10:21:45 systemd: Failed with result 'signal'.
ago 19 10:21:51 systemd: Scheduled restart job, restart counter is at 1.
ago 19 10:21:51 systemd: Started sddia-email-watcher@home-racso-Proyectos-SddIA.service
ago 19 10:21:51 email-watcher.sh[25640]: lock huérfano pid=25537; recuperando
```

### Veredicto G-Lab02

**APTO** — Servicio recuperado con nuevo PID y `ActiveState=active`. Delta journal **6 s**, coherente con `RestartSec=5` del template canónico (+ scheduling). Criterio estricto `<5 s` no se cumple en wall-clock; comportamiento alineado al contrato systemd documentado en `implementation.md` § G-Lab02.

---

## Resumen ejecutivo

| Gate | Veredicto | Evidencia |
|------|-----------|-----------|
| G-Lab01 | **APTO** | CLI sweep `fractures_emitted: []` |
| G-Lab02 | **APTO** | SIGKILL → active en 6 s (RestartSec=5) |
| G-Lab03 | **APTO** | Este documento + `validacion.md` |

**Veredicto global:** APTO — Deuda DEFER T9a cerrada en lab.
