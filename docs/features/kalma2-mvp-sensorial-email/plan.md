---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
process: feature
phases: "T0-topologia,T1-ratificacion-ssot,T2-ley-y-codice,T3-eventos,T4-centinela,T5-triaje,T9a-aduana-sensorial"
branch_name: feat/kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
executor: tekton
gates: "G0-G5,G9a"
dossier_ref: docs/features/kalma2-mvp-paciente-0
execution_id: "fa4dde03-a0ec-426f-ade7-850246ba7575"
---

# Plan Dédalo — Ola A sensorial

Orden de forja: **norma → códice → eventos → centinela → proceso**. Entidades vía `entity-manager` salvo daemon (`daemon-creator`, F-01). Tras acuse CLI: DA-5.

## Patrones aplicados

| UUID | Nombre | Dónde |
|------|--------|-------|
| `b6a9ed14-3a0d-4f5b-8444-d1b867335daf` | ssot-paths-resolution | T1 R-02; toda ruta vía Cúmulo |

## T0 · Topología

- [x] Rama `feat/kalma2-mvp-sensorial-email` (`execution_id` `fa4dde03-…`)
- [x] `persist_ref` + `objectives.md`
- [x] Cascada 01A: `clarify.md`, `spec.md`, `plan.md` (esta ejecución)
- [x] Dossier padre presente (`clarify`/`spec`/`plan`)

**G0:** patrón documental v1.2.1. Sin él, RAW bloquea genoma (DA-4).

## T1 · SSOT ratificado

- [x] R-01 `codex-contract` → v1.2.0, `dlt` opcional
- [x] R-02 `process_domain_roots` += packing Kalma2
- [x] Los 4 códices preexistentes validan sin `dlt`

**G1:** retrocompat + jurisdicción descubrible. Bloqueante para T2.

## T2 · Ley y activo

- [x] `email-triage-matrix` (padre §5, 5 secciones + Restricciones Duras) — uuid `3d8c7e09-6d98-422d-909f-5b233ba7fcf2`
- [x] `codex-kalma2-assistant` con UUID real de la norma, `process_membership: [email-triage-gateway]`, `dlt` pre-mint — uuid `c43544f3-c557-4cc3-8a03-7175282f2c88`
- [x] Filas en índices de normas y códices

**G2:** `canonical_hash == hash_signature`; `token_id: null`; `composition[]` resoluble.

## T3 · ECST

- [x] `email-received` / `email-triaged` (padre §3) — `574fe330-…` / `6a4b0e9a-…`
- [x] Suscripción `Email_Received` → `email-triage-gateway`
- [x] Índice `events/domain` (22 clases)

**G3:** catálogo + registro: `Email_Received` suscribe proceso existente; `Email_Triaged: []` (spec §9.2). CLI `event-bus-audit` `emit_kaizen_alert:false` → `exitCode:0`; 0 huérfanos `Email_*`. `Email_Triaged` marca EMPTY_SUBSCRIBERS+PURGE_BLACKHOLE (F-05; testigo durable).

## T4 · Centinela

- [x] Definición `email-watcher.md` in-ciclo (F-01) — uuid `773a11e7-…`
- [x] Cápsula Rust IMAP RO + crate workspace + launcher
- [x] Template systemd `@@SDDIA_CORE_ROOT@@` (`RestartSec=5`)
- [x] `.dev/.env.example` vars `SDDIA_EMAIL_*` sin secretos

**G4:** `rg` sobre `SddIA/daemons/email-watcher/src`: cero `execute-process`, cero `SddIA/`, cero `STORE`/`EXPUNGE`/`MOVE`/`COPY`, cero ruta `/home/`.

## T5 · Triaje

- [x] `email-triage-gateway` bajo packing del códice + `index.md` — `9cb9a63a-…`
- [x] `agenda-manager` + binding `agenda:persist` — `feb7314d-…`
- [x] Triaje-C determinista con early-exit; Clasificacion condicionada; Emision con `decision_path` y coste
- [x] Testigo durable `{eda_instance.proofs}/email-triaged/{event_id}.json` (sobrevive `purge_after`)

**G5:** list-headers → `decision_path: deterministic`, coste en ceros, `Clasificacion` `skipped`; `classification_ran: false`. Unit: verbosidad no concluye Triaje-C.

## T9a · Aduana sensorial

- [x] G5 e2e CLI (`email-triage-gateway` + fixture List-Id)
- [x] WUI: `GET /api/status` proyecta `Email_Triaged` (domain o proof post-purge)
- [x] Idempotencia UID (unit `uids_after` + roundtrip watermark)
- [x] `daemon-heartbeat-audit` sweep: `fractures_emitted: []` (sin lock vivo del centinela)
- [x] `event-bus-audit` CLI `exitCode:0` (F-04 corte UTF-8 parcheado in-ciclo; uuid tool intacto)
- [x] e2e correo IMAP real → WUI (UID 104385 «Kalma2 validación 01A» · proof `5e7e24e0-…`)
- [ ] Heartbeats reales de `email-watcher` en loop continuo (requiere `./start-sddia.sh` reiniciado)
- [ ] Resiliencia SIGKILL &lt;5 s via systemd (lab OS; template `Restart=always` `RestartSec=5`)
- [x] Cierre documental en rama (`validacion.md` APTO · PBI en `docs/todos/done/`)

**G9a:** APTO software + lab IMAP E2E. Defer: SIGKILL systemd formal.

## T10 · Post-auditoría IMAP (A-01…A-05)

- [x] A-01: `SDDIA_EMAIL_INITIAL_LOOKBACK_DAYS` (defecto 60) → IMAP `SINCE` en primer sondeo
- [x] A-02: recuperación lock huérfano (`sddia-daemon-runtime` + `_run_daemon.sh` + cleanup `start-sddia`)
- [x] A-03/A-05: `email-watcher` en `start-sddia.sh` / `start-sddia.md` v1.2.2 (condicional `SDDIA_EMAIL_IMAP_HOST`)
- [x] A-04: bóveda instancia `.SddIA/.dev/.env` (manual Racso)
- [x] A-06: UNSEEN prioritario + cap por sondeo + watermark contiguo
- [x] Template systemd: `EnvironmentFile=-%f/.SddIA/.dev/.env`
- [ ] Backfill histórico >60 días (fuera MVP; ola futura)

## Touchpoints (01A)

Padre plan filas 1–20 + 23 (`agenda:persist` solo) + 30 (evolution en T9a). No tocar 21–22, 24–29.
