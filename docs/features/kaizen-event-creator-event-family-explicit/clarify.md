---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: refactorization
version_clarify: "1.0.0"
purpose: Retirar default D1.9 y alinear runtime event-creator con genoma fractal
---

# Clarificación — event-creator event_family explícito

Transcript de decisiones (2026-05-29).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`refactorization`** v1.2.0 |
| Rama | `feat/refactorization-kaizen-event-creator-event-family-explicit` |
| `persist_ref` | `docs/features/kaizen-event-creator-event-family-explicit` |
| PBI | `docs/todos/pending/[Kaizen] event-creator — eliminar default event_family domain.md` |
| Origen | D1.9 — `docs/features/telemetria-reactiva-eda-fase1/clarify.md` |
| Disparador PBI §5 | ✅ Fase 3 mergeada (`telemetria-reactiva-eda-fase3/validacion.md` APTO, PR #54) |

---

## D2 — Triaje de bloqueos

| Área | Estado | Veredicto |
|------|--------|-----------|
| Genoma fractal `events/{telemetry,orchestration,domain}/` | ✅ En `main` (Fase 1) | Sin bloqueo |
| `events-contract` v1.1.0 exige `event_family` en Clase | ✅ En `main` | Sin bloqueo |
| `event-creator.md` con fallback `domain` | ✅ Documentado v1.1.0 | **Objetivo de poda** |
| `run_event_forge` runtime | ⚠️ Legacy plano (`SddIA/events/{name}.md`) | **Alcance crítico** — paridad spec/runtime |
| `creator_inputs_from_entity` piloto `event` | ⚠️ Omite `event_family` | **Alcance** — propagar seed |
| Emisores instancia (`eda_bus_utils`, acciones) | ✅ Mayoría con `event_family` explícito | Sin bloqueo estructural |
| PBI maestro Telemetría | En `done/` post-fases | Kaizen independiente |

**Conclusión:** no hay bloqueos de merge upstream. El riesgo principal es **desalineación spec Fase 1 vs handler lab** — este Kaizen cierra esa brecha además de retirar el default.

---

## D3 — Laudo de diseño

### D3.1 Contrato proceso vs artefacto

| Capa | Regla |
|------|-------|
| **Clase ECST** (`{name}.md`) | `event_family` obligatorio en cabecera (sin cambio — `events-contract`) |
| **Input `event-creator`** | Tras Kaizen: **obligatorio**; ausente/vacío → `ValueError` en Validación de Arquitectura |
| **Fallback `domain`** | **Eliminado** — procesos legacy deben migrar payload antes de forjar |

### D3.2 Normalización runtime (`run_event_forge`)

| Decisión | Motivo |
|----------|--------|
| Reimplementar rutas fractal `{directories.events}/{event_family}/{event_name}.md` | Paridad con `event-creator.md` y Fase 1 spec §6 |
| Actualizar índice de **familia** (`{family}/index.md`), no raíz plana | Coherencia Códice Trinidad |
| Incluir `event_family` en cabecera YAML forjada | AC contrato v1.1.0 |
| Rechazar enum fuera de `{telemetry, orchestration, domain}` | Cerbero lógico en handler |

### D3.3 Puente `entity-manager`

| Decisión | Motivo |
|----------|--------|
| `semantic_seed.event_family` **requerido** en piloto `event` | Evitar reintroducir default por omisión en seed |
| Documentar en `entity-manager.md` el campo | O2 inventario |

---

## D4 — Inventario preliminar (O2)

| Emisor | `event_family` hoy | Acción Kaizen |
|--------|---------------------|---------------|
| `ola-c-event-entity/execution.md` smoke | ❌ Ausente en JSON ejemplo | Añadir `"event_family": "domain"` |
| `creator_inputs_from_entity` | ❌ No propaga | Propagar + validar |
| `run_event_forge` | ❌ Ignora campo | Implementar + validar |
| Docs specs Fase 4/5 (`event-creator`) | ✅ Mencionan explícito | Verificar smokes ejecutables |
| Emisores instancia (`write_fractal_event`) | ✅ Explícito | Fuera alcance forja Clase |

Barrido final en fase Dedalo (`spec.md`) con criterio grep documentado.

---

## D5 — Opciones evaluadas

| Opción | Descripción | Decisión |
|--------|-------------|----------|
| A | Solo retirar default en `.md`; runtime sin cambio | Rechazada — deja handler legacy roto vs genoma |
| B | Retirar default + alinear `run_event_forge` + seeds | **Elegida** |
| C | Mantener default en runtime pero warning | Rechazada — viola mandato PBI O1 |

---

## D6 — Regresión mínima (O4)

| Smoke | Esperado |
|-------|----------|
| Forja con `"event_family": "domain"` | Artefacto en `SddIA/events/domain/{name}.md` |
| Forja con `"event_family": "telemetry"` | Artefacto en `SddIA/events/telemetry/{name}.md` |
| Forja sin `event_family` | Error de validación (no forja silenciosa) |
| `entity-manager` piloto `event` + seed completo | Handoff + índice familia |

---

## Referencias

- PBI: `docs/todos/pending/[Kaizen] event-creator — eliminar default event_family domain.md`
- Spec origen: `docs/features/telemetria-reactiva-eda-fase1/spec.md` §6.1
- Handler: `SddIA/scripts/qa/execute_process_capsules.py` → `run_event_forge`, `creator_inputs_from_entity`
