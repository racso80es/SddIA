---
document_id: TODO-BACKLOG-PENDIENTE-POST-PR11
title: "[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio"
format: markdown
version: "1.0.0"
created: "2026-05-20"
updated: "2026-05-20"
status: "abierto"
priority: alta
blocks: "Cierre PBI-005 al 100 % / reducción deuda operativa manual"
supersedes: null
related:
  - docs/todos/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
  - docs/todos/[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI execute-process y execute-action.md
  - docs/todos/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md
  - docs/todos/done/[ARQUITECTURA] Orquestación fractal PR presentado — delivery-close-cycle y PullRequest_Presented.md
  - SddIA/process/accept-pr.md
  - SddIA/process/delivery-close-cycle.md
  - SddIA/process/feature.md
---

# Backlog pendiente (consolidado)

> **Contexto:** Tras cierre de **PR #11** (`delivery-close-cycle` v1.1 + `emit-pr-presented-event`, merge `d53d956` vía `accept-pr`), la orquestación fractal de **presentación** de PR está en `main`. Este manifiesto agrupa lo que **sigue abierto** para no dispersar deuda en múltiples TODOs huérfanos.

## Cerrado recientemente (no reabrir)

| Entrega | Evidencia |
|---------|-----------|
| Orquestación fractal PR presentado | `docs/todos/done/[ARQUITECTURA] Orquestación fractal PR presentado…` — PR #11 |
| EDA `Domain_Entity_*` universal | `docs/todos/done/[ARQUITECTURA] EDA — Eventos Domain_Entity…` |
| Intérprete dinámico `execute-process` | PR #9 — `refactor-execute-process-engine` |
| Laboratorio `feature` fase 1 (`workspace-init`) | `docs/todos/done/[ARQUITECTURA] Laboratorio — Handler físico proceso feature.md` |

---

## Prioridad 1 — PBI-005 Hito 3 (CA-3): Hooks Git orgánicos

**Objetivo:** Que operaciones Git rutinarias depositen eventos en el bus **sin** invocaciones CLI manuales ni runbooks ad hoc.

| ID | Tarea | Criterio de hecho |
|----|-------|-------------------|
| H3.1 | Diseñar contrato de hooks (`pre-push`, `post-merge` o equivalente) alineado a `pull-request-orchestration.md` | Documento en `SddIA/evolution/` o norma táctica |
| H3.2 | `pre-push` → delegar en **`delivery-close-cycle`** (o sub-secuencia: push ya ocurrió → solo sello si PR existe) | Tras push, JSON `PullRequest_Presented` en `eda_bus.pending` sin `execute-process` manual |
| H3.3 | `post-merge` en `main` → delegar en **`accept-pr`** / `emit-pr-merged-event` | Tras merge local, `PullRequest_Merged` sin `--action` suelto |
| H3.4 | Prohibir `gh pr merge` en hooks; respetar SSOT `accept-pr` | Revisión Argos / `pr-acceptance-protocol.md` |
| H3.5 | Smoke reproducible + entrada en `docs/features/pbi-005-hito3-git-hooks/` (crear feature) | `validacion.md` con event_ids |

**Bloquea:** CA-3 y DoD PBI-005 («ausencia de alucinación causal» al 100 %).

**Referencia:** `docs/todos/[OPERATIVO] Planificación de Backlog… (Ola A).md` § CA-3.

---

## Prioridad 2 — Deuda Ola C: retirada de shims CLI

**Manifiesto detallado:** `docs/todos/[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI execute-process y execute-action.md`

| ID | Tarea | Archivos afectados (muestra) |
|----|-------|------------------------------|
| OC.1 | Inventariar `--input-file` y `--action` en repo | `docs/features/**/execution.md`, `execute-process.py` docstring |
| OC.2 | Migrar a `--process` / `--inputs` y `execute-action.py` directo | `pbi-005-action-engine/execution.md`, `pbi-005-debt-liquidation/execution.md`, `refactor-execute-process-engine/execution.md`, `ola-c-event-entity/execution.md` |
| OC.3 | Sustituir `execute-process.py --action emit-pr-merged-event` por **`accept-pr`** o `execute-action.py` | `pbi-005-hito2-action-engine/execution.md`, guías merge |
| OC.4 | Eliminar `warn_deprecated_input_file`, `shim_execute_action`, flag `--action` | `SddIA/scripts/qa/execute-process.py` |
| OC.5 | Actualizar `SddIA/actions/execute-process.md` | Contrato canónico único |

**Criterio de cierre:** Ningún `execution.md` ni script QA invoca rutas deprecadas.

---

## Prioridad 3 — Laboratorio: procesos físicos completos

| ID | Proceso | Gap actual | Objetivo |
|----|---------|------------|----------|
| L.1 | **`accept-pr`** | Merge manual vía `git-manager.py` + `execute-action` suelto (PR #11) | Handler `run_process("accept-pr")` con fases Auditoría → Merge → Sello → Push → Higiene |
| L.2 | **`delivery-close-cycle`** | Fases 1–3 (`Snapshot`, Argos ×2) `simulated` | Handlers mínimos o gates documentados; Impacto SddIA condicional no-op explícito |
| L.3 | **`feature`** | Fases 2–6 `simulated` | Perfil IDE completo fuera de alcance lab; mantener `execution_report` honesto (ya en `feature.md`) |

**Nota:** Presentación PR (fases 4–6) ya físicas en lab — PR #11.

---

## Prioridad 4 — EDA producción e integridad

| ID | Tarea | Estado |
|----|-------|--------|
| E.1 | IOTA **físico** en CI/validación (sin solo `SDDIA_LAB_SIMULATE_IOTA=1`) | ⏳ |
| E.2 | Validación de esquema en `emit-domain-mutation` antes de `pending/` | ⏳ — Ola C V3 |
| E.3 | `verify-process-integrity.py` — alinear `hash_signature` de procesos con drift | ⏳ — múltiples `.md` en `SddIA/process/` fallan hoy |
| E.4 | Recalcular `hash_signature` tras cada cambio de `phases` en procesos tocados | Disciplina en PRs de proceso |

---

## Prioridad 5 — Ola C V3 (visión, no bloqueante PBI-005)

**Manifiesto:** `docs/todos/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md`

| Componente | Estado |
|------------|--------|
| `event-sweeper.py` + recibos `[UUID].[PURPOSE].notificado` | ⏳ |
| Subcarpetas `receipts/` por estado | ⏳ |
| Middleware `.procesado` / `.error` (sello recibo) | ⏳ — hoy `delivery_state` en JSON |

---

## Prioridad 6 — Higiene documental

| ID | Tarea |
|----|-------|
| D.1 | Actualizar PBI-005 operativo: marcar orquestación PR presentado ✅; enlazar este backlog |
| D.2 | Eliminar duplicados obsoletos en `docs/todos/` si reaparecen (p. ej. copia de `request-change-incorporation` pre-`done/`) |
| D.3 | Reexportar PDF operativo desde `.md` si se exige paridad binaria |
| D.4 | Crear `docs/features/pbi-005-hito3-git-hooks/` al iniciar Hito 3 |

---

## Matriz resumen

| Bloque | Prioridad | Esfuerzo estimado | Desbloquea |
|--------|-----------|-------------------|------------|
| Hito 3 hooks Git | **P1** | Medio | PBI-005 100 % |
| Ola C shims CLI | **P2** | Medio | Deuda forense / CI |
| Handlers `accept-pr` | **P3** | Alto | Cierre sin pasos manuales |
| IOTA + hash procesos | **P4** | Medio | Gobernanza genoma |
| Ola C V3 coreografía | **P5** | Alto | Visión largo plazo |
| Docs / PDF | **P6** | Bajo | Paridad administrativa |

---

## Definición de hecho global (este backlog)

- [ ] **H3.1–H3.5** completos y PBI-005 CA-3 en verde.
- [ ] **OC.1–OC.5** completos (TODO Ola C cerrado → `done/`).
- [ ] **L.1** `accept-pr` ejecutable como proceso en laboratorio.
- [ ] Al menos un runbook de merge usa solo `accept-pr` + watcher (sin `git-manager` suelto en guía).
- [ ] Este archivo movido a `docs/todos/done/` o `status: cerrado`.

---

## Referencias rápidas

| Tema | Ruta |
|------|------|
| Presentación PR (cerrado) | `SddIA/process/delivery-close-cycle.md` v1.1 |
| Fusión PR | `SddIA/process/accept-pr.md` |
| Norma PR | `SddIA/norms/pull-request-orchestration.md` |
| Feature PR #11 | `docs/features/pr-presented-orchestration/` |
| PBI operativo | `docs/todos/[OPERATIVO] Planificación de Backlog… (Ola A).md` |
