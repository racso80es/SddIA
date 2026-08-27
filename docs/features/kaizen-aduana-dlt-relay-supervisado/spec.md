---
feature_name: kaizen-aduana-dlt-relay-supervisado
created: "2026-08-27"
process: feature
base: main
scope: aduana-dlt-relay-supervisor-causa-real
version_spec: "1.0.0"
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
persist_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
branch_name: feat/kaizen-aduana-dlt-relay-supervisado
execution_id: "a7f3e291-6c4b-4d8e-9a1f-3b5e7c8d0e2f"
dedalo_verdict: ok
laudo: relay-centinela-ambas-jurisdicciones-causa-real-fail-soft
laudos:
  - L-FORGE
  - L-SUPERVISOR
  - L-HEALTH
  - L-REQUIRED
  - L-CAUSE
  - L-FRACTURE-ONCE
  - L-PEAJE
  - L-QUEUE
  - L-FOSSIL
  - L-RESCUE
---

# Spec — kaizen-aduana-dlt-relay-supervisado

## 1. Decisiones Dedalo

| ID | Decisión | Rationale |
|----|----------|-----------|
| **L-FORGE** | Forja de `{name}.md` vía `./sddia-run.sh --process daemon-creator` (F-01: `entity-manager` **no** declara `entity_class: daemon`; no inventar clase en este ciclo). Portar forja nativa de daemon en `factory.rs` **antes** de la invocación; hoy `materialize_by_inputs` no acepta daemon → residual `forja pendiente porte`. Prohibido bisturí de `SddIA/daemons/iota-publish-relay.md`. Launchers en `SddIA/scripts/daemons/` y `SddIA/daemons/*.sh` son delivery (DA-2 no los lista); se materializan con la forja / plantilla, no a mano sueltos. | DA-2; precedente email-watcher; `residual_runner` L746 |
| **L-SUPERVISOR** | El centinela **no** es `node server.mjs` como PID de `_start_daemon`. `_start_daemon` exige ELF nativo del launcher. Binario Rust (crate `iota-publish-relay` bajo `SddIA/daemons/`) usa `sddia-daemon-runtime` (lock, side-channel, `Daemon_Heartbeat`) y **supervisa** el hijo Node (restart loop). El publisher sigue siendo Node (`DT-DLT-RELAY-NODE` intacto). | DLT-CA1/CA5; `_is_native_elf`; ceguera lógica: el Rust no interpreta dominio, solo vitalidad + spawn |
| **L-HEALTH** | Sonda de territorio = `GET {host}:{port}/health` (ya existe en `server.mjs`). Prohibido POST `/v1/publish` como probe. `_wait_http` sustituye `sleep 1 && echo ACTIVO`. | DLT-CA2; endpoint vigente L156 |
| **L-REQUIRED** | Arranque + enable systemd **solo** si: perfil ≠ consumer **y** `SDDIA_LAB_SIMULATE_IOTA=0` **y** `IOTA_PUBLISH_RELAY_URL` loopback **y** el entrypoint de instancia existe (ruta vía bóveda/Cúmulo, nunca path de cliente en Core). Si REQUIRED y no hay binding → `[ERROR]` y fallo de ignición (no S+ Grade). Simulación / consumer: no arrancar, no ERROR. | PBI §3.1; Mayeuta D3 |
| **L-CAUSE** | Pre-sellado: capturar `join` Err, `invoke` Err y `success != true`. Persistir `last_batch_anchor_error` en los eventos del lote. Subscriber en `batch_mode_iota` si ancla inválida: `error_trace = "batch-anchor-failed: " + causa`. Prohibido literal único `batch-missing-merkle-anchor`. Reutilizar `capsule_error_trace`. | DLT-CA3; `iota-relay-unreachable` ya lo emite la cápsula |
| **L-FRACTURE-ONCE** | Un `System_Fracture_Detected` **por fallo de pre-sellado de lote**, no por evento. Payload REQUIRED del evento + `friction_id: F-DLT-RELAY-SIN-SUPERVISOR` en traza/`attempted_action`. Evita 28 PBI Kintsugi por un relay caído. | DLT-CA4; contrato `system-fracture-detected` |
| **L-PEAJE** | Fail-soft **con cola**. `route_domain_batch` sigue `success: true` a nivel envelope de lote; el subscriber IOTA falla con causa real; el evento no es terminal-ciego. | PBI §3.5; Mayeuta D5 |
| **L-QUEUE** | Persistencia instancia: clave Cúmulo nueva `eda_instance.dlt_reanchor` = `.SddIA/dlt/reanchor-queue` (bump `cumulo.paths.json`; helper `resolve_dlt_reanchor_dir` espejo de `resolve_eda_proofs_dir`). En fallo de pre-sellado: JSON `{event_uuid, path, error_trace, queued_at}` por UUID. Drain: si `/health` OK al inicio de `route_domain_batch`, re-intentar lote Merkle **sin** pasar por `pending/`. | DLT-CA10; ceguera espacial |
| **L-FOSSIL** | `invoke_iota_publisher` **permanece** (tests / vía no-batch). Producción no cae a fallback unitario en error de transporte (relay down ⇒ unitario también falla). No extirpar este PR. | PBI § fósil; Fase 3 cerrada |
| **L-RESCUE** | Fase 0: cápsula `iota-immutable-publisher` con `payload: [string, …]` (mismo contrato Merkle que el batch). Cero `route_domain_event`. Move `dead-letter/subscribers/` → `processed/`. Acta en `eda_instance.proofs`. `anchored_retroactively: true`. Censo real al inventario, no el 28 de la semilla si diverge. Parada si el lote falla. | DLT-CA6..CA9 |

## 2. Circuito

```
ignición
  systemd: _systemd_ignite + enable sddia-iota-publish-relay@%f  [si L-REQUIRED]
  script:  _start_daemon iota-publish-relay
       → ELF supervisor → child node (instance path)
       → _wait_http GET /health  ⇒ ACTIVO | ERROR

route_domain_batch
  drain cola L-QUEUE si /health OK
  pre-sello Merkle (cápsula)
    OK  → delivery_state + proofs
    FAIL → last_batch_anchor_error + cola + 1× System_Fracture_Detected
  route_domain_event(..., batch_mode_iota=true)
    ancla válida → skip publisher
    inválida → failed + batch-anchor-failed:{causa}

Fase 0 (una vez)
  inventario DL → un Merkle → proofs → processed/ → acta
```

## 3. Centinela `iota-publish-relay`

### 3.1 Contrato `{name}.md`

| Campo | Valor |
|-------|--------|
| `name` | `iota-publish-relay` |
| `context` | `ecosystem-evolution` (infra de instancia, no sensor IMAP) |
| `capabilities` | `iota-relay-supervise`, `dlt-publish-http` |
| `execution.entrypoint` | `SddIA/daemons/iota-publish-relay.sh` |
| `execution.runtime` | `native-rust` (supervisor; hijo Node es delivery) |
| `execution.heartbeat_interval_seconds` | `30` (≥ 5) |
| `jurisdiction` | canónico daemons-contract |

Ceguera: el binario no lee genoma ni invoca `execute-process`. Solo lock, heartbeat, spawn/reap del hijo, probe `/health`.

### 3.2 Ruta del hijo Node

Resolver en este orden (ninguno cableado a un cliente):

1. `SDDIA_IOTA_RELAY_DIR` (bóveda instancia)
2. `{repo}/.SddIA/services/iota-publish-relay` (convención de instancia, relativa)
3. Ausente + L-REQUIRED → ERROR de ignición

`IOTA_PUBLISH_RELAY_HOST` / `PORT` / `URL` ya en bóveda. PATH Node: `_setup_node_path` existente.

### 3.3 Ignición y cleanup

- Eliminar bloque inline L496–510 y `IOTA_RELAY_PID`.
- **Paridad de fábrica systemd (dos listados hoy divergentes del antecesor):**
  1. `start-sddia.sh` → `_materialize_systemd_units` (bucle L234).
  2. `instance_creator.rs` → `SYSTEMD_FACTORY_DAEMONS`.
  Ambos deben incluir `iota-publish-relay` (render `sddia-iota-publish-relay@.service` desde `sddia-daemon@.service.template`).
- `_systemd_ignite`: `_enable_instance_unit sddia-iota-publish-relay` **bajo L-REQUIRED**, **antes** del `exit 0` de jurisdicción systemd (hoy L441 deja ciego el bloque DLT).
- Jurisdicción `script`: arranque vía `_start_daemon` + gate health; si L-REQUIRED falla → `cleanup 1` (no S+ Grade).
- `cleanup()` systemd: sin caso especial PID (unidad vive). `script`: stop vía `.lock` (`DAEMON_NAMES` / lista de arranque incluye el relay si se levantó); borrar rama `IOTA_RELAY_PID`.
- Mensaje `ACTIVO` **solo** tras `_wait_http` a `/health` (URL desde `IOTA_PUBLISH_RELAY_HOST`+`PORT` o path de `IOTA_PUBLISH_RELAY_URL`; no hardcodear 8787 en el texto de éxito).

### 3.4 `_start_daemon`

Tras PID ELF OK, si el nombre es `iota-publish-relay`: `_wait_http` health. Fallo → no ACTIVO, return 1.

## 4. Motor (`route_domain_core.rs`)

### 4.1 Pre-sellado

Sustituir `if let Ok(Ok(result))` por match exhaustivo:

| Resultado | Acción |
|-----------|--------|
| `Ok(Ok(body))` success | vigente (digest, merkle, proofs, `delivery_state`) |
| `Ok(Ok(body))` !success | `capsule_error_trace`; cola; fractura×1; **no** silenciar |
| `Ok(Err(e))` / `Err(_)` join | traza `iota-thread-panicked` / e; cola; fractura×1 |

Causa se copia a cada UUID del lote en memoria/`delivery_state.last_batch_anchor_error` para el subscriber.

### 4.2 Subscriber

Ramas L842/L850: usar causa arrastrada si existe; si no, `batch-anchor-failed: batch-missing-merkle-anchor` (prefijo siempre presente).

### 4.3 Fractura

Escritura en `eda_bus.pending` (mismo patrón que heartbeat-audit). Campos: `process_name: route-domain-event`, `error_trace` con causa + friction_id, `agent_emitter: execute-process`, `attempted_action: merkle-batch-preseal`.

## 5. Fase 0 — Rescate

1. Supervisor+hijo vivos; `/health` 200.
2. Glob `eda_bus.dead_letter` / `subscribers/*.cumulo.iota-immutable-publisher.json` con traza Merkle-missing en ventana; resolver `event_uuid` → envelope (processed o DL raíz).
3. Extraer payloads canónicos; un `publish_immutable_data` array.
4. Escribir `{uuid}.json` en `eda_instance.proofs`; parchear `delivery_state` (`merkle_anchored`, digest real, `merkle_root`, `anchored_retroactively`, `anchored_at` UTC del sello).
5. Mover artefactos subscriber DL → `processed/` (no `pending/`).
6. Acta `merkle-acta-dlt-backfill-20260827.json` en proofs: raíz, digest, censo UUID, ventana, `retroactive: true`.
7. DLT-CA7: `is_valid_iota_anchor` — cero `batched-digest`.

## 6. Tests

| ID | Fixture | Esperado |
|----|---------|----------|
| T-CAUSE | pre-sello `success:false` `error: iota-relay-unreachable: …` | subscriber `error_trace` contiene esa causa; prefijo `batch-anchor-failed:` |
| T-SILENCE | join `Err` | no se omite; fractura 1 |
| T-FRACTURE | un lote N eventos fail | exactamente 1 `System_Fracture_Detected` |
| T-VALID | digest real + merkle | `is_valid_iota_anchor` true; `batched-digest` false |
| T-QUEUE | fail → fichero cola; health OK → drain | sellado sin pending |

## 7. Fuera

- `entity_class: daemon` en `entity-manager` (deuda F-01; no este PR salvo bloqueo absoluto del creator).
- Relay Node → Rust publisher.
- Espejo de Consciencia UI.
- Fail-hard de cierre de entrega.
