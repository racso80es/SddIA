---
document_id: PBI-PPR-124-PBI-STALE-PENDING
title: "[OPERATIVO] PBI stale pending — purga copia duplicada Kalma2-feature (PPR #124)"
format: markdown
version: "1.0.1"
created: "2026-07-21"
updated: "2026-07-22"
status: done
priority: baja
process: bug-fix
uuid: e8b4a1c7-2f3d-4e6a-9b0c-7d5e8f2a1c3b
source_feature: docs/features/kaizen-kalma2-feature-cycle-observability
source_correlation_id: G79QSzhWBfGLLEQ1HhJiyAjcCfdCt1SCFY2RHTRjG66F
source_audit: docs/features/kaizen-kalma2-feature-cycle-observability/validacion.md
fix_ref: docs/fixes/pbi-stale-pending-purge-ppr-124
validacion_ref: docs/fixes/pbi-stale-pending-purge-ppr-124/validacion.md
branch_name: fix/pbi-stale-pending-purge-ppr-124
pr_url: https://github.com/racso80es/SddIA/pull/124
related:
  - docs/todos/done/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md
  - docs/features/kaizen-kalma2-feature-cycle-observability/validacion.md
incident_ref: "PBI_PENDING_STALE_COPY:NO_APTO — duplicado pending con status abierto mientras done/ tiene status done"
---

# [OPERATIVO] PBI stale pending — purga copia duplicada Kalma2-feature (PPR #124)

## Mandato

Eliminar la copia stale en `docs/todos/pending/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md` tras confirmar que el PBI canónico vive en `docs/todos/done/` con `status: done` y `pbi_archived: true` en `validacion.md`.

| Campo | Valor |
|-------|--------|
| Check origen | `PBI_PENDING_STALE_COPY: NO_APTO` |
| Canónico | `docs/todos/done/[Kaizen] ciclo Kalma2-feature — …` · `document_id: PBI-KAIZEN-KALMA2-FEATURE-CYCLE-OBS` |
| Stale | ~~`docs/todos/pending/[Kaizen] ciclo Kalma2-feature — …`~~ · **purgado 2026-07-22** |

## Criterio de cierre

- [x] Archivo stale eliminado de `docs/todos/pending/`.
- [x] Sin referencias rotas en índices o `related:` de este PBI (ruta pending retirada).
- [x] `PBI_PENDING_STALE_COPY` → APTO en validación de este fix.

## Fuera de alcance

- Reapertura del PBI Kaizen (ya cerrado en feature).
- Merge PR #124.
