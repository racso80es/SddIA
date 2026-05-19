---
feature_name: ola-c-event-entity
created: "2026-05-19"
process: feature
branch_name: feat/ola-c-event-entity
persist_ref: docs/features/ola-c-event-entity
---

# Objetivos — Ola C: Evento como entidad de dominio

## Misión

Elevar **Evento** al rango de entidad de dominio de primer nivel en el ecosistema SddIA (Ola C), legalizando su ontología en el mapa cartográfico del Core, resegmentando el bus de eventos runtime frente a la capa de personalización por proyecto, y preparando el genoma (`SddIA/events/`) para contratos funcionales versionados.

## Alcance por hitos

| Hito | Contenido | Estado |
|------|-----------|--------|
| **0 — Topología** | README, Cúmulo `eda_bus`, consumidores, `.gitignore` | ✅ |
| **1 — Enmienda + genoma base** | `CONSTITUTION_CORE.md` §3.1, `events-contract.md`, `index.md`, `contracts.events` | ✅ |
| **2 — event-creator** | `event-creator.md`, `process/index.md`, `interaction-triggers.json` | ✅ |
| **3–6 — Forja Tekton** | `entity-manager`, clases ECST forense, Argos | ⏳ |

## Alcance inicial completado (Hito 0)

1. **Documentación de feature:** `objectives.md`, `clarify.md`, `spec.md`, `plan.md` bajo `persist_ref`.
2. **README:** fila **Event** + sección genoma / runtime / instancia.
3. **Cúmulo:** `directories.events`, `eda_bus` → `docs/events/`, `eda_instance.customization`.
4. **Consumidores** del bus actualizados; Git vía `git-manager`.

## Próximo alcance (Hito 1)

- Enmienda **`SddIA/CONSTITUTION_CORE.md`** (Evento de Dominio).
- Forja **`SddIA/events/events-contract.md`** e **`index.md`**.
- Clave **`contracts.events`** en Cúmulo.

## Fuera de alcance inmediato

- Clases ECST concretas (`pull-request-merged.md`, …) → Fase 4 del plan.
- `event-creator`, piloto `entity-manager` → Fases 2–3.
- Endurecer `payload_schema_hash` a REQUIRED → post-transición Ola A.

## Ley aplicada

- Proceso `feature` v1.2.0 (`SddIA/process/feature.md`).
- Norma `features-documentation-pattern` v1.0.0.
- Norma `git-via-skills-or-process.md` — Git exclusivamente vía `git-manager`.
- SSOT de rutas: `SddIA/core/cumulo.paths.json`.

## Criterio de éxito

- Ontología **Event** visible en README con distinción estricta de tres rutas.
- `cumulo.paths.json` refleja la nueva topología sin literales obsoletos en consumidores activos del bus.
- Cola `processing` registrada y cableada en watcher.
- Documentación `clarify.md` y `spec.md` aprobables por Mayeuta/Dedalo antes de forja de contratos.
