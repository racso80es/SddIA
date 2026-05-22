---
feature_name: refactor-topologia-eventos-ola-c-v3
created: "2026-05-22"
process: refactorization
purpose: Kaizen topología bus EDA — cabecera por estado + route-domain-event como proceso
updated: "2026-05-22"
---

# Clarificación — Refactor topología eventos Ola C V3+

Transcript de decisiones (2026-05-22) para el PBI Kaizen sobre gestión de eventos emitidos.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`refactorization`** v1.2.0 (Kaizen arquitectónico; paridad documental con `feature`) |
| Nombre operativo | **refactor-topologia-eventos-ola-c-v3** |
| Rama | `feat/refactor-topologia-eventos-ola-c-v3` ✅ |
| `persist_ref` | `docs/features/refactor-topologia-eventos-ola-c-v3` |
| Manifiesto | `docs/todos/pending/TODO_Refactor_Topologia_Eventos_Ola_C_V3.md` |
| Relación V3 | Evolución parcial sobre `docs/features/ola-c-v3-coreografia` (no reescritura del genoma Event) |

---

## D2 — Proceso vs feature (validación patrón arquitectónico)

| Pregunta | Decisión |
|----------|----------|
| ¿Por qué `refactorization` y no `feature`? | Sin capacidad funcional nueva; refactor de topología y promoción acción→proceso |
| ¿Paridad con `feature`? | **Sí** — misma cadena V5, mismo `features-documentation-pattern`, misma fase 1 `workspace-init` |
| Beta laboratorio | Fase 1 **ejecutada** vía `execute-process.py`; fases 2–6 **simuladas** (agentes IDE) |
| Corrección beta aplicada | `execute_process_capsules.py`: `process_label`/`refactor_goal`/`refactor_name` para `refactorization` |

---

## D3 — Topología objetivo (PBI §1)

```
.events/
  pending/                    ← entrada ECST (inmutable hasta sweeper)
    [UUID].json
  processing/                 ← evento en curso
    [UUID].json               ← cabecera (copia/referencia según spec)
    subscribers/
      [UUID].[subscriber].json
  processed/
    [UUID].json               ← cabecera si aún no existía en este estado
    subscribers/
      [UUID].[subscriber].json
  dead-letter/
    [UUID].json
    subscribers/
      [UUID].[subscriber].json
```

| Pregunta | Decisión |
|----------|----------|
| ¿Padre sigue inmutable en `pending/`? | **Sí** — entrada canónica; copias en otros estados son réplicas de cabecera, no mutación del original |
| ¿Convivencia multi-carpeta? | **Sí** — un evento puede tener cabecera/testigos en varios estados según avance por suscriptor |
| ¿Sustituye `subscribers/` plano V3? | **Sí** — anidar bajo cada estado; actualizar SSOT `eda_bus` en Cúmulo |

---

## D4 — Promoción acción → proceso (PBI §2)

| Pregunta | Decisión |
|----------|----------|
| Artefacto destino | `SddIA/process/route-domain-event.md` (proceso orquestador) |
| Acción legacy | Deprecar tras migración; mantener shim temporal en lab si Argos lo exige |
| Responsabilidades | ECST gate, fan-out async, testigos en `processing/subscribers/`, promoción con resultado decorado, réplica cabecera, purge processing al cerrar último suscriptor |
| Forja | **`entity-manager`** + `process-creator` cuando spec lo concrete (no forja manual) |

---

## D5 — Watcher (PBI §3)

| Pregunta | Decisión |
|----------|----------|
| Entrypoint | `event-watcher.py` delega en **`execute-process`** con `process_name: route-domain-event` |
| Sweeper | Revisar si `event-sweeper.py` conserva rol o se fusiona parcialmente en el proceso (Dedalo en spec) |

---

## D6 — Documentación (PBI §4)

| Artefacto | Acción |
|-----------|--------|
| `route-domain-event.md` (action) | Migrar → process; actualizar `actions/index.md` |
| `event-subscriptions.json` | Sin cambio semántico inicial |
| `ola-c-v3-coreografia/*` | Referencia histórica; este refactor documenta delta en `spec.md` |
| README / CONSTITUTION | Solo si Cúmulo detecta divergencia ontología bus |

---

## D7 — Riesgos abiertos (→ spec Dedalo) ✅

Resueltos en `spec.md`:

| Riesgo | Resolución |
|--------|------------|
| Decoración testigo post-respuesta | Campos `result_status`, `delegation`, `error_trace` (§5) |
| Idempotencia fan-out | Skip si testigo terminal; reintento solo en `processing/subscribers/` (§5.4) |
| Sweeper vs cabeceras | Sweeper adaptado; purga pending + archivo processed; dead-letter persistente (§8) |
| Handler lab beta | `route_domain_event_core.py` + registro en `execute_process_capsules.py` (§6.4, plan K2.5) |

---

## D8 — Punto 5 manifiesto: fan-out asíncrono (plan K5)

| Pregunta | Decisión |
|----------|----------|
| ¿Cuándo se implementa async? | **Hito K5** tras K2 (core sync estable) |
| ¿Se elimina modo sync? | **No** — flag lab `SDDIA_LAB_ROUTE_SYNC=1` para regresión CI |
| ¿Decoración testigo? | Obligatoria según `spec.md` §5.2/5.3 en K5.3–K5.4 |
| ¿Cierre documental? | Renumerado a **K6** (E2E incluye escenarios sync + async) |
