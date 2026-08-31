---
feature_name: email-watcher-heartbeat-keepalive
created: "2026-08-30"
process: bug-fix
base: main
scope: email-watcher
version_spec: "1.0.0"
branch_name: fix/email-watcher-heartbeat-keepalive
persist_ref: docs/fixes/email-watcher-heartbeat-keepalive
pbi_ref: docs/todos/pending/[FIX] email-watcher — fractura sistémica (6c0db1296181).md
document_id: PBI-FIX-FRACTURE-6c0db1296181
execution_id: "9dbcfea6-4df8-47ac-873a-cf9bce846929"
fracture_hash: 6c0db1296181
incident_ref: "System_Fracture_Detected — 6c0db1296181"
laudo: A
---

# Especificación — Keepalive heartbeat `email-watcher`

## Diagnóstico (causa raíz)

| Síntoma | Evidencia |
|---------|-----------|
| `System_Fracture_Detected` `6c0db1296181` | Traza: omitió 3 ciclos; `last_heartbeat=2026-08-30T07:51:47Z`; umbral=3 ≈ 90 s |
| PID vivo + host despierto | Auditor exige `pid_alive`; journal sin suspend; hermanos latiendo |
| Starvation en `poll_once` | IMAP connect/login/examine/`uid_search`/`BODY.PEEK` sin `tick()` ni timeout de socket |
| Paridad rota | `telegram-watcher`, `event-watcher`, `github-bridge-watcher`, `event-sweeper` ya tienen keepalive 10 s; **`email-watcher` no** |

`run_loop` solo llama `centinela.tick()` **después** de `poll_once` y en el wait de `SDDIA_EMAIL_POLL_SECONDS`. Si el poll bloquea ≥90 s, Argos ve `missed_cycles>=3` con proceso vivo.

**Laudo A (este ciclo):** corrección de cápsula daemon — hilo keepalive. El sello es el síntoma; el trabajo es el hueco de keepalive. No existe `SddIA/process/email-watcher.md`; `email-watcher` es centinela (`directories.daemons`).

## Corrección

### H1 — Keepalive asíncrono (único CA de código)

En `SddIA/daemons/email-watcher/src/main.rs`, modo continuo (`run_loop`):

1. Constantes `HEARTBEAT_TICK_SECONDS = 10` y `HEARTBEAT_EMIT_FAIL_BUDGET = 5` (paridad telegram/event).
2. Tras `bootstrap`, envolver `DaemonRuntime` en `Arc<Mutex<DaemonRuntime>>`.
3. `spawn_heartbeat_worker`: hilo que `lock` → `tick(&top)` cada 10 s; presupuesto de fallos → panic termodinámico (mismo contrato que hermanos).
4. En el bucle principal: adquirir lock, `poll_once`, `tick` post-poll, wait con ticks; liberar lock antes/durante sleeps largos según patrón telegram (`drop` del guard antes del sleep del poll interval si el lock se sostiene solo alrededor de trabajo IMAP).
5. `shutdown` al salir del bucle (con lock).
6. Log de arranque: keepalive cada `{HEARTBEAT_TICK_SECONDS}s`.

### H2 — `--once` intacto

Rama `--once`: **sin** `thread::spawn` / keepalive. Contrato JSON-IO (`once_envelope`) inalterado. Un solo `bootstrap` → `poll_once` → `tick` → `shutdown`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `cargo build -p email-watcher` OK |
| CA2 | `cargo test -p email-watcher` verdes (tests existentes + cualquier test de no-arranque keepalive en `--once` si se añade) |
| CA3 | Modo continuo: hilo keepalive activo; `tick()` cada ≤10 s aunque `poll_once` bloquee |
| CA4 | `--once`: no arranca hilo keepalive; envelope JSON-IO sin cambio de schema |
| CA5 | No mutar `missed_cycles_threshold` / `suspend_skew_seconds` / umbrales Argos |
| CA6 | `validacion.md` global `APTO`, `pbi_archived: true`; PBI en `docs/todos/done/` en el mismo PR |

## Alcance prohibido

| Prohibido | Motivo |
|-----------|--------|
| Mutar umbrales para silenciar el sello | A-NO-MUTAR-UMBRALES-PARA-SILENCIAR |
| Forjar `SddIA/process/email-watcher.md` | Ontología: es daemon, no proceso |
| Timeout IMAP / sustituir `uid_search("ALL")` | Mejora legítima; PBI aparte; no es el CA de keepalive |
| Keepalive en `iota-publish-relay` | Jurisdicción ajena |
| Mutar `daemon-heartbeat.md` emisores | `PBI-OPER-LATIDO-ONTOLOGICO-001` |
| `SDDIA_PHAGOCYTE_APPLY=1` como cierre sin keepalive | No cierra el hueco estructural |
| Alterar `fracture_hash` / traza / `fracture_process` del PBI | A-FRACTURE-HASH-INMUTABLE |
| Mutar genoma protegido (`process/`, `norms/`, `agents/`, …) | DA-2; este fix toca solo cápsula bajo `directories.daemons` |

## Referencias de paridad

- `docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/spec.md`
- `docs/fixes/centinelas-heartbeat-fracture/spec.md`
- Implementación canónica: `SddIA/daemons/telegram-watcher/src/main.rs` (`spawn_heartbeat_worker`)

## Corte de esta fase (Dedalo)

Diseño: `spec.md` + `plan.md`. Sin parche Rust en esta fase. Ejecución = Tekton bajo el mismo `persist_ref`.
