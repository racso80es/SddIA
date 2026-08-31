---
document_id: PBI-OPER-LATIDO-ONTOLOGICO-001
title: "[OPERATIVO] Latido Ontológico (System Heartbeat)"
format: markdown
version: "2.1.0"
created: "2026-08-27"
updated: "2026-08-31"
status: pending
priority: alta
process: feature
type: feature
dispatch: false
uuid: cafd87eb-817f-4eee-a169-f9cd6019e931
suggested_branch: feat/latido-ontologico-vitalidad-organos
persist_ref_suggested: docs/features/latido-ontologico-vitalidad-organos
depends_on: []
related_pbis:
  - id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
    rol: "Archivado (done/). El genoma iota-publish-relay.md ya existe y el relay late. Este PBI ya no lo desbloquea; solo cierra el drift de emisores autorizados (iota sigue ausente de daemon-heartbeat.md)."
  - id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
    rol: "Archivado (done/). NO es consumidor de System_Vitality_Probed. Lee heartbeat-audit.json × map-snapshot × stats.json × revoked_entities.json vía query-ecosystem-health y GET /api/system-health. Este PBI no es su proveedor de señal."
friction_ids:
  - F-AUDITORIA-CENSO-GENOMA
  - F-AUDITOR-PID-MUERTO-SILENCIO
  - F-ORGANO-INTERFAZ-SIN-LATIDO
  - F-CONTRATO-HEARTBEAT-DRIFT
  - F-VITALIDAD-NO-PROCESO-INVISIBLE
related:
  - SddIA/events/telemetry/daemon-heartbeat.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/engine/execute-process/src/engine/daemons.rs
  - SddIA/core/event-telemetry-subscriptions.json
  - SddIA/daemons/index.md
  - SddIA/daemons/iota-publish-relay.md
  - SddIA/daemons/daemons-contract.md
  - SddIA/interfaces/kalma2-bridge/
  - SddIA/scripts/daemons/kalma2-bridge.sh
  - SddIA/process/query-ecosystem-health.md
  - start-sddia.sh
---

# [OPERATIVO] Latido Ontológico (System Heartbeat)

> **Refinamiento v2.1.0 (2026-08-31).** v2.0.0 diagnosticó bien el síntoma y erró en el territorio: trató como «en vuelo» un PBI DLT ya archivado, atribuyó a Espejo un contrato de señal que nunca implementó, y describió mal la semántica del auditor (fractura ≠ muerte). Las citas por número de línea de v2 están caducadas; este texto ancla símbolos y ficheros. Afirmaciones descartadas: §7 (v1) y §8 (v2).

## 0. Rectificación antientrópica (territorio 2026-08-31)

| Afirmación v2.0.0 | Veredicto | Evidencia |
|-------------------|-----------|-----------|
| `iota-publish-relay` sin `{name}.md`, «trabajo en vuelo, sin commit» | **Obsoleto** | `SddIA/daemons/iota-publish-relay.md` uuid `78e94d53-0445-4394-b399-3e594cabc511`, fila en `index.md`, crate con `DaemonRuntime`. PBI DLT en `docs/todos/done/`, status `archivado`. |
| Este PBI es proveedor de señal de Espejo (`System_Vitality_Probed`) | **Alucinación de contrato** | Espejo (done) fusiona `heartbeat-audit.json` + `map-snapshot.json` + `stats.json` + `revoked_entities.json`. No declara ni consume `System_Vitality_Probed`. Endpoint vivo: `GET /api/system-health`. |
| Censo = 5 centinelas con genoma; relay invisible al auditor | **Obsoleto** | Índice: 6 catalogados (iota + event-watcher + event-sweeper + telegram-watcher + github-bridge-watcher + email-watcher). El relay **entra** en `list_indexed_daemon_ids`. |
| `_sddia_stop_lock_pid` en `start-sddia.sh:168` | **Línea caducada** | `cleanup()` → `kalma2-bridge.lock` (símbolo, no línea). Sigue siendo la única escritura/lectura de ese lock en el repo: nadie lo materializa. |
| `audit_staleness` / `ingest_regime` / `missed_cycles` en líneas 146–314 | **Líneas caducadas** | Símbolos vigentes: `ingest_regime`, `audit_staleness`, `audit_running_daemon` en `daemon_heartbeat.rs`. |
| `cumulo.paths.json` v1.6.4 | **Versión caducada** | `version: 1.9.0`. |
| Trinidad `telemetry` / `orchestration` / `domain` / `progress` | **Inexacto** | `events-contract` §6: tres familias. `eda_fractal.progress` es **hoja de bus**, no `event_family`. |
| Matar un centinela → `System_Fracture_Detected` a 3 ciclos | **Falso frente al auditor** | `audit_running_daemon`: sin lock → `Ok(None)`; `!pid_alive` → `Ok(None)`. Fractura solo si hay lock, PID vivo y `missed_cycles >= umbral`. Muerte ≠ cuelgue. |
| Forjar `kalma2-bridge` como centinela sensorial | **Colisión ontológica** | `daemons-contract` §2 (Ceguera Lógica): no invocar `execute-process`, no leer genoma. El puente es WUI HTTP bajo `SddIA/interfaces/`, launcher y unidad systemd **ya existen**. No es un watcher periférico. |
| `daemon-heartbeat-audit.md:70` «no arranca ni mata» | **Cita errónea** | L.70 = discriminación `host_suspend`. El límite está en § Límites: «No arranca ni mata Centinelas». |
| `verify-tools-index` en `sddia-qa/src/main.rs:146` | **Línea caducada** | Dispatch en `main.rs` (`verify-tools-index` / `verify-process-integrity`). |
| `GET /api/runtime-profile` en `main.rs:75-88,1856` | **Líneas caducadas** | Ruta viva en el match HTTP del bridge; coexistente con `GET /api/system-health`. |
| `DT-CUMULO-INDEX-AUDIT` como deuda registrada | **ID inventado** | Cero ocurrencias fuera de este PBI. Queda como **propuesta**, no como ítem de registro. |

## 1. Falla Estructural y Contexto

El latido de centinelas **está implementado y auditado**: `Daemon_Heartbeat` (`event_family: telemetry`, uuid `9c5190ac-ac8a-46b6-b61d-67d45ff7caf1`), emitido por `DaemonRuntime::emit_heartbeat` (lock + side-channel + ECST en `eda_fractal.telemetry`), consumido por Argos vía `daemon-heartbeat-audit`. Umbral SSOT: `SddIA/daemons/heartbeat-audit.thresholds.json` → `missed_cycles_threshold` (default 3) con overlay de instancia.

### Lo que ya existe (no se reconstruye)

| Órgano | Artefacto | Ancla |
|--------|-----------|-------|
| Clase | `Daemon_Heartbeat` | `SddIA/events/telemetry/daemon-heartbeat.md` |
| Emisión | Lock PID + side-channel + evento fractal | `DaemonRuntime::emit_heartbeat` |
| SSOT intervalo | `execution.heartbeat_interval_seconds` (default 30 s, piso 5 s) | `{name}.md`; `parse_daemon_spec` / `daemon_interval` |
| Auditoría de **cuelgue** | `missed_cycles = trunc(elapsed / interval)` si lock **y** PID vivo; fractura si `>= umbral` y sin `fracture_event_id` | `audit_running_daemon` |
| Censo | `list_indexed_daemon_ids` = stems `{name}.md` en `SddIA/daemons/` excepto `index` y `daemons-contract` | `daemons.rs` |
| Ingesta amplia | `ingest_regime` barre **todo** el side-channel y la carpeta telemetry | No condiciona la fractura al censo |
| Suscripción | `Daemon_Heartbeat` → argos / `daemon-heartbeat-audit` | `event-telemetry-subscriptions.json` |
| Tick de sweep | 30 s (`HEARTBEAT_AUDIT_SWEEP_SECONDS`) | `event-sweeper` |
| Gate de ignición | `_wait_required_heartbeats` sobre `REQUIRED_DAEMONS=(event-watcher event-sweeper)` + `iota-publish-relay` si L-REQUIRED | `start-sddia.sh` |
| Gate HTTP Kalma2 | `_wait_http` sobre `KALMA_URL/` (systemd y script); fallo → `[ERROR]` y `cleanup 1` | `start-sddia.sh` — **ya existe**, one-shot |
| Señal de colapso | `System_Fracture_Detected` → `eda_bus.pending` | `events/domain/system-fracture-detected.md` |
| Read Model de salud | `query-ecosystem-health` → `.SddIA/observability/ecosystem-health.json`; panel Kalma2 | Espejo, done |
| Relay IOTA | Genoma + `DaemonRuntime` + launcher + `REQUIRED` condicional + `/health` | PBI DLT archivado |

### Semántica real del auditor (no la del borrador)

`audit_staleness` itera el censo genómico y delega en `audit_running_daemon`:

1. Sin lock → silencio (no fractura).
2. Lock con PID muerto → silencio (no fractura).
3. Lock + PID vivo + `missed_cycles >= umbral` → `System_Fracture_Detected` (idempotente por `fracture_event_id`).
4. `status: degraded` en el latido **no** emite fractura; Espejo lo pinta amarillo.

Consecuencia: «morir sin señal» tiene **dos** eslabones, no uno. El genoma habilita el censo; el auditor, además, **ignora la muerte**. Un `kill` de un centinela catalogado no cumple el criterio de aceptación que v2 escribió (VIT-CA3). Evidencia positiva del camino 3: PBI pending `email-watcher` fractura `1933c0a0fe2c` (3 ciclos omitidos con umbral 3) — el circuito de **cuelgue** funciona para órganos catalogados.

`daemon_interval` degrada a 30 s si falta el `{name}.md`. Genoma ausente ⇒ el órgano no entra en el bucle de fractura **y** el divisor, si alguien audita por otro camino, es un default silencioso.

### Censo actual (post-DLT)

| Órgano | `{name}.md` | `DaemonRuntime` | Lock | En `REQUIRED_DAEMONS` | Auditado si PID vivo |
|--------|-------------|-----------------|------|------------------------|----------------------|
| `event-watcher`, `event-sweeper`, `telegram-watcher`, `github-bridge-watcher`, `email-watcher` | Sí | Sí | Sí (si arrancó) | watcher+sweeper siempre; iota condicional; resto opcional/sensorial | Sí |
| `iota-publish-relay` | **Sí** (cerrado por PBI DLT) | Sí | Sí | Si L-REQUIRED | Sí |
| `kalma2-bridge` | **No** | **No** | **No** | **No** (gate HTTP one-shot) | **No** |

`kalma2-bridge` hoy: crate en `SddIA/interfaces/kalma2-bridge/`, launcher `SddIA/scripts/daemons/kalma2-bridge.sh` (`exec` del ELF), unidad `sddia-kalma2-bridge@.service`, arranque script vía `"$BRIDGE_BIN" &`. `cleanup()` intenta `_sddia_stop_lock_pid .../kalma2-bridge.lock` que nadie escribe; en jurisdicción script el `kill $(jobs -p)` sí cubre el hijo. En systemd, `cleanup` no toca locks (sale antes).

### Drift de contrato (sigue vigente, ahora peor)

`daemon-heartbeat.md` § Emisores autorizados lista solo `event-watcher`, `telegram-watcher`, `github-bridge-watcher`. El índice cataloga **seis** centinelas que emiten latido, incluido `iota-publish-relay` y los REQUIRED `event-sweeper` / (condicionalmente) iota. El contrato valida contra un subconjunto falso del territorio.

### Lo que no es proceso

Índices de Cúmulo y la config de Cerbero no tienen PID. Espejo **no** los verifica: pinta tools/skills según `stats.json` y daemons según `heartbeat-audit.json`. Integridad de `cumulo.paths.json`, overlay `local.paths.json` y `execution-contexts.md` sigue sin sonda ni fractura.

## 2. Objetivo Medible

Que un órgano vital **colgado** (PID vivo, latido ausente) y un invariante **no-proceso** roto no puedan permanecer en silencio; y que la **muerte** de un órgano catalogado deje de ser un `Ok(None)`.

Éxito si:
1. Todo proceso persistente del que depende una capacidad o bien late bajo `DaemonRuntime` y entra en el censo, o bien queda explícitamente fuera del contrato de centinelas con otra sonda de liveness (HTTP) que fractura en runtime — no solo en ignición.
2. `audit_running_daemon` trata lock huérfano / PID muerto de un id catalogado como incidente (fractura idempotente), no como skip.
3. Existe una sonda periódica de invariantes no-proceso que emite hecho auditable + fractura en rojo.
4. `daemon-heartbeat.md` declara exactamente los emisores que emiten (los 6 del índice, o el subconjunto que realmente corre `DaemonRuntime`).
5. La ignición no declara grado operativo con un REQUIRED sin latido **ni** con Kalma2 HTTP caído (esto último ya está; no reimplementar).
6. Cero variables de bóveda que declaren el intervalo de latido (el divisor de `missed_cycles`).

## 3. Decisiones Arquitectónicas Obligatorias

### 3.1. Prohibido inventar ontología paralela al latido existente
Se **descarta** `System_Heartbeat_Emitted`. `Daemon_Heartbeat` cubre liveness de procesos. Se **descarta** `System_Degraded`: `System_Fracture_Detected` (Argos, infra) y `Domain_Entity_Degraded` (Radamanto, termodinámica de entidades) cubren el espectro.

### 3.2. `kalma2-bridge` es órgano de interfaz, no sensor periférico
No se le aplica Ceguera Lógica plena (`daemons-contract` §2). El crate **no** se mueve a `SddIA/daemons/`. El launcher y la unidad systemd **no** se re-forjan.

Sí se le dota del **circuito de vitalidad** para entrar en el censo de Argos:

```text
Catalogado = {name}.md en SddIA/daemons/ + fila en index.md
           + DaemonRuntime en el crate existente (lock + side-channel + Daemon_Heartbeat)
           + emisor en events/telemetry/daemon-heartbeat.md
           + jurisdicción explícita: órgano de interfaz (excepción documentada al §2 del contrato, no un watcher)
```

Forja del `{name}.md` vía `daemon-creator` / `entity-manager` (DA-2). `execution.entrypoint` apunta al launcher ya existente `SddIA/scripts/daemons/kalma2-bridge.sh`.

Latido ≠ HTTP. Un PID vivo que deja de servir `KALMA_URL` seguiría verde para Argos si solo late. Por eso la sonda HTTP de §3.4 es complementaria (mismo patrón que iota: `tick_with_status` /health no produce fractura; la sonda sí puede).

`iota-publish-relay.md` **no** es entrega de este PBI. Fase 1 solo alinea el contrato de emisores.

### 3.3. El auditor debe gritar la muerte, no solo el cuelgue
Extender `audit_running_daemon` (o un paso hermano en `audit_staleness`):

- Id catalogado + lock presente + `!pid_alive` → `System_Fracture_Detected` (causa: PID muerto / lock huérfano), idempotente.
- Id catalogado **REQUIRED** (o lista de órganos vitales) + sin lock tras ignición → fuera de este PBI si systemd aún no ha escrito lock; no inventar carrera con el bootstrap. El caso accionable es lock huérfano, no «aún no arrancó».

Sin este cambio, VIT-CA3 es inverificable: el territorio actual **no** fractura al `kill`.

### 3.4. Lo que no es proceso se verifica; HTTP de Kalma2 se re-verifica en runtime
Forjar proceso **`system-vitality-probe`** (agente `argos`) que emite **`System_Vitality_Probed`** en `eda_fractal.telemetry`. Única clase nueva autorizada.

No es el Read Model de Espejo. Espejo ya tiene el suyo. Este evento es telemetría de **invariantes** + veredicto para fractura; un consumidor UI posterior sería PBI aparte sobre una feature cerrada.

Sondas Fase 3:

| Sonda | Verificación física | Base existente | Notas |
|-------|--------------------|----------------|-------|
| `bus.topology` | `cumulo.paths.json` parseable + overlay `.SddIA/local.paths.json` no-`{}` + existencia de hojas `eda_fractal.*` | Cúmulo v1.9.0; `instance-creator` (F-DEP-08) | Overlay vacío es fallo conocido, no «fichero ausente» |
| `cumulo.tools_index` | Índice de tools ↔ YAML fuente | `sddia-qa verify-tools-index` | No construir `verify-cumulo-indices` aquí |
| `cerbero.config` | `execution-contexts.md` parseable + `.SddIA/cerbero/revoked_entities.json` JSON válido | `CERBERO_CONFIG_ERROR` en `cerbero_di_rbac.rs` | Integridad de config, **no** «disponibilidad» de un servicio |
| `kalma2.http` | `GET ${KALMA_URL}/` (o `/api/runtime-profile`) responde | `_wait_http` de ignición; rutas en el bridge | Runtime, no one-shot. Distinto de `Daemon_Heartbeat` |

### 3.5. La sonda reutiliza el tick de `event-sweeper`
Se **descarta** un centinela nuevo solo para el tick y se **descarta** `cron`. Extender `event-sweeper` con cadencia `SDDIA_VITALITY_PROBE_SECONDS` (default 300, piso 30). Precedente: el sweeper ya invoca `daemon-heartbeat-audit` vía `sddia-run.sh` (la Ceguera del sweeper ya está perforada; no se abre un frente nuevo). Actualizar capabilities del genoma vía `entity-manager`.

### 3.6. Veredicto negativo = fractura, no log
Sonda en rojo → `System_Fracture_Detected` en `eda_bus.pending`, `friction_id` + causa física, idempotente por incidente (reset cuando la sonda vuelve a verde). Prohibida rama silenciosa.

### 3.7. Un intervalo, un SSOT
El intervalo de latido vive **solo** en el genoma y viaja en lock + side-channel. Prohibido `SDDIA_HEARTBEAT_*` en bóveda. Bóveda reservada a cadencias que **no** dividen `missed_cycles` (`SDDIA_VITALITY_PROBE_SECONDS`, `SDDIA_EMAIL_POLL_SECONDS`, `SDDIA_GITHUB_BRIDGE_POLL_SECONDS`).

### 3.8. Metabolismo Adaptativo — condicionado (Fase 4)
Sin cambio respecto a v2: auto-declaración del intervalo; Radamanto observa (`Kaizen_Alert_Required`), no gobierna; SLO `3 × intervalo_máximo`; medir peaje de emisión antes de optimizar; si < 1 % del ciclo, archivar como no-mejora. Objeciones en §7.B.

## 4. Alcance

### Dentro
- Sincronizar `daemon-heartbeat.md` con los seis emisores reales del índice (incluidos `event-sweeper`, `email-watcher`, `iota-publish-relay`).
- Forja de `kalma2-bridge.md` + `DaemonRuntime` en el crate de interfaz + emisor; corrección del stop sobre lock inexistente (o escribir el lock de verdad).
- Extensión del auditor: PID muerto / lock huérfano → fractura.
- Proceso `system-vitality-probe` + clase `System_Vitality_Probed` + fila en `telemetry/index.md`.
- Extensión de `event-sweeper` y fractura ante sonda roja.
- Aduana de censo: cero side-channels/locks sin `{name}.md` correspondiente (hoy el único huérfano esperado es kalma2, y ni siquiera escribe lock).

### Fuera
- **Panel visual** — Espejo cerrado. No renderizar. No afirmar que este evento alimenta el panel actual.
- **Supervisión del relay IOTA** — hecha. Solo drift de contrato de emisores.
- **Validador universal de índices** — propuesta `DT-CUMULO-INDEX-AUDIT` (ID no registrado). La sonda consume `verify-tools-index` existente.
- **Reinicio automático** — `governance-daemon-manager` / `daemon-kill-switch`. La sonda percibe. systemd ya puede `Restart=` el puente.
- **Re-forjar launcher/unidad systemd de Kalma2** — ya existen.
- **Mover el crate** `interfaces/kalma2-bridge` → `daemons/`.
- **Métricas históricas.**
- **Gate de ignición HTTP de Kalma2** — ya implementado; no duplicar. Sí se añade probe periódico post-ignición.

### Entropía detectada (fuera)
- Agentes sin campo `type:`: el kernel lo exige; `agents-contract` no. Deuda de alineación kernel↔contrato, no de este PBI.
- `SddIA/core/event-subscriptions.json` coexiste con `event-domain-subscriptions.json` (SSOT Cúmulo: el segundo). Contenidos divergentes (p. ej. `compile-ecosystem-map-snapshot` solo en el SSOT). Extirpar el fósil en PBI aparte.

## 5. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
|----|----------|--------------|
| VIT-CA1 | `daemon-heartbeat.md` lista exactamente los stems catalogados en `SddIA/daemons/index.md` que emiten `Daemon_Heartbeat`. | Diff contrato vs índice; `hash_signature` recalculado. |
| VIT-CA2 | `kalma2-bridge.md` forjado (uuid, execution, fila en índice); el crate escribe lock + side-channel vía `DaemonRuntime`. | Presencia del `.md`; `.SddIA/daemons/status/kalma2-bridge.lock`; `.SddIA/daemons/state/heartbeats/kalma2-bridge.json` con proceso vivo. |
| VIT-CA2b | Cero ficheros en `heartbeats/` o `status/*.lock` sin `{name}.md` correspondiente (salvo stems SKIP: `index`, `daemons-contract`). | Diff runtime vs `list_indexed_daemon_ids`. |
| VIT-CA3 | PID muerto con lock huérfano de un id catalogado emite `System_Fracture_Detected` (idempotente). | `kill -9` de un centinela de prueba o kalma2 catalogado + sweep; evento en `eda_bus.pending`; segundo sweep no duplica. |
| VIT-CA3b | PID vivo que deja de latir sigue emitiendo fractura a `missed_cycles >= umbral` (regresión del camino ya vivo; p. ej. email-watcher). | Sweep + umbral SSOT, no hardcoded «3» en el criterio si el overlay cambia. |
| VIT-CA4 | Clase `System_Vitality_Probed` en `SddIA/events/telemetry/` con uuid y `event_family: telemetry`. | `{name}.md` + fila en `telemetry/index.md`. |
| VIT-CA5 | `system-vitality-probe` ejecuta las 4 sondas de §3.4 y devuelve veredicto por sonda con causa física. | `./sddia-run.sh --process system-vitality-probe` → JSON censo completo. |
| VIT-CA6 | Sabotear `execution-contexts.md` pone `cerbero.config` en rojo con el fichero en `error_trace`. | Smoke de sabotaje. |
| VIT-CA7 | Sonda en rojo → una fractura; verde → rojo otra vez → segunda fractura, no 3+. | Sabotaje → reparación → sabotaje. |
| VIT-CA8 | `event-sweeper` respeta `SDDIA_VITALITY_PROBE_SECONDS` (piso 30). | Unit del parseo. |
| VIT-CA9 | Tras Fase 2, `kalma2-bridge` aparece en el censo que itera `audit_staleness`. | `list_indexed_daemon_ids` incluye `kalma2-bridge`. |
| VIT-CA10 | Ignición: Kalma2 HTTP caído sigue siendo `[ERROR]` (no regress). Probe runtime `kalma2.http` en rojo fractura **después** de una ignición exitosa. | No reescribir `_wait_http`; smoke post-arranque tumbando el puerto. |
| VIT-CA11 | Cero `SDDIA_HEARTBEAT` en código y `.dev/.env.example`. | `rg 'SDDIA_HEARTBEAT'`. |

## 6. Orden de ejecución

### Fase 1 — Verdad del contrato
Sincronizar emisores de `daemon-heartbeat.md` con el índice (6 stems). Barata. Desbloquea auditorías posteriores contra contrato verdadero.

### Fase 2 — Censo de interfaz + muerte audible
Forja `kalma2-bridge.md` + `DaemonRuntime`. Extensión del auditor (§3.3). Aduana censo. IOTA no se reimplementa.

### Fase 3 — Invariantes no-proceso + HTTP runtime
Clase y proceso de sonda, cuatro sondas, sweeper, fractura. No tocar el panel de Espejo.

### Fase 4 — Metabolismo Adaptativo (condicionada)
No abrir sin medición §3.8. Título original conservado por trazabilidad: *[OPERATIVO] Metabolismo Adaptativo: Rango de Latido y Gobernanza de Radamanto*.

## 7. Refutación del borrador v1

Sin cambio de veredictos. Citas de línea de v2 se ignoran; valen los símbolos del §1 y §0.

### A. Especificación y clarificación

| Afirmación del borrador | Veredicto | Evidencia |
|-------------------------|-----------|-----------|
| Crear `System_Heartbeat_Emitted` | **Redundante** | `Daemon_Heartbeat` ya ocupa el rol. |
| El evento auditará nodos | **Error ontológico** | La clase es hecho; la auditoría es el proceso. |
| Auditará la aduana RBAC de Cerbero | **Inexacto** | Gate in-process; sonda = integridad de config. |
| Auditará índices de Cúmulo | **Capacidad parcial** | Solo `verify-tools-index` / `verify-process-integrity`. |
| Auditará el puente Kalma2 | **Confirmado como déficit residual** | Sin genoma ni `DaemonRuntime`; launcher/systemd/HTTP-gate de ignición **sí** existen. |
| «Chispazo de Nivel 1» / familia `progress` | **Vocabulario mixto** | Familias: `telemetry` \| `orchestration` \| `domain`. `progress` es ruta de bus. |
| Radamanto suscrito al latido / emite `System_Degraded` | **Colisión / clase inexistente** | Suscriptor: Argos. Clases reales: `System_Fracture_Detected`, `Domain_Entity_Degraded`. |
| Contrato del evento en `events-contract.md` | **Ruta equivocada** | Clase en `events/{family}/{name}.md`. |
| Centinela cron nuevo | **Redundante** | `event-sweeper` ya tiene tick. |

### B. Metabolismo Adaptativo

| Afirmación del borrador | Veredicto | Evidencia |
|-------------------------|-----------|-----------|
| `SDDIA_HEARTBEAT_MIN_MS` / `MAX_MS` | **Unidad incoherente** | Todo el mecanismo en segundos; piso 5 s. |
| Alojarlas en `.dev/.env` | **Bóveda equivocada** | `env_hierarchy`: global `.dev/.env` → instancia `.SddIA/.dev/.env`. |
| Intervalo de latido en env | **Doble SSOT** | El intervalo es el divisor de `missed_cycles`. |
| Radamanto relaja/acelera el pulso | **Capacidad inexistente** | Sin `set_var` en Radamanto; relajar alarga la ceguera a `3 × max`. |
| Acelerar el pulso mejora Kalma2 | **Premisa falsa** | Latencia de estímulo: poll del bus (`POLL_SECONDS = 2` en `event-watcher`) y long-poll sensorial, no el latido. |
| Relajar ahorra ciclos | **No cuantificado** | Medir antes (§3.8). |
| Ticks por bóveda | **Inexistentes** | `HEARTBEAT_TICK_SECONDS = 10` y `HEARTBEAT_EMIT_FAIL_BUDGET = 5` son constantes Rust. |

## 8. Refutación de v2.0.0 (este refinamiento)

| Afirmación v2 | Veredicto | Corrección |
|---------------|-----------|------------|
| Relay sin genoma / PBI DLT en vuelo | **Territorio superado** | Genoma y supervisión cerrados. Este PBI no desbloquea DLT. |
| Espejo espera `System_Vitality_Probed` | **Contrato inventado** | Espejo consume artefactos de instancia ya fusionados. |
| Muerte → fractura a 3 ciclos | **Semántica inventada** | El auditor salta PID muerto. Fase 2 debe cambiarlo o el CA es mentira. |
| Kalma2 = centinela como los watchers | **Colisión con Ceguera** | Órgano de interfaz catalogado; crate y systemd se quedan. |
| «Cinco» órganos con genoma | **Conteo caducado** | Seis. |
| Ceguera de 3 días del relay como débito abierto de este PBI | **Lección histórica, no alcance** | Causa (hijo de terminal sin supervisor) ya corregida en DLT. La lección que **sigue** abierta es kalma2 sin censo y el silencio ante PID muerto. |

**Conclusión v2.1.0:** no falta un segundo latido. Falta (1) meter el puente de interfaz en el censo sin falsear su ontología, (2) hacer audible la muerte además del cuelgue, (3) sondear lo que no tiene PID, (4) alinear el contrato de emisores con los seis centinelas reales. El panel ya existe; no se le inventa un proveedor.
