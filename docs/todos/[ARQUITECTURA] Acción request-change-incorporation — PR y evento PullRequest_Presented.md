---
document_id: TODO-ACTION-REQUEST-CHANGE-INCORPORATION
title: "[ARQUITECTURA] Acción request-change-incorporation — Solicitud de incorporación de cambios (PR + bus EDA)"
format: markdown
version: "1.0.0"
created: "2026-05-19"
status: "pendiente"
priority: alta
blocks: "CA-3 parcial / cierre hueco PullRequest_Presented en ciclo de entrega"
related:
  - SddIA/events/pull-request-presented.md
  - SddIA/actions/emit-pr-merged-event.md
  - SddIA/process/delivery-close-cycle.md
  - SddIA/process/feature.md
  - SddIA/process/accept-pr.md
  - SddIA/norms/pull-request-orchestration.md
  - docs/todos/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
---

# TODO: Acción `request-change-incorporation` (solicitud de incorporación de cambios)

## Objetivo

Forjar y cablear una **acción atómica de dominio** — nombre canónico propuesto: **`request-change-incorporation`** — responsable de:

1. **Abrir o actualizar** la Pull Request de incorporación de cambios hacia `main` (sustituyendo invocaciones ad hoc a `gh pr create` fuera del genoma).
2. **Emitir** la instancia ECST **`PullRequest_Presented`** en `eda_bus.pending` (`cumulo.paths.json`), con payload conforme a `SddIA/events/pull-request-presented.md`.

Simetría deseada con el par existente:

| Momento del ciclo | Acción / sello | Clase de evento |
|-------------------|----------------|------------------|
| Presentación de PR | **`request-change-incorporation`** *(este TODO)* | `PullRequest_Presented` |
| Fusión soberana (`accept-pr`) | `emit-pr-merged-event` | `PullRequest_Merged` |

## Problema que cierra

| Síntoma | Causa raíz |
|---------|------------|
| PR #7 sin evento `PullRequest_Presented` | `gh pr create` no integra el bus; no se invocó `emit-pr-presented-event` |
| Handler de `emit-pr-presented-event` solo en `execute-process.py` | **✅ Mitigado (PR #9):** catalogada en `SddIA/actions/emit-pr-presented-event.md` + `execute-action.py`; shim legacy en `execute-process.py` |
| `delivery-close-cycle` documenta solo `PullRequest_Merged` | Falta fase explícita de presentación + sello ECST |

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

> **Nota:** La acción puede **delegar internamente** en la semántica de `emit-pr-presented-event` o absorberla; si se absorbe, deprecar emisor duplicado en documentación y unificar handler físico en `execute-action.py`.

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

## Integración obligatoria en flujos (checklist)

La acción **no cumple su propósito** hasta estar referenciada y ejecutable desde:

### Procesos (fases `delegates_to`)

| Proceso | Fase / punto de inserción | Cambio |
|---------|---------------------------|--------|
| **`delivery-close-cycle`** | Fase «Sync remoto y PR» (antes o después de push) | Sustituir/documentar apertura de PR vía `action:request-change-incorporation`; propagar `pr_url` a outputs del proceso |
| **`feature`** | Cierre → delegación a `delivery-close-cycle` | Verificar que `branch_name` y `persist_ref` llegan al subproceso |
| **`bug-fix`** | Idem vía `delivery-close-cycle` | Mismo contrato de inputs |
| **`refactorization`** | Idem | Mismo contrato |

### Normas y contratos

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/norms/pull-request-orchestration.md` | Declarar `request-change-incorporation` como vía canónica de **presentación** de PR (complemento de `accept-pr` para **fusión**) |
| `SddIA/events/pull-request-presented.md` | Añadir emisor autorizado `request-change-incorporation` (y/o mantener `emit-pr-presented-event` como alias interno) |
| `SddIA/norms/execution-contexts.md` | Registrar cápsula asociada en el contexto `pr-lifecycle` |

### Laboratorio (handlers físicos)

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/scripts/qa/execute-process.py` | Handler `--action request-change-incorporation` (o vía `execute-action.py` si se unifica motor de acciones) |
| `SddIA/scripts/qa/execute-action.py` | Registrar en `PHYSICAL_HANDLERS` tras forja |
| `docs/features/*/execution.md` | Reemplazar ejemplos sueltos de `gh pr create` por invocación de la acción |

### Catálogo Cúmulo

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/actions/request-change-incorporation.md` | Forja vía `action-creator` |
| `SddIA/actions/index.md` | Fila con **Capabilities** sincronizadas |
| `SddIA/core/event-subscriptions.json` | Sin cambio obligatorio (ya hay suscriptor IOTA en `PullRequest_Presented`) |

### Relación con Hito 3 (hooks Git)

Los hooks `pre-push` / `post-merge` del PBI-005 **pueden llamar** a esta acción en lugar de duplicar lógica; documentar precedencia: **proceso de entrega** = acción explícita; **hooks** = automatización opcional del mismo contrato.

---

## Criterios de aceptación

1. Tras ejecutar la acción con rama publicada, existe **PR en remoto** (`pr_url` en salida) y JSON **`PullRequest_Presented`** en `docs/events/pending/`.
2. `event-watcher.py --once` enruta el evento a `processed/` con `delivery_state.cumulo: success` (IOTA según entorno).
3. Un flujo **`feature` → `delivery-close-cycle`** documentado invoca la acción sin `gh` directo en guías de ejecución.
4. La acción está catalogada en `actions/index.md` con UUID y capabilities.
5. Prueba reproducible documentada en `docs/features/.../execution.md` o script QA.

## Tareas (backlog de implementación)

### Fase 1 — Forja del genoma

- [ ] Redactar semilla y ejecutar `action-creator` → `SddIA/actions/request-change-incorporation.md`
- [ ] Actualizar `pull-request-presented.md` (emisores)
- [ ] Sincronizar `actions/index.md`

### Fase 2 — Cápsula física

- [x] Handler `emit-pr-presented-event` en `execute-action.py` (+ shim deprecado en `execute-process.py`) — PR #9
- [x] Payload de prueba `tmp/emit-pr-presented-refactor.json`
- [x] Smoke: acción → `pending/` → watcher → `processed/` con IOTA (`docs/features/refactor-execute-process-engine/validacion.md`)
- [ ] Handler de **`request-change-incorporation`** (apertura PR + sello; distinto de solo emitir evento)

### Fase 3 — Cableado de procesos

- [ ] Actualizar `delivery-close-cycle.md` (fase PR + outputs `pr_url`)
- [ ] Revisar `feature.md` / `bug-fix.md` / `refactorization.md` (handoff a cierre)
- [ ] Actualizar `pull-request-orchestration.md`

### Fase 4 — Gobernanza y deuda

- [ ] Decidir destino de `emit-pr-presented-event` (alias, absorción o acción independiente legacy)
- [ ] Actualizar PBI-005 operativo (CA-3 / matriz hooks vs acción)
- [ ] Enlazar desde `docs/todos/[ARQUITECTURA] Laboratorio — Handler físico proceso feature.md` si el handler de `feature` delega en `delivery-close-cycle`

## Definición de hecho

- [ ] Ningún runbook de feature en el repo usa `gh pr create` sin pasar por `request-change-incorporation` (salvo excepción documentada en norma).
- [ ] Al menos un PR de laboratorio deja rastro `PullRequest_Presented` correlacionado con `pr_url`.
- [ ] Checklist de integración en flujos (sección anterior) al 100 %.

## Referencias

| Artefacto | Ruta |
|-----------|------|
| Clase ECST | `SddIA/events/pull-request-presented.md` |
| Sello merge (par) | `SddIA/actions/emit-pr-merged-event.md` |
| Proceso fusión | `SddIA/process/accept-pr.md` |
| Cierre de entrega | `SddIA/process/delivery-close-cycle.md` |
| Handler presented (lab) | `SddIA/scripts/qa/execute-process.py` → `_emit_pr_presented` |
| PBI hooks (Hito 3) | `docs/todos/[OPERATIVO] Planificación de Backlog... (Ola A).md` |
