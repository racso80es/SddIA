---
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
uuid: "17380fcf-0630-45d3-9813-611d80beec0d"
title: "[LABORATORIO] MVP Paciente 0: Anclaje de Telemetría DLT con Billetera Local"
format: markdown
version: "1.2.0"
status: abierto
type: laboratorio
priority: alta
process: feature
assigned_to: Tekton, Tormentosa (auditoría)
created: "2026-08-21"
updated: "2026-09-06"
derived_from: PBI-LAB-PACIENTE0-SDDIA-AP
feature_name: lab-paciente0-dlt-telemetry-mvp
persist_ref: docs/features/lab-paciente0-dlt-telemetry-mvp
branch_name: feat/lab-paciente0-dlt-telemetry-mvp
related:
  - SddIA/tools/iota-immutable-publisher/src/main.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/route_fractal_core.rs
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/events/telemetry/raw-execution-finished.md
  - SddIA/events/domain/domain-entity-telemetry-captured.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/event-subscriptions.json
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - .SddIA/services/iota-publish-relay/server.mjs
---

# [LABORATORIO] MVP Paciente 0: Anclaje de Telemetría DLT con Billetera Local

## 0. Registro de Refinamiento (v1.0.0 → v1.1.0 → v1.2.0)

| Vector | Borrador v1.0.0 | v1.1.0 | Realidad auditada y corrección v1.2.0 |
|---|---|---|---|
| **Emisión ECST** | Refactorizar `execute-process` para emitir `Domain_Entity_Telemetry_Captured`. | Peaje CLI emite `Raw_Execution_Finished`; emisor exclusivo del snapshot = `radamanto` (`radamanto-batch`). | **Confirmado.** `radamanto_batch_core.rs` `emit_telemetry_captured_failsoft` → `write_fractal_event(..., "domain")`. Contrato: `domain-entity-telemetry-captured.md`. |
| **Despacho vs barrido** | `event-sweeper` captura, despacha publisher y espera `success`. | Watcher despacha; sweeper solo GC de `pending/`. | **v1.1.0 a medias.** Genoma `event-watcher.md` cita solo `pending/` + `route-domain-event`; el binario **sí** vigila `eda_fractal.domain` y delega `route-domain`. Sweeper **no** purga fractal. Purga de dominio fractal = `route-domain` `purge_after=true` (Opción B). |
| **Publisher** | Adaptar Rust para leer del sweeper y firmar MoveVM. | Cápsula Rust stdin/stdout; firma en relay. | **Confirmado.** `action=publish_immutable_data`, `network` + `payload` (string \| array). `network` lo inyecta `route_domain_core` como literal `"testnet"`: no existe `IOTA_NETWORK` en bóveda. |
| **Secretos** | `IOTA_LOCAL_PRIVATE_KEY`, `IOTA_NETWORK`. | `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`, `IOTA_PUBLISH_RELAY_URL`. | **Ajuste:** `IOTA_WALLET_SECRET` o `.SddIA/.dev/wallet.key` (`sddia_io::load_iota_wallet_secret`). `IOTA_ANCHOR_PACKAGE_ID` lo consume el **relay Node**, no la cápsula Rust. `SDDIA_LAB_MOCK_IOTA_URL` no vacío **precede** al relay y anula anclaje físico. |
| **UUID PBI** | — | `a1b2c3d4-e5f6-4789-a012-3456789dlt1` | **Inválido** (no hex / no UUID v4). Sustituido por `17380fcf-0630-45d3-9813-611d80beec0d`. |
| **Testigo DLT** | — | `.events/processed/subscribers/` + `result_status` + digest. | **Inexacto para este evento.** `Domain_Entity_Telemetry_Captured` vive en `./.events/domain/`. Fractal llama `dispatch_subscriber` (no `handle_subscriber`): **no** sella testigos V3+ en `processed/subscribers/`. Digest se muta en memoria y el JSON padre se **purga** tras consenso. Hueco de implementación: persistir digest durable **antes** del `unlink`. |
| **Suscripción instancia** | — | Overlay en instancia Paciente 0. | **Alucinación de Vía C.** `eda_instance.customization` / `event-subscriptions.local.json` está documentado como **deuda no cableada**. SSOT = `event-domain-subscriptions.json` (+ paridad legado `event-subscriptions.json`). |
| **Ruta host** | — | `/home/racso/Proyectos/SddIA_AP/.SddIA/.dev/.env` | **Infracción agnosticismo Core.** Bóveda = `env_hierarchy.instance` (`.SddIA/.dev/.env`). Paciente 0 es la instancia de fuego, no un path de genoma. |

---

## 1. Propósito

Validar anclaje on-chain de snapshots `Domain_Entity_Telemetry_Captured` en IOTA Rebased Testnet, con billetera local, **sin** Gas Station / sponsored tx.

Objetivo dual:

1. **Verdad estructurada:** eternizar el snapshot ECST que Radamanto ya emite (métricas de ejecución), no inventar un sobre paralelo.
2. **Tacto inerte:** el circuito sensorial (`email-triage-gateway` y WUI Kalma2) no espera confirmación DLT. El anclaje es fan-out asíncrono del bus fractal domain.

**Dos planos de Done (no mezclar):**

| Plano | Dónde | Qué cierra |
|---|---|---|
| **A — Core (este repo / este PR)** | Suscripción + persistencia de digest + tests con `SDDIA_LAB_SIMULATE_IOTA` | Merge-gate. CI GitHub. |
| **B — Instancia lab** | Bóveda + relay + Testnet real en Paciente 0 | Aceptación de laboratorio. **No** es check de GitHub. |

---

## 2. Hechos de arquitectura (SSOT)

### 2.1 Cadena canónica (producción)

1. Cápsula termina → Peaje Termodinámico emite `Raw_Execution_Finished` en `./.events/telemetry/` (fail-soft).
2. `event-watcher` / `route-telemetry` → `radamanto-batch` (+ `telemetry-compliance-audit`).
3. Radamanto escribe `Domain_Entity_Telemetry_Captured` en `./.events/domain/` (`event_family: domain`, `emitter_agent: radamanto`).
4. Watcher enruta con `route-domain` (`route_fractal_core.route_domain_fractal_event`, `purge_after=true`).
5. Fan-out actual: solo `memory-evolution-ingest`. **Falta** `iota-immutable-publisher`.
6. Consenso OK → unlink del JSON domain. Fallo de un suscriptor → `eda_fractal.dead_letter` (no sweeper).

Lab E2E síncrono: `SDDIA_LAB_ROUTE_SYNC=1` (Radamanto invoca `route-domain` in-process). Producción: watcher. No exigir `SDDIA_LAB_ROUTE_SYNC` en Paciente 0.

### 2.2 Payload ECST que se ancla

Radamanto emite (campos reales, no desiderata):

- REQUIRED: `entity_type`, `entity_id`, `execution_metrics` (`duration_ms`, `exit_code`, `success_status`), `origin_stimulus` (`event_type: Raw_Execution_Finished`, `event_id`).
- OPTIONAL presentes: `asset_id`, `state_after.last_execution_ms`, `state_after.last_exit_code`, `evolution_footprint: null`.
- `entity_type` lo resuelve `resolve_entity_type` (no está limitado a `{tool,skill,process}`).

`invoke_iota_publisher` serializa el **evento ECST completo** como `payload` string (no un Merkle de métricas sueltas). Merkle + `merkle_root` solo aplica al lote `route_domain_batch` sobre `pending/` (array de strings). Este MVP fractal es **un evento = una tx**.

### 2.3 Cápsula y relay

- Cápsula: `SddIA/tools/iota-immutable-publisher` (Rust nativo). El genoma `.md` aún declara Engine TypeScript: **fósil**; fuera de alcance salvo que el ciclo toque esa entidad vía `entity-manager`.
- Relay HTTP: `.SddIA/services/iota-publish-relay/server.mjs` (`POST /v1/publish`, `GET /health`).
- Supervisor: daemon Rust `iota-publish-relay` (lock + heartbeat + spawn Node). No sustituye al hijo Node.
- Precedencia de publicación: `SDDIA_LAB_SIMULATE_IOTA` → `SDDIA_LAB_MOCK_IOTA_URL` → `IOTA_PUBLISH_RELAY_URL`. Simulación produce digest `lab-sim-*` (válido en CI; **inválido** como prueba física).

### 2.4 Bóveda (lógica, no host)

`env_hierarchy.instance` prevalece sobre `env_hierarchy.global`.

| Variable | Rol |
|---|---|
| `IOTA_WALLET_SECRET` o `wallet.key` | Material de firma (relay). Prohibido en JSON stdin. |
| `IOTA_ANCHOR_PACKAGE_ID` | Package Move en el relay. |
| `IOTA_PUBLISH_RELAY_URL` | p.ej. `http://127.0.0.1:8787/v1/publish` |
| `SDDIA_LAB_SIMULATE_IOTA=0` | Exigir camino físico en instancia lab. |
| `SDDIA_LAB_MOCK_IOTA_URL` | Vacío en fuego físico. |

---

## 3. Alcance Core (este PR)

### 3.1 Suscripción

Añadir en `SddIA/core/event-domain-subscriptions.json` (SSOT) y paridad `SddIA/core/event-subscriptions.json`:

```json
{
  "agent": "cumulo",
  "tool": "iota-immutable-publisher",
  "intent": "Anclaje DLT inmutable de snapshot de telemetría en IOTA Rebased."
}
```

bajo `Domain_Entity_Telemetry_Captured`, **después** de `memory-evolution-ingest`. No sustituir ingest.

Clase ECST § Suscripciones: `entity-manager` `update` + `markdown_body_replacements`. Prohibido Write directo en `SddIA/events/`.

JSON de suscripciones **no** es genoma DA-2.

### 3.2 Fail-soft de credenciales (agnosticismo)

Hoy `config-missing: IOTA_WALLET_SECRET` es **failed**. Con fan-out de alta frecuencia eso envía el domain a `dead-letter` en **cualquier** instancia sin bóveda IOTA y bloquea la purga aunque ingest haya sido OK.

**Laudo obligatorio:** para `event_type == Domain_Entity_Telemetry_Captured`, ausencia de secreto/relay → status `skipped-config-missing` (pertenece a `OK_STATUSES` fractal). Resto de tipos DLT (PR, mutación genómica, etc.) **no** cambian.

### 3.3 Digest durable pre-purga

Tras éxito IOTA en camino fractal, persistir **antes** de `purge_after`:

1. `delivery_state.transaction_digest` en el JSON domain (además del stamp `cumulo.iota-immutable-publisher`).
2. Testigo durable `{eda_instance.proofs}/dlt-telemetry/{event_id}.json` con `transaction_digest`, `event_id`, `entity_id`, `network`.

Rechazar `"batched-digest"` y vacíos. Digest simulado `lab-sim-*` válido solo si `SDDIA_LAB_SIMULATE_IOTA` está activo.

### 3.4 Fuera de este PR

- Mutar Peaje / contrato `Raw_Execution_Finished`.
- Firma MoveVM dentro de la cápsula Rust.
- Cablear Vía C overlays.
- Gas Station / paymaster.
- Paths absolutos de Paciente 0 en genoma.
- Anclaje físico Testnet como check de GitHub.

---

## 4. Plan de ejecución

### Fase 1 — Core: suscripción + digest + tests

- [ ] Suscripción dual JSON + clase ECST vía `entity-manager`.
- [ ] Persistencia digest + proof en `eda_instance.proofs`.
- [ ] Skip `skipped-config-missing` solo para este `event_type`.
- [ ] Tests `execute-process`: fan-out ingest+IOTA; simulado `lab-sim-*`; sin secreto → skip no DLQ; ingest intacto.

### Fase 2 — Instancia lab (Paciente 0, post-merge o paralelo operador)

- [ ] Bóveda instancia: secreto + package + relay URL + `SDDIA_LAB_SIMULATE_IOTA=0` + mock vacío.
- [ ] Daemon `iota-publish-relay` + `GET /health`.
- [ ] Estímulo real o lab de triaje → `Raw_Execution_Finished` → snapshot domain → proof con digest explorador.

---

## 5. Criterios de aceptación

### Plano A — Core (gate PR / CI)

- [ ] **CA-1 (Tacto inerte):** `email-triage-gateway` y el Peaje no invocan `iota-immutable-publisher`. El anclaje ocurre solo como suscriptor de `Domain_Entity_Telemetry_Captured`.
- [ ] **CA-2 (ECST):** Emisores intactos: CLI → `Raw_Execution_Finished`; Radamanto → `Domain_Entity_Telemetry_Captured`.
- [ ] **CA-3 (Fan-out):** `memory-evolution-ingest` permanece. Segundo suscriptor = Cúmulo + `iota-immutable-publisher`. Paridad de los dos JSON.
- [ ] **CA-4 (Digest durable):** Tras éxito (simulado o físico) existe proof en `eda_instance.proofs` con `transaction_digest` ≠ `batched-digest` y ≠ vacío. El JSON domain puede haber sido purgado.
- [ ] **CA-5 (Agnosticismo):** Sin `IOTA_WALLET_SECRET` ni relay, el suscriptor DLT de este evento es `skipped-config-missing`; ingest OK; cero dead-letter por config-missing.
- [ ] **CA-6 (CI):** Checks GitHub del PR verdes (`run_id` en `validacion.md`). `global: APTO` prohibido con CA-CI en `PENDIENTE-CI`.

### Plano B — Instancia (no gate CI)

- [ ] **CA-B1 (Firma local):** Relay usa `IOTA_WALLET_SECRET` / `wallet.key` de la bóveda de instancia.
- [ ] **CA-B2 (Físico):** Digest del proof verificable en explorador IOTA Rebased Testnet (no `lab-sim-*`).
- [ ] **CA-B3 (Limpieza fractal):** Consenso ingest+IOTA → JSON domain ausente en `./.events/domain/` y no en `dead-letter/` por este flujo.

---

## 6. Riesgos aceptados (lab)

- Alta frecuencia: cada ejecución de cápsula puede generar una tx Testnet. Aceptado en MVP; sin throttle en este PBI.
- Genoma `iota-immutable-publisher.md` Engine TypeScript: fósil conocido.
- `derived_from: PBI-LAB-PACIENTE0-SDDIA-AP` es trazabilidad; el PBI padre no está en este árbol `docs/todos`.
