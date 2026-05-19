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

## Alcance de esta entrega (fase documental + topología)

1. **Documentación de feature:** `objectives.md`, `clarify.md`, `spec.md` bajo `persist_ref`.
2. **README:** fila **Event** en ontología de activos; sección genoma vs runtime vs instancia (sin tocar `CONSTITUTION_CORE.md`).
3. **Cúmulo (`cumulo.paths.json`):**
   - `directories.events` → `SddIA/events` (Clases de Evento).
   - `eda_bus` → bus runtime bajo `.docs/events/` con colas `pending`, `processing`, `processed`, `dead_letter`.
   - `eda_instance.customization` → `.SddIA/events` (personalización por proyecto).
4. **Consumidores:** actualizar fallbacks y referencias literales en acciones, watcher, `execute-process.py`, normas operativas y `.gitignore`.
5. **Git:** rama `feat/ola-c-event-entity` vía `git-manager` (Opción B); commits vía cápsula.

## Fuera de alcance (Ola C posterior)

- Forja de `events-contract.md`, `SddIA/events/index.md` y clases concretas (`PullRequest_Merged.md`, etc.).
- Extensión de `entity-manager` / `event-creator` para `entity_class: event`.
- Handler físico completo del proceso `feature` en `execute-process.py`.
- Enmienda a `CONSTITUTION_CORE.md` (decisión explícita del Arquitecto: coherencia vía README únicamente).

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
