---
feature_name: kaizen-aduana-dlt-relay-supervisado
created: "2026-08-27"
process: feature
phases: "T0-docs,T1-forge-daemon,T2-supervisor-crate,T3-ignition,T4-fase0-rescue,T5-cause-fracture,T6-queue,T7-fossil-regime,T8-tests-smoke,T9-aduana-doc"
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
persist_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
branch_name: feat/kaizen-aduana-dlt-relay-supervisado
execution_id: "a7f3e291-6c4b-4d8e-9a1f-3b5e7c8d0e2f"
dedalo_verdict: ok
pbi_ref: docs/todos/pending/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
---

# Plan — kaizen-aduana-dlt-relay-supervisado

Orden no negociable: **rescate (T4) antes de reforma estructural completa**, pero T1–T3 (supervisor vivo + health) son **prerrequisito físico** del rescate. Un PR. Git solo `skill:git-manager`.

## T0 · Documentación (esta entrega Dedalo)

- `clarify.md`, `objectives.md`, `spec.md`, `plan.md` bajo `persist_ref`.
- Topología DA-4 activa. Tekton no arranca en T0.

## T1 · Forja nativa `daemon-creator` (gate L-FORGE)

1. Portar `run_daemon_forge` en `SddIA/engine/execute-process/src/forges/factory.rs` + rama `daemon_name` / `entity_class` no inventada vía EM.
2. Invocación: `./sddia-run.sh --process daemon-creator` con inputs del contrato (`daemon_name: iota-publish-relay`, `context: ecosystem-evolution`, `execution.runtime: native-rust`, `heartbeat_interval_seconds: 30`, capabilities L-FORGE/spec §3.1).
3. Criterio de salida: `{directories.daemons}/iota-publish-relay.md` + fila en `daemons/index.md` + residual_runner **sin** `forja pendiente porte`.
4. Si el porte bloquea → **parar y escalar**; prohibido bisturí de `{name}.md`.

`delegates_to` runtime: `action:execute-process` → `daemon-creator` → `action:crypto-broker` (cadena ya en proceso).

## T2 · Crate supervisor + launchers (L-SUPERVISOR)

1. Crate `SddIA/daemons/iota-publish-relay/` (Cargo workspace): `DaemonRuntime` + spawn/reap hijo Node + restart loop + probe `/health`.
2. Entrypoint `SddIA/daemons/iota-publish-relay.sh` + launcher `SddIA/scripts/daemons/iota-publish-relay.sh` (patrón kalma2-bridge / event-watcher).
3. Resolver hijo Node: `SDDIA_IOTA_RELAY_DIR` → `.SddIA/services/iota-publish-relay` (relativo instancia).
4. Lock en `daemons_instance.status`; side-channel + `Daemon_Heartbeat` vía runtime.

## T3 · Ignición paridad script/systemd (L-REQUIRED, L-HEALTH)

1. `start-sddia.sh`: borrar inline L496–510 / `IOTA_RELAY_PID`; cablear arranque condicional.
2. `_materialize_systemd_units` **y** `SYSTEMD_FACTORY_DAEMONS` (+tests instance_creator): añadir `iota-publish-relay`.
3. `_systemd_ignite`: enable `@%f` si L-REQUIRED **antes** de `exit 0`.
4. `_start_daemon` + `_wait_http` `/health`; sin `ACTIVO` falso.
5. `cleanup()`: coherente (systemd sin pkill relay; script vía lock).
6. Docs `start-sddia.md` bump menor si el contrato de ignición cambia.

## T4 · Fase 0 — Rescate Merkle (L-RESCUE) — **criterio de parada**

1. Supervisor+hijo vivos; `/health` 200.
2. Inventario real `eda_bus.dead_letter` / `subscribers/*.cumulo.iota-immutable-publisher.json` (ventana 2026-08-25..27; censo = acta, no el 28 semilla si diverge).
3. Un lote `iota-immutable-publisher` `payload: [string,…]`; cero `route_domain_event`; cero reinyección a `pending/`.
4. Proofs → `eda_instance.proofs`; `delivery_state` + `anchored_retroactively: true`.
5. Move DL subscriber → `processed/`.
6. Acta `merkle-acta-dlt-backfill-20260827.json`.
7. Fallo del lote → **stop**; no T5+ con backfill parcial.

Procedimiento: script/lab one-shot bajo `persist_ref` o invocación cápsula vía `./sddia-run.sh --tool iota-immutable-publisher` (JSON stdin); no genoma a mano.

## T5 · Causa real + fractura (L-CAUSE, L-FRACTURE-ONCE)

1. `route_domain_batch`: match exhaustivo del join/invoke (spec §4.1).
2. Arrastrar causa a `last_batch_anchor_error`; subscriber L842/L850 → `batch-anchor-failed:{causa}`.
3. Emitir **1** `System_Fracture_Detected` por fallo de pre-sellado (patrón `daemon_heartbeat::emit_system_fracture` → `eda_bus.pending`).
4. Tests T-CAUSE, T-SILENCE, T-FRACTURE (spec §6).

## T6 · Cola re-anclaje (L-QUEUE, L-PEAJE)

1. Bump `cumulo.paths.json`: `eda_instance.dlt_reanchor`.
2. Helper resolve + enqueue en fallo; drain al inicio de `route_domain_batch` si `/health` OK.
3. Test T-QUEUE. Régimen fail-soft documentado (sin fail-hard de entrega).

## T7 · Fósil + régimen (L-FOSSIL / Fase 3)

1. Auditoría callers/tests de `invoke_iota_publisher`.
2. Decisión sellada en `implementation.md`: **permanece** (spec L-FOSSIL); no fallback unitario en este PR.
3. Nota deuda `DT-DLT-RELAY-NODE` / F-01 `entity_class: daemon` sin implementar.

## T8 · Smokes aceptación

| CA | Smoke |
|----|-------|
| DLT-CA1 | lock + heartbeat-audit + launcher |
| DLT-CA2 | relay saboteado → no ACTIVO |
| DLT-CA3/4 | relay down → dead-letter con causa + 1 fractura |
| DLT-CA5 | kill hijo/supervisor → revive (Restart= / loop) |
| DLT-CA6..9 | acta + corpus |
| DLT-CA10 | cola drain sin humano |

## T9 · Cierre documental en rama

- `implementation.md` / `execution.md`.
- `validacion.md` APTO, `pbi_archived: true`.
- PBI → `docs/todos/done/` (mismo PR).
- Evolution register del hito.
- `delivery-close-cycle` (Tekton + Racso). Git vía `git-manager`.

## Dependencias

```
T0
 └─ T1 forja daemon
      └─ T2 crate supervisor
           └─ T3 ignición (ambas jurisdicciones)
                └─ T4 Fase 0 rescate  ← PARADA si falla
                     └─ T5 causa + fractura
                          └─ T6 cola
                               ├─ T7 fósil/régimen
                               └─ T8 smokes
                                    └─ T9 cierre
```

T7 puede paralelizarse con T6 tras T5. T4 **antes** de merge con reforma “ciega” al corpus.

## Prohibido en ejecución

- Bisturí de `SddIA/daemons/*.md` / genoma sin creator (DA-2).
- Reinyectar eventos rescatados a `pending/`.
- `ACTIVO` sin `/health`.
- Literal único `batch-missing-merkle-anchor` como `error_trace`.
- Segundo PR documental.
- `sleep` / polling CI / `gh run rerun` mismo SHA (DA-5/DA-6).
- Migrar publisher Node→Rust en este PR.
