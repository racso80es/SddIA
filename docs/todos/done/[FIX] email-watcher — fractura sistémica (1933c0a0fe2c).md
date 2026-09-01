---
document_id: PBI-FIX-FRACTURE-1933c0a0fe2c
title: "[FIX] email-watcher — fractura sistémica"
format: markdown
version: "1.1.0"
created: "2026-08-31"
updated: "2026-09-01"
status: "cerrado"
closed: "2026-09-01"
fix_ref: docs/fixes/email-watcher-elf-fosil-1933c0a0fe2c
laudo: C
priority: alta
process: bug-fix
type: bug-fix
fracture_hash: 1933c0a0fe2c
fracture_process: email-watcher
incident_ref: "System_Fracture_Detected — 1933c0a0fe2c"
refined: true
source_audit: "Journal systemd user 2026-08-31 09:05–09:20 CEST; lock PID 7064; heartbeat-audit.json 2026-09-01; ELF release/debug mtime; git bee97e23/d3ef9036; main.rs spawn_heartbeat_worker; enrich_fracture_pbi_kaizen.rs cubo heartbeat_starvation; phagocyte predicate"
review_notes: "Refinado v1.1.0 — el sello es válido; el cubo Mayeuta receta keepalive ya cerrado en 6c0db1296181. Causa de ESTE hash = ELF fósil en PID 7064 (release 2026-08-26) + trigger DNS IMAP ~09:09 CEST. No reabrir genoma."
suggested_work: recycle-fossil-elf
persist_ref_suggested: docs/fixes/email-watcher-elf-fosil-1933c0a0fe2c
related_pbis:
  - id: PBI-FIX-FRACTURE-6c0db1296181
    rol: "Padre de genoma: keepalive 10 s mergeado 2026-08-31T09:18:44+02:00 (d3ef9036). Este sello es residual de la instancia no reciclada, no un segundo hueco de código."
  - id: PBI-FIX-FRACTURE-fe227c6e32d3
    rol: "Antecesor 1532 ciclos / host caído (ola 20260819). Modo distinto: aquí missed_cycles=3 con host despierto y hermanos latiendo."
  - id: PBI-FIX-FRACTURE-521b4f60d746
    rol: "Antecesor ignición/lock huérfano; PR #182. Traza de este sello no es lock huérfano."
  - id: PBI-OPER-LATIDO-ONTOLOGICO-001
    rol: "Cita este hash como evidencia del camino 3 (cuelgue con PID vivo). Correcto como síntoma; no prescribe re-forjar keepalive."
architectural_constraints:
  - A-NO-REFORJAR-KEEPALIVE
  - A-NO-MUTAR-UMBRALES-PARA-SILENCIAR
  - A-FRACTURE-HASH-INMUTABLE
  - A-NO-PHAGOCYTE-ESTE-HASH
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/process/phagocyte-recovered-fracture-pbis.md
  - SddIA/daemons/email-watcher.md
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - docs/fixes/email-watcher-heartbeat-keepalive/validacion.md
  - docs/fixes/iota-publish-relay-elf-fosil-r1/implementation.md
---

# [FIX] email-watcher — fractura sistémica

> **Refinamiento v1.1.0.** El sello Cúmulo (`fracture_hash` + traza) es válido. El diagnóstico Mayeuta (cubo `heartbeat_starvation`) es **parcialmente cierto y tácticamente falso**: describe inanición con PID vivo, pero prescribe reintroducir keepalive ya cerrado. Afirmaciones descartadas en §7.

## 1. Identidad del sello (no tocar)

| Campo | Valor | Notas de refinamiento |
|-------|--------|------------------------|
| `fracture_hash` | `1933c0a0fe2c` | SHA-256[:12] de la traza. Inmutable. |
| `fracture_process` | `email-watcher` | **No es un proceso SddIA.** Es el `daemon_id` en `payload.process_name` (`emit_system_fracture`). El proceso que corrió es `daemon-heartbeat-audit`. No existe `SddIA/process/email-watcher.md`. |
| Emisor | `argos` | Correcto. |
| Acción intentada | `daemon-heartbeat-audit` | Correcto. |
| Traza | `Centinela email-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-31T07:10:19Z` | Literal del handler. `missed_cycles=3` = umbral mínimo. Ventana ≈ **90 s** (`3 × heartbeat_interval_seconds=30`). `07:10:19Z` = **09:10:19 CEST**. |

El resolutor y el fagoctio leen `fracture_hash` / `fracture_process` / la traza. Prohibido alterar esos tres campos.

## 2. Hechos verificados (2026-09-01)

Zona horaria journal: CEST = UTC+2.

| Hecho | Evidencia |
|-------|-----------|
| El auditor **solo emite** esta traza si el PID del lock está vivo | `audit_running_daemon`: PID muerto → silencio o traza `lock huérfano`, no `omitió N ciclos`. Esto no es muerte del centinela. |
| Misma instancia que el sello padre | Lock `pid=7064`, `started_at=2026-08-30T15:42:06Z` (17:42:05 CEST). Sigue vivo al 2026-09-01. Es el PID que `6c0db1296181` ya identificó como «instancia actual post-boots». |
| ELF en ejecución | `/home/racso/Proyectos/SddIA/SddIA/target/release/email-watcher`. mtime **2026-08-26 16:33 CEST**. Sin cadena `keepalive heartbeat` en el binario. Fuente `main.rs` mtime **2026-08-31 09:18 CEST**. |
| Keepalive **sí** está en genoma | `spawn_heartbeat_worker` + `HEARTBEAT_TICK_SECONDS=10` en `main.rs`. Commit `bee97e23` 2026-08-31T09:02:02+02:00. Merge `d3ef9036` 09:18:44+02:00. PBI padre archivado; `validacion.md` APTO. |
| ELF debug también fósil | `SddIA/target/debug/email-watcher` mtime **2026-08-28 07:31 CEST**. `_sddia_resolve_daemon_binary` rechazaría ambos en un arranque nuevo (`mtime < fuente`). El proceso actual **no pasó** por ese resolutor tras el merge. |
| Trigger de la ventana | Journal 09:09 CEST: `imap connect: failed to lookup address information: Name or service not known` (PID 7064). Misma ventana: DNS fail en github-bridge (`api.github.com`) y telegram-watcher (`api.telegram.org`). Host **no** suspendido: `event-watcher` escribe continuo 09:05+. Kernel: cero `suspend`/`sleep`/`lid` 08:50–09:25. |
| Hermanos con keepalive no sellaron latido | github-bridge y telegram-watcher ya tienen hilo keepalive (PRs #100/#98). Hay error DNS, no PBI-FIX-FRACTURE gemelo de 3 ciclos en pending para esos centinelas. |
| Recuperación posterior (no cierre) | `heartbeat-audit.json` 2026-09-01T06:49:56Z: `email-watcher` `classification=healthy`, `missed_cycles=0`. Watermark IMAP `updated_at=2026-08-31T23:58:45Z`. El PID fósil volvió a latir; el hueco de entrega (ELF ≠ fuente) sigue. |
| Fagoctio **no** aplica | Predicado `lock.started_at > last_heartbeat`. Aquí `2026-08-30T15:42:06Z` **<** `2026-08-31T07:10:19Z`. Ledger sin entrada `1933c0a0fe2c`. Autorizar `SDDIA_PHAGOCYTE_APPLY=1` no movería este PBI. |

Cronología CEST (31 ago):

```text
09:02  keepalive commit bee97e23 (fuente; ELF en disco intacto)
09:09  blip DNS (IMAP + GitHub + Telegram)
09:10  last_heartbeat del sello (07:10:19Z)
~09:12 fractura missed_cycles=3 (umbral)
09:15  Cúmulo materializa este PBI (Mayeuta v1 = fallback process_fix)
09:18  merge keepalive d3ef9036 — PID 7064 no se recicla
20:39  Mayeuta re-enriquece con cubo heartbeat_starvation (receta keepalive)
```

## 3. Qué hizo mal Mayeuta

Dos capas, no una.

**v1.0.0 inicial (09:15, pre-cubo / cubo no aplicado):** fallback `process_fix` — «Auditar proceso `email-watcher`». Alucinación ontológica: no hay proceso oficial con ese nombre.

**v1.0.0 re-enrich (20:39, cubo `is_heartbeat_starvation_trace`):** tokens de traza correctos (`Centinela` + `omitió` + `ciclos consecutivos de Daemon_Heartbeat`). Diagnóstico de inanición con PID vivo: **correcto**. `process_name` = `daemon_id`: **correcto**. Propuesta «Emitir latido en worker / paridad keepalive»: **receta genérica del cubo**, ciega a:

1. Keepalive ya mergeado 11 h antes del re-enrich.
2. El ELF que fracturó es anterior al parche.
3. El trigger empírico es DNS en `imap::connect`, no ausencia de `spawn_heartbeat_worker` en el árbol git.

`analyze_fracture_kaizen` no inspecciona `main.rs` ni mtime ELF. El cubo no distingue «hueco de genoma» de «hueco de entrega». Esa propuesta **no es mandato de diseño** para este hash.

## 4. Causa estructural (este sello)

Cadena causal, no conjetura:

1. `poll_once` → `imap::connect` bloquea el hilo del `run_loop` (DNS / TCP). En el ELF de 2026-08-26 **no hay** worker `tick()` paralelo.
2. Argos ve `missed_cycles>=3` con PID vivo → traza canónica.
3. El parche que corta esa cadena (`spawn_heartbeat_worker`, mutex no retenido durante I/O) está en **fuente** desde 09:02/09:18 CEST y **no** en el proceso 7064.
4. El blip DNS ~09:09 es el **disparador**. Los centinelas hermanos con keepalive loguean el mismo DNS y siguen latiendo.

Esto explica un sello de **exactamente 3 ciclos** en host despierto, misma instancia que `6c0db1296181`, minutos antes del merge del keepalive y sin reciclar el ELF.

No prueba que *todo* `uid_search("ALL")` sea el culpable de *este* hash (el log de 09:09 es `imap connect` / DNS). Timeout IMAP sigue siendo mejora legítima; **no** es el CA de este PBI.

## 5. Discriminación A vs B vs C

| Hipótesis | ¿Sostiene este sello? | Lectura |
|-----------|------------------------|---------|
| **(C) Residual de entrega / ELF fósil** | Sí | Fuente con keepalive; proceso y ambos ELFs anteriores. Trigger DNS. Hermanos keepalive inmunes al mismo blip. |
| **(A) Re-forjar keepalive en `main.rs`** | No | Duplicaría `6c0db1296181`. Tests y `validacion.md` APTO ya cubren el genoma. |
| **(B) Deuda documental / downtime histórico** | No como único cierre | No es 1532 ciclos ni host caído. El PID sigue vivo; el fósil sigue exec. Archivar sin reciclar deja el hueco. |
| Host suspend (`skew>=120`) | Refutado | event-watcher continuo; cero líneas kernel suspend; ventana 90 s < 120 s de skew. |
| Daemon muerto / lock huérfano | Refutado | Traza `omitió ciclos`; PID 7064 vivo. |
| Keepalive en runtime falló | No comprobable aún | El binario en ejecución **no contiene** el worker. Solo sería laudo si, *tras* reciclar ELF fresco, reaparece un sello de 3 ciclos. Entonces: PBI **nuevo**. |
| Bug watermark IMAP | Fuera | Ciclo cuenta/watermark cerrado. `last_uid` 5806 no explica omisión de heartbeat. |

**Laudo propuesto (Tekton; no sustituye al Vértice Biológico):** **(C) reciclar instancia** — `cargo build -p email-watcher` (release o el perfil que use el launcher) + restart del centinela para que el ELF vigente pase `_sddia_daemon_elf_fresh_vs_source`. Verificar en el proceso nuevo la cadena de keepalive. **Prohibido** reabrir `spawn_heartbeat_worker` como si no existiera.

No hay entrega `bug-fix` de genoma en vuelo. El mandato *bypass raw* del stub Kintsugi aplica **si** se abre un proceso oficial; reciclar el daemon de lab no es `gh`/`git`/`curl` de cierre.

## 6. Alcance

### Dentro

- Recompilar el crate `email-watcher` de modo que el ELF elegido por `_sddia_resolve_daemon_binary` tenga `mtime ≥` fuente.
- Reciclar PID 7064 (stop/start del launcher vigente: script o unidad systemd). El lock nuevo debe tener `started_at` posterior al reciclo.
- Confirmar: proceso nuevo lista keepalive; `missed_cycles=0` post-arranque; este PBI a `docs/todos/done/` (cierre documental; no exige segundo parche de `main.rs`).
- Si el cierre va en PR: reutilizar `persist_ref` `docs/fixes/email-watcher-heartbeat-keepalive` o nota breve de reciclo; **no** forjar rama `fix/email-watcher-heartbeat-keepalive` de nuevo.

### Fuera

- Mutar `missed_cycles_threshold` o `suspend_skew_seconds`.
- Re-implementar `spawn_heartbeat_worker` / cambiar `HEARTBEAT_TICK_SECONDS`.
- Forjar `SddIA/process/email-watcher.md`.
- Timeout de socket IMAP / sustituir `uid_search("ALL")` — PBI aparte si se aborda.
- `SDDIA_PHAGOCYTE_APPLY=1` como «cierre» de este hash.
- Keepalive de `iota-publish-relay`; contrato de emisores `daemon-heartbeat.md`.

### Residual observado (no bloquear)

El ledger `phagocytosed-fractures.json` (~1 MB) acumula dry-runs. Deuda de ledger, no de este sello.

ELF debug+release ambos fósiles: un `start-sddia.sh` *nuevo* fallaría la aduana hasta recompilar. El proceso huérfano de esa aduana es este PID, no un bug del resolutor.

## 7. Afirmaciones descartadas del stub v1.0.0

| Afirmación | Verdad |
|------------|--------|
| «Proceso `email-watcher`» | Centinela. Proceso oficial = `daemon-heartbeat-audit`. |
| «Colapso» de entrega + bypass raw como si Tekton hubiera evadido un proceso | No hay ciclo `bug-fix` de este hash en vuelo. El sello es auditoría de latido. |
| Veredicto `process_fix` / «Auditar proceso, acción y emisor» (09:15) | Fallback Mayeuta. Acción y emisor son correctos y no son la causa. |
| «Emitir latido en worker / paridad keepalive» (20:39) como trabajo de genoma | Ya entregado en `6c0db1296181`. El trabajo de **este** hash es reciclar el ELF. |
| «Causa raíz no clasificada automáticamente» (09:15) | El clasificador cubre la *clase* de traza; no cubre *entrega fósil*. La causa de este hash sí es clasificable por lock + mtime + journal. |

## 8. Criterio de cierre

**Si laudo C (recomendado)**

- [x] ELF de `email-watcher` con `mtime ≥` fuente; resolutor no lo marca fósil
- [x] Instancia reciclada (PID ≠ 7064 o `started_at` nuevo); keepalive presente en el proceso
- [x] Sweep / `heartbeat-audit.json`: `email-watcher` `missed_cycles=0` post-reciclo
- [x] Este TODO en `docs/todos/done/` (mismo PR si hay diff documental; sin re-parche de `main.rs`)

**Si laudo A (Vértice Biológico insiste en genoma)**

- [ ] Justificar por escrito qué línea de `spawn_heartbeat_worker` falta en HEAD. Si no falta: rechazar A.

**Si laudo B (solo archivo documental)**

- [ ] Registrar por escrito por qué el ELF fósil puede seguir en 7064
- [ ] Este TODO en `docs/todos/done/` **sin** mutar umbrales

**Si tras reciclo reaparece sello de 3 ciclos**

- No reutilizar este `fracture_hash`. Abrir PBI nuevo: keepalive en runtime falló (timeout IMAP / lock del mutex / tick).

Prohibido declarar Done con keepalive «añadido» otra vez en fuente y el PID 7064 aún ejecutando el release de 2026-08-26.
