---
document_id: PBI-FIX-FRACTURE-701c77ebeab8
uuid: "4c18aeb3-66d9-4b94-8c43-bde65cb430a8"
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.2.0"
created: "2026-08-30"
updated: "2026-08-30"
status: cerrado
refinement_status: clarified
priority: alta
process: bug-fix
type: fix
dispatch: false
suggested_branch: fix/iota-relay-supervisor-impatient-health
persist_ref_suggested: docs/fixes/iota-relay-supervisor-impatient-health
fracture_hash: 701c77ebeab8
fracture_process: route-domain-event
incident_ref: "System_Fracture_Detected — 701c77ebeab8"
friction_ids:
  - F-DLT-SUPERVISOR-IMPACIENTE
  - F-DLT-HEARTBEAT-HUECO
  - F-ESPEJO-VERDE-FALSO
architectural_constraints:
  - A-LATIDO-IGUAL-SERVICIO
  - A-GRACE-POST-SPAWN
  - A-NO-VERDE-SIN-BINDING
  - A-OLA0-SOLO-MAIN-RS
execution_file_lock:
  - SddIA/daemons/iota-publish-relay/src/main.rs
gates_this_wave:
  - RELAY-CA1
  - RELAY-CA4
related:
  - SddIA/daemons/iota-publish-relay.md
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/daemons/daemons-contract.md
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/ecosystem-health/src/lib.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - .SddIA/services/iota-publish-relay/server.mjs
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/todos/done/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md
  - docs/todos/done/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
source_audit: "2026-08-30 territorio: refused 8787 + murder-loop journal. 2026-08-30 clarificación: leídos DaemonRuntime::emit_heartbeat (status cableado alive, fn privada), record_heartbeat_at (ignora status, missed_cycles=0), color_daemon (green solo por last_heartbeat_at + missed<3), daemons-contract §6.1 (status alive|degraded|shutting_down; emisión periódica obligatoria)."
review_notes: "v1.0.0 semilla. v1.1.0 diagnóstico territorial. v1.2.0 clarificación: Ola 0 = CA1+CA4 en main.rs; Filtro A sobre 'degraded' vs espejo; omisión de tick como único palanca en este archivo."
---

# [FIX] route-domain-event — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Emisor | `execute-process` |
| Acción intentada | `merkle-batch-preseal` |

## Traza de error

```
merkle-batch-preseal failed: iota-relay-unreachable: http://127.0.0.1:8787/v1/publish: Connection Failed: Connect error: Connection refused (os error 111)
```

Prefijo de fricción `F-DLT-RELAY-SIN-SUPERVISOR`: hipótesis, no hecho (supervisor vivo; HTTP ausente).

## Mandato

Corregir la causa raíz. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

---

## 0. Clarificación v1.2.0 (laudo)

**Intención del Vértice Biológico:** Ola 0 = mutar **solo** `SddIA/daemons/iota-publish-relay/src/main.rs`. Gates: **RELAY-CA1** y **RELAY-CA4**. Sin mutación de genoma `{name}.md` ni de `sddia-daemon-runtime` en esta ola. Código propuesto **antes** de aplicar el parche (este documento fija el qué; Dedalo/Tekton el cómo en el ciclo `bug-fix`).

### 0.1. Filtro A — lo que el razonamiento acierta

| Tesis | Veredicto | Base |
|-------|-----------|------|
| Kill en el mismo tick que `spawn` porque `connection refused` es instantáneo | **Hecho** | `main.rs` L199–226: spawn → `tick` → `probe_health` → kill. Timeout 1500 ms no aplica a refused. Journal: pares mismo segundo. |
| Gracia post-spawn: prohibido kill por refused dentro de la ventana | **Coherente con CA1** | Cubre el bind de `server.mjs` (listen < 1 s medido a mano). |
| `centinela.tick()` hoy no mira `/health` | **Hecho** | `tick()` → `emit_heartbeat`; payload `status: "alive"` fijo. |

### 0.2. Filtro A — alucinaciones / incoherencias / inexactitudes

**I1 — `status: degraded` no apaga el espejo.**  
`daemons-contract` §6.1 ya admite `alive` \| `degraded` \| `shutting_down`. Eso **no** está cableado al espejo:

- `DaemonRuntime::emit_heartbeat` es **privada** y escribe `"status": "alive"` siempre. Desde `main.rs` no hay API para degradar.
- `record_heartbeat_at` **ignora** `payload.status`. Cualquier `Daemon_Heartbeat` pone `missed_cycles: 0` y `classification: healthy`.
- `color_daemon` (ecosystem-health): `green` / `heartbeat_ok` si hay `last_heartbeat_at` y `missed_cycles < 3`. **No lee `status`.**

Afirmar «emitir `degraded` ⇒ el espejo deja de ser `heartbeat_ok`» es **falso** con el genoma actual. Exigiría mutar runtime + audit + espejo: **fuera de Ola 0**.

**I2 — Omitir `tick()` sí apaga el verde y viola el contrato de emisión.**  
Única palanca **dentro de `main.rs`** para RELAY-CA4 observable (espejo no green): **no llamar** `centinela.tick()` cuando `/health` falla fuera de gracia → `missed_cycles` crece → umbral 3 (`heartbeat-audit.thresholds.json`) → `red` / `missed_cycles`.

Coste: `daemons-contract` §6.1 **obliga** emisión periódica. Argos interpretará la omisión como fractura de *latido omitido* (mismo canal que `email-watcher`), no como «hijo HTTP caído». Latencia del rojo: `3 × heartbeat_interval_seconds` = **90 s** (genoma del daemon: 30 s).

**I3 — `bootstrap()` emite un latido vivo antes de existir hijo.**  
Un pulso `alive` al arrancar es inevitable sin tocar runtime. CA4 habla de fallo **sostenido post-gracia**; ese pulso (≤ 30 s) queda **fuera de CA4**.

**I4 — RELAY-CA2…CA8 no caben en «solo `main.rs` + CA1/CA4».**  
CA2/CA3 son consecuencias empíricas de CA1 si el kill post-gracia se conserva (respawn). CA5–CA7 tocan logs, cola DLT y taxonomía en `route_domain_core.rs`. CA8 es cierre documental del proceso `bug-fix`, no del bucle. Ola 0 **no** cierra el PBI entero; cierra los dos gates. El resto queda **deuda explícita**, no se borra.

**I5 — Directriz Raw Kernel «solo Rust, sin explicación».**  
Aplica a la **fase Ejecución** de Tekton sobre `main.rs`, no a esta clarificación. Mayeuta no incrusta el binario del parche en el PBI. DA-2: `directories.daemons` es genoma; el parche de crate se aplica en ciclo `bug-fix` autorizado, no a mano ahora.

**I6 — `last_restart` no es el reloj de gracia.**  
Hoy se actualiza también en **spawn fallido**. La gracia debe anclarse a `Instant` de **spawn exitoso** (`child_spawned_at`), no al backoff de error.

### 0.3. Decisiones tomadas (Ola 0)

1. **CA1:** `GRACE_SECS >= TICK_SECS` (5). Valor de trabajo: **10 s** (2 ticks). Mientras `child_spawned_at.elapsed() < GRACE` **prohibido** `kill` por fallo de `/health` (incluido refused).
2. **CA4 (Ola 0, único medio honesto en `main.rs`):** si `/health` es false **y** no hay gracia activa → **no** invocar `centinela.tick()`. Si `/health` es true → `tick()`. Durante gracia, `tick()` permitido (arranque).
3. **Post-gracia:** se **conserva** el kill+respawn actual (restaura DLT-CA5). CA1 solo veta el kill *dentro* de la ventana.
4. **`status: degraded`:** deuda **Ola 1** (`sddia-daemon-runtime` + `daemon-heartbeat-audit` + `ecosystem-health`). No se finge en Ola 0.
5. Primera mutación de código = únicamente el archivo bajo `execution_file_lock`. Este PBI no es el parche.

### 0.4. Contrato del bucle (qué; no es el parche)

```
cada tick:
  spawn hijo si hace falta (backoff inalterado)
  health_ok ← probe_health
  in_grace ← child.is_some() ∧ child_spawned_at.elapsed() < GRACE_SECS
  si health_ok ∨ in_grace → centinela.tick()
  si no health_ok ∧ no in_grace ∧ child vivo → kill (respawn en ticks siguientes)
```

Tests mínimos en el mismo crate: (a) probe fallido con `elapsed < GRACE` no implica kill; (b) probe fallido con `elapsed >= GRACE` implica kill; (c) `tick` no se llama en rama (b). Extraer la decisión a fn pura testeable si el `Child` real no cabe en unitario.

---

## 1. Causa raíz verificada — supervisor impaciente

Traza = **transporte real** (refused 8787). No payload fósil, no HTTP 500, no operador IA.

El centinela Rust vive; el hijo Node no llega a `listen` porque el supervisor lo mata en el tick del spawn.

Cadena: `merkle-batch-preseal` → refused → cola re-anclaje + fractura → systemd `sddia-iota-publish-relay@…` active → loop spawn/probe/kill → `tick()` pinta el espejo green.

`server.mjs` tiene `GET /health` 200. Node manual imprime *listening*. Log de hijo a 0 bytes: muerte previa a `listen`.

### 1.1. Territorio (2026-08-30 ~10:33–10:36 CEST)

| Vector | Observación |
|--------|-------------|
| Puerto 8787 | Sin listener; curl refused |
| Lock `iota-publish-relay` | pid 5404, interval 30 s |
| Journal unidad | spawn→kill mismo segundo, cada 5 s |
| Espejo | `green` / `heartbeat_ok` |
| Cola re-anclaje | 7 JSON, todas refused (os error 111) |
| `relay.log` instancia | mtime 2026-08-27 19:54 |

---

## 2. Linaje (no reabrir)

| Antecesor | Hueco respecto a 701c77ebeab8 |
|-----------|-------------------------------|
| Kaizen DLT DLT-CA5 | Implementado como kill al primer probe fallido |
| `6a49e0ad310e` / R1 | Binario fósil; esta traza es refused real |
| `b3a715381787` | 500 en relay vivo; aquí no hay HTTP |
| Kaizen auto `prompt_adjustment` | Ortogonal |

Hash nuevo: no `regression_of`. Regresión **semántica** de DLT-CA5.

---

## 3. Alcance

### Ola 0 (esta clarificación — gates CA1 y CA4)

- Archivo: `SddIA/daemons/iota-publish-relay/src/main.rs` (crate ya forjado).
- Gracia post-spawn + omisión de `tick()` post-gracia si `/health` falla.
- Tests unitarios del crate que demuestren CA1 y CA4-Ola0.

### Fuera de Ola 0 (deuda; no borrar)

- Ola 1: `emit_heartbeat(status)` público; audit/espejo consumen `degraded` (I1/I2).
- RELAY-CA2, CA3 (verificación E2E de bind/respawn — se esperan como efecto de CA1, no se auditan aquí).
- RELAY-CA5 log hijo; CA6 cola re-anclaje; CA7 taxonomía `F-DLT-RELAY-SIN-SUPERVISOR`.
- Mutación de `iota-publish-relay.md`, `sddia-daemon-runtime`, `route_domain_core.rs`.
- Sustitución Node→Rust (`DT-DLT-RELAY-NODE`).
- `prompt_adjustment` / Kintsugi operador.

---

## 4. Criterios de aceptación (Ola 0)

| ID | Criterio | Verificación |
|----|----------|--------------|
| RELAY-CA1 | Tras spawn exitoso, **prohibido** kill por fallo de `/health` mientras `elapsed < GRACE_SECS` (10 s). El refused instantáneo del mismo tick **no** mata al hijo. | Test de la fn de decisión; journal post-parche: no hay par spawn/kill en el mismo segundo en arranque sano. |
| RELAY-CA4 | Tras gracia, si `/health` sigue en falso: **prohibido** `centinela.tick()`. El espejo deja de ser `heartbeat_ok`/`green` cuando `missed_cycles >= 3` ( palanca real; ver §0.2 I1–I2). | Test: rama post-gracia no llama tick. Runtime: `heartbeat-audit.json` / ecosystem-health `reason: missed_cycles` con 8787 caído de forma sostenida. |

Cierre documental del `bug-fix` (PBI a `done/`, `validacion.md` APTO) aplica cuando se ejecute el ciclo; **no** es gate de producto de Ola 0 aislada.

---

## 5. Conclusión Analítica y Propuesta Evolutiva

*(Mayeuta — v1.2.0)*

### Diagnóstico

Supervisor impaciente + latido del wrapper desacoplado del HTTP. Traza 111 = puerto vacío, no ausencia de proceso.

### Veredicto

`process_fix` (daemon `iota-publish-relay`). `prompt_adjustment` refutado.

### Ola 0

Gracia + no-tick post-gracia. No fingir `degraded` hasta Ola 1.

> Genoma/crate: ciclo `bug-fix` + DA-2. Este documento no muta `main.rs`.

## Criterio de cierre (PBI completo)

- [x] Ola 0: RELAY-CA1 y RELAY-CA4
- [ ] Deuda Ola 1 / CA5–CA7 aceptada o absorbida en PBI hijo
- [x] Argos APTO + PBI en `docs/todos/done/` en la rama del PR único
