---
feature_name: vanguardia-soberania-local
created: "2026-05-24"
process: feature
branch_name: feat/vanguardia-soberania-local
persist_ref: docs/features/vanguardia-soberania-local
tracks:
  - L.1
  - E.2
agent_planificador: dedalo
---

# Plan de implementación — Vanguardia Soberanía Local

Blueprint dual-track (L.1 + E.2) alineado a `clarify.md` D1–D9 y `spec.md` §3–§4.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ⏳ | `feat/vanguardia-soberania-local` |
| Objetivos | ✅ | `objectives.md` + L1-SPEC |
| Clarificación | ✅ | `clarify.md` |
| Especificación | ✅ | `spec.md` |
| Plan | ✅ | Este documento |
| **Hito 0 — Módulo ECST** | ✅ | `ecst_validation.py` |
| **Hito 1 — Track L.1** | ✅ | Higiene `accept-pr` |
| **Hito 2 — Track E.2** | ✅ | Aduana emisor |
| **Hito 3 — Smoke + validación** | ✅ | `validacion.md` APTO |

---

## 1. Hito 0 — Extracción `ecst_validation.py` (pre-requisito E.2)

- [ ] Crear `SddIA/scripts/qa/ecst_validation.py` con `load_event_class_schemas`, `validate_ecst_instance`, `validate_domain_mutation_event`.
- [ ] Mover implementación desde `route_domain_event_core.py` (sin cambio de semántica).
- [ ] Actualizar imports en `route_domain_event_core.py`.
- [ ] Verificar smoke router existente / ejecución lab `route-domain-event` sin regresión.

**Criterio de salida:** router compila y valida ECST igual que antes; módulo importable desde `execute-action.py`.

**Estimación:** 1 commit atómico.

---

## 2. Hito 1 — Track L.1 (`accept-pr` higiene auditable)

### 2.1 Helper higiene

- [ ] Implementar `_delete_branch_hygiene(repo, branch)` en `execute_process_capsules.py`.
- [ ] Secuencia: delete local (`remote: false, force: false`) → delete remoto (`remote: true, force: false`).
- [ ] Por op: capturar éxito/error; **prohibido** `except RuntimeError: closed = None`.

### 2.2 Cápsula Fase 4

- [ ] Refactor `capsule_accept_sync_cleanup` → usar helper.
- [ ] Poblar `state["closed_branch"]` y `state["hygiene_failure"]` según L1-SPEC.
- [ ] Push fallido: propagar excepción (no ejecutar delete).

### 2.3 Homólogo delivery

- [ ] Aplicar mismo patrón en `capsule_delivery_local_hygiene`.

### 2.4 Agregación orquestador

- [ ] `run_process`: propagar `hygiene_failure` a `data` si presente en `state`.
- [ ] Fase 4 en `execution_report.phases[]` incluye nodo completo.

### 2.5 Genoma y norma

- [ ] Actualizar `SddIA/process/accept-pr.md` § Fase 4 (payload delete, `hygiene_failure`).
- [ ] Nota en `SddIA/norms/git-operations.md` si aplica.

**Criterio de salida:** L1-CA1–L1-CA5 de `spec.md`.

**Estimación:** 1–2 commits atómicos.

---

## 3. Hito 2 — Track E.2 (aduana emisor)

### 3.1 `execute-action.py`

- [ ] Insertar `validate_domain_mutation_event` en `_run_emit_domain_mutation` pre-`_write_pending_event`.
- [ ] Aborto → envelope `success: false`, `exitCode: 1`, sin side-effect en bus.

### 3.2 `execute_process_capsules.py`

- [ ] Aduana en `emit_domain_mutation()` pre-`write_pending_event`.
- [ ] Aduana en `capsule_emit_domain_mutation` si path alternativo persiste.

### 3.3 Genoma acción

- [ ] `SddIA/actions/emit-domain-mutation.md` — Paso 1b Aduana ECST.
- [ ] Recalcular `hash_signature` si el genoma cambia materialmente.

**Criterio de salida:** E2-CA1–E2-CA4 de `spec.md`.

**Estimación:** 1–2 commits atómicos.

**Dependencia:** Hito 0 completado.

---

## 4. Hito 3 — Smoke, documentación feature, validación

### 4.1 Fixtures

- [ ] `_smoke-accept-pr-hygiene-ok.json` — escenario delete exitoso.
- [ ] `_smoke-accept-pr-hygiene-fail.json` — escenario delete fallido → `hygiene_failure`.
- [ ] `_smoke-emit-domain-mutation-valid.json` — create válido.
- [ ] `_smoke-emit-domain-mutation-invalid.json` — REQUIRED ausente.

### 4.2 Ejecución lab

- [ ] Documentar comandos en `execution.md` (post-implementación).
- [ ] Capturar stdout JSON y rutas `pending/` para evidencia.

### 4.3 Validación Argos

- [ ] `validacion.md` — tablas L1-CA* y E2-CA*, `global: APTO`.
- [ ] Actualizar `objectives.md` § Estado.
- [ ] Enlazar FIX absorbido; marcar checklist en backlog PBI § Prioridad 1.

### 4.4 Cierre feature

- [ ] `delivery-close-cycle` → PR único con ambos tracks.
- [ ] Mover FIX `accept-pr delete_branch` a `docs/todos/done/` en rama PR (cierre documental).

**Criterio de salida:** `validacion.md` APTO; smokes reproducibles.

---

## 5. Orden de ejecución recomendado

```text
H0 (ecst_validation)
    ├── H1 (L.1) ──┐
    └── H2 (E.2) ──┴── H3 (smoke + validación + PR)
```

Tracks H1 y H2 pueden desarrollarse en paralelo tras H0.

---

## 6. Commits atómicos sugeridos

| # | Contenido |
|---|-----------|
| 1 | `objectives` + inicio feature (ya parcial) |
| 2 | `clarify` + `spec` + `plan` |
| 3 | H0 — `ecst_validation.py` + refactor router |
| 4 | H1 — higiene L.1 + genoma `accept-pr` |
| 5 | H2 — aduana E.2 + genoma `emit-domain-mutation` |
| 6 | H3 — smokes + `execution.md` + `validacion.md` |

---

## 7. Dependencias

| Upstream | Relación |
|----------|----------|
| `pbi-005-hito3-ola-b` | `accept-pr` cápsula base |
| `pull-request-review-redesign` | Handoff upstream |
| `ola-c-event-entity` | Clases ECST + router |
| Backlog post-PR11 | Manifiesto P1 vanguardia |
| FIX delete_branch | Absorbido en L.1 |

---

## 8. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Delete remoto ya aplicado (merge GitHub) | `hygiene_failure` parcial — local OK, remoto error esperado documentado |
| Regresión router tras extracción ECST | H0 smoke router antes de H2 |
| Labs con ramas stale | `fetch --prune` en runbook smoke |

---

## 9. Definition of Done (feature)

- [ ] L1-CA1–L1-CA5 y E2-CA1–E2-CA4 verificados.
- [ ] `validacion.md` con `global: APTO`, `pbi_archived: true` (FIX/backlog según cierre documental).
- [ ] Un PR mergeado vía `accept-pr` con higiene auditable demostrada.
