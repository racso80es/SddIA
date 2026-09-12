---
feature_name: email-digest-preference-reply
created: "2026-09-12"
process: feature
phases:
  - digest-tokens-markup
  - reply-handler-eda
  - entity-manager-genome
  - exempt-e2e-evolution-docs
  - dcc-pr-ci-accept
branch_name: feat/email-digest-preference-reply
persist_ref: docs/features/email-digest-preference-reply
pbi_ref: docs/todos/pending/[OPERATIVO] Réplica del digest de ruido → preferencias.md
document_id: PBI-EMAIL-DIGEST-PREFERENCE-REPLY
uuid: "bba79183-b76b-4e05-aa02-5adf1d09b500"
execution_id: "b6930240-b8a7-440a-ab0d-f9f853b1855a"
---

# Plan — email-digest-preference-reply

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución (L1–L5) en el mismo ciclo hasta PR verde y `accept-pr`.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. Semilla `.tmp/feature-email-digest-preference-reply.json`. `execution_id` `b6930240-b8a7-440a-ab0d-f9f853b1855a`.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI v1.2.0 en el mismo commit. **Stop aquí.** Prohibido mutar `SddIA/engine/**` o genoma en L0.

## Fase L1 — Tokens y botonera (CA-1, CA-2, CA-3)

`email_noise_digest.rs`:

- `canonical_subject_key_from_addr` sobre `row.key`.
- Token 8/12 hex; mapa `tokens`.
- Mensaje numerado + pie CTA.
- `reply_markup` 2 botones/fila K.
- `save_state` preserva tokens en skip/vacío; reemplaza en notify.
- Extender `default_notify` y mocks.

```text
cd SddIA && cargo test -p execute-process --lib -- email_noise_digest
```

Core ∉ DA-2.

## Fase L2 — Handler réplica + EDA (CA-4…CA-8)

- `email_digest_preference_reply.rs`.
- `pub mod` + `canonical == "email-digest-preference-reply"`.
- Allowlist `event_file_path` en `route_domain_core.rs`.
- `event-domain-subscriptions.json` (core, no EM).
- Tests: foreign-ns, token miss, max emit, mute value, ign cero emit, PII ausente.

```text
cd SddIA && cargo test -p execute-process --lib -- email_digest_preference_reply
```

## Fase L3 — Genoma (CA-10)

Prefijo RAW. Topología `objectives.md` ya en rama.

1. EM create `email-digest-preference-reply` (flags spec §7). Verificar artefacto **solo** bajo root kalma2.
2. EM update códice membership + cuerpo.
3. EM update `email-noise-digest` (markup en fase notificación).

Prohibido `Write`/`StrReplace` sobre `SddIA/library/codexes/` y process de dominio.

## Fase L4 — Lazo P-EXEMPT-C + evolution + docs (CA-9, CA-11 local)

Test integración tmp: digest `noreply@` → callback max → ingest → `Email_Received` → Triaje-C skipped `P-EXEMPT-C`.

`sddia-qa evolution-register` ligando UUID proceso/códice + PBI. `implementation.md` + `execution.md`.

## Fase L5 — Cierre documental, DCC, CI, accept-pr

1. PBI → `docs/todos/done/` + `validacion.md` con `pbi_archived: true`. CA-CI = `PENDIENTE-CI` hasta run verde; `global` no APTO hasta entonces.
2. `./sddia-run.sh --process delivery-close-cycle` (sin skip).
3. Un log de checks del PR. Rojo → parche local + un push (DA-6). Verde → `run_id` en `validacion.md` + `global: APTO` + `accept-pr`.
