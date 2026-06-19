---
document_id: PBI-KAIZEN-START-SDDIA-IGNICION
title: "[Kaizen] start-sddia — ignición ecosistema Centinelas y Kalma2"
format: markdown
version: "1.0.0"
created: "2026-06-19"
status: pending
priority: media
process: feature
branch_name: feat/kaizen-start-sddia-ignicion
feature_ref: docs/features/kaizen-start-sddia-ignicion
validacion_ref: docs/features/kaizen-start-sddia-ignicion/validacion.md
uuid: 91e8e0af-df66-4602-a1bd-9952a87cae54
---

# PBI-KAIZEN: start-sddia — ignición ecosistema Centinelas y Kalma2

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KAIZEN-START-SDDIA-IGNICION` |
| **Estatus** | 🔄 En rama — pendiente PR |
| **Feature** | [`docs/features/kaizen-start-sddia-ignicion/`](../../features/kaizen-start-sddia-ignicion/) |
| **Rama** | `feat/kaizen-start-sddia-ignicion` |

## Resumen

Corregir y documentar `start-sddia.sh` para arrancar el ecosistema operativo SddIA (4 centinelas + Kalma2) con rutas SSOT, health check y apagado limpio.

## Entregables

| Entregable | Estado |
|------------|--------|
| `start-sddia.sh` corregido | ✅ |
| `start-sddia.md` | ✅ |
| Docs Kaizen (`objectives`, `spec`, `implementation`, `execution`) | ✅ |
| Validación en caliente 4/4 + HTTP 200 | ✅ |
| PR + `validacion.md` APTO | ⏳ |

## Objetivos cumplidos

Ver [`objectives.md`](../../features/kaizen-start-sddia-ignicion/objectives.md) — O1–O6 completados en implementación.

## Deuda derivada

- Centinelas EDA aún invocan `execute-process.py` en routing de eventos pendientes (post-migración Rust).
- PBIs `[FIX] *-watcher — fractura sistémica` en `docs/todos/pending/`.
