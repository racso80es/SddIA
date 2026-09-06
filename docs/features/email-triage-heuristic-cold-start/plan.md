---
feature_name: email-triage-heuristic-cold-start
created: "2026-09-06"
process: feature
phases:
  - crate-hash-and-handler
  - tests-slice1
  - entity-manager-genome
  - evolution-and-docs
  - dcc-pr-ci-accept
branch_name: feat/email-triage-heuristic-cold-start
persist_ref: docs/features/email-triage-heuristic-cold-start
pbi_ref: docs/todos/pending/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md
slice: 1
document_id: PBI-EMAIL-TRIAGE-HEURISTIC
uuid: "2d939386-db39-44f0-804f-1d5ab6ed78c2"
execution_id: "5b530130-8225-4904-98f0-a894523f9c7e"
---

# Plan — email-triage-heuristic-cold-start

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución (L1–L5) en el mismo ciclo hasta PR verde y `accept-pr`.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. Semilla `.tmp/feature-email-triage-heuristic-cold-start.json`.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. Commit de planificación antes de mutar código/genoma.

## Fase L1 — Crate + handler (CA1, CA2, CA3, CA5, CA6, CA9, CA10, CA11)

`user-preference-core`: `normalize_email_addr` + `canonical_subject_key_from_addr`.

`email_triage.rs`: coreografía spec §2. Reutilizar `query_context_block_with_capsule_fallback`. G5: skip Clasificacion si C concluyó **o** mute cerró. Prompt: concatenar bloque solo si `preferences` no vacío. `matched_rule` `P-MUTE-SENDER` / C-* como hoy. Exención: fase Triaje-C `skipped` reason `P-EXEMPT-C`.

Core ∉ DA-2 (handler + crate).

## Fase L2 — Tests (CA1–CA3, CA5–CA7, CA9–CA11)

```text
cd SddIA && cargo test -p user-preference-core --lib -- canonical_subject
cd SddIA && cargo test -p execute-process --lib -- email_triage
```

Incluir fixture store JSON mínimo (put_revision) para mute/exempt. Aserción negativa IMAP en fuente.

## Fase L3 — Genoma (CA6)

Prefijo RAW. Topología `objectives.md` ya en rama.

1. EM process `email-triage-gateway` update: `process_phases` + `process_version: 1.1.0` + `process_jurisdiction: domain` + `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`. Luego EM replacements del cuerpo (G5 mute, decision_path preference).
2. EM norm `email-triage-matrix` update: `tactical_norm_version: 1.1.0` + friction/hard_constraints (P-EXEMPT-C, P-MUTE-SENDER, preference). UUID `3d8c7e09-…` inmutable.
3. EM event `email-triaged` update + `markdown_body_replacements` sobre decision_path. UUID `6a4b0e9a-…` inmutable.

Prohibido `Write`/`StrReplace` sobre `SddIA/library/norms/`, `SddIA/events/`, process de códice. Coverage lo sella `emit-domain-mutation`.

## Fase L4 — Evolution + docs de ejecución

`sddia-qa evolution-register` (o equivalente canónico) ligando UUID proceso/norma/evento + PBI. `implementation.md` + `execution.md`.

## Fase L5 — Cierre documental, DCC, CI, accept-pr

1. PBI → `docs/todos/done/` + `validacion.md` con `pbi_archived: true`. CA-CI = `PENDIENTE-CI` hasta run verde; `global` no APTO hasta entonces.
2. `./sddia-run.sh --process delivery-close-cycle` (sin skip).
3. Un log de checks del PR. Rojo → parche local + un push (DA-6). Verde → `run_id` en `validacion.md` + `global: APTO` + `accept-pr`.
