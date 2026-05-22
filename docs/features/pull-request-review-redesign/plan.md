---
feature_name: pull-request-review-redesign
created: "2026-05-22"
process: feature
branch_name: feat/pull-request-review-redesign
persist_ref: docs/features/pull-request-review-redesign
phases: 7
agent_planificador: dedalo
---

# Plan de implementación — Aduana `pull-request-review`

Blueprint alineado al TODO arquitectónico (Fases 1–4) y a `clarify.md` D2–D7.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ✅ | `feat/pull-request-review-redesign` |
| Clarificación | ✅ | `clarify.md` |
| Especificación | ✅ | `spec.md` |
| Objetivos | ✅ | `objectives.md` |
| **Hito 1 — Genoma v2** | ✅ | `pull-request-review.md` v2.0.0 |
| **Hito 2 — Cableado bus** | ✅ | `event-subscriptions.json` + watcher |
| **Hito 3 — Handler lab** | ✅ | `execute_process_capsules.py` |
| **Hito 4 — Smoke + validación** | ✅ | E2E `62bcb6e1-…` |
| **Hito 5 — Purge legacy** | ⏳ | Labs `SddIA_1`…`SddIA_4` backlog |

---

## 1. Hito 1 — Reescritura genoma (limpieza v1.0.0)

- [x] Forjar `SddIA/process/pull-request-review.md` **v2.0.0** con fases §3.3 de `spec.md`.
- [x] Eliminar delegación **`agent:dedalo`** (limpieza semántica v1).
- [x] Añadir inputs `correlation_id`, `pr_url`; output `delivery_state`, `accept_pr_handoff`.
- [x] Recalcular `hash_signature`; actualizar `SddIA/process/index.md`.
- [x] Entrada evolución en `SddIA/evolution/` (transmutación v1→v2).

**Criterio de salida:** genoma parseable por `execute-process`; contrato `process-contract v1.3.0`.

---

## 2. Hito 2 — Intercepción `PullRequest_Presented` (TODO Fase 1 — captura)

- [ ] Añadir suscriptor aduana en `event-subscriptions.json` (conservar IOTA Cúmulo).
- [ ] Actualizar `pull-request-presented.md` — suscripciones y retirar «no-op».
- [ ] Documentar contrato de invocación en `route-domain-event` o extensión watcher (según patrón vigente en lab).
- [ ] Mapear `payload.branch` → `pr_branch`, `event_id` → `correlation_id`.

**Criterio de salida:** evento smoke promovido dispara registro de ejecución aduana en lab.

---

## 3. Hito 3 — Filtros Fase 1 (documental · técnico · Cerbero)

### 3.1 Documental
- [ ] Reglas Argos: frontmatter YAML en artefactos `docs/features/<feature>/`.
- [ ] Archivos obligatorios: `spec.md`, `plan.md`, `implementation.md`, `objectives.md`.

### 3.2 Técnico
- [ ] Encadenar cápsulas test/SAST vía `action:execute-process` (sin terminal cruda).
- [ ] Validar envelopes `capsule-json-io` en stdin/stdout de fases.

### 3.3 RBAC
- [ ] Fase Cerbero: cruce `allowed_policies` vs `context` del área genoma tocada.

**Criterio de salida:** violación simulada en cada dimensión produce `failed` aislado reproducible.

---

## 4. Hito 4 — Fase 2 bloqueo + feedback Argos

- [ ] Implementar abort determinista → `delivery_state: "failed"`.
- [ ] Modelo de comentarios atómicos (archivo:línea ↔ norma) en salida Argos / `validacion.md`.
- [ ] Handler lab: escenario smoke «PR rechazado» con diff ofensivo ficticio.

**Criterio de salida:** smoke negativo deja evento en `processed/` con `failed`.

---

## 5. Hito 5 — Fase 3 Kaizen (Cúmulo)

- [ ] Reglas clasificación severidad (bloqueante vs Kaizen).
- [ ] Plantilla persistencia `docs/todos/[ARQUITECTURA|OPERATIVO] …md`.
- [ ] Handler lab: escenario con deuda menor → TODO generado + `verdict: aprobado`.

**Criterio de salida:** archivo TODO existe; flujo no abortado.

---

## 6. Hito 6 — Fase 4 handoff (sin merge duplicado)

- [ ] Fase 7 invoca `action:execute-process` con `process_name: accept-pr`.
- [ ] Documentar en norma que **merge físico** permanece exclusivo de `accept-pr`.
- [ ] Correlacionar `correlation_id` entre Presented → Review → Merged.

**Criterio de salida:** smoke positivo encadena handoff simulado; no invoca `git-manager merge` en aduana.

---

## 7. Hito 7 — Validación y cierre feature

- [ ] `_smoke-pr-review-presented.json` + ejecución lab documentada en `execution.md`.
- [ ] `validacion.md` con tabla eventos y veredictos.
- [ ] Actualizar TODO arquitectónico con checklist ✅ y enlace a `persist_ref`.
- [ ] `delivery-close-cycle` para abrir PR de esta feature.

---

## 8. Purge legacy (paralelo / post-validación)

| Referencia obsoleta | Sustituto |
|---------------------|-----------|
| `SddIA/process/validate-pull-requests` | `SddIA/process/pull-request-review.md` |
| Tres agentes fósiles (architect, qa-judge, security-engineer) | Argos + Cerbero + Cúmulo |

Labs `SddIA_1`…`SddIA_4`: registrar en backlog operativo; **fuera de alcance** salvo linter `acceptable_pr.md` si bloquea CI local.

---

## 9. Dependencias

| Upstream | Relación |
|----------|----------|
| `pr-presented-orchestration` | Emite `PullRequest_Presented` |
| `ola-c-event-entity` | Contrato bus + watcher |
| `accept-pr` | Downstream merge |
| `pbi-005-hito3-ola-b` | Hooks alimentan presentación; aduana es posterior |
