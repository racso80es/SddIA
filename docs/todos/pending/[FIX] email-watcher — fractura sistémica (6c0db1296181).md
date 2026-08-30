---
document_id: PBI-FIX-FRACTURE-6c0db1296181
title: "[FIX] email-watcher — fractura sistémica"
format: markdown
version: "1.1.0"
created: "2026-08-30"
updated: "2026-08-30"
status: "abierto"
priority: alta
process: bug-fix
type: bug-fix
fracture_hash: 6c0db1296181
fracture_process: email-watcher
incident_ref: "System_Fracture_Detected — 6c0db1296181"
refined: true
source_audit: "Journal systemd user + kernel 2026-08-30; heartbeat-audit.json; lock/side-channel; phagocyte ledger; handlers/daemon_heartbeat.rs; email-watcher/src/main.rs; enrich_fracture_pbi_kaizen.rs"
review_notes: "Refinado v1.1.0 — el stub Cúmulo/Mayeuta contenía ontología falsa (proceso vs daemon), veredicto process_fix de fallback y mandato de colapso de entrega. Laudo A vs B explícito; causa estructural = starvation de latido en poll_once sin keepalive (paridad telegram/event/github/sweeper)."
suggested_branch: fix/email-watcher-heartbeat-keepalive
persist_ref_suggested: docs/fixes/email-watcher-heartbeat-keepalive
related_pbis:
  - id: PBI-FIX-FRACTURE-fe227c6e32d3
    rol: "Antecesor 1532 ciclos (2026-08-19); cerrado laudo B ola documental. No es el mismo modo: aquí missed_cycles=3 con host despierto."
  - id: PBI-FIX-FRACTURE-521b4f60d746
    rol: "Antecesor ignición/lock huérfano; resuelto PR #182. No aplica a este sello."
  - id: PBI-OPER-LATIDO-ONTOLOGICO-001
    rol: "Hermano: drift de emisores en daemon-heartbeat.md. No es causa de este sello (el centinela sí fue auditado)."
architectural_constraints:
  - A-NO-MUTAR-UMBRALES-PARA-SILENCIAR
  - A-KEEPALIVE-PARIDAD-CENTINELAS
  - A-FRACTURE-HASH-INMUTABLE
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/process/phagocyte-recovered-fracture-pbis.md
  - SddIA/daemons/email-watcher.md
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/spec.md
  - docs/fixes/centinelas-heartbeat-fracture/spec.md
  - docs/fixes/centinelas-fracture-ola-20260819/spec.md
---

# [FIX] email-watcher — fractura sistémica

> **Refinamiento v1.1.0.** El cuerpo auto-generado (Cúmulo + Mayeuta) es un sello de incidente válido y un diagnóstico **inválido**. Las afirmaciones descartadas quedan en §7.

## 1. Identidad del sello (no tocar)

| Campo | Valor | Notas de refinamiento |
|-------|--------|------------------------|
| `fracture_hash` | `6c0db1296181` | SHA-256[:12] de la traza; verificado. Inmutable para el resolutor. |
| `fracture_process` | `email-watcher` | **No es un proceso SddIA.** Es el `daemon_id` que Argos pone en `payload.process_name` (`emit_system_fracture`). El proceso que corrió es `daemon-heartbeat-audit`. |
| Emisor | `argos` | Correcto. |
| Acción intentada | `daemon-heartbeat-audit` | Correcto. |
| Traza | `Centinela email-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-30T07:51:47Z` | Literal del handler. `missed_cycles=3` = umbral mínimo. Ventana ≈ **90 s** (`3 × heartbeat_interval_seconds=30`). |

El resolutor y el fagoctio leen `fracture_hash` / `fracture_process` / la traza. Prohibido alterar esos tres campos.

## 2. Hechos verificados (2026-08-30)

Zona horaria del journal: CEST = UTC+2. `last_heartbeat` de la traza = **09:51:47 CEST**.

| Hecho | Evidencia |
|-------|-----------|
| El auditor **solo emite** si el PID del lock está vivo | `audit_running_daemon`: `pid_alive` false → `Ok(None)` (sin fractura). Esto no es muerte de proceso; es **omisión de latido con proceso vivo**. |
| Host **no** estaba suspendido en la ventana | `event-watcher` escribe en journal de forma continua 09:49–09:54 CEST. `journalctl` 09:45–10:05: **cero** líneas `suspend`/`sleep`/`lid`. |
| Instancia implicada | systemd user `sddia-email-watcher@home-racso-Proyectos-SddIA.service`, PID **5614**, arranque 06:37:49 CEST (04:37:49Z). Journal de ese PID: una sola línea (`lock huérfano pid=176452`). **Cero** errores IMAP en esa vida. |
| CPU de esa unidad hasta el stop | 11,163 s de CPU en ~3 h 44 min de wall → no es bucle ocupado; compatible con bloqueo I/O o sleep del poll. |
| Recuperación previa al reboot | `.SddIA/observability/ecosystem-health.json` `fused_at=2026-08-30T08:19:36Z` (10:19:36 CEST): `email-watcher` `missed_cycles=0`, `reason=heartbeat_ok`. |
| Stop posterior (no causa) | 10:21:32 CEST: shutdown de **todos** los centinelas (`Reached target shutdown.target`). Boot `-2` termina 10:21:43. Irrelevante para el sello de 09:51. |
| Instancia actual (post-boots) | Lock `started_at=2026-08-30T15:42:06Z`, PID 7064, `classification=healthy`, `missed_cycles=0`. Fagoctio: `trace_before_lock` (traza 07:51:47Z **anterior** al lock vigente). |
| Watermark IMAP | `.SddIA/daemons/state/email-watcher.json` `updated_at=2026-08-30T04:37:50Z` (arranque de 5614). Ausencia de UIDs nuevos ≠ daemon muerto. |

## 3. Qué hizo mal Mayeuta

`analyze_fracture_kaizen` no tiene tokens para `omitió`, `ciclos`, `heartbeat` ni `Daemon_Heartbeat`. La traza no matchea `timeout`/`block`/`abort`/`failed`/`colaps`. Cae al **fallback**:

- «Causa raíz no clasificada… requiere laudo humano» — tautología, no análisis.
- Veredicto `process_fix` + «Auditar proceso `email-watcher`» — **alucinación ontológica**: no existe `SddIA/process/email-watcher.md`. `email-watcher` es centinela (`SddIA/daemons/email-watcher.md`, uuid `773a11e7-3a42-4eba-a383-79dd6ef8c263`).

Ese párrafo **no es un diagnóstico**. Queda archivado como síntoma del clasificador, no como mandato de diseño.

## 4. Causa estructural (código, no conjetura de runtime)

`run_loop` en `email-watcher/src/main.rs`:

1. `poll_once(...)` — IMAP connect/login/examine + `uid_search("ALL")` + search incremental + `UNSEEN` + hasta 50 `BODY.PEEK`. **Sin `tick()` dentro. Sin timeout de socket explícito.**
2. Luego `centinela.tick()` y un wait de `SDDIA_EMAIL_POLL_SECONDS` (default 60) con tick cada 1 s.

`DaemonRuntime::tick` rate-limita a `heartbeat_interval_seconds` (30). Si `poll_once` bloquea ≥90 s, Argos ve `missed_cycles>=3` con PID vivo.

Paridad rota: `telegram-watcher`, `event-watcher`, `github-bridge-watcher` y `event-sweeper` ya tienen hilo keepalive `tick()` cada 10 s (`docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e`, `docs/fixes/centinelas-heartbeat-fracture`). **`email-watcher` no** (`thread::spawn` / `HEARTBEAT_TICK` ausentes). `iota-publish-relay` tampoco; jurisdicción ajena a este PBI.

Esto basta para explicar un sello de **exactamente 3 ciclos** en un host despierto con hermanos latiendo. No prueba que *este* poll durara 90 s (no hay log IMAP de 5614), pero es el único mecanismo local coherente con §2.

## 5. Discriminación A vs B

| Hipótesis | ¿Sostiene este sello? | Lectura |
|-----------|------------------------|---------|
| **(A) Starvation de latido en `poll_once`** | Sí | PID vivo + umbral mínimo + hermanos activos + ausencia de keepalive. Precedente idéntico en telegram-watcher. |
| **(B) Deuda documental / downtime histórico** | No para *este* hash | Ola 20260819 (fe227c6e32d3) era 1532 ciclos / host caído. Aquí 3 ciclos / host despierto. Archivar como B **sin** keepalive deja el hueco. |
| Host suspend (`skew>=120`) | Refutado en esta ventana | Journal + event-watcher continuo. Además `suspend_skew_seconds=120` > 90 s de umbral: un sleep 90–119 s *sí* fracturaría sin clasificar `host_suspend`; pero no hay evidencia de sleep aquí. |
| Daemon muerto / lock huérfano | Refutado | Auditor exige PID vivo. Lock huérfano es de **arranques posteriores** (5313, 7064), no del sello. |
| Bug watermark IMAP | Fuera | Ciclo `email-watcher-imap-account-watermark` ya cerrado. `last_uid` estable no explica omisión de heartbeat. |
| Drift emisores `daemon-heartbeat.md` | Fuera | El centinela **sí** entra en `audit_staleness` (tiene `{name}.md`). El drift es que emite sin figurar como emisor autorizado; no impidió la fractura. Jurisdicción `PBI-OPER-LATIDO-ONTOLOGICO-001`. |

**Laudo propuesto (Tekton, no sustituye al Vértice Biológico):** **(A) corrección de cápsula daemon** — hilo keepalive en modo continuo, paridad con los otros centinelas. El sello `6c0db1296181` es el síntoma; el trabajo es el hueco de keepalive, no «arreglar el proceso email-watcher».

Fagoctio (`trace_before_lock`) solo dice que la **instancia actual** no es la del sello. Autoriza archivo documental *después* de decidir A o B; no es prueba de que no haya defecto de genoma/delivery.

## 6. Alcance del fix (si laudo A)

### Dentro

- Hilo keepalive `centinela.tick()` cada 10 s en `run_loop` continuo, con `Arc<Mutex<DaemonRuntime>>` (mismo patrón que telegram-watcher). Prohibido en `--once`.
- Tests: build `-p email-watcher`; `--once` sin alterar contrato JSON-IO; keepalive no arranca en `--once`.
- Documentar en `persist_ref` (`spec.md` / `implementation.md` / `execution.md` / `validacion.md`).
- Archivar este PBI en el **mismo** PR (cierre documental en rama).

### Fuera

- Mutar `missed_cycles_threshold` o `suspend_skew_seconds` para silenciar el sello.
- Forjar `SddIA/process/email-watcher.md`.
- Timeout IMAP / sustituir `uid_search("ALL")` — mejora legítima, **PBI aparte** si se aborda; no es el CA de keepalive.
- Aplicar `SDDIA_PHAGOCYTE_APPLY=1` como «cierre» de este hash sin keepalive.
- `iota-publish-relay` keepalive; umbrales Argos; contrato de emisores `daemon-heartbeat.md`.

### Residual observado (no bloquear)

El ledger `.SddIA/daemons/state/phagocytosed-fractures.json` acumula decenas de entradas dry-run del mismo `document_id` en cada sweep sano. El fagoctio no mueve el fichero sin `apply`. Deuda de ledger, no de este centinela.

## 7. Afirmaciones descartadas del stub v1.0.0

| Afirmación del stub | Verdad |
|---------------------|--------|
| «Proceso `email-watcher`» | Centinela. Proceso oficial = `daemon-heartbeat-audit`. |
| «Colapso» + mandato de entrega Kintsugi como si Tekton hubiera evadido un proceso | No hay entrega en vuelo. El mandato *bypass raw* sigue vigente **si** se abre `bug-fix`; no describe este incidente. |
| Veredicto `process_fix` / «Auditar proceso, acción y emisor» | Fallback Mayeuta sin tokens de heartbeat. Acción y emisor ya son correctos y no son la causa. |
| «Causa raíz no clasificada automáticamente» | El clasificador no cubre esta clase de traza. La causa estructural sí es clasificable por inspección de `run_loop`. |

## 8. Criterio de cierre

**Si laudo A (recomendado)**

- [ ] Keepalive en `email-watcher` modo continuo, paridad 10 s
- [ ] `--once` sin hilo keepalive; tests del crate verdes
- [ ] `validacion.md` global `APTO`, `pbi_archived: true`
- [ ] Este TODO en `docs/todos/done/` en la rama del PR

**Si laudo B (Vértice Biológico desestima A)**

- [ ] Registrar por escrito por qué el hueco de keepalive no se cierra
- [ ] Archivo documental (fagoctio apply o ola) **sin** mutar umbrales
- [ ] Este TODO en `docs/todos/done/`

Prohibido declarar Done con el sello abierto en `pending/` y el keepalive aún ausente si el laudo es A.
