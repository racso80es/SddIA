---
feature_name: aplicaciones-forge-redeploy
process: null
branch: feature/kalma2-workspace-1xn-sequential
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[OPERATIVO] Workspace 1×N — redeploy instancia forjadora Aplicaciones.md
---

# Validación — redeploy Paciente 0 (Aplicaciones)

**Paciente 0:** `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`  
**Puerto WUI:** `8766` (`SDDIA_CLIENT_PORT` en bóveda instancia).

| AC | Evidencia |
|----|-----------|
| AC-OP-1..5 | Laudo §2.2; acta APTO `docs/audits/instance-deploy-home-racso-Aplicaciones-Asistencia_Tormentosa_SddIA-20260928T184255Z.md` |
| AC-OP-6 | `docs/audits/purge-legacy-SddIA_AP-20260928T181500Z.md` |

**Causa raíz `wui_http` (acta 06:41Z):** `instance-health-verify` consultaba `GET /api/status` sin `event_id` (HTTP 400). Corrección Core: probe `GET /api/system-health`; `kalma2-bridge` devuelve liveness 200 en `/api/status` sin `event_id` (binario desplegado en instancia).
