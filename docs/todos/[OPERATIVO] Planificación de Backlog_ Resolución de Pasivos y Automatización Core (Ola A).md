---
document_id: PBI-005
title: "[OPERATIVO] Planificación de Backlog — Resolución de Pasivos y Automatización Core (Ola A)"
format: markdown
version: "1.2.0"
updated: "2026-05-19"
status: "en_progreso"
supersedes_pdf: "[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).pdf"
feature_ref: docs/features/pbi-005-debt-liquidation
feature_ref_hito2: docs/features/pbi-005-hito2-action-engine
---

# [OPERATIVO] Planificación de Backlog: Resolución de Pasivos y Automatización Core (Ola A)

> **Nota de gobernanza (2026-05-19):** Esta copia **Markdown** es la versión operativa actualizada del PBI. Refleja **Hito 1** y **Hito 2** (motor de acciones) en rama de entrega / merge. El PDF homónimo en esta carpeta corresponde a la versión **1.0.0** previa; reexportar el PDF desde este `.md` cuando se requiera paridad física.

Este documento formaliza el **Product Backlog Item (PBI)** estratégico destinado a liquidar los pasivos técnicos heredados de la **Ola A**, validar los mecanismos destructivos del genoma y automatizar la interacción física con el sistema de control de versiones **Git**. Su propósito es consolidar los cimientos de la infraestructura antes del despliegue masivo de la arquitectura coreográfica (**Ola C**).

---

## Registro de ejecución (actualización 2026-05-19)

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

### Hito 2 — Motor de acciones (entrega)

| Campo | Valor |
|-------|--------|
| **Feature** | `pbi-005-hito2-action-engine` |
| **Rama entrega** | `feat/pbi-005-action-engine` |
| **Commit base** | `0cce8ba` — `execute-action.py`, `markdown-table-editor`, purga `sync-entity-index.py` |
| **PR** | https://github.com/racso80es/SddIA/pull/7 — **MERGED** |
| **Merge commit (`main`)** | `dbf606b98eec2603f48d509612a00fba169018de` |
| **Proceso merge** | `accept-pr` vía `git-manager` + `emit-pr-merged-event` |
| **Evento merge** | `aaf010d6-88e4-432b-b65e-1470d3923fb0` → `docs/events/processed/` |
| **Documentación feature** | `docs/features/pbi-005-hito2-action-engine/` |

---

## 1. Estructura del PBI y matriz de tareas

Para evitar fugas de entropía y garantizar el cumplimiento del estándar **S+ Grade**, se catalogan las tres faenas operativas en la siguiente matriz de ejecución.

| Faena / componente | Descripción operativa y objetivo técnico | Impacto | **Estado** |
|--------------------|------------------------------------------|---------|------------|
| **Validación de purga** (`delete operation`) | Prueba de humo sobre `test-cli-skill` vía puerta oficial `execute-process.py` → `entity-manager` (`lifecycle_operation: delete`). Verificar eliminación física del `.md` y purga de fila en catálogo (`SddIA/skills/index.md`) vía bus + `sync-entity-index`. | Medio / Validación | **✅ Completado** |
| **Expansión DLT en delete** *(desglose Hito 1b)* | Suscriptor `cumulo` + `tool: iota-immutable-publisher` en `Domain_Entity_Deleted` (`SddIA/core/event-subscriptions.json`), simétrico a `PullRequest_Merged`. | Medio / Genoma EDA | **✅ Completado** |
| **Motor de acciones** (`execute-action.py`) | Desacoplar acoplamiento rígido de `sync-entity-index.py` en el daemon. Intérprete universal `execute-action.py` y `tool:markdown-table-editor` para soberanía de Cúmulo. | Alto / Deuda técnica | **✅ Completado** (Hito 2) |
| **Automatización Git** (hooks de integración) | Scripts en `.git/hooks/` (`pre-push` / `post-merge`) para emitir `PullRequest_Presented` y `PullRequest_Merged` al bus sin invocaciones CLI manuales. | Bajo / Automatización | **⏳ Pendiente** (Hito 3) |

### Evidencia Hito 1 (cerrado)

- **Purga:** `SddIA/skills/test-cli-skill.md` eliminado; fila purgada en `SddIA/skills/index.md`.
- **Evento delete:** `Domain_Entity_Deleted` → `docs/events/pending/f55090e3-...` → `processed/`.
- **Genoma:** `event-subscriptions.json` incluye `iota-immutable-publisher` en `Domain_Entity_Deleted`.
- **Merge PR #6:** `PullRequest_Merged` → `processed/` (IOTA laboratorio: `SDDIA_LAB_SIMULATE_IOTA=1` en validación Hito 1).

### Evidencia Hito 2 (motor de acciones)

- **`execute-action.py`:** puerta CLI `--action` / `--inputs` | `--input-file`; handler físico `sync-entity-index` → `markdown-table-editor`.
- **`markdown-table-editor`:** `SddIA/tools/markdown-table-editor.md` + cápsula `SddIA/scripts/tools/markdown-table-editor/`.
- **Watcher:** `event-watcher.py` despacha acciones vía `execute-action.py` (sin rama rígida a `sync-entity-index.py`).
- **Purga:** `SddIA/scripts/qa/sync-entity-index.py` eliminado del repositorio.

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
| **Estado global** | **Parcialmente completado** — Hitos 1–2; **Hito 3** (hooks Git) en backlog |

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

#### CA-3: Enlace orgánico de ciclo de vida (Git Hooks) — **⏳ PENDIENTE**

| Subcriterio | Estado | Notas |
|-------------|--------|-------|
| Git rutinario deposita eventos en bus sin CLI manual | ⏳ | Merge Hito 2 usa `accept-pr` + `emit-pr-merged-event` explícitos |
| Payloads con hashes reales del entorno | ✅ | Validado en merges soberanos |
| Ruta `eda_bus.pending` | ✅ | SSOT `cumulo.paths.json` |

---

## 3. Definición de hecho (DoD)

| Criterio DoD | Estado |
|--------------|--------|
| **Ausencia de alucinación causal** | 🟡 Parcial — watcher activo; hooks pendientes |
| **Idempotencia estricta** | 🟡 Validado en purga y motor de acciones |
| **Preservación del historial** | ✅ `docs/events/processed/` intacto |
| **CA-1 + CA-2 + CA-3** | 🟡 **CA-1 y CA-2** cumplidos; **CA-3** abierto |
| **Entrega en `main` con trazabilidad** | ✅ Hito 1; Hito 2 en cierre de merge |

**Veredicto actual:** PBI-005 **no cerrado al 100%**; cierre total condicionado a **Hito 3** (hooks Git) y deuda de laboratorio documentada en `docs/todos/`.

---

## 4. Próximos pasos (backlog activo)

1. **Acción `request-change-incorporation`:** abrir PR + emitir `PullRequest_Presented`; cablear en `delivery-close-cycle` y procesos de entrega (ver TODO arquitectura dedicado).
2. **Hito 3 — Hooks Git:** `pre-push` / `post-merge` → pueden delegar en la acción anterior o emitir `PullRequest_*` vía `git-manager`.
3. **Deuda laboratorio:** handler físico de `feature` en `execute-process.py` (ver TODO dedicado).
4. **EDA universal:** emisión `Domain_Entity_*` para todas las clases en `entity-manager` (ver TODO arquitectura).
5. **Reexportar PDF** desde este `.md` si se requiere paridad documental binaria.

---

## 5. Referencias cruzadas

| Artefacto | Ruta |
|-----------|------|
| Feature Hito 1 | `docs/features/pbi-005-debt-liquidation/` |
| Feature Hito 2 | `docs/features/pbi-005-hito2-action-engine/` |
| Genoma suscripciones | `SddIA/core/event-subscriptions.json` |
| Proceso merge | `SddIA/process/accept-pr.md` |
| TODO EDA entidades | `docs/todos/[ARQUITECTURA] EDA — Eventos Domain_Entity para todas las entidades de dominio.md` |
| TODO feature laboratorio | `docs/todos/[ARQUITECTURA] Laboratorio — Handler físico proceso feature.md` |
| TODO PR + Presented | `docs/todos/[ARQUITECTURA] Acción request-change-incorporation — PR y evento PullRequest_Presented.md` |

---

## Historial de versiones del documento

| Versión | Fecha | Cambio |
|---------|-------|--------|
| 1.0.0 | (PDF origen) | Matriz inicial; tres faenas pendientes |
| 1.1.0 | 2026-05-19 | Hito 1 + 1b completados; CA-1 cumplido; PR #6 |
| 1.2.0 | 2026-05-19 | Hito 2 completado; CA-2 cumplido; referencias hito2 y TODOs de deuda |
