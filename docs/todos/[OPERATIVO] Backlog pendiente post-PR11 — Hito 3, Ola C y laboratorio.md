---
document_id: TODO-BACKLOG-PENDIENTE-POST-PR11
title: "[OPERATIVO] Backlog pendiente post-PR11 — Ola C, laboratorio e higiene"
format: markdown
version: "1.2.0"
created: "2026-05-20"
updated: "2026-05-20"
status: "abierto"
priority: alta
blocks: "Deuda Ola C shims CLI / handlers laboratorio / EDA producción"
supersedes: null
related:
  - docs/todos/done/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
  - docs/features/pbi-005-hito3-ola-b
  - docs/features/pbi-005-hito3-git-hooks
  - docs/todos/[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI execute-process y execute-action.md
  - docs/todos/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md
  - SddIA/process/accept-pr.md
  - SddIA/process/delivery-close-cycle.md
---

# Backlog pendiente (consolidado)

> **Contexto (2026-05-20):** **PBI-005 cerrado al 100 %** en `main` (PR #13, merge `ed543c8`, CA-3 completo). Orquestación fractal PR (PR #11), aduana `pre-commit` (PR #12) y hooks ciclo PR Ola B (PR #13) en producción. Este manifiesto agrupa la deuda **posterior al PBI** — no reabrir Hitos 1–3.

---

## Cerrado — no reabrir

| Entrega | Evidencia |
|---------|-----------|
| **PBI-005 completo** (Hitos 1–3) | `docs/todos/done/[OPERATIVO] Planificación de Backlog… (Ola A).md` v1.5.1 |
| Orquestación fractal PR presentado | PR #11 — `docs/todos/done/… Orquestación fractal PR presentado…` |
| EDA `Domain_Entity_*` universal | `docs/todos/done/… EDA — Eventos Domain_Entity…` |
| Intérprete dinámico `execute-process` | PR #9 — `refactor-execute-process-engine` |
| Laboratorio `feature` fase 1 (`workspace-init`) | `docs/todos/done/… Laboratorio — Handler físico proceso feature.md` |
| Hito 3 **Ola A** — `pre-commit` Argos | PR #12 — `docs/features/pbi-005-hito3-git-hooks/` |
| Hito 3 **Ola B** — hooks `pre-push` / `post-merge` | PR #13 — `docs/features/pbi-005-hito3-ola-b/` |

### Trazabilidad PBI-005 Hito 3 (CA-3)

| Ola | PR | Presented | Merged | Merge `main` |
|-----|-----|-----------|--------|--------------|
| A | #12 | `0c9a8a63-…` | `34cfbad5-…` | `12119f7` |
| B | #13 | `c15a00f4-…` | `a1cf6541-…` | `ed543c8` |

---

## Prioridad 1 — Deuda Ola C: retirada de shims CLI

**Manifiesto detallado:** [`docs/todos/[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI execute-process y execute-action.md`](%5BARQUITECTURA%5D%20Deuda%20Ola%20C%20%E2%80%94%20Retirar%20compatibilidad%20CLI%20execute-process%20y%20execute-action.md)

| ID | Tarea | Archivos afectados (muestra) |
|----|-------|------------------------------|
| OC.1 | Inventariar `--input-file` y `--action` en repo | `docs/features/**/execution.md`, `execute-process.py` docstring |
| OC.2 | Migrar a `--process` / `--inputs` y `execute-action.py` directo | `pbi-005-action-engine/execution.md`, `pbi-005-debt-liquidation/execution.md`, `refactor-execute-process-engine/execution.md`, `ola-c-event-entity/execution.md`, `pbi-005-hito3-ola-b/execution.md` |
| OC.3 | Sustituir `execute-process.py --action emit-pr-merged-event` por **`accept-pr`** o `execute-action.py` | `pbi-005-hito2-action-engine/execution.md`, guías merge legacy |
| OC.4 | Eliminar `warn_deprecated_input_file`, `shim_execute_action`, flag `--action` | `SddIA/scripts/qa/execute-process.py` |
| OC.5 | Actualizar `SddIA/actions/execute-process.md` | Contrato canónico único |

**Criterio de cierre:** Ningún `execution.md` ni script QA invoca rutas deprecadas; TODO Ola C → `docs/todos/done/`.

---

## Prioridad 2 — Laboratorio: procesos físicos completos

| ID | Proceso | Gap actual | Objetivo |
|----|---------|------------|----------|
| L.1 | **`accept-pr`** | Cápsula física PR #13 (Auditoría → Merge → Sello → Push); `delete_branch` e higiene parcial | Runbook único sin `git-manager` suelto; higiene ramas completa |
| L.2 | **`delivery-close-cycle`** | Fases 1–3 (`Snapshot`, Argos ×2) `simulated` | Handlers mínimos o gates documentados |
| L.3 | **`feature`** | Fases 2–6 `simulated` | Perfil IDE fuera de alcance lab; `execution_report` honesto |

**Nota:** Presentación PR (fases 4–6) y fusión vía cápsula `accept-pr` operativas desde PR #11 y PR #13.

---

## Prioridad 3 — EDA producción e integridad

| ID | Tarea | Estado |
|----|-------|--------|
| E.1 | IOTA **físico** en CI/validación (sin solo `SDDIA_LAB_SIMULATE_IOTA=1`) | ⏳ |
| E.2 | Validación de esquema en `emit-domain-mutation` antes de `pending/` | ⏳ — Ola C V3 |
| E.3 | `verify-process-integrity.py` + gate `pre-commit` | ✅ PR #12 |
| E.4 | Recalcular `hash_signature` tras cambio de `phases` en procesos | ✅ Disciplina PR #12+ |

---

## Prioridad 4 — Ola C V3 (visión largo plazo)

**Manifiesto:** [`docs/todos/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md`](%5BARQUITECTURA%5D%20Especificaci%C3%B3n%20T%C3%A9cnica%20Avanzada_%20El%20Genoma%20de%20Eventos%20y%20Coreograf%C3%ADa%20As%C3%ADncrona%20(Ola%20C)%20V3.md)

| Componente | Estado |
|------------|--------|
| `event-sweeper.py` + recibos `[UUID].[PURPOSE].notificado` | ⏳ |
| Subcarpetas `receipts/` por estado | ⏳ |
| Middleware `.procesado` / `.error` (sello recibo) | ⏳ — hoy `delivery_state` en JSON |

---

## Prioridad 5 — Higiene documental

| ID | Tarea | Estado |
|----|-------|--------|
| D.1 | PBI-005 operativo v1.5.1 completado | ✅ → `docs/todos/done/…` |
| D.2 | Eliminar duplicados obsoletos en `docs/todos/` | ⏳ |
| D.3 | Reexportar PDF operativo desde `.md` | ⏳ |
| D.4 | Feature Hito 3 Ola A + Ola B | ✅ PR #12 + PR #13 |
| D.5 | Actualizar `TODO-BLINDAJE-IA-OBRERA` Fase C (pre-commit + hooks) | ⏳ |

---

## Matriz resumen

| Bloque | Prioridad | Esfuerzo | Desbloquea |
|--------|-----------|----------|------------|
| Ola C shims CLI | **P1** | Medio | Deuda forense / CI |
| Handlers lab (`accept-pr` completo) | **P2** | Medio | Runbooks sin pasos manuales |
| IOTA CI + integridad genoma | **P3** | Bajo–Medio | Gobernanza EDA producción |
| Ola C V3 coreografía | **P4** | Alto | Visión largo plazo |
| Docs / PDF / blindaje | **P5** | Bajo | Paridad administrativa |

---

## Definición de hecho (este backlog)

- [x] **PBI-005** y **CA-3** al 100 % (`main`, PR #13).
- [ ] **OC.1–OC.5** completos.
- [ ] **L.1** runbook merge solo `accept-pr` + watcher (sin `git-manager` suelto en guías legacy).
- [ ] **E.1** IOTA físico en CI.
- [ ] Este archivo → `status: cerrado` o `docs/todos/done/` cuando P1–P3 estén resueltos.

---

## Referencias rápidas

| Tema | Ruta |
|------|------|
| PBI-005 (cerrado) | `docs/todos/done/[OPERATIVO] Planificación de Backlog… (Ola A).md` |
| Feature Ola B | `docs/features/pbi-005-hito3-ola-b/` |
| Feature Ola A | `docs/features/pbi-005-hito3-git-hooks/` |
| Presentación PR | `SddIA/process/delivery-close-cycle.md` v1.1 |
| Fusión PR | `SddIA/process/accept-pr.md` |
| Contrato hooks CA-3 | `SddIA/evolution/git-hooks-ca3-ola-b-contract.md` |
| Norma PR | `SddIA/norms/pull-request-orchestration.md` |
