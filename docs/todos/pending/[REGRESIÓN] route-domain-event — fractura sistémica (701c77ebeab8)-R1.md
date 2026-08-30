---
document_id: PBI-FIX-FRACTURE-701c77ebeab8-R1
uuid: "f20bc046-a297-4977-9432-52d1c8f976c0"
title: "[REGRESIÓN] route-domain-event — fractura sistémica"
format: markdown
version: "1.2.0"
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
source_audit: "2026-08-30T16:50Z host: systemd unit sddia-iota-publish-relay@home-racso-Proyectos-SddIA; ELF debug mtime 2026-08-28; journal 4163 spawn / 4161 kill; ss :8787 vacío; curl /health refused; reanchor-queue 19 UUID; processed/dead-letter sin merkle; último digest local 2026-08-29T14:56:15Z Grh4v5dBZHBKaQXW; PR #233 merged 2026-08-30T09:24Z; HEAD main ff53b95. v1.2.0: launchers forked debug-first en event-watcher/telegram-watcher/github-bridge-watcher/event-sweeper; _sddia_resolve_daemon_binary sin paridad ELF/mtime del orquestador; drain usa path grabado pending/; Ola 1 en fuente HEAD."
review_notes: "v1.0.0 stub Cúmulo + Mayeuta prompt_adjustment (alucinación). v1.1.0 refinamiento Tekton: causa = ELF fósil + cola huérfana; no es fallo de prompt. v1.2.0 clarificación de alcance (Filtro A): aduana en resolutor compartido de centinelas (no execute-process; MIME-ELF insuficiente); drenaje por UUID en processed/dead-letter (IOTA no mueve el padre); espejo verde = ELF vivo sin Ola 1, no reabrir diseño de telemetría."
friction_ids:
  - F-CAPSULA-BINARIO-FOSIL
  - F-BUILD-DEV-DESALINEADO-CON-RUNTIME
  - F-DLT-SUPERVISOR-IMPACIENTE
  - F-ESPEJO-VERDE-FALSO
  - F-DLT-REANCHOR-COLA-SIN-DRENAJE
architectural_constraints:
  - A-BINARIO-COHERENTE-CON-FUENTE
  - A-RESOLUTOR-CENTINELA-UNICO
  - A-IDENTIDAD-VS-FUENTE-NO-MIME
  - A-NO-VERDE-SIN-BINDING
  - A-FRACTURE-HASH-INMUTABLE
  - A-NO-REABRIR-OLA0-FUENTE
  - A-NO-REABRIR-OLA1-DISENO
  - A-DRENAJE-SIN-PENDING-HUERFANO
  - A-DRENAJE-POR-UUID-NO-PATH
execution_file_lock:
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/daemons/event-watcher.sh
  - SddIA/daemons/telegram-watcher.sh
  - SddIA/daemons/github-bridge-watcher.sh
  - SddIA/daemons/event-sweeper.sh
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
gates_this_wave:
  - RELAY-R1-CA1
  - RELAY-R1-CA2
  - RELAY-R1-CA3
  - RELAY-R1-CA4
  - RELAY-R1-CA5
  - RELAY-R1-CA6
related:
  - SddIA/daemons/iota-publish-relay.md
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/scripts/daemons/_run_daemon.sh
  - SddIA/scripts/daemons/_exec_daemon.sh
  - SddIA/daemons/event-watcher.sh
  - SddIA/daemons/telegram-watcher.sh
  - SddIA/daemons/github-bridge-watcher.sh
  - SddIA/daemons/event-sweeper.sh
  - SddIA/daemons/email-watcher.sh
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
    rol: "Canónico del resolutor (regression_of). Ola 1 degraded/espejo marcada cerrada; binario en ejecución anterior a ambos parches. Diseño en HEAD; no reabrir."
  - id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
    rol: "Hermano: ELF fósil de iota-immutable-publisher (perfil release). Distinto crate, mismo patrón F-CAPSULA-BINARIO-FOSIL."
---

# [REGRESIÓN] route-domain-event — fractura sistémica

> **Refinamiento v1.2.0.** El stub Cúmulo es un sello válido. La síntesis Mayeuta (`prompt_adjustment`) es **inválida**. Afirmaciones descartadas en §7. Alcance de las tres palancas corregido en §3.1 (Filtro A).

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

Zona: CEST = UTC+2. Auditoría Tekton ~16:50Z / 18:50 CEST. Relectura de launchers y drain en v1.2.0.

| Hecho | Evidencia |
|-------|----------|
| Último anclaje DLT local | `2026-08-29T14:56:15Z` `PullRequest_Presented` digest `Grh4v5dBZHBKaQXW`. **Cero** sellos con mtime/ts 2026-08-30. |
| Eventos de registro hoy | ≥13 `PullRequest_Presented` + ≥14 `PullRequest_Merged` en bus. 8 `System_Fracture_Detected`. |
| Cola re-anclaje | 19 UUID en `.SddIA/dlt/reanchor-queue/`, todos `iota-relay-unreachable` (os error 111). Ventana `04:43Z`–`16:43Z`. **Ninguno** permanece en `pending/`. 15 `dead-letter`, 4 `processed`. `merkle_anchored` ausente. |
| Path grabado en cola | Cada JSON de cola apunta a `.events/pending/{uuid}.json` (path absoluto al encolar). Ese fichero **ya no existe**. |
| Centinela systemd | `sddia-iota-publish-relay@home-racso-Proyectos-SddIA.service` **active running**. PID 7127. Arranque 15:42:06Z (reinicio post-boot). Primera ignición del día 04:37:49Z. |
| Cadena de ignición | `scripts/daemons/{name}.sh` → `_run_daemon.sh` → `_exec_daemon.sh` → `SddIA/daemons/{name}.sh` (aquí se resuelve el ELF). |
| Hijo HTTP | `ss` sin listener `:8787`. `curl /health` connection refused. `pgrep` sin `server.mjs`. |
| Journal del unit | Desde 06:37 CEST: `hijo Node pid=…` + `/health falló con hijo vivo; reinicio` **en el mismo segundo**, cada ~5 s. Conteo ≥4163 / 4161. |
| ELF vs fuente | `SddIA/target/debug/iota-publish-relay` mtime **2026-08-28 07:31**. Fuente `main.rs` en HEAD **2026-08-30** (commits `8de55ea` gracia, y posteriores `tick_with_status`). `target/release/` **ausente**. |
| Resolución de binario (este sello) | `_sddia_resolve_daemon_binary`: primer `release/` existente y ejecutable → si no, **debug fósil**. Sin `_sddia_is_native_elf`. Sin mtime vs fuente. |
| Callers del resolutor compartido | `SddIA/daemons/iota-publish-relay.sh`, `SddIA/daemons/email-watcher.sh`, `SddIA/scripts/daemons/kalma2-bridge.sh`. |
| Launchers forked (debug-first) | `event-watcher.sh`, `telegram-watcher.sh`, `github-bridge-watcher.sh`, `event-sweeper.sh` en `SddIA/daemons/`: prefieren `target/debug/` **aunque exista release**. No pasan por `_sddia_resolve_daemon_binary`. |
| Paridad orquestador | `_sddia_resolve_orchestrator` sí usa `_sddia_is_native_elf` + F-DEP-07 (debug solo si mtime > release). Si **solo** hay debug, también acepta fósil. MIME-ELF no distingue fósil de HEAD. |
| Parche Ola 0 | PR [#233](https://github.com/racso80es/SddIA/pull/233) merge `2026-08-30T09:24Z`. Código de gracia en el árbol. **No** en el proceso 7127. |
| Parche Ola 1 (fuente HEAD) | `decide_supervisor_tick` emite `degraded` si `!health_ok && !in_grace`. `record_heartbeat_at` mapea `status=degraded` → `classification=degraded`. `color_daemon` pinta yellow. **No** en el ELF del 28 ago. |
| Bóveda | `SDDIA_LAB_SIMULATE_IOTA=0`. `IOTA_PUBLISH_RELAY_URL=http://127.0.0.1:8787/v1/publish`. `IOTA_WALLET_SECRET` presente. No es simulación. |
| Espejo | `heartbeat-audit.json` `iota-publish-relay` `classification=healthy`, `missed_cycles=0`, side-channel `status=alive`. Argos no lee `/health`; confía en el payload. |
| Drenaje | `try_drain_dlt_reanchor_queue` retorna si `!iota_relay_health_ok()`. Si el `path` grabado no es fichero, `continue` (salta el UUID). No busca `processed/` ni `dead-letter/` por UUID. |
| Log hijo | `.SddIA/daemons/logs/iota-publish-relay.log`: un `EADDRINUSE` (carrera kill/respawn). `relay.log` del servicio: última línea `listening` **27 ago**. |

## 3. Qué hizo mal Mayeuta

Tokens de timeout/colapso/bypass → fallback `prompt_adjustment`: «detener, emitir fractura, no continuar entrega».

Eso **no describe este incidente**. No hay entrega Tekton en vuelo. `System_Fracture_Detected` **ya** se emitió (lote Merkle). El operador pidió auditoría y PBI, no un bypass de `route-domain-event`.

El párrafo Mayeuta queda como síntoma del clasificador, no como mandato de diseño.

## 3.1 Filtro A — razonamiento de alcance (v1.2.0)

Tres tesis de ampliación. Veredicto por tesis; lo rechazado no entra en el laudo.

### Tesis 1 — «Aduana en toda la capa de centinelas, no solo iota-publish-relay»

| Afirmación | Veredicto | Corrección |
|-----------|-----------|------------|
| El defecto de resolución vive en `_sddia_resolve_daemon_binary` (release ausente → debug fósil) | **Hecho** para este sello y para `email-watcher` / `kalma2-bridge`. | Función: primer `-f && -x` de release, luego debug. Cero identidad vs fuente. |
| `event-watcher` / `telegram-watcher` quedan expuestos **por la misma función** | **Inexacto** | No la llaman. Tienen launchers **forked** en `SddIA/daemons/{name}.sh` que prefieren **debug sobre release**. Exposición **peor** y por **otro** código. Mismo vector `F-BUILD-DEV-DESALINEADO-CON-RUNTIME`. |
| Inyectar la aduana en «el orquestador base» | **Ambiguo; la lectura `execute-process` es alucinación de lugar** | `execute-process` / `_sddia_resolve_orchestrator` **no** spawnea centinelas. Lugar correcto: `_sddia_resolve_daemon_binary` + convergencia de launchers forked hacia esa función. |
| «Validación estructural del ELF» | **Parcial; MIME no cierra este sello** | `_sddia_is_native_elf` (mime `application/x-executable`) ya existe para el orquestador. El fósil del 28 ago **es** ELF válido. Portar solo MIME/F-DEP-07 (mtime debug vs release) **no** habría evitado el incidente: no había release; el debug fósil seguiría ganando. Aduana exigida: **coherencia ELF ↔ fuente/HEAD** (`A-IDENTIDAD-VS-FUENTE-NO-MIME`), no parser de cabeceras. |

**Laudo tesis 1:** la aduana de identidad de este R1 vive en el **resolutor compartido de centinelas**, no en `iota-publish-relay/src/main.rs` ni en `execute-process`. Los cuatro launchers debug-first deben converger. El sello 701c77ebeab8 se cierra cuando **este** unit ejecuta ELF=HEAD; la convergencia evita el mismo sello en hermanos.

### Tesis 2 — «Fuga de idempotencia / cola re-anclaje zombi»

| Afirmación | Veredicto | Corrección |
|-----------|-----------|------------|
| Con IOTA caído los anclajes quedan huérfanos y `try_drain` no los rescata | **Hecho** | 19 UUID en cola; `pending/` vacío; `merkle_anchored` ausente. |
| El drain busca el original **únicamente en `pending/`** (hardcode) | **Inexacto** | Lee el campo `path` del JSON de cola. Ese path **fue** `pending/` al encolar. Si `!event_path.is_file()` → `continue` (omite el UUID; no aborta el bucle). No hay fallback por UUID. |
| «Cuando IOTA falla, el evento se retira de `pending/` y se transfiere a dead-letter o processed» | **Compresión causal falsa** | `stamp_batch_anchor_error` **no** mueve el padre: estampa `last_batch_anchor_error` y encola. Quien purga `pending/` es el ciclo de bus (`try_sweep_event` / sweeper) **tras consenso de suscriptores**, independiente del éxito DLT. Por eso 15 DL + 4 processed sin merkle. |
| Drenaje debe rescatar payloads desde carpetas de destino | **Aceptado** | Resolver por UUID en `pending/` → `processed/` → `dead-letter/` (eda_bus). No reinyectar a `pending/` (comentario L-QUEUE vigente). |

**Laudo tesis 2:** refactor de `try_drain_dlt_reanchor_queue` obligatorio en este R1. Localizar payload por UUID, no por path fósil. No atribuir el move a IOTA.

### Tesis 3 — «Falsa inmunidad / espejo verde falso»

| Afirmación | Veredicto | Corrección |
|-----------|-----------|------------|
| El centinela emitía latido que el espejo pintó healthy con hijo HTTP caído | **Hecho** | `classification=healthy`, `status=alive`, `missed_cycles=0`. Probe `/health` refused. |
| El binario **en ejecución** debe asimilar Ola 1 (degraded post-gracia si probe falla) | **Aceptado como CA, no como diseño nuevo** | Ola 1 **ya está en HEAD**: `decide_supervisor_tick`, `record_heartbeat_at`, `color_daemon`. El ELF del 28 ago no la ejecuta. CA5 = el proceso vivo, no reabrir `PBI-FIX-FRACTURE-701c77ebeab8-OLA1`. |
| «`/health` devuelve false» | **Inexacto** | `probe_health` = TCP + respuesta HTTP con `200` en el cuerpo. Connection refused ⇒ `health_ok=false`. No hay JSON `false`. |
| Argos / runtime de telemetría debe auditar subprocesos / leer `/health` | **Fuera** | Argos no es supervisor del hijo Node. El contrato §6.1 es `status` del latido. Tras rebuild, `degraded` debe apagar el verde. Si el audit **vivo** aún ignora `status` tras rebuild, el hueco viaja en este R1 **sin** reabrir el PBI Ola 1. |
| Extender probe `/health` a event-watcher / telegram-watcher | **Alucinación de alcance** | No tienen hijo HTTP. Latido `alive` periódico es contrato, no ceguera. |

**Laudo tesis 3:** no reescribir Ola 1. El ELF que systemd ejecute debe emitir `degraded` post-gracia si el probe no es HTTP 200. CA5 lo verifica. Prohibido mutar `missed_cycles_threshold` para silenciar el sello.

## 4. Causa estructural

Tres defectos encadenados. El sello es el síntoma del primero; el segundo impide el rescate; el tercero oculta el fallo al espejo **mientras el ELF fósil siga vivo**.

### 4.1 ELF fósil + resolutor de centinelas incompleto (causa del sello)

Ola 0 (gracia 10 s) y Ola 1 (`tick_with_status` / `degraded`) están en `main`. El unit ejecuta un ELF del **28 ago** que **mata al hijo en el tick de spawn** (probe refused inmediato). El Node no completa el bind. `:8787` nunca sirve. `iota-immutable-publisher` recibe connection refused → misma traza que el canónico cerrado.

El selector de **este** crate es `_sddia_resolve_daemon_binary` (debug por ausencia de release). No es la precedencia release-fósil de cápsulas (`PBI-FIX-FRACTURE-6a49e0ad310e-R1`, crate `iota-immutable-publisher`).

Vector hermano (no este sello, sí este R1): cuatro launchers forked debug-first. Misma clase de desalineación, distinto código.

### 4.2 Cola huérfana (RELAY-CA6 Ola 0, nunca cerrada)

`stamp_batch_anchor_error` escribe en `eda_instance.dlt_reanchor` con `path` = ubicación **en ese instante** (pending). El sweeper / fan-out **purga** el padre de `pending/` cuando hay consenso de suscriptores, **aunque** IOTA falló. `try_drain` exige que ese path siga siendo fichero. Resultado: 19 registros locales de registro DLT **sin** objeto on-chain y **sin** drenaje automático cuando el relay vuelva.

### 4.3 Espejo verde falso (deploy, no hueco de diseño en HEAD)

Ola 1 declaró CA9–CA12 cerrados en fuente. El binario en ejecución **no** las implementa. `tick()` implícito del fósil escribe `status: alive`. `record_heartbeat_at` / `color_daemon` del **código HEAD** ya honran `degraded`; el proceso vivo no llega a emitir ese status. El murder-loop **no** incrementa `missed_cycles`.

## 5. Discriminación A vs B

| Hipótesis | ¿Sostiene este sello? | Lectura |
|-----------|------------------------|---------|
| **(A) ELF del centinela desalineado del HEAD + drain huérfano** | Sí | Fuente ≠ proceso; journal = Ola 0 sin desplegar; cola no drena. |
| **(B) Reabrir Ola 0 y reescribir `GRACE_SECS`** | No | La fuente **ya** tiene gracia. Re-parchear `main.rs` sin rebuild deja el sello intacto. |
| Reabrir Ola 1 y reescribir `record_heartbeat_at` | No, salvo prueba post-rebuild | HEAD ya mapea `degraded`. El verde falso es el ELF fósil. |
| Relay Node ausente / bóveda vacía | Refutado | `server.mjs` + `node_modules` presentes. Secretos presentes. `SIMULATE=0`. |
| `prompt_adjustment` / operador evadió Kintsugi | Refutado | Fracturas emitidas. No hay ciclo de entrega a castrar. |
| Misma causa que `6a49e0ad310e-R1` (publisher release) | Parcial (patrón), no identidad | Crate distinto. No mezclar PRs. |
| Host suspend / centinela muerto | Refutado | PID vivo, journal continuo, hermanos latiendo. |
| Aduana MIME-ELF en `_sddia_resolve_orchestrator` habría evitado el sello | Refutado | El orquestador no lanza este unit. MIME no detecta fósil-vs-HEAD. |

**Laudo:** **(A)** ciclo `bug-fix` con tres palancas: (1) resolutor único de centinelas con identidad ELF↔fuente + restart del unit `iota-publish-relay`; (2) drenaje por UUID fuera de `pending/`; (3) el binario **en ejecución** debe portar Ola 0+1 (espejo `degraded` si probe `/health` no es 200 post-gracia). No reabrir el diseño de gracia ni el de Ola 1.

## 6. Alcance del fix (si laudo A)

### Dentro

- Compilar `iota-publish-relay` desde HEAD y hacer que el launcher/systemd ejecute **ese** ELF (release preferente o debug reconstruido y **más nuevo que la fuente**). Verificar probe `/health` → HTTP 200 y ausencia de kill-en-el-mismo-segundo.
- Aduana de identidad en `_sddia_resolve_daemon_binary`: testigo ELF ↔ fuente/HEAD (mtime de crate o hash; paridad conceptual con anclaje de cápsulas y con `_sddia_resolve_orchestrator`, **sin** copiar MIME-only ni el PR `capsula-binario-fosil-release-stale` a ciegas). Fallo = no exec + error explícito, no silent fallback a fósil.
- Convergencia: `SddIA/daemons/{event-watcher,telegram-watcher,github-bridge-watcher,event-sweeper}.sh` delegan en el resolutor compartido. Fin del debug-first forked.
- `try_drain_dlt_reanchor_queue`: si el `path` grabado no existe, resolver `{uuid}.json` en `eda_bus.pending` → `processed` → `dead-letter`. No dejar cola zombi. No reinyectar a `pending/`.
- Rescate de los 19 UUID de 2026-08-30 (Merkle retroactivo o re-anclaje) con acta.
- Confirmar que el ELF **en ejecución** emite `status: degraded` post-gracia si el probe no es HTTP 200, y que Argos/espejo **no** pintan `healthy`/`green`. Si Ola 1 ya está en fuente, basta el rebuild **de este crate** (+ restart). Si el runtime de audit/espejo **del proceso vivo** aún ignora `status` tras ese rebuild, el hueco viaja en este R1 (no reabrir el PBI Ola 1 como canónico).

### Fuera

- Revertir `GRACE_SECS` / reescribir Ola 0 en fuente «por si acaso».
- Reabrir `PBI-FIX-FRACTURE-701c77ebeab8-OLA1` o reescribir `record_heartbeat_at` / `color_daemon` sin evidencia post-rebuild.
- Mutar `missed_cycles_threshold` para silenciar el sello.
- Inyectar aduana de centinelas en `_sddia_resolve_orchestrator` / `execute-process` (no spawnea daemons).
- Parser de cabeceras ELF / MIME como único gate de identidad.
- Probe `/health` en centinelas sin hijo HTTP (`event-watcher`, `telegram-watcher`, …).
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
| RELAY-R1-CA1 | ELF del unit `iota-publish-relay` = HEAD (`cargo build -p iota-publish-relay`). Probe `/health` HTTP 200. | `stat` ELF ≥ commit gracia; `curl -sf` 200; PID hijo `server.mjs` vivo. |
| RELAY-R1-CA2 | Journal sin murder-loop (spawn y kill en el mismo segundo). | `journalctl --user -u sddia-iota-publish-relay@…` ventana post-restart. |
| RELAY-R1-CA3 | Nuevo evento DLT-suscrito obtiene `transaction_digest` / `merkle_anchored`. | Un `PullRequest_*` o lote Merkle post-fix. |
| RELAY-R1-CA4 | Los 19 UUID de la cola de hoy anclados o acta Merkle `anchored_retroactively`; cola vacía o solo entradas post-rescate. Drain localiza payload si el path pending ya no existe. | `.SddIA/dlt/reanchor-queue/` + `delivery_state`; prueba con UUID cuyo JSON esté en `processed/` o `dead-letter/`. |
| RELAY-R1-CA5 | Con hijo caído post-gracia: side-channel `status=degraded` y `classification` ≠ `healthy` (espejo no `green`). | Parar el hijo a propósito o ventana de fallo controlada **sobre el ELF que systemd ejecuta**. |
| RELAY-R1-CA6 | Catálogo `SddIA/daemons/index.md`: todo `SddIA/daemons/{name}.sh` resuelve vía `_sddia_resolve_daemon_binary`. Ningún launcher prefija debug sobre release. El resolutor rechaza ELF más viejo que la fuente del crate (o equivalente documentado). | `grep` de resolutor en los 6 launchers; caso negativo: debug fósil + fuente nueva ⇒ no exec. |

## 9. Criterio de cierre

- [ ] RELAY-R1-CA1…CA6
- [ ] `validacion.md` `global: APTO`, `pbi_archived: true`
- [ ] Este TODO en `docs/todos/done/` en la **misma** rama del PR

Prohibido declarar Done con `:8787` refused y este hash en `pending/`.
