---
feature_name: iota-relay-heartbeat-degraded
created: "2026-08-30"
process: bug-fix
base: main
scope: ola1-ca9-ca12-heartbeat-degraded
branch_name: fix/iota-relay-heartbeat-degraded
persist_ref: docs/fixes/iota-relay-heartbeat-degraded
pbi_ref: docs/todos/pending/[FIX] iota-publish-relay — Ola 1 latido degradado (701c77ebeab8).md
document_id: PBI-FIX-FRACTURE-701c77ebeab8-OLA1
execution_id: "39567569-6670-42d6-8174-116954dda036"
parent_persist_ref: docs/fixes/iota-relay-supervisor-impatient-health
---

# Especificación — Ola 1 latido `degraded` (`701c77ebeab8`)

## Problema

Ola 0 (PR #233, `RELAY-CA1`/`CA4`) dejó el espejo ciego al estado del hijo HTTP. Tres hechos vigentes:

| Tesis | Base |
|-------|------|
| `emit_heartbeat` escribe `"status": "alive"` siempre; `tick()` no acepta status | `sddia-daemon-runtime` `emit_heartbeat` privada L341–364 |
| `record_heartbeat_at` ignora `payload.status`; `missed_cycles=0`, `classification: healthy` | `daemon_heartbeat.rs` L198–206 |
| `color_daemon` pinta `green`/`heartbeat_ok` si hay `last_heartbeat_at` y `missed < 3` | `ecosystem-health` L348–360 |

La palanca CA4 (omitir `tick()` post-gracia) viola `daemons-contract` §6.1: emisión periódica obligatoria; `status` ∈ `alive` \| `degraded` \| `shutting_down`. Argos interpreta *latido omitido* (inanición), no «hijo HTTP caído».

Ola 0 **no** se reabre. `GRACE_SECS=10` y kill+respawn permanecen.

## Cambio requerido (Ola 1)

Cadena única: **runtime → audit → espejo → relay**. Palanca CA4 honesta = `status: degraded`, no omisión.

### 1. Runtime — `sddia-daemon-runtime`

| Decisión | Contrato |
|----------|----------|
| API | `tick_with_status(&top, status: &str)`. `tick(&top)` permanece y delega a `tick_with_status(top, "alive")` — otros centinelas **no** se tocan (`execution_file_lock`). |
| `emit_heartbeat` | Recibe `status`. Payload ECST y side-channel usan el mismo valor. **Prohibido** forzar `"alive"`. |
| Enum | Solo `alive` \| `degraded` \| `shutting_down`. Valor inválido → `Err` (no mentir `alive`). |
| `bootstrap` | Sigue emitiendo `alive` (pulso pre-hijo; I3 del padre). |

### 2. Audit — `daemon_heartbeat.rs`

`record_heartbeat_at` persiste `payload.status` en la entrada del daemon.

| `payload.status` | `classification` | `missed_cycles` |
|------------------|------------------|-----------------|
| `alive` (o ausente legado) | `healthy` / `recovered` si `had_fracture` (semántica actual) | 0 |
| `degraded` | `degraded` (**≠** `healthy`) | 0 |
| `shutting_down` | `shutting_down` | 0 |

El latido **sí** llegó: no incrementar `missed_cycles`. No reset de semántica «servicio HTTP OK» cuando `degraded`.

### 3. Espejo — `color_daemon`

Lee `status` de la entrada de audit. Precedencia:

1. `revoked` → `red` / `revoked` (inalterado)
2. `missed_cycles >= 3` → `red` / `missed_cycles` (inalterado)
3. `status == "degraded"` y `missed < 3` → **no** `green` / **no** `heartbeat_ok`. Color `yellow`. `reason: heartbeat_degraded`
4. `status == "shutting_down"` y `missed < 3` → no `green`. Color `yellow`. `reason: heartbeat_shutting_down`
5. `has_hb` y (`status` ausente legado o `alive`) y `missed < 3` → `green` / `heartbeat_ok` (compat)

### 4. Relay — `iota-publish-relay/src/main.rs`

Sustituye omit-tick de Ola 0. CA1 intacta.

```
cada tick:
  spawn hijo si hace falta (backoff inalterado)
  health_ok ← probe_health
  in_grace ← child.is_some() ∧ child_spawned_at.elapsed() < GRACE_SECS
  status ← (health_ok ∨ in_grace) ? alive : degraded
  centinela.tick_with_status(status)   # siempre, salvo shutdown
  si no health_ok ∧ no in_grace ∧ child vivo → kill
```

`SupervisorTickAction`: `heartbeat_status: &'static str` + `kill_child: bool`. `emit_heartbeat: bool` desaparece.

| `health_ok` | `in_grace` | `child_alive` | `heartbeat_status` | `kill` |
|-------------|------------|---------------|--------------------|--------|
| true | * | * | `alive` | no |
| false | true | * | `alive` | no |
| false | false | true | `degraded` | sí |
| false | false | false | `degraded` | no |

### Fuera de alcance (deuda; no borrar)

| ID | Deuda | Ola |
|----|-------|-----|
| RELAY-CA2 / CA3 | E2E bind/respawn (efecto CA1; journal) | 1b observación |
| RELAY-CA5 | Log hijo no vacío tras bind | 1b |
| RELAY-CA6 | Cola `eda_instance.dlt_reanchor` drena cuando publish OK | 1b instancia |
| RELAY-CA7 | Taxonomía: refused con supervisor vivo ≠ `F-DLT-RELAY-SIN-SUPERVISOR` | 2 |
| — | Mutación genoma `{name}.md` / `daemons-contract.md` (§6.1 ya admite `degraded`) | — |
| — | Otros centinelas (siguen `tick()` → `alive`) | — |
| — | Reabrir Ola 0 / revertir `GRACE_SECS` | — |
| — | `DT-DLT-RELAY-NODE` / `prompt_adjustment` | — |

## Criterios de aceptación (Ola 1)

| ID | Criterio |
|----|----------|
| RELAY-CA9 | `tick_with_status` / `emit_heartbeat` aceptan `status`; payload y side-channel no fuerzan `alive`. `tick()` sin status ≡ `alive`. |
| RELAY-CA10 | `Daemon_Heartbeat` con `status: degraded` → audit `classification: degraded`; `missed_cycles` no sube. |
| RELAY-CA11 | Espejo: `degraded` + `missed < 3` ⇒ **no** `green`/`heartbeat_ok`. `reason: heartbeat_degraded`. |
| RELAY-CA12 | Relay post-gracia `/health` false: **sí** `tick(degraded)`; **sí** kill. En gracia o health OK: `tick(alive)`. |

Cierre documental del `bug-fix` (PBI a `done/`, `validacion.md` APTO) cuando CA9–CA12 verdes. CA5–CA7 = DIFERIDO explícito si 1b/2 no viajan en el mismo PR.

## Corte de este commit

Diseño (`spec.md` + `plan.md`) + commit. Sin parche Rust. Sin `delivery-close-cycle`.
