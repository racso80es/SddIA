---
feature_name: kaizen-aduana-dlt-relay-supervisado
created: "2026-08-27"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
branch_name: feat/kaizen-aduana-dlt-relay-supervisado
persist_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
pbi_ref: docs/todos/pending/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
execution_id: "cdd000a0-75d3-4bf9-9a4b-c1d889860ed2"
mayeuta_session_id: "a7f3e291-6c4b-4d8e-9a1f-3b5e7c8d0e2f"
correlation_id: ""
incident_ref: "Ceguera DLT 2026-08-25 → 2026-08-27 — 0 anclajes on-chain, 28 eventos en dead-letter"
mayeuta_verdict: ok
laudo: relay-centinela-ambas-jurisdicciones-causa-real-fail-soft
---

# Clarificación — kaizen-aduana-dlt-relay-supervisado

Transcript Mayeuta (2026-08-27). Semilla PBI v1.0.0. Filtro A contra genoma vigente. Relé IDE: Dedalo forja `spec.md`/`plan.md` en el mismo ciclo; Tekton no arranca.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` (ciclo Kaizen; semilla `type: kaizen`, `process: feature`) |
| `feature_name` | `kaizen-aduana-dlt-relay-supervisado` |
| Rama | `feat/kaizen-aduana-dlt-relay-supervisado` |
| `persist_ref` | `docs/features/kaizen-aduana-dlt-relay-supervisado` |
| `document_id` | `PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO` |
| `uuid` PBI | `1243c58b-8e93-4897-ba3e-3efc26564673` |
| `correlation_id` | vacío en estímulo |
| Init lab | `./sddia-run.sh --process feature` + skips archive/delivery + `SDDIA_AGENT_RUNTIME_COMMAND=""` (vacío, no unset: la bóveda no pisa) |
| `execution_id` ciclo | `cdd000a0-75d3-4bf9-9a4b-c1d889860ed2` |
| Fase | Mayeuta + Dedalo (plan). **Tekton no arranca.** |
| Antecesor | `kaizen-ignicion-soberana-centinelas` — IOTA relay **fuera** de systemd (residual explícito) |

**Toll:** un `persist_ref`, un PR. Cierre documental en rama (PBI → `docs/todos/done/` + `validacion.md` APTO) en el mismo PR.

**Fricciones semilla:** `F-DLT-RELAY-SIN-SUPERVISOR`, `F-DLT-BATCH-ERROR-ENMASCARADO`, `F-IGNITION-ACTIVO-SIN-VERIFICAR`.

---

## D1 — Filtro A: semilla vs territorio

| Afirmación semilla | Territorio (2026-08-27) | Veredicto |
|--------------------|-------------------------|-----------|
| Spawn desnudo `(cd … && node server.mjs > relay.log) &` + `ACTIVO` incondicional | `start-sddia.sh:503-506` | **Confirmado** |
| `cleanup()` en `systemd` sale antes de matar `IOTA_RELAY_PID` | `start-sddia.sh:120-122` → `exit` antes de L134 | **Confirmado** |
| Relay **no** está en `REQUIRED_DAEMONS` / launchers | `REQUIRED_DAEMONS=(event-watcher event-sweeper)`; cero `iota*` en `SddIA/scripts/daemons/` | **Confirmado** |
| `_wait_http` existe y no se usa en DLT | L144+; bloque DLT usa `sleep 1` | **Confirmado** |
| `route_domain_batch` traga fallo (`if let Ok(Ok(_))` sin `else`) | `route_domain_core.rs:1647-1717` | **Confirmado** |
| Síntoma `batch-missing-merkle-anchor` opaco | L842 / L850 | **Confirmado** |
| Bóveda / secretos como causa | Semilla; Mayeuta no reabre bóveda | **Aceptado** (hipótesis previas refutadas en PBI §7) |

**Laudo causal:** causa raíz = proceso Node sin supervisor + motor que sustituye causa física por síntoma Merkle. No es falla de bóveda ni de `event-watcher`.

---

## D2 — Residual crítico del antecesor (jurisdicción systemd)

| Hecho | Implicación |
|-------|-------------|
| Con `DAEMON_JURIS=systemd`, `start-sddia.sh` hace `_systemd_ignite` + heartbeats y **`exit 0` en L441** | El bloque IOTA (L496+) **nunca se ejecuta** en el perfil dominante post-`kaizen-ignicion-soberana` |
| Semilla pide solo `_start_daemon iota-publish-relay` | Eso cura jurisdicción `script`; **deja ciego el perfil systemd** |

**Laudo:** supervisión del relay es obligatoria en **ambas** jurisdicciones.

| Jurisdicción | Dentro (este ciclo) |
|--------------|---------------------|
| `script` | Forja daemon + launcher en `SddIA/scripts/daemons/` + lock/log/heartbeat; `start-sddia.sh` vía `_start_daemon`; sin spawn inline |
| `systemd` | Unidad de instancia alineada al patrón `@%f` del antecesor (`sddia-iota-publish-relay@%f` o equivalente forjado), enable en `_systemd_ignite` cuando DLT sea REQUIRED |

Sin unidad systemd, el residual documentado en `kaizen-ignicion-soberana` **permanece**. Extiende §3.1 del PBI; no lo contradice.

---

## D3 — Clasificación REQUIRED / irrelevante

**Laudo:**

- **REQUIRED** cuando `SDDIA_LAB_SIMULATE_IOTA=0` **y** `IOTA_PUBLISH_RELAY_URL` apunta a loopback (aduana local).
- **Irrelevante / no arrancar** en perfil `consumer`, o con simulación activa (`SDDIA_LAB_SIMULATE_IOTA=1`), o URL no-loopback sin aduana local.
- Ruta del servicio: **solo Cúmulo / bóveda de instancia** — prohibido cablear path de cliente en Core.
- Ignición: sin binding HTTP verificado (`_wait_http` al endpoint de publish) → **`[ERROR]`**, nunca `ACTIVO`. Contabilizar en resumen de ignición.

---

## D4 — Propagación de causa + fractura

**Laudo:**

1. Capturar envelope de error de `iota-immutable-publisher` en el pre-sellado de lote.
2. `error_trace` del dead-letter: prefijo `batch-anchor-failed:` + causa real (`iota-relay-unreachable: …`, `config-missing: …`, …). Prohibido dejar `batch-missing-merkle-anchor` como cadena literal única.
3. Rama `else` / fallo de join: **debe** registrar (log/telemetría mínima accionable).
4. Emitir `System_Fracture_Detected` con `friction_id: F-DLT-RELAY-SIN-SUPERVISOR` (y traza con causa física). La ceguera DLT es fractura de dominio, no log de depuración.

---

## D5 — Peaje Termodinámico (régimen)

| Opción | Veredicto |
|--------|-----------|
| Fail-hard (ciclo no cierra sin ancla) | **Fuera** este ciclo (rompe entregas ya fail-soft de facto) |
| Fail-soft sin registro | **Prohibido** (estado actual; peor mundo) |
| Fail-soft **con cola de re-anclaje persistente** + visibilidad | **Dentro — sello** |

Cola permanente (Fase 2) hace rutinario el rescate; DLT-CA10 es el criterio de cierre del régimen.

---

## D6 — Fase 0 — Rescate (precede a reforma)

**Laudo — orden no negociable:**

1. Relay vivo y verificado en el endpoint configurado.
2. Inventario de dead-letter `*.cumulo.iota-immutable-publisher.json` con `batch-missing-merkle-anchor` en ventana 2026-08-25..27 (censo semilla = 28; **recuento real al arrancar Fase 0** es la cifra de acta).
3. Un **único** lote Merkle (no N transacciones sueltas).
4. Pruebas en `eda_instance.proofs` (Cúmulo: `.SddIA/proofs`); `delivery_state` con `merkle_anchored` + digest real.
5. Reintegrar `dead-letter/` → `processed/` — **jamás** `pending/` (cero re-disparo de subscribers).
6. Acta `merkle-acta-dlt-backfill-20260827.json` (raíz, digest, censo UUID, ventana).
7. Honestidad: `anchored_retroactively: true` + timestamp de anclaje real; prohibido fingir contemporaneidad.
8. Si el lote falla → **parada y escala**; prohibido Fase 1 con backfill parcial sin acta.

---

## D7 — Código fósil `invoke_iota_publisher`

| Opción | Laudo |
|--------|-------|
| Rehabilitar como fallback unitario si falla el pre-sellado | Decisión Dedalo en **Fase 3** tras auditoría de callers/tests |
| Extirpar | Idem |

**Este ciclo:** no implementar ambas; Fase 3 documenta la elección. Fuera de Estabilización diseñar el fallback.

---

## D8 — Fuera de alcance

- Panel / Espejo de Consciencia (`PBI-KAIZEN-ESPEJO-CONSCIENCIA-001`) — este PBI es **proveedor de señal**, no UI.
- Migración Node → cápsula Rust (`DT-DLT-RELAY-NODE`) — ortogonal; primero supervisión.
- Cambios de contrato de firma o paquete ancla IOTA.
- Mutación manual de genoma (DA-2): forja solo vía `daemon-creator` / `entity-manager`.
- Segundo PR documental post-merge.

---

## Criterios PBI → laudo

| ID | Laudo |
|----|-------|
| DLT-CA1 | D2+D3 — daemon forjado + launcher/lock/heartbeat; parity systemd |
| DLT-CA2 | D3 — `_wait_http`; sin `ACTIVO` falso |
| DLT-CA3 | D4 — causa real en `error_trace` |
| DLT-CA4 | D4 — `System_Fracture_Detected` |
| DLT-CA5 | D2 — reinicio por supervisor (script heartbeat **o** Restart= systemd) |
| DLT-CA6..CA9 | D6 — Fase 0 |
| DLT-CA10 | D5 — cola permanente post-Fase 2 |

---

## Handoff Dedalo

Consumido. Laudos en `spec.md` (`L-FORGE` … `L-RESCUE`). Plan T0–T7 en `plan.md`. Tekton detenido por mandato de sesión.
