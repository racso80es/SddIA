---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
updated: "2026-08-18"
process: feature
branch_name: feat/kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
status: implementing
agent: tekton
execution_id: "fa4dde03-a0ec-426f-ade7-850246ba7575"
---

# Implementación — kalma2-mvp-sensorial-email

## T0

| Ítem | Estado |
|------|--------|
| Rama `feat/kalma2-mvp-sensorial-email` | hecho (`fa4dde03-…`) |
| Cascada 01A (`objectives`/`clarify`/`spec`/`plan`) | hecho |
| Dossier padre intacto | hecho |

## T1 [PATTERN-b6a9ed14-3a0d-4f5b-8444-d1b867335daf]

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/library/codexes/codex-contract.md` | v1.2.0; `dlt` opcional + invariantes §1.1 |
| `SddIA/core/cumulo.paths.json` | v1.6.3; `process_domain_roots` += packing Kalma2 |

Los 4 códices preexistentes carecen de `dlt` (grep 0). Retrocompat OK.

## T2

| Entidad | UUID | Sello |
|---------|------|-------|
| `email-triage-matrix` | `3d8c7e09-6d98-422d-909f-5b233ba7fcf2` | `Domain_Entity_Created` `882d5e03-ebf3-45d3-87be-b0f22bb68da7` |
| `codex-kalma2-assistant` | `c43544f3-c557-4cc3-8a03-7175282f2c88` | `Domain_Entity_Created` `f1101aa0-5042-42d1-899b-e4a60973d06a` |

G2: `canonical_hash == hash_signature` (`sha256:01738b6c…`); `token_id: null`; `mint_status: pre-mint`; `composition[].norm` resuelve a fichero existente.

## T3

| Entidad | UUID | Notas |
|---------|------|-------|
| `email-received` | `574fe330-137f-4f3a-b72d-dba189c6c406` | `Email_Received` → `email-triage-gateway` |
| `email-triaged` | `6a4b0e9a-42e1-425c-8a16-9344eae4f246` | registro `[]` (spec §9.2); índice domain = 22 |

## T4

| Artefacto | UUID / sello |
|-----------|----------------|
| `email-watcher.md` | `773a11e7-3a42-4eba-a383-79dd6ef8c263` · hash `sha256:1c66d5e5…` |
| Crate `SddIA/daemons/email-watcher/` | IMAP 2.4 + `BODY.PEEK[]` + watermark post-emisión |
| Template systemd | `Restart=always` `RestartSec=5` `WorkingDirectory=%f` |

F-01: `entity-manager` no declara `daemon`; forja de definición in-ciclo + UUID `crypto-broker`.

## T5

| Entidad | UUID |
|---------|------|
| `email-triage-gateway` | `9cb9a63a-bb86-4b97-8a75-4dac2f2cb5ce` |
| `agenda-manager` | `feb7314d-b86d-4653-a876-507c824ec9e2` |

Handler nativo `engine/handlers/email_triage.rs`. Binding `agenda:persist` en `capability-bindings.md` v1.5.0.

Post-purge WUI: Emision escribe `{eda_instance.proofs}/email-triaged/{event_id}.json`; `kalma2-bridge` proyecta el testigo.

## T9a

Ver `execution.md`. Lab IMAP/systemd pendiente. No `delivery-close-cycle`.

## T10 · Post-auditoría IMAP (A-01…A-05)

| ID | Cambio | Artefacto |
|----|--------|-----------|
| A-01 | Ventana inicial 60 días (`SINCE`, no `ALL`) | `email-watcher/src/main.rs` · `SDDIA_EMAIL_INITIAL_LOOKBACK_DAYS` |
| A-02 | Recuperación lock huérfano | `sddia-daemon-runtime/src/lib.rs` · cleanup `start-sddia.sh` |
| A-03/A-05 | Ignición unificada | `start-sddia.sh` · `start-sddia.md` v1.2.2 · `SddIA/scripts/daemons/email-watcher.sh` |
| A-04 | Bóveda instancia | `.SddIA/.dev/.env` (manual Racso) |
| A-06 | Catch-up bloquea correo nuevo | `UNSEEN` prioritario + `SDDIA_EMAIL_MAX_UIDS_PER_POLL` + watermark contiguo |
| — | Template systemd bóveda instancia | `EnvironmentFile=-%f/.SddIA/.dev/.env` |

## Fricciones

| ID | Hecho | Contención |
|----|-------|------------|
| F-01 | Sin clase `daemon` en entity-manager | Definición in-ciclo |
| F-02 | `codex-contract` no es `domain-codex` | T1 in-ciclo |
| F-03 | Genoma no se duplica por ola | Dossier padre SSOT |
| F-04 | `event-bus-audit` cortaba dead-letter en byte 120 (panic UTF-8 / `—`) | Parche in-ciclo `utf8_prefix`; uuid tool intacto. Unit `utf8_prefix_does_not_split_emdash`. |
| F-05 | `Email_Triaged: []` + `purge_after` borra el JSON de domain | Testigo durable en `eda_instance.proofs` (homólogo PEC) |
| F-06 | Panic UTF-8 en `header_value` (`Precedence:` sobre bytes inválidos) | Parche in-ciclo: comparación vía `as_bytes()` |
| F-07 | `route-domain` / `route-domain-event` exigían `branch` a `email-triage-gateway` | Rama dedicada + `event_file_path` en `dispatch_subscriber` |
