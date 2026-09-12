---
feature_name: email-digest-preference-reply
created: "2026-09-12"
process: feature
branch_name: feat/email-digest-preference-reply
persist_ref: docs/features/email-digest-preference-reply
execution_id: "b6930240-b8a7-440a-ab0d-f9f853b1855a"
items_applied:
  - digest-tokens-markup
  - reply-handler-eda
  - entity-manager-process
  - entity-manager-digest-update
  - entity-manager-codex
  - exempt-e2e-tests
---

# Ejecución — email-digest-preference-reply

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 ./sddia-run.sh --process feature`

`execution_id` `b6930240-b8a7-440a-ab0d-f9f853b1855a`. Commit planificación `0381635`.

## Código (L1–L2)

Handler digest: tokens + botonera. Handler réplica + EDA.

```text
cd SddIA && cargo test -p execute-process --lib -- email_noise_digest email_digest_preference_reply
# 15 passed
```

Incluye `max_loop_triggers_p_exempt_c`.

## Genoma (L3)

| Entidad | EM execution_id | Sello | hash_new |
|---------|-----------------|-------|----------|
| process `email-digest-preference-reply` 1.0.0 | `440b7788-6a4c-449f-a185-3feca494e12c` | Domain_Entity_Created `91d7286f-9946-4f3e-ad8b-fd7b226c9737` | `sha256:fdfabc7ca44dddb2c30c6dba16fb182e30edf3f81c0caaf4a4eb2a85e8fa4132` |
| process `email-noise-digest` 1.0.1 | `c8ca21cd-d4fb-4baa-bac7-a767c40b3bd5` | Domain_Entity_Updated `8581c387-0a0c-4cf0-adcc-7c82e2a35ff7` | `sha256:c8ed5433e9c7d800857278ab5bd51e144629ac3d519eacd88c503001ce167af2` |
| códice `codex-kalma2-assistant` 1.0.2 | `d2256a9f-b4f9-41b5-94fa-82949bfc3f22` | Domain_Entity_Updated (idempotente reuso) | `sha256:e3a9e45e0a030fb359f5343a9505acf86488ba71204e4d8c9ec69b86921cfdf9` |

Root: `SddIA/library/codexes/codex-kalma2-assistant/process`. UUID proceso réplica `c77f96ad-ad01-466e-87cf-950b1976555b`.

## Tests

15/15 `execute-process --lib` filtros `email_noise_digest` + `email_digest_preference_reply`.

## Evolution

`sddia-qa evolution-register` → `7e4c1a90-2b6d-4f18-9c3a-5d8e0b1a2476` (`exitCode` 0, `alta`).
