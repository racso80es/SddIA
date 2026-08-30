---
document_id: PBI-FIX-FRACTURE-701c77ebeab8-OLA1
uuid: "68054549-3db0-452c-bfa4-9b978727e1ae"
title: "[FIX] iota-publish-relay — Ola 1 latido degradado"
format: markdown
version: "1.0.0"
created: "2026-08-30"
updated: "2026-08-30"
status: abierto
refinement_status: clarified
priority: alta
process: bug-fix
type: fix
dispatch: false
parent_pbi: PBI-FIX-FRACTURE-701c77ebeab8
parent_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (701c77ebeab8).md
suggested_branch: fix/iota-relay-heartbeat-degraded
persist_ref_suggested: docs/fixes/iota-relay-heartbeat-degraded
fracture_hash: 701c77ebeab8
fracture_process: route-domain-event
incident_ref: "System_Fracture_Detected — 701c77ebeab8"
regression_of: null
friction_ids:
  - F-DLT-HEARTBEAT-HUECO
  - F-ESPEJO-VERDE-FALSO
  - F-DLT-RELAY-SIN-SUPERVISOR
architectural_constraints:
  - A-LATIDO-IGUAL-SERVICIO
  - A-NO-VERDE-SIN-BINDING
  - A-EMISION-PERIODICA
  - A-STATUS-CONTRATO-61
execution_file_lock:
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/ecosystem-health/src/lib.rs
  - SddIA/daemons/iota-publish-relay/src/main.rs
gates_this_wave:
  - RELAY-CA9
  - RELAY-CA10
  - RELAY-CA11
  - RELAY-CA12
related:
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/ecosystem-health/src/lib.rs
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/daemons/daemons-contract.md
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (701c77ebeab8).md
  - docs/fixes/iota-relay-supervisor-impatient-health/
source_audit: "2026-08-30 Ola 0 cerrada (PR #233, RELAY-CA1/CA4). I1/I2 del padre siguen vigentes: emit_heartbeat privada status=alive; record_heartbeat_at ignora payload.status; color_daemon green solo por last_heartbeat_at+missed<3. Omisión de tick() viola daemons-contract §6.1."
review_notes: "v1.0.0 absorbe deuda explícita del padre. Ola 1 = cadena degraded (runtime+audit+espejo) y restauración de emisión periódica. CA5–CA7 en este PBI, no en esta ola. DT-DLT-RELAY-NODE y prompt_adjustment fuera."
---

# [FIX] iota-publish-relay — Ola 1 latido degradado

Absorbe la deuda explícita de `PBI-FIX-FRACTURE-701c77ebeab8` (Ola 0 APTO). Mismo `fracture_hash` para que Cúmulo deduplique a este abierto (`already_open`) y no abra `[REGRESIÓN]`.

## Linaje

| Campo | Valor |
|-------|--------|
| Padre | `PBI-FIX-FRACTURE-701c77ebeab8` (Ola 0: gracia + omit-tick; `validacion.md` APTO) |
| PR padre | https://github.com/racso80es/SddIA/pull/233 |
| Hueco | I1/I2: `status: degraded` no apaga el espejo; omit-tick viola §6.1 |

Ola 0 **no** se reabre. CA1 (gracia) permanece.

---

## 0. Laudo Ola 1

**Intención:** cablear `daemons-contract` §6.1 (`alive` \| `degraded` \| `shutting_down`) hasta el espejo. Restaurar `centinela.tick()` periódico. Palanca CA4 honesta = `status`, no omisión de latido.

### 0.1. Hechos (Filtro A)

| Tesis | Veredicto | Base |
|-------|-----------|------|
| `emit_heartbeat` escribe `"status": "alive"` siempre | **Hecho** | `sddia-daemon-runtime` L341–364; fn **privada**; `tick()` no acepta status |
| `record_heartbeat_at` ignora `payload.status` | **Hecho** | `daemon_heartbeat.rs` L198–206: `missed_cycles=0`, `classification: healthy` |
| Espejo green sin leer status | **Hecho** | `color_daemon`: green si `last_heartbeat_at` y `missed < 3` |
| Omit-tick de Ola 0 viola §6.1 | **Hecho** | emisión periódica obligatoria; Argos interpreta *latido omitido* (email-watcher), no «hijo HTTP caído» |

### 0.2. Decisiones

1. **API:** `DaemonRuntime::tick(&top, status)` o `tick_with_status`; `emit_heartbeat` usa el `status` inyectado (`alive` \| `degraded` \| `shutting_down`). Side-channel y payload ECST alineados.
2. **Audit:** `record_heartbeat_at` persiste `payload.status`. Si `degraded`: `classification` ≠ `healthy` (p. ej. `degraded`); **no** reset de semántica «servicio HTTP OK». `missed_cycles` sigue 0 (el latido **sí** llegó).
3. **Espejo:** `color_daemon` lee `status`. `degraded` ⇒ no `green` / no `heartbeat_ok` (amarillo o equivalente; `reason: heartbeat_degraded`). `missed_cycles >= 3` sigue `red`.
4. **Relay:** post-gracia, `/health` false → `tick(..., "degraded")` **y** kill+respawn (CA1 intacta). `/health` true o en gracia → `tick(..., "alive")`. **Prohibido** omitir `tick()` salvo shutdown.
5. Genoma `{name}.md` (`iota-publish-relay.md`, `daemons-contract.md`): solo si el contrato YAML cambia; vía `entity-manager` (DA-2). Ola 1 **no** exige mutar el contrato: §6.1 ya admite `degraded`.

### 0.3. Contrato de tick (sustituye omit-tick de Ola 0)

```
cada tick:
  spawn hijo si hace falta (backoff inalterado)
  health_ok ← probe_health
  in_grace ← child.is_some() ∧ child_spawned_at.elapsed() < GRACE_SECS
  status ← (health_ok ∨ in_grace) ? alive : degraded
  centinela.tick(status)          # siempre, salvo shutdown
  si no health_ok ∧ no in_grace ∧ child vivo → kill
```

---

## 1. Alcance

### Ola 1 (esta ola — gates CA9–CA12)

| Archivo | Cambio |
|---------|--------|
| `sddia-daemon-runtime/src/lib.rs` | status inyectable en heartbeat |
| `daemon_heartbeat.rs` | persistir y clasificar `degraded` |
| `ecosystem-health/src/lib.rs` | `color_daemon` consume status |
| `iota-publish-relay/src/main.rs` | tick siempre; `degraded` post-gracia si `/health` false |

Tests: runtime (status en payload); audit (degraded ≠ healthy); espejo (no green); relay (`emit_heartbeat` true en rama post-gracia; `kill` sigue).

### Misma deuda, olas posteriores de este PBI (no borrar)

| ID | Deuda | Ola |
|----|-------|-----|
| RELAY-CA2 / CA3 | E2E bind/respawn (efecto CA1; journal) | 1b observación |
| RELAY-CA5 | Log hijo no vacío tras bind | 1b |
| RELAY-CA6 | Cola `eda_instance.dlt_reanchor` drena cuando publish OK | 1b instancia |
| RELAY-CA7 | Taxonomía: refused con supervisor vivo ≠ `F-DLT-RELAY-SIN-SUPERVISOR` ciego (`route_domain_core.rs`) | 2 |

### Fuera de este PBI

- Sustitución Node→Rust (`DT-DLT-RELAY-NODE`).
- `prompt_adjustment` / Kintsugi operador (refutado en el padre).
- Reabrir Ola 0 / revertir `GRACE_SECS`.

---

## 2. Criterios de aceptación (Ola 1)

| ID | Criterio | Verificación |
|----|----------|--------------|
| RELAY-CA9 | `tick`/`emit_heartbeat` acepta `status`; payload y side-channel no fuerzan `alive`. | Test crate runtime |
| RELAY-CA10 | `Daemon_Heartbeat` con `status: degraded` → audit `classification` degradada; `missed_cycles` no sube. | Test `record_heartbeat_at` |
| RELAY-CA11 | Espejo: `degraded` + `missed < 3` ⇒ **no** `green`/`heartbeat_ok`. `reason: heartbeat_degraded` (o equivalente estable). | Test `color_daemon` |
| RELAY-CA12 | Relay post-gracia `/health` false: **sí** `tick(degraded)`; **sí** kill. En gracia o health OK: `tick(alive)`. | Test fn decisión + bucle |

Cierre documental del `bug-fix` (PBI a `done/`, `validacion.md` APTO) cuando CA9–CA12 verdes. CA5–CA7 pueden quedar DIFERIDO en `validacion.md` si 1b/2 no viajan en el mismo PR; no se borran.

## Criterio de cierre

- [ ] Ola 1: RELAY-CA9…CA12
- [ ] CA5–CA7 APTO o DIFERIDO explícito con PBI/ola siguiente
- [ ] Argos APTO + este PBI en `docs/todos/done/` en la rama del PR único
