---
feature_name: eda-domain-entities-splus
created: "2026-05-20"
process: feature
branch_name: feat/eda-domain-entities-splus
persist_ref: docs/features/eda-domain-entities-splus
pbi_ref: PBI-005
todo_ref: docs/todos/[ARQUITECTURA] EDA — Eventos Domain_Entity para todas las entidades de dominio.md
grade: S+
protocol: Protocolo de Acero
status: planificacion
---

# Objetivos — EDA Domain Entities S+

## Misión

Cerrar la brecha EDA en mutaciones genómicas: las **ocho** clases de entidad (`skill`, `event`, `process`, `agent`, `tool`, `action`, `norm`, `codex`) deben pasar por `entity-manager` con emisión determinista de `Domain_Entity_*`, ampliado con el **Protocolo de Acero** (topología fractal, mandato DLT, idempotencia y aduana Argos permanente).

## Contexto

| Hecho | Impacto |
|-------|---------|
| Piloto v1 solo cubre `skill` + `event` | Seis clases en create/update sin sello |
| Síntoma `markdown-table-editor` | Forja manual sin `Domain_Entity_Created` |
| TODO v3.0.0 consolidado | Fuente SSOT de objetivos S+ |
| Pausa táctica Yunque Rúnico | Fase 0 (norma/contrato) antes del martillo sobre forges |

## Alcance por fases

| Fase | Contenido | Estado |
|------|-----------|--------|
| **0 — Protocolo de Acero** | `origin_topology`, mandato DLT, idempotencia, audit permanente | ⏳ |
| **A — Piloto entity-manager** | 6 clases, forges lab, handoff, sello | ⏳ |
| **B — Validación + Argos** | E2E por clase, aduana en `delivery-close-cycle` | ⏳ |
| **C — Backfill** | Huérfanas: `--emit --skip-dlt` + **`--anchor-merkle` obligatorio** al cierre | ⏳ |

## Alcance inicial (esta feature — planificación)

1. Documentación bajo `persist_ref`: `objectives.md`, `clarify.md`, `spec.md`, `plan.md`.
2. Rama `feat/eda-domain-entities-splus`.
3. Enlace bidireccional con TODO-EDA-DOMAIN-ENTITIES v3.

## Fuera de alcance inmediato

- Implementación de forges (`run_*_forge`) — post-planificación, Fase A.
- Retirada de shims CLI `execute-process` (deuda Ola C separada).
- Endurecer `payload_schema_hash` a REQUIRED en ECST.
- Extensión fractal local más allá de `tool` (`scope=local`).

## Ley aplicada

- Proceso `feature` v1.2.0 (`SddIA/process/feature.md`).
- Norma `features-documentation-pattern` v1.0.0.
- Invariante: solo `entity-manager` cierra con `action:emit-domain-mutation`.
- Git exclusivamente vía `skill:git-manager` en fases de ejecución.
- SSOT rutas: `SddIA/core/cumulo.paths.json`.

## Criterio de éxito (feature completa)

### Capa base

- Matriz de entidades sin ❌ en create/update (8 clases).
- Evento ECST en `eda_bus.pending` tras cada mutación válida.
- Handlers físicos en `execute_process_capsules.py` (artefacto + índice + handoff).

### Capa S+

- `origin_topology` en payload; fan-out vía `applies_to_origin_topology` en suscripciones.
- DLT obligatorio por entidad en **circuito operativo**; backfill con `--skip-dlt` en emit + **`--anchor-merkle` obligatorio** para cierre Fase C.
- Idempotencia demostrada (doble invocación → un evento).
- Argos bloquea `delivery-close-cycle` ante Ruido de Sistema.

## Handoff

Tras aprobación de `clarify.md`, `spec.md` y `plan.md` → Tekton ejecuta Fase 0 según blueprint.
