---
feature_name: eda-telegram-notify-pr-merged
created: "2026-09-05"
process: feature
purpose: Estabilización Filtro A del PBI v1.1.0 tras init lab
version_clarify: "1.0.0"
execution_id: "fccb9d32-8996-4594-8293-71c27926a017"
pbi_ref: docs/todos/pending/PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED.md
document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
pbi_uuid: "5880d6fc-99f3-4ecf-8c9e-a4885d45f117"
pbi_version: "1.2.0"
ola: 2
---

# Clarificación — eda-telegram-notify-pr-merged

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `fccb9d32-8996-4594-8293-71c27926a017`. Rama `feat/eda-telegram-notify-pr-merged`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.1.0 (Filtro A aplicado). WIP previo en `main` aparcado en stash; genoma restaurado a HEAD antes del init.

## Decisiones

| ID | Laudo |
|----|-------|
| L-AGENT | Suscriptor Telegram = `argos` → `send-telegram-notification`. Simetría de titularidad con Presented, no de formato de mensaje. |
| L-COMPOSE | Composición en `build_telegram_message_from_event` (`route_domain_core.rs`). Cero acoplamiento en `accept_pr.rs`. |
| L-SSOT | Registro canónico: `event-domain-subscriptions.json`. Paridad legado en `event-subscriptions.json`. Intent sin «anomalías». |
| L-FIELDS | Mensaje: `source_branch`, `target_branch` (payload, fallback `main`), hash 7 chars, `author`, `security_clearance.auditor`+`policy_applied`, `pr_url` si no vacío, `correlation_id` envelope 8 chars si `len>=8`. Sin `timestamp`. Sin `traceability_*`. |
| L-PRURL | `accept-pr` no pasa `pr_url`. Fase 1 no hace backfill. Ejemplo canónico sin URL. |
| L-FORGE | Clase `pull-request-merged` vía `entity-manager` `update` + `markdown_body_replacements`. Prohibido `update` genérico (regenera UUID). Payload schema intacto → versión Clase **1.0.0** (el forge de replacements no bumpa SemVer de frontmatter). Hash sí se recálcula; `emit-domain-mutation` alinea `eda-coverage.json`. |
| L-CORE | JSON de suscripciones y `route_domain_core.rs` no son genoma DA-2. Mutación directa en fase Ejecución. |
| L-TEST | Tests unitarios del compositor (required, con/sin `pr_url`, anomalía ignorada). Sin cifra global de `cargo test`. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo tras checks verdes del PR. |

## Fuera (ola 1)

Clase 1.1.0 / mostrar huérfanos; timezone; consolidar JSON de suscripciones; documentar Telegram en `pull-request-presented.md`; mutar `send-telegram-notification`; backfill `pr_url`.

## Ola 2 — laudos Filtro A

| ID | Laudo |
|----|-------|
| L-O2-ACTION | Un suscriptor Argos `action: notify-humanized-pr-merged`. Quitar `tool: send-telegram-notification` de `PullRequest_Merged`. Tool+action = doble Telegram. |
| L-O2-CTX | Prompt LLM = campos ECST + envelope. Payload **no** trae commits/diffs. `git-manager` no entrega diff de contenido (`diff_name_only` fuera de MVP). |
| L-O2-DISPATCH | `run_from_event(repo, event)` en `dispatch_subscriber` (espejo persist-pec). El camino genérico action solo pasa payload → perdería `correlation_id`. |
| L-O2-PROMPT | Texto `[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. MAX 2 LINES]` es **contenido** de `request.prompt`, no el prefijo creator (`DO NOT BYPASS EDA BUS…`). |
| L-O2-ASSEMBLE | Handler reutiliza `build_telegram_message_from_event`; concatena `🧠 Síntesis de Valor:` solo si LLM no vacío; invoca `send-telegram-notification` una vez. |
| L-O2-FAILSOFT | Gemini fail/timeout/HTTP/vacío/key ausente → estático solo; action `success`. Telegram fail = fallo suscriptor. IOTA independiente. |
| L-O2-FORGE | Contrato action vía `entity-manager` create. Handler nativo obligatorio. No init `feature` desde `main`. No `accept-pr` (Kintsugi). |
| L-O2-EXAMPLE | Ejemplo canónico = plantilla de formato. La síntesis ilustrativa no es oracle de test. |
