---
feature_name: eda-telegram-notify-pr-merged
created: "2026-09-05"
process: feature
phases:
  - json-subscriptions
  - composer-and-tests
  - entity-manager-event
  - evolution-and-docs
  - dcc-pr-ci-accept
branch_name: feat/eda-telegram-notify-pr-merged
persist_ref: docs/features/eda-telegram-notify-pr-merged
pbi_ref: docs/todos/pending/PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED.md
ola: 2
document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
uuid: "5880d6fc-99f3-4ecf-8c9e-a4885d45f117"
execution_id: "fccb9d32-8996-4594-8293-71c27926a017"
---

# Plan — eda-telegram-notify-pr-merged

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución (L1–L5) en el mismo ciclo hasta PR verde y `accept-pr`.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. Semilla `.tmp/feature-eda-telegram-notify-pr-merged.json`.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. Stash de WIP código en `main` se reaplica tras este commit.

## Fase L1 — JSON suscripciones (CA1, CA5)

`SddIA/core/event-domain-subscriptions.json` y `SddIA/core/event-subscriptions.json`: +suscriptor Argos/Telegram. Intent sin «anomalías». IOTA intacto. Core ∉ DA-2.

## Fase L2 — Compositor + tests (CA2, CA3, CA4)

`route_domain_core.rs`: rama `"PullRequest_Merged"` según spec (`target_branch` desde payload). Tests DD-8. Verificar cero invocaciones Telegram en `accept_pr.rs`.

```text
cd SddIA && cargo test -p execute-process --lib -- telegram_message_for_pr_merged
```

## Fase L3 — Genoma evento (CA6)

Prefijo RAW. Topología `objectives.md` ya en rama.

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-pull-request-merged.json
```

`markdown_body_replacements` sobre § Suscripciones. Prohibido `Write`/`StrReplace` sobre `SddIA/events/`. Coverage del uuid lo sella `emit-domain-mutation`.

## Fase L4 — Evolution + docs de ejecución

`sddia-qa evolution-register` (`modificacion`, motor `execute-process` + evento). `implementation.md` / `execution.md`. Si toca `directories.evolution`: `sddia-qa gate-evolution --json --range` exit 0 antes de push.

## Fase L5 — Cierre, DCC, CI, accept-pr

PBI → `docs/todos/done/`. `validacion.md` con CA-CI `PENDIENTE-CI` hasta `run_id` verde; entonces `global: APTO` + `pbi_archived: true`. `delivery-close-cycle`. Tras checks verdes del PR: `accept-pr`.

---

# Plan ola 2 (PBI v1.2.0)

PBI reabierto en `docs/todos/pending/`. Misma rama / persist_ref / PR #262. **No** `feature` init desde `main`. **No** `accept-pr` (Kintsugi `1e62e8b851f8`).

Commit de esta parada = diseño ola 2 (PBI + clarify/objectives/spec/plan). Ejecución L2-O2… en commits posteriores.

## Fase L2-O2-0 — Diseño (esta parada)

PBI v1.2.0 Filtro A. Cascada persist_ref actualizada.

## Fase L2-O2-1 — Handler + despacho + tests (CA2, CA3, CA4, CA7)

Módulo nativo + `run_from_event` en `dispatch_subscriber`. Reutilizar compositor. JSON core (no DA-2): action en lugar de tool.

```text
cd SddIA && cargo test -p execute-process --lib -- notify_humanized
```

## Fase L2-O2-2 — Forja action (DA-2)

Prefijo RAW. Topología `objectives.md` ya en rama.

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-notify-humanized-pr-merged.json
```

`entity_class: action` `create`. Prohibido Write sobre `SddIA/actions/`. Si Cerbero revoked: relay lab ola 1, no mutación manual.

## Fase L2-O2-3 — Clase evento § Suscripciones (CA6)

EM `update` + `markdown_body_replacements` sobre `pull-request-merged` (uuid inmutable). Coverage lo sella `emit-domain-mutation`.

## Fase L2-O2-4 — Evolution + docs ejecución

`sddia-qa evolution-register`. `implementation.md` / `execution.md` ola 2. Gate evolution si toca `directories.evolution`.

## Fase L2-O2-5 — Push al PR #262

Mismo PR. No DCC anidado con rama `HEAD`. No `accept-pr` hasta laudo sobre fractura DCC.
