---
document_id: TODO-ACTION-REQUEST-CHANGE-INCORPORATION
title: "[ARQUITECTURA] Acción request-change-incorporation — Solicitud de incorporación de cambios (PR + bus EDA)"
format: markdown
version: "1.1.0"
created: "2026-05-19"
updated: "2026-05-20"
closed: "2026-05-20"
status: "abortado"
priority: alta
superseded_by: docs/todos/done/[ARQUITECTURA] Orquestación fractal PR presentado — delivery-close-cycle y PullRequest_Presented.md
related:
  - SddIA/events/pull-request-presented.md
  - SddIA/actions/emit-pr-merged-event.md
  - SddIA/process/delivery-close-cycle.md
  - SddIA/process/feature.md
  - SddIA/process/accept-pr.md
  - SddIA/norms/pull-request-orchestration.md
  - docs/todos/pending/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
  - docs/todos/done/[ARQUITECTURA] Orquestación fractal PR presentado — delivery-close-cycle y PullRequest_Presented.md
---

# TODO: Acción `request-change-incorporation` — ABORTADO

> **Pivot 2026-05-20 (S+):** Diseño monolítico **abortado**. La orquestación canónica quedó en **`delivery-close-cycle`** v1.1 + sello **`emit-pr-presented-event`**. Ver entrega en `docs/todos/done/[ARQUITECTURA] Orquestación fractal PR presentado…` (PR #11).

Este documento conserva el **diseño objetivo** descartado como referencia histórica.

## Objetivo (diseño descartado)

Forjar y cablear una **acción atómica de dominio** — nombre canónico propuesto: **`request-change-incorporation`** — responsable de:

1. **Abrir o actualizar** la Pull Request de incorporación de cambios hacia `main` (sustituyendo invocaciones ad hoc a `gh pr create` fuera del genoma).
2. **Emitir** la instancia ECST **`PullRequest_Presented`** en `eda_bus.pending` (`cumulo.paths.json`), con payload conforme a `SddIA/events/pull-request-presented.md`.

Simetría deseada con el par existente:

| Momento del ciclo | Acción / sello | Clase de evento |
|-------------------|----------------|------------------|
| Presentación de PR | **`request-change-incorporation`** *(abortado)* | `PullRequest_Presented` |
| Fusión soberana (`accept-pr`) | `emit-pr-merged-event` | `PullRequest_Merged` |

## Problema que cerraba

| Síntoma | Causa raíz |
|---------|------------|
| PR #7 sin evento `PullRequest_Presented` | `gh pr create` no integra el bus; no se invocó `emit-pr-presented-event` |
| Handler de `emit-pr-presented-event` solo en `execute-process.py` | **✅ Mitigado (PR #9):** catalogada en `SddIA/actions/emit-pr-presented-event.md` + `execute-action.py`; shim legacy en `execute-process.py` |
| `delivery-close-cycle` documenta solo `PullRequest_Merged` | **✅ Resuelto:** fase de presentación + sello ECST en `delivery-close-cycle` v1.1 |

## Alcance de la acción (diseño objetivo)

### Inputs propuestos

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `source_branch` | string | Sí | Rama feature a incorporar (ej. `feat/pbi-005-action-engine`) |
| `target_branch` | string | No | Default `main` |
| `title` | string | Sí | Título del PR |
| `body` | string | No | Cuerpo Markdown (o ruta `body_file` resuelta por Cúmulo) |
| `repository_path` | string | Sí | Raíz del workspace (inyectada por orquestador; validada por Cúmulo) |
| `correlation_id` | string | No | UUID v4 para Sagas; generar vía `crypto-broker` si ausente |
| `status` | string | No | Default `presented` (payload ECST) |

### Outputs propuestos (envelope `actions-contract`)

| Campo | Descripción |
|-------|-------------|
| `success` | boolean |
| `pr_url` | URL del PR creado o actualizado |
| `event_id` | UUID de `PullRequest_Presented` |
| `target_path` | Ruta relativa del JSON en `pending/` |

### Orquestación (pasos lógicos)

1. **Cerbero** — contexto `pr-lifecycle` / `ecosystem-evolution` según `execution-contexts.md`.
2. **Precondición git** — rama `source_branch` publicada en `origin` (`skill:git-manager` → `push` si la política lo exige).
3. **Apertura de PR** — delegación acordada en norma (`pull-request-orchestration.md`): típicamente `skill:shell-executor` + `gh pr create` / `gh pr view` ( **`gh` prohibido en `git-manager`** ).
4. **Sello ECST** — construir `PullRequest_Presented` (`branch`, `status`; opcional `pr_url` si la Clase lo admite tras evolución).
5. **Persistencia** — `skill:filesystem-manager` → `{eda_bus.pending}/<event_id>.json`.
6. **Cierre** — stdout envelope; **sin** `route-domain-event` ni IOTA en esta acción (el watcher procesa `pending/`).

> **Nota:** La acción podía **delegar internamente** en la semántica de `emit-pr-presented-event` o absorberla; el pivot adoptó proceso + sello separados.

### Capabilities propuestas (YAML)

- `change-incorporation-request`
- `pull-request-open-or-update`
- `pr-presented-event-emission`
- `event-bus-pending-write`
- `delegate-git-manager`
- `delegate-shell-executor`
- `delegate-crypto-broker`
- `delegate-filesystem-manager`

### Contexto RBAC propuesto

`pr-lifecycle` (alineado a `accept-pr` y `pull-request-orchestration.md`).

---

## Integración obligatoria en flujos (checklist histórico)

La acción **no cumplió su propósito**; la integración quedó cubierta por `delivery-close-cycle` + `emit-pr-presented-event`.

### Procesos (fases `delegates_to`)

| Proceso | Fase / punto de inserción | Cambio |
|---------|---------------------------|--------|
| **`delivery-close-cycle`** | Fase «Sync remoto y PR» | **✅ Implementado** vía proceso + `emit-pr-presented-event` |
| **`feature`** | Cierre → delegación a `delivery-close-cycle` | Verificar que `branch_name` y `persist_ref` llegan al subproceso |
| **`bug-fix`** | Idem vía `delivery-close-cycle` | Mismo contrato de inputs |
| **`refactorization`** | Idem | Mismo contrato |

### Laboratorio (handlers físicos)

| Artefacto | Estado |
|-----------|--------|
| `emit-pr-presented-event` en `execute-action.py` | **✅ PR #9** |
| Handler monolítico `request-change-incorporation` | **Abortado** — no forjado |

## Tareas (backlog de implementación — congelado)

### Fase 1 — Forja del genoma

- [ ] ~~Redactar semilla y ejecutar `action-creator`~~ — **abortado**
- [ ] ~~Actualizar `pull-request-presented.md` (emisores)~~ — **abortado**
- [ ] ~~Sincronizar `actions/index.md`~~ — **abortado**

### Fase 2 — Cápsula física

- [x] Handler `emit-pr-presented-event` en `execute-action.py` (+ shim deprecado en `execute-process.py`) — PR #9
- [x] Payload de prueba `tmp/emit-pr-presented-refactor.json`
- [x] Smoke: acción → `pending/` → watcher → `processed/` con IOTA (`docs/features/refactor-execute-process-engine/validacion.md`)
- [ ] ~~Handler de `request-change-incorporation`~~ — **abortado**

### Fase 3 — Cableado de procesos

- [x] Actualizar `delivery-close-cycle.md` (fase PR + outputs `pr_url`) — vía Orquestación fractal PR #11
- [ ] Revisar `feature.md` / `bug-fix.md` / `refactorization.md` (handoff a cierre)
- [ ] Actualizar `pull-request-orchestration.md`

## Referencias

| Artefacto | Ruta |
|-----------|------|
| Entrega sustituta | `docs/todos/done/[ARQUITECTURA] Orquestación fractal PR presentado — delivery-close-cycle y PullRequest_Presented.md` |
| Clase ECST | `SddIA/events/pull-request-presented.md` |
| Sello merge (par) | `SddIA/actions/emit-pr-merged-event.md` |
| Proceso fusión | `SddIA/process/accept-pr.md` |
| Cierre de entrega | `SddIA/process/delivery-close-cycle.md` |
| PBI hooks (Hito 3) | `docs/todos/pending/[OPERATIVO] Planificación de Backlog... (Ola A).md` |
