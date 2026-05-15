---
feature_name: codex-sync-laboratories
status: completed
updated: 2026-05-15
operation_id: CODEX_SYNC_BATCH_V1
spec_ref: docs/features/codex-sync-laboratories/spec.md
---

# Estado de Ejecución: Codex Sync Laboratories

## Línea base

- Especificación: `docs/features/codex-sync-laboratories/spec.md` (v1.0.0).
- Capability: `codex_sync` en `SddIA/agents/cumulo.instructions.json`.
- SSOT origen: `SddIA/library/codexes/`, `SddIA/library/norms/`.
- Materialización: 2026-05-15 (`CODEX_SYNC_BATCH_V1`).

## Checklist de Materialización por Laboratorio

### Laboratorio SddIA_1 (Backend Admin)
- [x] Poda de normas legadas completada.
- [x] Estructura `.SddIA/library/` generada.
- [x] Códice `codex-backend-admin-splus.md` inyectado.
- [x] Normas atómicas correspondientes inyectadas (3).
- [x] `local.paths.json` actualizado.

### Laboratorio SddIA_2 (Frontend Admin)
- [x] Poda de normas legadas completada.
- [x] Estructura `.SddIA/library/` generada.
- [x] Códice `codex-frontend-admin-splus.md` inyectado.
- [x] Normas atómicas correspondientes inyectadas (5).
- [x] `local.paths.json` actualizado.

### Laboratorio SddIA_3 (Backend Admin)
- [x] Poda de normas legadas completada.
- [x] Estructura `.SddIA/library/` generada.
- [x] Códice `codex-backend-admin-splus.md` inyectado.
- [x] Normas atómicas correspondientes inyectadas (3).
- [x] `local.paths.json` actualizado.

### Laboratorio SddIA_4 (Frontend Product)
- [x] Poda de normas legadas completada.
- [x] Estructura `.SddIA/library/` generada.
- [x] Códice `codex-frontend-product-splus.md` inyectado.
- [x] Normas atómicas correspondientes inyectadas (5).
- [x] `local.paths.json` actualizado.

## Pendiente upstream (post-batch)

- [x] `codex_sync` paso d: destino `.SddIA/library/codexes/` y `.SddIA/library/norms/` en `cumulo.instructions.json`.
- [x] `sddia-distribution-protocol.md` alineado (simetría fractal; obsolescencia de `.SddIA/norms/` para importados).
- [x] Starter-kit: normas en `.SddIA/library/norms/`, `local.paths.json` y `migrate-local-constitutions-once.py` actualizados.
