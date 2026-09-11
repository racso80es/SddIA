---
feature_name: email-noise-heuristic-digest
created: "2026-09-11"
process: feature
phases:
  - handler-aggregation
  - tests-cursor-telegram
  - entity-manager-genome
  - evolution-and-docs
  - dcc-pr-ci-accept
branch_name: feat/email-noise-heuristic-digest
persist_ref: docs/features/email-noise-heuristic-digest
pbi_ref: docs/todos/pending/[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona).md
document_id: PBI-EMAIL-NOISE-HEURISTIC-DIGEST
uuid: "bd5eab3f-0693-408a-ab04-de8b43a36c54"
execution_id: "8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3"
---

# Plan — email-noise-heuristic-digest

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución (L1–L5) en el mismo ciclo hasta PR verde y `accept-pr`.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. Semilla `.tmp/feature-email-noise-heuristic-digest.json`. `execution_id` `8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3`.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI v1.2.0 en el mismo commit. **Stop aquí.** Prohibido mutar `SddIA/engine/**` o genoma en L0.

## Fase L1 — Handler agregación (CA-1, CA-2, CA-3, CA-9)

`SddIA/engine/execute-process/src/engine/handlers/email_noise_digest.rs`:

- Resolver `proofs_root` igual que `email_triage.rs`.
- Parse RFC3339 de `since`/`until`; abort si `since >= until`.
- Filtro cerrado spec §3. `normalize_email_addr`. Ranking spec §4.
- Clamp de cursor leído (escribir estado = L2).
- `pub mod` + dispatch en `engine/mod.rs` (despacho CLI puede quedar L3 si se prefiere verde de tests unitarios sobre `run()` antes del catálogo).

Core ∉ DA-2.

## Fase L2 — Cursor, truncado, Telegram (CA-4…CA-7, CA-9)

- RW `{daemons_instance.state}/email-noise-digest.json`.
- Skip / clamp / vacío / fallo de cápsula.
- Formateador 4000 + omisión. `parse_mode: null`.
- Tests: fixtures tmp (`tempfile`); aserción de `parse_mode` en el JSON enviado (mock de `invoke_capsule_json` o lab-mock outbound).

```text
cd SddIA && cargo test -p execute-process --lib -- email_noise_digest
```

## Fase L3 — Genoma (CA-8, CA-10)

Prefijo RAW. Topología `objectives.md` ya en rama.

1. EM process create `email-noise-digest` con `process_jurisdiction: domain`, `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`, `process_contract_version: 1.4.0`, fases spec §7. Verificar artefacto **solo** bajo ese root e `index.md` de kalma2 (cero fila en `SddIA/process/` ni en software-engineering).
2. EM códice update `codex-kalma2-assistant`: membership + cuerpo.
3. Dispatch `canonical == "email-noise-digest"` si no quedó en L1.

Prohibido `Write`/`StrReplace` sobre `SddIA/library/codexes/` y process de dominio. Coverage = sello EM.

## Fase L4 — Evolution + docs de ejecución

`sddia-qa evolution-register` ligando UUID proceso/códice + PBI. `implementation.md` + `execution.md`.

## Fase L5 — Cierre documental, DCC, CI, accept-pr

1. PBI → `docs/todos/done/` + `validacion.md` con `pbi_archived: true`. CA-CI = `PENDIENTE-CI` hasta run verde; `global` no APTO hasta entonces.
2. `./sddia-run.sh --process delivery-close-cycle` (sin skip).
3. Un log de checks del PR. Rojo → parche local + un push (DA-6). Verde → `run_id` en `validacion.md` + `global: APTO` + `accept-pr`.
