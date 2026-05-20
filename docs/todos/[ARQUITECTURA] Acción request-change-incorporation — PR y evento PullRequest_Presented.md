---
document_id: TODO-PR-PRESENTED-FRACTAL-ORCHESTRATION
title: "[ARQUITECTURA] Orquestación fractal PR presentado — proceso delivery-close-cycle + emit-pr-presented-event"
format: markdown
version: "2.0.0"
created: "2026-05-19"
updated: "2026-05-20"
status: "listo para PR"
priority: alta
blocks: "CA-3 parcial / cierre hueco PullRequest_Presented en ciclo de entrega"
feature_ref: docs/features/pr-presented-orchestration
related:
  - SddIA/events/pull-request-presented.md
  - SddIA/actions/emit-pr-presented-event.md
  - SddIA/actions/emit-pr-merged-event.md
  - SddIA/process/delivery-close-cycle.md
  - SddIA/process/accept-pr.md
  - SddIA/process/feature.md
  - SddIA/norms/pull-request-orchestration.md
  - docs/features/pr-presented-orchestration/spec.md
  - docs/features/pr-presented-orchestration/clarify.md
  - docs/todos/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
---

# TODO: Orquestación fractal PR presentado (evolución del impasse request-change-incorporation)

> **Pivot 2026-05-20 (S+):** Se **aborta** la acción monolítica `request-change-incorporation`. La orquestación pertenece a **`delivery-close-cycle`**; el sello EDA permanece en **`emit-pr-presented-event`**. Feature activa: [`docs/features/pr-presented-orchestration/`](../features/pr-presented-orchestration/).

## Objetivo (v2)

Cerrar el hueco **PullRequest_Presented** en el ciclo de entrega con **simetría fractal** respecto a la fusión:

| Momento del ciclo | Orquestador | Hacer físico | Acción atómica (solo bus) | Clase ECST |
|-------------------|-------------|--------------|---------------------------|------------|
| **Presentación** | `delivery-close-cycle` | `git-manager` push + `shell-executor` + `gh pr create` | `emit-pr-presented-event` | `PullRequest_Presented` |
| **Fusión** | `accept-pr` | `git-manager` merge + push + higiene | `emit-pr-merged-event` | `PullRequest_Merged` |

## Problema que cierra

| Síntoma | Causa raíz | Resolución v2 |
|---------|------------|---------------|
| PR #7 sin evento `PullRequest_Presented` | `gh pr create` sin sello EDA | Proceso encadena paso B → `emit-pr-presented-event` |
| Acción combinada PR+bus (propuesta v1) | Violación SRP / caja negra | **Descartada** |
| `delivery-close-cycle` con fase `emit-pr-merged-event` | Cableado erróneo (merge ≠ presentación) | Sustituir por sello **Presented** |
| Handler `emit-pr-presented-event` solo en lab | Mitigado PR #9 | Mantener; extender inputs si aplica `pr_url` |

## Alcance v2 (no forjar request-change-incorporation)

### Paso A — Forja física (proceso, fase «Apertura en forja»)

- `skill:shell-executor` + `gh pr create` (prohibido en `git-manager`).
- Capturar `pr_url` en outputs del proceso.

### Paso B — Registro ontológico (acción pura)

- `action:emit-pr-presented-event` con `branch`, `status`, `emitter_agent: delivery-close-cycle`.
- Opcional v1.1: input/output `pr_url` correlacionado en payload ECST.

### Precondición

- `skill:git-manager` → `push` de `branch_name` antes del paso A.

---

## Avance de objetivos

| Objetivo | Estado | Evidencia |
|----------|--------|-----------|
| Clarificación S+ (abortar acción combinada) | ✅ | `docs/features/pr-presented-orchestration/clarify.md` D2–D5 |
| Especificación técnica | ✅ | `docs/features/pr-presented-orchestration/spec.md` |
| Objetivos feature formalizados | ✅ | `docs/features/pr-presented-orchestration/objectives.md` |
| Handler `emit-pr-presented-event` en laboratorio | ✅ | PR #9 — `execute-action.py` |
| Smoke presented → watcher | ✅ | `refactor-execute-process-engine/validacion.md` |
| Genoma `delivery-close-cycle` v1.1 (fases A→B→C) | ✅ | `SddIA/process/delivery-close-cycle.md` |
| Norma `pull-request-orchestration.md` | ✅ | §3 Presentación |
| Quitar fase errónea `emit-pr-merged-event` del cierre | ✅ | Sustituido por Sello Presentación |
| Handler proceso fases 4–6 | ✅ | `execute_process_capsules.py` |
| Runbooks sin `gh` suelto | ✅ | `docs/features/pr-presented-orchestration/execution.md` |
| `pr_url` en payload ECST (D6) | ✅ | evento/acción v1.1 + handler |

---

## Integración obligatoria (checklist v2)

### Procesos

| Proceso | Cambio | Estado |
|---------|--------|--------|
| **`delivery-close-cycle`** | Fases: push → `gh` → `emit-pr-presented-event`; outputs `pr_url`, `event_id` | ⏳ |
| **`feature`** / **`bug-fix`** / **`refactorization`** | Handoff sin cambio de contrato; verificar `branch_name` | ⏳ revisión |
| **`accept-pr`** | Sin cambio (par de fusión) | ✅ |

### Normas y contratos

| Artefacto | Cambio | Estado |
|-----------|--------|--------|
| `pull-request-orchestration.md` | Presentación = proceso; no acción monolítica | ⏳ |
| `pull-request-presented.md` | Emisor: solo `emit-pr-presented-event`; opcional `pr_url` | ⏳ |
| `emit-pr-presented-event.md` | Inputs `pr_url`, `correlation_id` si v1.1 | ⏳ |
| ~~`request-change-incorporation.md`~~ | **No forjar** | ✅ decidido |

### Laboratorio

| Artefacto | Cambio | Estado |
|-----------|--------|--------|
| `execute_process_capsules.py` | `delivery-close-cycle` fases 4–6 | ⏳ |
| `execute-action.py` | Payload `pr_url` opcional | ⏳ |
| ~~Handler `request-change-incorporation`~~ | **Cancelado** | ✅ |

---

## Criterios de aceptación (v2)

1. Tras `delivery-close-cycle` con rama publicada: **PR remoto** (`pr_url`) y JSON **`PullRequest_Presented`** en `docs/events/pending/`.
2. `event-watcher.py --once` enruta a `processed/` con IOTA según entorno.
3. Flujo **`feature` → `delivery-close-cycle`** documentado; `gh` solo vía `shell-executor` dentro del proceso.
4. **No existe** acción `request-change-incorporation` en catálogo.
5. **No existe** fase `PullRequest_Merged` en `delivery-close-cycle`.
6. Prueba reproducible en `docs/features/pr-presented-orchestration/validacion.md`.

---

## Tareas (backlog)

### Fase 0 — Especificación y clarificación ✅

- [x] Síntesis S+ documentada (`clarify.md` D2)
- [x] Spec técnica (`spec.md`)
- [x] Objetivos feature (`objectives.md`)
- [x] TODO pivot v2.0.0

### Fase 1 — Genoma y normas ✅

- [x] `delivery-close-cycle.md` v1.1.0 (7 fases; quitar `emit-pr-merged-event`)
- [x] `pull-request-orchestration.md` — sección presentación
- [x] `pull-request-presented.md` + `emit-pr-presented-event.md` v1.1 (pr_url opcional)
- [x] Actualizar referencias en `refactor-execute-process-engine/objectives.md`
- [ ] PBI-005 operativo (CA-3 wording)

### Fase 2 — Cápsula física (proceso) ✅

- [x] Handler fases push / gh / emit en `execute_process_capsules.py`
- [x] Extender `execute-action.py` (`pr_url`, `correlation_id` en payload)
- [x] `_smoke-close-cycle-presented.json` + `validacion.md` + smoke lab OK

### Fase 3 — Gobernanza ✅

- [x] PBI-005: CA-3 parcial — presentación vía `delivery-close-cycle` (hooks Hito 3 abiertos)
- [x] Perfil laboratorio en `feature.md` + `delivery-close-cycle.md`
- [x] Handlers opcionales Snapshot / Higiene + `hash_signature` verificado
- [x] Enlace `docs/todos/done/[ARQUITECTURA] Laboratorio — Handler físico proceso feature.md`
- [ ] PR merge a `main` + mover TODO a `done/`

### Descartado (v1)

- ~~Forja `request-change-incorporation`~~
- ~~Handler acción combinada~~
- ~~Absorción / deprecación de `emit-pr-presented-event`~~

---

## Definición de hecho

- [ ] Checklist integración v2 al 100 %
- [ ] PR de laboratorio con `PullRequest_Presented` correlacionado a `pr_url`
- [ ] Ningún runbook usa `gh pr create` fuera de `delivery-close-cycle` (salvo excepción normativa)

## Referencias

| Artefacto | Ruta |
|-----------|------|
| Feature (spec/clarify) | `docs/features/pr-presented-orchestration/` |
| Clase ECST | `SddIA/events/pull-request-presented.md` |
| Sello presentación | `SddIA/actions/emit-pr-presented-event.md` |
| Sello fusión (par) | `SddIA/actions/emit-pr-merged-event.md` |
| Proceso fusión | `SddIA/process/accept-pr.md` |
| Cierre de entrega | `SddIA/process/delivery-close-cycle.md` |
