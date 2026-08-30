---
feature_name: iota-relay-supervisor-impatient-health
created: "2026-08-30"
process: bug-fix
base: main
scope: ola0-ca1-ca4-supervisor-grace-omit-tick
branch_name: fix/iota-relay-supervisor-impatient-health
persist_ref: docs/fixes/iota-relay-supervisor-impatient-health
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (701c77ebeab8).md
document_id: PBI-FIX-FRACTURE-701c77ebeab8
execution_id: "dd623714-7946-4eef-bc25-6dd67f3c2ce3"
---

# Especificación — fractura `701c77ebeab8` (supervisor impaciente)

## Problema

`merkle-batch-preseal` falló con:

```
iota-relay-unreachable: http://127.0.0.1:8787/v1/publish: Connection Failed: Connect error: Connection refused (os error 111)
```

El centinela Rust vive. El hijo Node no llega a `listen`: el bucle hace `spawn` → `tick()` → `probe_health` → `kill` en el **mismo tick**. `TcpStream::connect_timeout` con refused es instantáneo; `HEALTH_TIMEOUT_MS` (1500) no aplica. Journal: pares spawn/kill el mismo segundo, cada `TICK_SECS` (5). El espejo pinta `green` / `heartbeat_ok` porque `tick()` emite `status: "alive"` siempre.

Distinto de `6a49e0ad310e` (payload fósil) y `b3a715381787` (HTTP 500 con relay vivo). Aquí no hay HTTP.

## Cambio requerido (Ola 0)

| Área | Artefacto | Vía |
|------|-----------|-----|
| Gracia post-spawn + omisión de latido | `SddIA/daemons/iota-publish-relay/src/main.rs` | parche crate (ciclo `bug-fix`; `execution_file_lock`) |
| Tests CA1/CA4 | mismo archivo, `#[cfg(test)]` | fn pura; sin `Child` real |

Un único archivo. Prohibido mutar `sddia-daemon-runtime`, `ecosystem-health`, `iota-publish-relay.md`, `route_domain_core.rs`.

### Contrato de tick (laudo v1.2.0)

```
cada tick:
  spawn hijo si hace falta (backoff inalterado)
  health_ok ← probe_health
  in_grace ← child.is_some() ∧ child_spawned_at.elapsed() < GRACE_SECS
  si health_ok ∨ in_grace → centinela.tick()
  si no health_ok ∧ no in_grace ∧ child vivo → kill (respawn en ticks siguientes)
```

| Constante | Valor | Invariante |
|-----------|-------|------------|
| `TICK_SECS` | 5 (existente) | no tocar |
| `GRACE_SECS` | **10** | `>= TICK_SECS`; 2 ticks |
| Reloj de gracia | `child_spawned_at: Option<Instant>` | solo spawn **exitoso**; no `last_restart` (ese también se actualiza en spawn fallido) |
| Comparación | `elapsed < GRACE_SECS` | igualdad (`== 10 s`) **fuera** de gracia |

### Tabla de decisión

| `health_ok` | `in_grace` | `child_alive` | `tick()` | `kill` |
|-------------|------------|---------------|----------|--------|
| true | * | * | sí | no |
| false | true | * | sí | no |
| false | false | true | **no** | sí |
| false | false | false | no | no |

Pulso `alive` de `bootstrap()` (pre-hijo) queda fuera de CA4 (I3). No fingir `status: degraded` (I1; Ola 1).

## Criterios de aceptación (Ola 0)

| ID | Criterio |
|----|----------|
| RELAY-CA1 | Tras spawn exitoso, prohibido kill por fallo de `/health` mientras `elapsed < GRACE_SECS`. Refused del mismo tick no mata al hijo. |
| RELAY-CA4 | Tras gracia, si `/health` sigue en falso: prohibido `centinela.tick()`. Palanca real: `missed_cycles >= 3` → espejo `red` / `missed_cycles` (latencia 90 s). |
| T-A | Test: probe fallido + `elapsed < GRACE` → no kill, sí tick. |
| T-B | Test: probe fallido + `elapsed >= GRACE` → kill. |
| T-C | Test: rama T-B no emite heartbeat. |

## Fuera de alcance (deuda; no borrar)

- Ola 1: `emit_heartbeat(status)` público; audit/espejo consumen `degraded`.
- RELAY-CA2/CA3 (E2E bind/respawn — efecto esperado de CA1, no auditados aquí).
- RELAY-CA5 log hijo; CA6 cola re-anclaje; CA7 taxonomía `F-DLT-RELAY-SIN-SUPERVISOR`.
- Mutación de genoma `{name}.md`, runtime, `route_domain_core.rs`.
- Sustitución Node→Rust (`DT-DLT-RELAY-NODE`).
- Cierre documental / `delivery-close-cycle` en **este** commit (corte: spec+plan).
