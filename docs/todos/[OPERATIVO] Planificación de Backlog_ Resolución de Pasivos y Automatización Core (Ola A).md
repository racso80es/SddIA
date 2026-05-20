---
document_id: PBI-005
title: "[OPERATIVO] Planificación de Backlog — Resolución de Pasivos y Automatización Core (Ola A)"
format: markdown
version: "1.5.0"
updated: "2026-05-20"
status: "validacion_ola_b_pre_merge"
feature_ref_hito3_ola_b: docs/features/pbi-005-hito3-ola-b
supersedes_pdf: "[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).pdf"
feature_ref: docs/features/pbi-005-debt-liquidation
feature_ref_hito2: docs/features/pbi-005-action-engine
feature_ref_hito3: docs/features/pbi-005-hito3-git-hooks
feature_ref_eda_splus: docs/features/eda-domain-entities-splus
---

# [OPERATIVO] Planificación de Backlog: Resolución de Pasivos y Automatización Core (Ola A)

> **Nota de gobernanza (2026-05-20):** Hitos 1–2 y **Hito 3 Ola A** en `main`. **Hito 3 Ola B** implementada en rama `feat/pbi-005-hito3-ola-b` — `validacion.md` **APTO (lab)**; cierre PBI al 100 % pendiente de PR + merge en `main`.

Este documento formaliza el **Product Backlog Item (PBI)** estratégico destinado a liquidar los pasivos técnicos heredados de la **Ola A**, validar los mecanismos destructivos del genoma y automatizar la interacción física con el sistema de control de versiones **Git**. Su propósito es consolidar los cimientos de la infraestructura antes del despliegue masivo de la arquitectura coreográfica (**Ola C**).

---

## Registro de ejecución (actualización 2026-05-20)

### Hito 1 (cerrado en `main`)

| Campo | Valor |
|-------|--------|
| **Feature** | `pbi-005-debt-liquidation` |
| **Rama entrega** | `feat/pbi-005-debt-liquidation` (eliminada post-merge) |
| **PR** | https://github.com/racso80es/SddIA/pull/6 — **MERGED** |
| **Merge commit (`main`)** | `562d0da2120bec9f4b0bb797440eaee492467642` |
| **Cierre documental** | `703f505` — `validacion.md`, `execution.md` |
| **Proceso merge** | `accept-pr` vía `git-manager` + `emit-pr-merged-event` |
| **Documentación feature** | `docs/features/pbi-005-debt-liquidation/` |

### Hito 2 — Motor de acciones (cerrado en `main`)

| Campo | Valor |
|-------|--------|
| **Feature** | `pbi-005-hito2-action-engine` |
| **Rama entrega** | `feat/pbi-005-action-engine` (eliminada post-merge) |
| **Entrega previa** | PR #7 — `0cce8ba` / `dbf606b` (`execute-action`, `markdown-table-editor`, purga legacy) |
| **Consolidación capas** | PR #8 — squash `caab46e` (handler `feature`, `bus-operator`, micro-tools EDA, forense) |
| **PR definitivo** | https://github.com/racso80es/SddIA/pull/8 — **MERGED** |
| **Merge commit (`main`)** | `caab46ed4fa116977813ab35150ee05ca0358ecb` |
| **Proceso merge** | `gh pr merge --squash` + `emit-pr-merged-event` + watcher |
| **Evento merge** | `d121213d-4950-4927-8aae-0a9b26d6e8fb` → `docs/events/processed/` |
| **Documentación feature** | `docs/features/pbi-005-action-engine/` |
| **Manifiesto hito** | `docs/todos/PBI-005-Hito2-TODO.md` — fases 1–6 ✅ |

### Hito EDA S+ — Cobertura genómica universal (laboratorio, rama `feat/eda-domain-entities-splus`)

| Campo | Valor |
|-------|--------|
| **Feature** | `eda-domain-entities-splus` |
| **Documentación** | `docs/features/eda-domain-entities-splus/` |
| **Backfill Fase C** | `backfill-manifest.json` — 40 `Domain_Entity_Created` retroactivos (`cumulo-eda-backfill`, `--skip-dlt`) |
| **Acta Merkle** | `merkle-acta-eda-backfill-fase-c-20260520.json` — `transaction_digest` registrado (lab: `SDDIA_LAB_SIMULATE_IOTA=1`) |
| **Aduana Argos** | `delivery-close-cycle` → `audit-entity-eda-coverage.py --scan`; `orphan_count: 0` post-backfill |

### Hito 3 — Ola A: Aduana `pre-commit` (cerrado en `main`)

| Campo | Valor |
|-------|--------|
| **Feature** | `pbi-005-hito3-git-hooks` |
| **Rama entrega** | `feat/pbi-005-hito3-git-hooks` (eliminada post-merge) |
| **PR** | https://github.com/racso80es/SddIA/pull/12 — **MERGED** |
| **Merge commit (`main`)** | `12119f73168b78713fde861f6a26aa7754ca873c` |
| **Cierre documental** | `9c72799` — `validacion.md`, `finalize-process.md` |
| **Presentación** | `delivery-close-cycle` → `PullRequest_Presented` `0c9a8a63-f4c0-4174-a0d1-69cb56eb8a7b` |
| **Fusión** | `accept-pr` (git-manager) + `emit-pr-merged-event` → `34cfbad5-009e-4ace-b597-571de282f280` |
| **DLT** | Watcher sin `SDDIA_LAB_SIMULATE_IOTA` — `delivery_state.cumulo: success` en ambos eventos |
| **Entregable** | `SddIA/scripts/qa/git-hooks/` (`pre-commit`, `pre_commit_gate.py`, `install-hooks.ps1`) |
| **Documentación feature** | `docs/features/pbi-005-hito3-git-hooks/` |

> **Ola B:** hooks `pre-push` / `post-merge` — feature `pbi-005-hito3-ola-b`, validación Argos APTO (lab); PR pendiente.

### Hito 3 — Ola B: Hooks ciclo PR (validación lab, pre-merge)

| Campo | Valor |
|-------|--------|
| **Feature** | `pbi-005-hito3-ola-b` |
| **Rama entrega** | `feat/pbi-005-hito3-ola-b` |
| **Validación** | `docs/features/pbi-005-hito3-ola-b/validacion.md` — **APTO (lab)** |
| **Eventos smoke** | Presented `e71a367b-…`; Merged `e7812b3a-…`; huérfano `890b6a55-…` → `processed/` |
| **Entregable** | `pre-push`, `post-merge`, `hook_common.py`, cápsula `accept-pr`, instalador O5 |
| **Documentación** | `docs/features/pbi-005-hito3-ola-b/` |

---

## 1. Estructura del PBI y matriz de tareas

Para evitar fugas de entropía y garantizar el cumplimiento del estándar **S+ Grade**, se catalogan las tres faenas operativas en la siguiente matriz de ejecución.

| Faena / componente | Descripción operativa y objetivo técnico | Impacto | **Estado** |
|--------------------|------------------------------------------|---------|------------|
| **Validación de purga** (`delete operation`) | Prueba de humo sobre `test-cli-skill` vía puerta oficial `execute-process.py` → `entity-manager` (`lifecycle_operation: delete`). Verificar eliminación física del `.md` y purga de fila en catálogo (`SddIA/skills/index.md`) vía bus + `sync-entity-index`. | Medio / Validación | **✅ Completado** |
| **Expansión DLT en delete** *(desglose Hito 1b)* | Suscriptor `cumulo` + `tool: iota-immutable-publisher` en `Domain_Entity_Deleted` (`SddIA/core/event-subscriptions.json`), simétrico a `PullRequest_Merged`. | Medio / Genoma EDA | **✅ Completado** |
| **Motor de acciones** (`execute-action.py`) | Intérprete universal, `skill:bus-operator`, micro-tools EDA, `tool:markdown-table-editor`; watcher ciego; handlers EDA (`emit-pr-*`, `emit-domain-mutation`). | Alto / Deuda técnica | **✅ Completado** (Hito 2 PR #7–#8; ampliado PR #9) |
| **Intérprete procesos** (`execute-process.py`) | Refactor Kaizen: core + cápsulas, validación contrato MD, `CAPSULE_ACTION_REGISTRY`; shims Ola C documentados. | Alto / Arquitectura | **✅ Completado** (PR #9, `docs/features/refactor-execute-process-engine/`) |
| **Aduana Git `pre-commit`** (Hito 3 Ola A) | `SddIA/scripts/qa/git-hooks/` — `verify-process-integrity` + Existencia en Bus (`audit-entity-eda-coverage --scan`). | Medio / Blindaje IA | **✅ Completado** (PR #12) |
| **Hooks ciclo PR** (Hito 3 Ola B) | `pre-push` → `delivery-close-cycle`; `post-merge` → `accept-pr`; O1–O5. | Bajo / Automatización | **🟡 APTO lab** — PR pendiente (`feat/pbi-005-hito3-ola-b`) |

### Evidencia Hito 1 (cerrado)

- **Purga:** `SddIA/skills/test-cli-skill.md` eliminado; fila purgada en `SddIA/skills/index.md`.
- **Evento delete:** `Domain_Entity_Deleted` → `docs/events/pending/f55090e3-...` → `processed/`.
- **Genoma:** `event-subscriptions.json` incluye `iota-immutable-publisher` en `Domain_Entity_Deleted`.
- **Merge PR #6:** `PullRequest_Merged` → `processed/` (IOTA laboratorio: `SDDIA_LAB_SIMULATE_IOTA=1` en validación Hito 1).

### Evidencia Hito 2 (motor de acciones)

- **`execute-action.py`:** puerta CLI; `sync-entity-index` → `agent:cumulo` → `skill:bus-operator` → `tool:markdown-table-editor`.
- **`bus-operator`:** `SddIA/skills/bus-operator.md` + `scripts/skills/bus-operator.py`; micro-tools `read-event-subscriptions`, `manage-event-receipt`, `transit-event-payload`.
- **`execute-process.py`:** ~~handler `if feature`~~ → **intérprete dinámico agnóstico** (PR #9, `refactor-execute-process-engine`); `workspace-init` genérico; fases agente `simulated`.
- **`markdown-table-editor`:** contrato + cápsula en `SddIA/scripts/tools/markdown-table-editor/`.
- **Watcher:** `event-watcher.py` despacha vía `execute-action.py` (sin `sync-entity-index.py`).
- **Purga:** `SddIA/scripts/qa/sync-entity-index.py` ausente.
- **Validación:** `docs/features/pbi-005-action-engine/validacion.md` — **APTO**.

### Evidencia refactor intérprete procesos (PR #9)

- **`execute-process.py`:** intérprete dinámico (`execute_process_core` + `execute_process_capsules`); sin ramas por nombre de proceso.
- **`execute-action.py`:** `emit-domain-mutation`, `emit-pr-presented-event`, `emit-pr-merged-event`.
- **`event-subscriptions.json`:** IOTA en `PullRequest_Presented` restaurado (`18d80ea`).
- **Validación:** `docs/features/refactor-execute-process-engine/validacion.md` — **APTO**.

**Invocación canónica (acción índice):**

```json
{
  "action": "sync-entity-index",
  "inputs": {
    "entity_class": "skill",
    "entity_name": "example",
    "lifecycle_operation": "delete"
  }
}
```

```powershell
python SddIA/scripts/qa/execute-action.py --action sync-entity-index --input-file payload.json
```

---

## 2. Especificación del ítem de backlog (PBI-005)

| Atributo | Valor |
|----------|--------|
| **ID** | PBI-005 |
| **Título** | [OPERATIVO] Liquidación de Pasivos de la Ola A y Automatización de la Capa de Enlace Git |
| **Prioridad** | Alta (bloqueante para desarrollo iterativo estable) |
| **Estado global** | **Validación Ola B APTO (lab)** — merge PR en `main` para cierre 100 % |

### 2.1. Criterios de aceptación (CAs)

#### CA-1: Verificación del mecanismo destructivo — **✅ CUMPLIDO**

| Subcriterio | Estado | Evidencia |
|-------------|--------|-----------|
| `entity-manager` + `delete` remueve archivo físico | ✅ | Commit `c42d25f` |
| Tabla de índice reescrita sin corromper filas adyacentes | ✅ | `execute-action` + watcher |
| Emisión ECST en bus runtime | ✅ | `docs/events/pending/` → `processed/` |

#### CA-2: Desacoplamiento de la infraestructura del daemon — **✅ CUMPLIDO** (Hito 2)

| Subcriterio | Estado | Notas |
|-------------|--------|-------|
| Eliminar `sync-entity-index.py` de laboratorio | ✅ | Purga en commit `0cce8ba` |
| `event-watcher.py` sin referencias rígidas a scripts | ✅ | Despacho genérico `execute-action.py` |
| Canalización vía motor genérico de acciones | ✅ | `execute-action.py` operativo |

#### CA-3: Enlace orgánico de ciclo de vida (Git Hooks) — **🟡 APTO lab (pre-merge)**

| Subcriterio | Estado | Notas |
|-------------|--------|-------|
| Aduana `pre-commit` (Ola A) | ✅ | PR #12 — `pre_commit_gate.py` |
| Git rutinario deposita eventos sin CLI manual (Ola B) | 🟡 | Hooks implementados; smoke lab V-B1–V-B6 ✅; PR pendiente |
| Payloads con hashes reales del entorno | ✅ | PR #12 + smoke Ola B |
| Ruta `eda_bus.pending` | ✅ | SSOT `cumulo.paths.json` |

---

## 3. Definición de hecho (DoD)

| Criterio DoD | Estado |
|--------------|--------|
| **Ausencia de alucinación causal** | 🟡 Hooks Ola B APTO lab; merge PR pendiente |
| **Idempotencia estricta** | ✅ Purga, motor acciones, O1 pre-push |
| **Preservación del historial** | ✅ `docs/events/processed/` intacto |
| **CA-1 + CA-2 + CA-3** | 🟡 CA-1/2 ✅; CA-3 APTO lab — merge PR |
| **Entrega en `main` con trazabilidad** | 🟡 Ola B en rama feature; Hitos 1–2 + Ola A en `main` |

**Veredicto actual:** PBI-005 **pre-cierre** — Ola B **APTO (lab)** en `feat/pbi-005-hito3-ola-b`; cierre 100 % tras PR + `accept-pr` en `main`.

---

## 4. Próximos pasos (backlog activo)

> **SSOT consolidado de pendientes:** [`docs/todos/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md`](%5BOPERATIVO%5D%20Backlog%20pendiente%20post-PR11%20%E2%80%94%20Hito%203,%20Ola%20C%20y%20laboratorio.md)

1. ~~**Orquestación fractal PR presentado**~~ ✅ PR #11 (`docs/todos/done/… Orquestación fractal PR presentado…`).
2. ~~**EDA universal `Domain_Entity_*`**~~ ✅ (`docs/todos/done/… EDA — Eventos Domain_Entity…`).
3. ~~**Hito 3 Ola A — `pre-commit` Argos**~~ ✅ PR #12 (`docs/features/pbi-005-hito3-git-hooks/`).
4. ~~**Hito 3 Ola B — hooks PR (CA-3 resto)**~~ 🟡 APTO lab — PR pendiente (`docs/features/pbi-005-hito3-ola-b/`).
5. **Deuda Ola C shims CLI:** ver backlog consolidado § Prioridad 2.
6. **Reexportar PDF** desde este `.md` si se requiere paridad documental binaria.

---

## 5. Referencias cruzadas

| Artefacto | Ruta |
|-----------|------|
| Feature Hito 1 | `docs/features/pbi-005-debt-liquidation/` |
| Feature Hito 2 | `docs/features/pbi-005-action-engine/` |
| Feature Hito 3 Ola A | `docs/features/pbi-005-hito3-git-hooks/` |
| Feature Hito 3 Ola B | `docs/features/pbi-005-hito3-ola-b/` |
| Backlog consolidado | `docs/todos/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md` |
| Manifiesto Hito 2 | `docs/todos/PBI-005-Hito2-TODO.md` |
| Genoma suscripciones | `SddIA/core/event-subscriptions.json` |
| Proceso merge | `SddIA/process/accept-pr.md` |
| TODO EDA entidades | `docs/todos/[ARQUITECTURA] EDA — Eventos Domain_Entity para todas las entidades de dominio.md` |
| TODO feature laboratorio | `docs/todos/[ARQUITECTURA] Laboratorio — Handler físico proceso feature.md` (**cerrado** PR #8) |
| TODO PR + Presented | `docs/todos/done/[ARQUITECTURA] Orquestación fractal PR presentado — delivery-close-cycle y PullRequest_Presented.md` |

---

## Historial de versiones del documento

| Versión | Fecha | Cambio |
|---------|-------|--------|
| 1.0.0 | (PDF origen) | Matriz inicial; tres faenas pendientes |
| 1.1.0 | 2026-05-19 | Hito 1 + 1b completados; CA-1 cumplido; PR #6 |
| 1.2.0 | 2026-05-19 | Hito 2 base (PR #7); CA-2 cumplido; referencias hito2 |
| 1.3.0 | 2026-05-20 | Hito 2 consolidado (PR #8): bus-operator, handler feature, manifiesto TODO y forense APTO |
| 1.4.0 | 2026-05-20 | Hito 3 Ola A (PR #12): pre-commit Argos; CA-3 parcial; Ola B en backlog |
| 1.5.0 | 2026-05-20 | Hito 3 Ola B: hooks pre-push/post-merge, validacion APTO lab; pre-merge PR |
