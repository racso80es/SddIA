---
document_id: TODO-LAB-FEATURE-HANDLER
title: "[ARQUITECTURA] Laboratorio — Handler físico del proceso feature"
format: markdown
version: "1.0.0"
created: "2026-05-19"
status: "cerrado"
priority: media
related:
  - SddIA/process/feature.md
  - SddIA/scripts/qa/execute-process.py
  - docs/features/pbi-005-hito2-action-engine/objectives.md
---

# TODO: Handler físico de `feature` en `execute-process.py`

## Objetivo

Cerrar la brecha entre el contrato normativo del proceso **`feature`** (seis fases con `git-manager`, Mayeuta, Dedalo, Tekton, Argos, `delivery-close-cycle`) y el comportamiento actual del laboratorio, que devuelve **`fases simuladas (sin handler fisico)`** para cualquier proceso distinto de `entity-manager`, `skill-creator` y `event-creator`.

## Deuda observada (Hito 2 PBI-005)

| Esperado (contrato) | Real (laboratorio / sesión) |
|---------------------|-----------------------------|
| Fase 1: `git-manager` crea `branch_name` | `execute-process` simula; rama creada manualmente |
| Fase 2: Mayeuta → `clarify.md`, `objectives.md` | Solo `objectives.md` mínimo escrito a mano |
| Fases 3–6 | No ejecutadas en Asalto técnico Hito 2 |

**Riesgo:** `success: true` en JSON sin efectos físicos → falsa sensación de “feature iniciada”.

## Criterios de aceptación

1. **Fase 1 mínima viva:** `run_process("feature", …)` invoca `scripts/skills/git-manager.py` con `checkout` + rama `branch_name` desde `base_branch` (fetch/pull según norma).
2. **Materialización documental mínima:** crear `persist_ref/` y `objectives.md` con frontmatter obligatorio si no existe (plantilla acotada, sin sustituir Mayeuta en features completas).
3. **Modo degradado explícito:** flag o `context` que permita “solo fase 1” sin simular fases 2–6 como `executed`.
4. **Sin falso positivo:** si fases 2–6 no tienen handler, `execution_report` debe marcar `simulated` o `skipped`, no `executed`.

## Tareas

- [x] Añadir rama `if canonical == "feature":` en `SddIA/scripts/qa/execute-process.py`.
- [x] Encapsular invocación `git-manager` (mismo patrón que `accept-pr` / docs `pbi-005-debt-liquidation/execution.md`).
- [ ] Documentar en `SddIA/process/feature.md` el perfil **laboratorio** vs **runtime IDE completo**.
- [x] Prueba: payload `feature-pbi005-hito2-init.json` deja rama + `objectives.md` sin pasos manuales.

## Referencias

| Artefacto | Ruta |
|-----------|------|
| Proceso | `SddIA/process/feature.md` |
| Laboratorio | `SddIA/scripts/qa/execute-process.py` |
| Ejemplo merge Hito 1 | `docs/features/pbi-005-debt-liquidation/execution.md` |
