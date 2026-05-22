---
document_id: PBI-FIX-EVENT-PENDING-SWEEPER
title: "[FIX] event-pending-sweeper — padre permanece en pending tras enrutamiento"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "abierto"
priority: alta
process: bug-fix
incident_ref: "Bus EDA — testigos en processed/ pero JSON padre persiste en pending/"
feature_ref_target: docs/fixes/event-pending-sweeper
related:
  - SddIA/scripts/daemons/event-watcher.py
  - SddIA/scripts/daemons/event-sweeper.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/events/events-contract.md
  - README.md
---

# [FIX] event-pending-sweeper — padre permanece en pending tras enrutamiento

## 0. Mandato del PBI

Debe iniciarse como **`bug-fix`** bajo `docs/fixes/event-pending-sweeper/`.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Reproducir** el síntoma de forma determinista | Smoke: emit → watcher `--once` → padre sigue en `pending/` con testigos en `processed/subscribers/` |
| **O2** | **Corregir** cierre operativo del ciclo EDA | Tras enrutamiento exitoso, padre purgado de `pending/` sin invocación manual separada del sweeper |
| **O3** | **Preservar** semántica V3+ | Eventos con `dead-letter/` **no** se purgan; alerta Kaizen intacta |
| **O4** | **Blindaje regresión** | Smoke documentado: emit → watcher → ausencia de padre en `pending/` cuando consenso alcanzado |

---

## 1. Incidente (2026-05-22)

| Campo | Valor |
|-------|--------|
| Contexto | Forja `entity-manager` → `route-domain-event` (proceso) vía watcher |
| Síntoma | Suscriptores ejecutados; testigos en `.events/processed/subscribers/`; **padre ECST permanece en `.events/pending/`** |
| Workaround | Ejecutar manualmente `python SddIA/scripts/daemons/event-sweeper.py --once` |
| Evidencia lab | Tras sweeper manual: purga de `4d4f14b9-…` y `c172ee3c-…`; Kaizen correcto para dead-letter `5b99aa98-…` y `99459a47-…` |

### Traza operativa

```
emit-domain-mutation / entity-manager
  → .events/pending/<uuid>.json
  → event-watcher --once
      → execute-process route-domain-event
      → processed/subscribers/*.json  ✅
      → pending/<uuid>.json           ❌ (permanece)
  → (manual) event-sweeper --once
      → pending/<uuid>.json purgado   ✅
```

**Mensaje engañoso en watcher:** `"enrutado (padre permanece en pending)"` — comportamiento documentado en Ola C V3+, pero **no operativo** cuando el sweeper no corre en paralelo.

---

## 2. Diagnóstico técnico (hipótesis)

| # | Hipótesis | Evidencia |
|---|-----------|-----------|
| H1 | **Desacople watcher ↔ sweeper** | README paso 2 vs paso 5 son daemons independientes; watcher no invoca sweeper |
| H2 | **Diseño V3+ correcto pero incompleto en runtime** | `events-contract.md` §4: purga solo vía `event-sweeper.py` |
| H3 | **Lab/dev ejecuta solo watcher** | Flujos documentados (`--once`) no encadenan sweeper |
| H4 | **No es fallo de route-domain-event** | `route_domain_event_core` promueve testigos; no llama `archive_event_after_sweep` |

### Cadena afectada

```
pending/ (padre)
  → event-watcher.py
  → route-domain-event (route_domain_event_core.py)
  → processed/subscribers/ (testigos OK)
  → [GAP] sin invocación automática a sweep
  → pending/ (padre huérfano hasta sweeper manual)
```

---

## 3. Alcance del fix (Tekton)

### Hito 1 — Reproducción

- [ ] Smoke mínimo documentado en `clarify.md`.
- [ ] Baseline: contar JSON en `pending/` antes/después de watcher sin sweeper.

### Hito 2 — Corrección (opciones a evaluar en `spec.md`)

- [ ] **A)** Invocar `sweep_once` / `try_sweep_event(uuid)` al final de `_run_watcher` tras route exitoso.
- [ ] **B)** Invocar purga consensuada al cierre de `route_domain_event()` cuando todos los suscriptores requeridos están terminales.
- [ ] **C)** Extraer helper compartido en `eda_bus_utils.py`; watcher y sweeper lo reutilizan.

Preferencia inicial: **B + C** (cierre inmediato post-orquestación; watcher mantiene sweeper periódico para eventos stale).

### Hito 3 — Retroactivo bus local

- [ ] Documentar estado post-fix de eventos dead-letter existentes (`5b99aa98-…`, `99459a47-…`).

### Hito 4 — Regresión

- [ ] Smoke CI/lab: emit sintético → watcher `--once` → `pending/` vacío para UUID con consenso.
- [ ] Dead-letter: padre **no** purgado; stderr Kaizen emitido.

---

## 4. Proceso de inicio

```json
{
  "process": "bug-fix",
  "fix_name": "event-pending-sweeper",
  "branch_name": "fix/event-pending-sweeper",
  "persist_ref": "docs/fixes/event-pending-sweeper",
  "bug_summary": "Corregir cierre operativo del bus EDA: tras enrutamiento exitoso vía event-watcher/route-domain-event, purgar padre de pending/ sin depender de sweeper manual; preservar Kaizen en dead-letter.",
  "base_branch": "main"
}
```

---

## 5. Criterio de cierre del PBI

- [ ] Argos **APTO** en `docs/fixes/event-pending-sweeper/validacion.md`.
- [ ] Smoke O4 verde en rama `fix/event-pending-sweeper`.
- [ ] Mensaje watcher actualizado (sin implicar estado terminal incorrecto).
- [ ] Este TODO movido a `docs/todos/done/`.

---

## 6. Referencias

| Artefacto | Ruta |
|-----------|------|
| Watcher | `SddIA/scripts/daemons/event-watcher.py` |
| Sweeper | `SddIA/scripts/daemons/event-sweeper.py` |
| Orquestador | `SddIA/scripts/qa/route_domain_event_core.py` |
| Purga consenso | `SddIA/scripts/qa/eda_bus_utils.py` → `archive_event_after_sweep` |
| Contrato ciclo | `SddIA/events/events-contract.md` §4 |
| Pipeline README | `README.md` § Pipeline runtime |
