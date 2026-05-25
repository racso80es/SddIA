---
feature_name: ola-c-v3-coreografia
created: "2026-05-22"
updated: "2026-05-25"
process: feature
branch_name: feat/ola-c-v3-coreografia-cierre
persist_ref: docs/features/ola-c-v3-coreografia
related_todo: docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
tracks:
  - P4
status: en_curso
supersedes_iteration: feat/ola-c-v3-coreografia
upstream_prs:
  - 24
  - 25
  - 27
  - 29
---

# Objetivos — Ola C V3: Coreografía Asíncrona

## Meta (iteración 2026-05-25)

**Consolidar y cerrar documentalmente** la coreografía asíncrona del bus EDA bajo `./.events/`: padre inmutable, testigos atómicos por suscriptor, middleware de promoción y recolección diferida. El código base ya está en `main` (PRs #24–#29); esta iteración cierra brechas documentales y alinea normativa con la topología **V3+ simétrica**.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| PR #24 (`feat/ola-c-v3-coreografia`) | Bus `.events/`, testigos, `event-sweeper.py` — **mergeado** |
| PR #25 (`refactor-topologia-eventos-ola-c-v3`) | Cabeceras por estado + `route-domain-event` como proceso — **mergeado** |
| PR #27 + #29 | Retirada acción legacy; sweep inline post-route; Kaizen terminal — **mergeado** |
| Backlog P4 marca ⏳ | **Desactualizado** — triaje en `clarify.md` |
| Manifiesto arquitectura §5 | Fecha 2026-05-19; no refleja entrega posterior |
| Vanguardia E.2 + E.1 IOTA CI | Precedencias P1/P3 cerradas; P4 desbloqueado para cierre |

## Dogma operativo (Estado de Suscriptores)

1. **Padre inmutable** — `pending/[UUID].json` no se muta durante procesamiento.
2. **Testigos atómicos** — `[UUID].[subscriber_id].json` en `*/subscribers/` (processing → processed | dead-letter).
3. **Fallo asimétrico** — dead-letter detiene propagación; padre intacto hasta sweep/Kaizen.
4. **Recolección diferida** — `event-sweeper.py` (daemon) + `try_sweep_event` (inline en route) purgan solo con consenso `processed/`.
5. **Ejecución táctica** — agente lee, ejecuta, devuelve resultado al middleware; cero estado en el padre.

## Hitos — estado real

| Hito | Contenido | Código | Documentación |
|------|-----------|:------:|:-------------:|
| C3.1 | SSOT `event_bus`, `.gitignore`, `eda_bus_utils` | ✅ | ✅ |
| C3.2 | Bootstrap topología al arranque | ✅ | ⚠️ spec plana vs V3+ simétrica |
| C3.3 | Testigos + middleware promoción | ✅ | ✅ |
| C3.4 | `event-sweeper.py` + alerta Kaizen | ✅ | ⚠️ validacion incompleta |
| C3.5 | Topología simétrica V3+ (cabeceras por estado) | ✅ PR #25 | ⚠️ spec original desactualizada |
| C3.6 | Cierre documental + backlog P4 | ⏳ | ⏳ **esta iteración** |

## Objetivos medibles (residuales)

| ID | Objetivo | Criterio |
|----|----------|----------|
| **C3-O1** | **Paridad spec ↔ runtime** | `spec.md` describe topología V3+ simétrica; sin referencias obsoletas a `subscribers/` plano ni `receipts/` |
| **C3-O2** | **Validación APTO** | `validacion.md` con frontmatter `global: APTO`, checks CA trazados a PRs #24–#29 + smoke reproducible |
| **C3-O3** | **Backlog sincronizado** | Manifiesto operativo § P4 refleja ✅ con nota de brechas residuales (p. ej. CI sweeper opcional) |
| **C3-O4** | **Manifiesto arquitectura** | Actualizar §4–§5 en `docs/todos/done/… Ola C V3.md` o enlace a feature APTO |
| **C3-O5** | **Contrato delivery_state** | Documentar en spec: legacy en emisión; trazabilidad runtime = testigos; no mutación padre |
| **C3-O6** | **Smoke CI (opcional)** | Job o step que ejecute `event-sweeper.py --once --json` tras E2E lab sin regresión |

## No objetivos

- Reintroducir recibos `.notificado` o middleware `.procesado`/`.error`.
- Cambiar genoma ECST (`SddIA/events/`) salvo aclaración contractual.
- L1-O5 runbooks, IOTA CI (E.1), vanguardia — ya entregados.
- Daemon sweeper permanente en operador local (documentar invocación manual/`--once` basta).

## Artefactos

| Ámbito | Rutas |
|--------|-------|
| Runtime | `eda_bus_utils.py`, `route_domain_event_core.py`, `event-sweeper.py`, `event-watcher.py` |
| Normativa | `SddIA/process/route-domain-event.md`, `SddIA/events/events-contract.md`, `README.md` |
| Feature | `clarify.md`, `spec.md`, `validacion.md`, `execution.md` |
| Tests | `test_eda_bus_v3plus.py`, `run-eda-e2e-lab.py` |

## Estado fases feature

| Fase | Estado |
|------|--------|
| Inicialización | ✅ rama `feat/ola-c-v3-coreografia-cierre` |
| Clarificación | ✅ `clarify.md` |
| Objetivos | ✅ Este documento |
| Especificación | ✅ `spec.md` V3+ |
| Plan / Implementación | ✅ Doc + CI job |
| Validación | ✅ `validacion.md` APTO |
| Cierre PR | ⏳ |
