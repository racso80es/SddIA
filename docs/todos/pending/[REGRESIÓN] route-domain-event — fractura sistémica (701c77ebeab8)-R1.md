---
document_id: PBI-FIX-FRACTURE-701c77ebeab8-R1
uuid: "f20bc046-a297-4977-9432-52d1c8f976c0"
title: "[REGRESIÓN] route-domain-event — fractura sistémica"
format: markdown
version: "1.1.0"
created: "2026-08-30"
updated: "2026-08-30"
status: "abierto"
priority: alta
process: bug-fix
type: regression
dispatch: false
fracture_hash: 701c77ebeab8
fracture_process: route-domain-event
incident_ref: "System_Fracture_Detected — 701c77ebeab8"
regression_of: PBI-FIX-FRACTURE-701c77ebeab8-OLA1
regression_of_ola0: PBI-FIX-FRACTURE-701c77ebeab8
suggested_branch: fix/iota-publish-relay-elf-fosil-r1
persist_ref_suggested: docs/fixes/iota-publish-relay-elf-fosil-r1
refined: true
source_audit: "2026-08-30T16:50Z host: systemd unit sddia-iota-publish-relay@home-racso-Proyectos-SddIA; ELF debug mtime 2026-08-28; journal 4163 spawn / 4161 kill; ss :8787 vacío; curl /health refused; reanchor-queue 19 UUID; processed/dead-letter sin merkle; último digest local 2026-08-29T14:56:15Z Grh4v5dBZHBKaQXW; PR #233 merged 2026-08-30T09:24Z; HEAD main ff53b95."
review_notes: "v1.0.0 stub Cúmulo + Mayeuta prompt_adjustment (alucinación). v1.1.0 refinamiento Tekton: causa = ELF fósil del centinela + cola re-anclaje huérfana; no es fallo de prompt."
friction_ids:
  - F-CAPSULA-BINARIO-FOSIL
  - F-BUILD-DEV-DESALINEADO-CON-RUNTIME
  - F-DLT-SUPERVISOR-IMPACIENTE
  - F-ESPEJO-VERDE-FALSO
  - F-DLT-REANCHOR-COLA-SIN-DRENAJE
architectural_constraints:
  - A-BINARIO-COHERENTE-CON-FUENTE
  - A-NO-VERDE-SIN-BINDING
  - A-FRACTURE-HASH-INMUTABLE
  - A-NO-REABRIR-OLA0-FUENTE
  - A-DRENAJE-SIN-PENDING-HUERFANO
execution_file_lock:
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
gates_this_wave:
  - RELAY-R1-CA1
  - RELAY-R1-CA2
  - RELAY-R1-CA3
  - RELAY-R1-CA4
  - RELAY-R1-CA5
related:
  - SddIA/daemons/iota-publish-relay.md
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/scripts/daemons/_run_daemon.sh
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/ecosystem-health/src/lib.rs
  - .SddIA/services/iota-publish-relay/server.mjs
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (701c77ebeab8).md
  - docs/todos/done/[FIX] iota-publish-relay — Ola 1 latido degradado (701c77ebeab8).md
  - docs/todos/done/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
  - docs/fixes/iota-relay-supervisor-impatient-health/
  - docs/fixes/capsula-binario-fosil-release-stale/
related_pbis:
  - id: PBI-FIX-FRACTURE-701c77ebeab8
    rol: "Ola 0 — gracia post-spawn (PR #233). Fuente en main. ELF runtime no la ejecuta."
  - id: PBI-FIX-FRACTURE-701c77ebeab8-OLA1
    rol: "Canónico del resolutor (regression_of). Ola 1 degraded/espejo marcada cerrada; binario en ejecución anterior a ambos parches."
  - id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
    rol: "Hermano: ELF fósil de iota-immutable-publisher (perfil release). Distinto crate, mismo patrón F-CAPSULA-BINARIO-FOSIL."
---

# [REGRESIÓN] route-domain-event — fractura sistémica

> **Refinamiento v1.1.0.** El stub Cúmulo es un sello válido. La síntesis Mayeuta (`prompt_adjustment`) es **inválida**. Afirmaciones descartadas en §7.

## 1. Identidad del sello (no tocar)

| Campo | Valor | Notas |
|-------|--------|--------|
| `fracture_hash` | `701c77ebeab8` | SHA-256[:12] de la traza. Inmutable. Homólogo de Ola 0 y Ola 1. |
| `fracture_process` | `route-domain-event` | Proceso que falló el pre-sellado Merkle. El centinela implicado es `iota-publish-relay`. |
| `regression_of` | `PBI-FIX-FRACTURE-701c77ebeab8-OLA1` | Resolutor: primer `done/` con el mismo hash. Linaje Ola 0 = `PBI-FIX-FRACTURE-701c77ebeab8`. |
| Emisor | `execute-process` | Correcto. |
| Acción intentada | `merkle-batch-preseal` | Correcto. |
| Traza | `merkle-batch-preseal failed: iota-relay-unreachable: http://127.0.0.1:8787/v1/publish: Connection Failed: Connect error: Connection refused (os error 111)` | Literal. Misma que el canónico cerrado. |

Prohibido alterar `fracture_hash` / `fracture_process` / traza.

## 2. Hechos verificados (2026-08-30)

Zona: CEST = UTC+2. Auditoría Tekton ~16:50Z / 18:50 CEST.

| Hecho | Evidencia |
|-------|----------|
| Último anclaje DLT local | `2026-08-29T14:56:15Z` `PullRequest_Presented` digest `Grh4v5dBZHBKaQXW`. **Cero** sellos con mtime/ts 2026-08-30. |
| Eventos de registro hoy | ≥13 `PullRequest_Presented` + ≥14 `PullRequest_Merged` en bus. 8 `System_Fracture_Detected`. |
| Cola re-anclaje | 19 UUID en `.SddIA/dlt/reanchor-queue/`, todos `iota-relay-unreachable` (os error 111). Ventana `04:43Z`–`16:43Z`. **Ninguno** permanece en `pending/`. 15 `dead-letter`, 4 `processed`. `merkle_anchored` ausente. |
| Centinela systemd | `sddia-iota-publish-relay@home-racso-Proyectos-SddIA.service` **active running**. PID 7127. Arranque 15:42:06Z (reinicio post-boot). Primera ignición del día 04:37:49Z. |
| Hijo HTTP | `ss` sin listener `:8787`. `curl /health` connection refused. `pgrep` sin `server.mjs`. |
| Journal del unit | Desde 06:37 CEST: `hijo Node pid=…` + `/health falló con hijo vivo; reinicio` **en el mismo segundo**, cada ~5 s. Conteo ≥4163 / 4161. |
| ELF vs fuente | `SddIA/target/debug/iota-publish-relay` mtime **2026-08-28 07:31**. Fuente `main.rs` en HEAD **2026-08-30** (commits `8de55ea` gracia, y posteriores `tick_with_status`). `target/release/` **ausente**. |
| Resolución de binario | `_sddia_resolve_daemon_binary`: release (no hay) → **debug fósil**. |
| Parche Ola 0 | PR [#233](https://github.com/racso80es/SddIA/pull/233) merge `2026-08-30T09:24Z`. Código de gracia en el árbol. **No** en el proceso 7127. |
| Bóveda | `SDDIA_LAB_SIMULATE_IOTA=0`. `IOTA_PUBLISH_RELAY_URL=http://127.0.0.1:8787/v1/publish`. `IOTA_WALLET_SECRET` presente. No es simulación. |
| Espejo | `heartbeat-audit.json` `iota-publish-relay` `classification=healthy`, `missed_cycles=0`, side-channel `status=alive`. Argos no lee `/health`. |
| Drenaje | `try_drain_dlt_reanchor_queue` retorna si `!iota_relay_health_ok()` **o** si el JSON ya no está en `pending/`. Autorecuperación imposible con el estado actual. |
| Log hijo | `.SddIA/daemons/logs/iota-publish-relay.log`: un `EADDRINUSE` (carrera kill/respawn). `relay.log` del servicio: última línea `listening` **27 ago**. |

## 3. Qué hizo mal Mayeuta

Tokens de timeout/colapso/bypass → fallback `prompt_adjustment`: «detener, emitir fractura, no continuar entrega».

Eso **no describe este incidente**. No hay entrega Tekton en vuelo. `System_Fracture_Detected` **ya** se emitió (lote Merkle). El operador pidió auditoría y PBI, no un bypass de `route-domain-event`.

El párrafo Mayeuta queda como síntoma del clasificador, no como mandato de diseño.

## 4. Causa estructural

Tres defectos encadenados. El sello es el síntoma del primero; el segundo impide el rescate; el tercero oculta el fallo al espejo.

### 4.1 ELF fósil del centinela (causa del sello)

Ola 0 (gracia 10 s) y Ola 1 (`tick_with_status` / `degraded`) están en `main`. El unit ejecuta un ELF del **28 ago** que **mata al hijo en el tick de spawn** (probe refused inmediato). El Node no completa el bind. `:8787` nunca sirve. `iota-immutable-publisher` recibe connection refused → misma traza que el canónico cerrado.

Precedente: `PBI-FIX-FRACTURE-6a49e0ad310e-R1` (`docs/fixes/capsula-binario-fosil-release-stale/`) sobre **otro** crate (`iota-immutable-publisher`). Aquí el crate es `iota-publish-relay` y el selector es `_sddia_resolve_daemon_binary` (debug por ausencia de release), no la precedencia release-fósil de cápsulas.

### 4.2 Cola huérfana (RELAY-CA6, nunca cerrada)

`stamp_batch_anchor_error` escribe en `eda_instance.dlt_reanchor`. El sweeper / fan-out **purga** el evento de `pending/` aunque IOTA falló. `try_drain` exige el path original. Resultado: 19 registros locales de registro DLT **sin** objeto on-chain y **sin** drenaje automático cuando el relay vuelva.

### 4.3 Espejo verde falso

Ola 1 declaró CA9–CA12 cerrados. El binario en ejecución **no** las implementa. `record_heartbeat_at` / `color_daemon` del runtime vivo siguen tratando un tick periódico como `healthy`. El murder-loop **no** incrementa `missed_cycles`.

## 5. Discriminación A vs B

| Hipótesis | ¿Sostiene este sello? | Lectura |
|-----------|------------------------|---------|
| **(A) ELF del centinela desalineado del HEAD + drain huérfano** | Sí | Fuente ≠ proceso; journal = Ola 0 sin desplegar; cola no drena. |
| **(B) Reabrir Ola 0 y reescribir `GRACE_SECS`** | No | La fuente **ya** tiene gracia. Re-parchear `main.rs` sin rebuild deja el sello intacto. |
| Relay Node ausente / bóveda vacía | Refutado | `server.mjs` + `node_modules` presentes. Secretos presentes. `SIMULATE=0`. |
| `prompt_adjustment` / operador evadió Kintsugi | Refutado | Fracturas emitidas. No hay ciclo de entrega a castrar. |
| Misma causa que `6a49e0ad310e-R1` (publisher release) | Parcial (patrón), no identidad | Crate distinto. No mezclar PRs. |
| Host suspend / centinela muerto | Refutado | PID vivo, journal continuo, hermanos latiendo. |

**Laudo propuesto:** **(A)** ciclo `bug-fix` con tres palancas: (1) identidad ELF=fuente + restart del unit; (2) drenaje que localice eventos fuera de `pending/`; (3) el binario **en ejecución** debe portar Ola 0+1 (espejo `degraded` si `/health` false). No reabrir el diseño de gracia.

## 6. Alcance del fix (si laudo A)

### Dentro

- Compilar `iota-publish-relay` desde HEAD y hacer que el launcher/systemd ejecute **ese** ELF (release preferente o debug reconstruido). Verificar `GET /health` → 200 y ausencia de kill-en-el-mismo-segundo.
- Aduana de identidad: testigo contenido/fuente vs ELF del centinela (paridad conceptual con anclaje de cápsulas; **no** copiar el PR `capsula-binario-fosil-release-stale` a ciegas).
- `try_drain_dlt_reanchor_queue`: resolver payload desde `processed/` / `dead-letter` si `pending/` ya no existe; no dejar cola zombi.
- Rescate de los 19 UUID de 2026-08-30 (Merkle retroactivo o re-anclaje) con acta.
- Confirmar que el ELF **en ejecución** emite `status: degraded` post-gracia si `/health` false, y que Argos/espejo **no** pintan `healthy`/`green`. Si Ola 1 ya está en fuente, basta el rebuild; si el runtime de audit/espejo del proceso vivo aún ignora `status`, el hueco viaja en este R1 (no reabrir el PBI Ola 1 como canónico).

### Fuera

- Revertir `GRACE_SECS` / reescribir Ola 0 en fuente «por si acaso».
- Mutar `missed_cycles_threshold` para silenciar el sello.
- Sustituir Node→Rust (`DT-DLT-RELAY-NODE`).
- Jurisdicción `email-watcher` keepalive (`PBI-FIX-FRACTURE-6c0db1296181`).
- `SDDIA_LAB_SIMULATE_IOTA=1` como «cierre».

## 7. Afirmaciones descartadas del stub v1.0.0

| Afirmación del stub | Verdad |
|---------------------|--------|
| `prompt_adjustment` / «no continuar entrega» | No hay entrega. Hay ELF fósil y HTTP caído. |
| «Bloqueo operativo sin escalado Kintsugi» | `System_Fracture_Detected` × N hoy; Cúmulo materializó este R1. |
| Causa = operador | Causa = deploy del centinela + drain. |

## 8. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| RELAY-R1-CA1 | ELF del unit = HEAD (`cargo build -p iota-publish-relay`). `/health` 200. | `stat` ELF ≥ commit gracia; `curl -sf` 200; PID hijo `server.mjs` vivo. |
| RELAY-R1-CA2 | Journal sin murder-loop (spawn y kill en el mismo segundo). | `journalctl --user -u sddia-iota-publish-relay@…` ventana post-restart. |
| RELAY-R1-CA3 | Nuevo evento DLT-suscrito obtiene `transaction_digest` / `merkle_anchored`. | Un `PullRequest_*` o lote Merkle post-fix. |
| RELAY-R1-CA4 | Los 19 UUID de la cola de hoy anclados o acta Merkle `anchored_retroactively`; cola vacía o solo entradas post-rescate. | `.SddIA/dlt/reanchor-queue/` + `delivery_state`. |
| RELAY-R1-CA5 | Con hijo caído post-gracia: side-channel/`classification` ≠ `healthy` (o espejo no `green`). | Parar el hijo a propósito o ventana de fallo controlada. |

## 9. Criterio de cierre

- [ ] RELAY-R1-CA1…CA5
- [ ] `validacion.md` `global: APTO`, `pbi_archived: true`
- [ ] Este TODO en `docs/todos/done/` en la **misma** rama del PR

Prohibido declarar Done con `:8787` refused y este hash en `pending/`.
