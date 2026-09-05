---
feature_name: eda-telegram-notify-pr-merged
created: "2026-09-05"
process: feature
branch_name: feat/eda-telegram-notify-pr-merged
persist_ref: docs/features/eda-telegram-notify-pr-merged
pbi_ref: docs/todos/pending/PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED.md
execution_id: "fccb9d32-8996-4594-8293-71c27926a017"
document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
pbi_uuid: "5880d6fc-99f3-4ecf-8c9e-a4885d45f117"
pbi_version: "1.2.0"
ola: 2
status: in-progress
---

# Objetivos — eda-telegram-notify-pr-merged

## Misión

Cerrar la laguna de retroalimentación post-fusión: tras `accept-pr` → `PullRequest_Merged`, el Vértice Biológico recibe un resumen ejecutivo por Telegram, coreografiado por EDA (Argos + `send-telegram-notification`), simétrico en titularidad con `PullRequest_Presented`.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `fccb9d32-…`).
- Suscriptor Telegram en SSOT domain + paridad legado.
- Rama compositor + tests en `route_domain_core.rs`.
- Clase `pull-request-merged` § Suscripciones vía `entity-manager` (uuid inmutable, hash refresh).
- Cierre documental en rama + DCC + PR. `accept-pr` condicionado a CI verde.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama antes de mutar `SddIA/events/`.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde (`run_id`).
- `CONSTITUTION_CORE` Filtro A: no declarar Done sobre diffs locales.

## Criterios (PBI)

Ola 1: TG-MERGED-CA1…CA6 v1.1.0 (compositor estático + tool Telegram) — **supersedidos** en JSON/Clase por ola 2.

Ola 2: TG-MERGED-CA1…CA7 según PBI v1.2.0 (action + Gemini fail-soft + un envío).

## Misión ola 2

El mensaje post-merge fusiona metadatos estáticos (ola 1) con síntesis de valor (≤2 líneas, `gemini-http-infer`). Fail-soft si el LLM falla. Misma rama / PR #262. Sin `accept-pr`.
