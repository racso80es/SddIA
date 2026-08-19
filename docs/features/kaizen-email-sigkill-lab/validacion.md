---
feature_name: kaizen-email-sigkill-lab
branch: feat/kaizen-email-sigkill-lab
global: APTO
pbi_archived: true
created: "2026-08-19"
process: feature
document_id: PBI-KAIZEN-EMAIL-SIGKILL-01A
uuid: "a3f7c812-1e45-4b09-95d1-6e820f4dc301"
parent_pbi: PBI-KALMA2-MVP-01A
checks:
  G-Lab01_heartbeat: "APTO — daemon-heartbeat-audit sweep fractures_emitted: []"
  G-Lab02_sigkill: "APTO — SIGKILL absorbido; active en 6s (RestartSec=5 canónico)"
  G-Lab03_doc: "APTO — execution.md + validacion.md + PBI done"
git_changes:
  - docs/features/kaizen-email-sigkill-lab/
  - docs/todos/done/[OPERATIVO] email-watcher — validación SIGKILL systemd lab (kalma2-mvp-sensorial-email).md
---

# Validación — kaizen-email-sigkill-lab

**Veredicto global: APTO**

| Gate | Criterio | Estado |
|------|----------|--------|
| G-Lab01 | ≥3 ciclos heartbeat sin fractura | ✅ sweep `fractures_emitted: []` |
| G-Lab02 | SIGKILL + recuperación systemd | ✅ active + nuevo PID; delta 6s (RestartSec=5) |
| G-Lab03 | Cierre documental | ✅ |

Evidencia lab: 2026-08-19, host Racso, `SDDIA_EMAIL_IMAP_HOST` activo, `./start-sddia.sh` + systemd user unit.
