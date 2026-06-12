---
feature_name: telegram-fallback-responder
created: "2026-06-11"
process: feature
branch_name: feat/telegram-fallback-responder
persist_ref: docs/features/telegram-fallback-responder
master_pbi_ref: docs/todos/pending/PBI-TG-001- Implementación del Suscriptor de Triaje Inverso (Telegram Fallback Responder).md
master_pbi_id: PBI-TG-001
status: validacion_apto
depends_on:
  - docs/features/puente-sensorial-telegram
related:
  - SddIA/process/telegram-gateway.md
  - SddIA/tools/send-telegram-notification.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/scripts/daemons/telegram-watcher.py
  - SddIA/agents/mayeuta.md
---

# Objetivos — Suscriptor de Triaje Inverso (Telegram Fallback Responder)

## Misión

Implementar la **red de seguridad sensorial** del puente Telegram: un proceso reactivo que asimile la entropía conversacional no estructurada (texto libre que no sea comando), la transmute vía **Mayeuta** en una respuesta orgánica breve y la materialice al Vértice Biológico mediante **`send-telegram-notification`**, sin mutar la Capa 0 (`telegram-watcher`).

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| Puente Sensorial Telegram **Done** (PR mergeado) | Watcher, gateway, tool eferente y suscripciones PR/fractura operativos |
| `telegram-watcher` invoca solo `telegram-gateway` | El fallback debe vivir en **proceso + bus EDA**, no en el daemon |
| PBI exige evento `TelegramMessage_Received` | **No existe** en genoma actual — decisión de forja/routing en `clarify.md` |
| Restricción Ceguera Espacial | Prohibido tocar `.py` / `.bat` / `.sh` del watcher |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | **Forja del proceso** | `SddIA/process/telegram-fallback-responder.md` con frontmatter atómico, fases Filtro C → Síntesis (Mayeuta) → Materialización (`send-telegram-notification`) |
| **O2** | **Filtro C (guarda)** | Si `payload.text` empieza por `/`, `!` o palabra reservada → abortar con `success` silencioso (ceder a especialistas) |
| **O3** | **Síntesis Mayeuta** | Prompt PBI inyectado literalmente; respuesta ≤ 2 líneas, identidad Tormentosa/Aiúa |
| **O4** | **Materialización eferente** | `send-telegram-notification` con `message` = output Fase 2 y `chat_id` = `payload.chat_id` original |
| **O5** | **Suscripción EDA** | Entrada en `event-domain-subscriptions.json` para el evento de recepción acordado; integridad JSON + `eda-coverage.json` |
| **O6** | **Cierre documental** | Un PR: genoma + docs + PBI en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`) |

## Alcance de esta entrega

1. Rama `feat/telegram-fallback-responder` desde `main`.
2. Cascada documental bajo `persist_ref`: `clarify.md`, `objectives.md`, `spec.md`, `plan.md`, `implementation.md`, `execution.md`, `validacion.md`.
3. Genoma: proceso + suscripción (+ evento si Dedalo confirma forja).
4. Smokes lab documentados en `plan.md`.

## Fuera de alcance

- Modificación de `telegram-watcher.py`, `.bat` o `.sh`.
- Orquestación LLM para comandos estructurados (competencia de `telegram-gateway`).
- Webhooks, multi-chat o allowlist dinámica.
- Respuestas > 2 líneas o tono asistente genérico.

## Ley aplicada

- Proceso **`feature`** v1.3.0 (`SddIA/process/feature.md`).
- Norma **`features-documentation-pattern`** v1.2.1.
- Contratos **`process-contract`**, **`events-contract`**, **`tools-contract`** vigentes.
- Cierre documental en rama: PBI → `docs/todos/done/` + `validacion.md` con `pbi_archived: true` en el mismo PR.

## Criterio de éxito (feature completa)

- Mensaje libre autorizado en Telegram recibe respuesta orgánica breve vía fallback (sin pasar por gateway de tareas).
- Comandos (`/`, `!`, reservados) no disparan fallback.
- Argos APTO; PBI archivado en `done/` en la rama del PR.
