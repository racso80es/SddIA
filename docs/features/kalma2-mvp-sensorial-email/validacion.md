---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
updated: "2026-08-18T17:50:00Z"
process: feature
branch: feat/kalma2-mvp-sensorial-email
branch_name: feat/kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
global: APTO
pbi_archived: true
approval_status: aprobado
verdict: aprobado
delivery_state: success
---

# Validación — kalma2-mvp-sensorial-email (Ola A)

## Veredicto

**APTO** — circuito **correo → Email_Received → email-triage-gateway → Email_Triaged → WUI** verificado en Paciente 0 con correo real.

| Gate | Estado | Evidencia |
|------|--------|-----------|
| G0–G5 | APTO | `execution.md` · units + CLI G5 |
| G9a lab IMAP | APTO | UID 104385 · asunto «Kalma2 validación 01A» |
| T10 A-01…A-06 | APTO | post-auditoría + F-06/F-07 |

## E2E lab (2026-08-18)

| Campo | Valor |
|-------|--------|
| UID IMAP | 104385 |
| Asunto | Kalma2 validación 01A |
| Emisor | racso80es@gmail.com |
| Veredicto | `passive` |
| `decision_path` | `llm` |
| Proof | `.SddIA/proofs/email-triaged/5e7e24e0-8121-4911-bafa-f9e39924d384.json` |
| WUI | `GET /api/status?event_id=5e7e24e0-8121-4911-bafa-f9e39924d384` → `Email_Triaged found` |

## Fricciones cerradas in-ciclo

| ID | Resolución |
|----|------------|
| F-06 | UTF-8 cabeceras IMAP (`header_value` vía `as_bytes`) |
| F-07 | Rama `email-triage-gateway` en `route_domain_core` + path en `route_fractal_core` |
| A-06 | UNSEEN prioritario + `SDDIA_EMAIL_MAX_UIDS_PER_POLL` + watermark contiguo |

## Pendiente fuera MVP

- Backfill histórico >60 días (ola futura)
- SIGKILL systemd lab formal (template presente; no gate bloqueante software)
- Catch-up masivo 60 días: mitigado por A-06; no bloquea correo nuevo

## Dictamen

```json
{
  "global": "APTO",
  "pbi_archived": true,
  "branch": "feat/kalma2-mvp-sensorial-email",
  "document_id": "PBI-KALMA2-MVP-01A"
}
```
