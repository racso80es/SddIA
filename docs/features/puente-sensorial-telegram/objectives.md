---
feature_name: puente-sensorial-telegram
created: "2026-05-29"
process: feature
branch_name: feat/puente-sensorial-telegram
persist_ref: docs/features/puente-sensorial-telegram
pbi_ref: docs/todos/done/Puente Sensorial Telegram Ingesta Externa y Notificación.md
document_id: PBI-PUENTE-SENSORIAL-TELEGRAM
status: validacion_apto
depends_on:
  - docs/features/ampliacion-configuracion-entornos
  - docs/features/ola-c-v3-coreografia
related:
  - SddIA/scripts/daemons/event-watcher.py
  - SddIA/scripts/qa/env_loader.py
  - SddIA/core/event-subscriptions.json
  - docs/todos/kitchen/ED Centinela Soberanía de Ejecución y Autogestión SddIA.md
---

# Objetivos — Puente Sensorial Telegram

## Misión

Dotar al Vértice Biológico de un **canal bidireccional** con Telegram: **ingesta aferente** de voluntad (texto → eventos de dominio en el bus local) y **notificación eferente** reactiva (PR presentado, fractura del sistema) sin presencia en terminal, preservando **Ceguera Espacial** de las tools y **aislamiento** del motor de eventos frente a la red pública.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| No existe integración Telegram en el genoma | Forjar tool, proceso, eventos y daemon |
| Jerarquía de bóvedas operativa | Token y chat_id viven en `.SddIA/.dev/.env` |
| `event-watcher` + `route-domain-event` operativos | Eventos depositados en `pending/` serán coreografiados por Ola C |
| PBI kitchen **ED Centinela** en cocina | MVP del watcher **sin** contrato Centinela completo; alineación futura |
| Argos reacciona a `PullRequest_Presented` | Notificación humana vía tool, no lógica embebida en auditor |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | **Inmunidad Capa 0** | Mensaje de `chat_id` no autorizado: sin `execute-process`, sin evento, sin llamada IA |
| **O2** | **Idempotencia** | Reinicio de `telegram-watcher` no reprocesa updates ya consumidos (`last_update_id` persistente) |
| **O3** | **Tool eferente ciega** | `send-telegram-notification` envía POST `/sendMessage`; input solo `message` + `parse_mode`; sin leer bus ni motivo del evento |
| **O3b** | **Táctica del Refugio** (Fricción de Acero) | Si Telegram rechaza por parsing (`400`), reintento inmediato **sin** `parse_mode`; alerta táctica en dispositivo aunque Argos/Mayeuta envíen markup roto |
| **O4** | **Gateway acoplado al bus** | `telegram-gateway` emite ≥1 instancia `Manual_Task_Requested` válida ECST en `./.events/pending/` |
| **O5** | **Kaizen por patrón** | Texto `TODO: …` produce `Kaizen_Idea_Captured` con payload acordado |
| **O6** | **Notificación PR** | Suscriptor (o handler lab) ante `PullRequest_Presented` invoca la tool con mensaje derivado del payload |
| **O7** | **Notificación fractura** | Idem ante `System_Fracture_Detected` |
| **O8** | **EDA coverage** | Entidades nuevas en `eda-coverage.json`; gate `--scan` sin huérfanos |
| **O9** | **Cierre documental** | Un PR: código + docs + PBI en `done/` + `validacion.md` APTO |

## Alcance de esta entrega (planificación completada)

1. Rama `feat/puente-sensorial-telegram` desde `main`.
2. Documentación bajo `persist_ref`: `clarify.md`, `objectives.md`, `spec.md`, `plan.md`.
3. Tekton T1–T6 ejecutado; documentación de cierre en rama.

## Fuera de alcance (MVP)

- Orquestación semántica con LLM (Mayeuta/Tekton) para consultas libres en Telegram.
- Contrato **ED Centinela** completo (`daemons-contract`, `governance-daemon-manager`, `Daemon_Heartbeat`).
- Webhooks Telegram (solo long polling en MVP).
- Bot multi-chat o listas de allowlist dinámicas.
- Retirada o migración de watchers legacy (`github_bridge_watcher` si existe fuera de repo).

## Ley aplicada

- Proceso **`feature`** v1.3.0 (`SddIA/process/feature.md`).
- Norma **`features-documentation-pattern`** v1.2.1.
- Contratos **`tools-contract`**, **`process-contract`**, **`events-contract`** vigentes.
- Cierre documental en rama: PBI → `docs/todos/done/` + `validacion.md` con `pbi_archived: true` en el mismo PR.

## Criterio de éxito (feature completa — post-Tekton)

- Operador puede recibir notificación de PR/fractura en Telegram configurando bóveda local (incluso si el texto del agente rompe MarkdownV2 — refugio plain).
- Operador puede enviar `TODO: …` o texto libre autorizado y ver evento en `pending/`.
- Smokes documentados en `plan.md` verdes (incl. AC7 refugio); Argos APTO.
- PBI archivado en `done/` en la rama del PR.

## Handoff

Tekton + Argos lab completados. Siguiente: `delivery-close-cycle` → PR único.
