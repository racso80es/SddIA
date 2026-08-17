---
feature_name: kalma2-mvp-paciente-0
created: "2026-08-17"
process: feature
phases: "T0-topologia,T1-ratificacion-ssot,T2-ley-y-codice,T3-eventos,T4-centinela,T5-triaje,T6-di-y-capsula,T7-tuberia-sync,T8-bridge-y-wui,T9-aduana"
branch_name: feat/kalma2-mvp-paciente-0
persist_ref: docs/features/kalma2-mvp-paciente-0
document_id: PBI-KALMA2-MVP-01
uuid: "d7d00838-9ee6-472f-a164-95dcba2ceb80"
executor: tekton
gates: 10
delivery_mode: "dos PBIs secuenciales (ratificado 2026-08-17)"
delivery_pbis:
  - id: PBI-KALMA2-MVP-01A
    scope: "T0-T5 + aduana sensorial"
    feature: kalma2-mvp-sensorial-email
  - id: PBI-KALMA2-MVP-01B
    scope: "T6-T8 + aduana de sincronización"
    feature: kalma2-mvp-sync-activos
ratifications: "R-01 y R-02 concedidas"
---

# Plan Dédalo — Línea de montaje Kalma2 MVP

## Estrategia

Dos olas con dependencia estricta. **Ola A** (T0–T5) cierra el circuito sensorial: sin ella no hay Paciente 0. **Ola B** (T6–T8) cierra la sincronización de activos: depende del códice forjado en T2 como carga a sincronizar. T9 es aduana común.

Orden de forja impuesto por dependencia de identidad: **norma → códice → eventos → centinela → proceso**. Forjar el códice antes de la norma produce un `composition[]` con UUID inexistente, es decir, un activo inválido desde el nacimiento.

Regla operativa en todas las fases: entidades exclusivamente vía `execute-process --process entity-manager`. Prohibida la mutación manual del genoma. Tras el acuse JSON del CLI, prohibido `sleep`, polling o `AwaitShell` (DA-5).

## Ola A — Circuito sensorial

### T0 · Topología documental

- [ ] Verificar `docs/features/kalma2-mvp-paciente-0/` con `clarify.md`, `spec.md`, `plan.md` presentes.
- [ ] Materializar `objectives.md` (frontmatter: `feature_name`, `created`, `process`).
- [ ] Crear rama `feat/kalma2-mvp-paciente-0` desde `main`.

**Gate G0:** topología conforme a `features-documentation-pattern` v1.2.1. Sin ella, RAW Kernel bloquea toda mutación de genoma (DA-4).

### T1 · Mutación de SSOT compartido — ratificada

R-01 y R-02 concedidas por el Vértice Biológico el 2026-08-17. Gate G1 desbloqueado; queda la ejecución material.

- [ ] **R-01** — `codex-contract.md` → v1.2.0 con bloque `dlt` opcional (`spec.md` §9.4).
- [ ] **R-02** — `cumulo.paths.json`: `process_domain_roots` += `SddIA/library/codexes/codex-kalma2-assistant/process`.
- [ ] Verificar que los 4 códices existentes siguen validando sin bloque `dlt` (retrocompatibilidad).

**Gate G1:** los 4 códices preexistentes validan sin el bloque nuevo y `process-creator` reconoce la jurisdicción añadida. **Bloqueante para T2**: sin R-01 el bloque `dlt` es Ruido de Sistema; sin R-02 el proceso empacado no es descubrible por Cúmulo.

### T2 · La ley y el activo

- [ ] Forjar norma `email-triage-matrix` (`nature: tactical-norm`, `scope: agnostic`, `category: workflow`) con las 5 secciones normativas de `spec.md` §5. Registrar el `uuid` emitido.
- [ ] Forjar códice `codex-kalma2-assistant` con `composition[]` apuntando al UUID **real** de la norma, `process_membership: [email-triage-gateway]` y bloque `dlt` en `pre-mint`.
- [ ] Verificar filas en `SddIA/library/norms/index.md` y `SddIA/library/codexes/index.md`.
- [ ] Actualizar `spec.md` §5 y §6 con los UUID emitidos si difieren de las reservas.

**Gate G2:** `canonical_hash == hash_signature`; `token_id: null`; `mint_status: pre-mint`; cada `composition[].norm` resuelve a un fichero existente. El activo nace con Cicatriz Digital completa y apto para minteo.

### T3 · Clases ECST

- [ ] Forjar `email-received` (payload de `spec.md` §3.1, con `body_ref` y sin cuerpo íntegro).
- [ ] Forjar `email-triaged` (payload de `spec.md` §3.2, con `decision_path` y `thermodynamic_cost`).
- [ ] Registrar suscripción `Email_Received` → `process: email-triage-gateway` en `event-domain-subscriptions.json`.
- [ ] Reconciliar `SddIA/events/domain/index.md`.

**Gate G3:** `event-bus-audit` no reporta clase huérfana ni suscripción a entidad inexistente.

### T4 · Centinela Periférico

- [ ] Forjar definición `SddIA/daemons/email-watcher.md` (`spec.md` §4) vía `daemon-creator`.
- [ ] Implementar cápsula Rust: conexión IMAP **read-only**, sondeo por `SDDIA_EMAIL_POLL_SECONDS`, watermark por UID, persistencia de `.eml` en `.SddIA/inbox/`, emisión de `Email_Received` y `Daemon_Heartbeat`, `.lock` con PID, apagado limpio ante SIGTERM.
- [ ] Añadir el crate al workspace `SddIA/Cargo.toml` y el launcher `SddIA/daemons/email-watcher.sh`.
- [ ] Materializar `SddIA/templates/systemd/sddia-email-watcher@.service.template` con marcador `@@SDDIA_CORE_ROOT@@`.
- [ ] Documentar en `.dev/.env.example` las variables `SDDIA_EMAIL_*` sin valores reales.

**Gate G4 (ceguera lógica, innegociable):** auditoría del código de la cápsula sin una sola aparición de `execute-process`, de lectura bajo `SddIA/`, de comando IMAP de escritura ni de ruta absoluta del host. Un solo hallazgo invalida la fase.

### T5 · Proceso de triaje

- [ ] Forjar `email-triage-gateway` bajo `SddIA/library/codexes/codex-kalma2-assistant/process/` (4 fases de `spec.md` §7) y crear el `index.md` de ese directorio.
- [ ] Forjar skill `agenda-manager` con `provides_capability: agenda:persist`; persistencia en `{instancia}/.SddIA/agenda/`.
- [ ] Registrar `agenda:persist` en `capability-bindings.md`.
- [ ] Implementar la fase `Triaje-C` como evaluador determinista de las reglas de la norma, con salida temprana en `noise`.
- [ ] Implementar la fase `Clasificacion` vía `llm:interact`, **condicionada** a que `Triaje-C` no haya concluido.
- [ ] Implementar `Emision` de `Email_Triaged` con `decision_path`, `matched_rule` y `thermodynamic_cost`.

**Gate G5 (peaje termodinámico):** correo con cabeceras de lista resuelto por `Triaje-C` produce `decision_path: deterministic` y `thermodynamic_cost` en ceros; el `execution_report` demuestra que la fase `Clasificacion` no se ejecutó. Correo con verbosidad comercial extrema no obtiene veredicto `actionable`.

## Ola B — Hito de Sincronización de Activos

### T6 · DI y cápsula de reclamación

- [ ] Forjar tool `github-raw-fetcher` con E/S `capsule-json-io` schema 2.0 (`spec.md` §8.3), `provides_capability: asset:fetch` y `deprecation_pivot` declarado.
- [ ] Registrar el binding `asset:fetch → tool:github-raw-fetcher` en `capability-bindings.md` (→ v1.5.0).
- [ ] Implementar la cápsula: base remota configurable por entorno, solo lectura pública, sin credenciales, devolviendo `content`, `declared_hash` y `origin_kind`.

**Gate G6:** invocación directa `--tool github-raw-fetcher` devuelve envelope conforme con `exitCode: 0 ⟺ success: true`.

### T7 · Tubería de actualización

- [ ] Forjar acción `download-remote-asset` con `requires_capability: asset:fetch` (`spec.md` §8.2).
- [ ] Forjar proceso `sync-client-assets` con las 4 fases de `spec.md` §8.1.
- [ ] Implementar la aduana de integridad: hash discordante ⇒ abortar **sin escribir** el fichero local.
- [ ] Implementar la inyección vía `fs:persist` (`skill:filesystem-manager`, sin modificarla) en `{instancia}/.SddIA/library/codexes/`.

**Gate G7 (pivote sin fractura):** `grep` de `github-raw-fetcher` en `sync-client-assets.md` y `download-remote-asset.md` devuelve cero coincidencias. Prueba activa: cambiar el `provider` de `asset:fetch` a un stub y comprobar que el circuito sigue completándose sin editar proceso ni acción.

### T8 · Puente y WUI

- [ ] Añadir `POST /api/sync-assets` en `SddIA/interfaces/kalma2-bridge/src/main.rs`: registrar en `dispatch`, handler homólogo a `handle_execute`, delegación fire-and-forget y respuesta `202` con `correlation_id`.
- [ ] Añadir el botón **Sincronizar Genoma** en `interfaces/kalma2/index.html` + emisor en `app.js`, con estilo coherente en `style.css`.
- [ ] Consumir el progreso por `GET /api/progress/stream?correlation_id=` (canal existente; no crear otro).

**Gate G8:** pulsar el botón devuelve `202` sin bloquear la UI; el prompt de Kalma2 sigue operativo durante la sincronización (soberanía de interacción).

## T9 · Aduana ontológica y cierre

- [ ] `event-bus-audit`: cero dead-letters, cero huérfanos derivados de las clases nuevas.
- [ ] `telemetry-compliance-audit`: `email-watcher` con recibos de heartbeat conformes.
- [ ] `daemon-heartbeat-audit`: sin omisión de tres ciclos consecutivos.
- [ ] `policy-validator`: contextos de ejecución y declaración de secretos conformes para las 11 entidades nuevas.
- [ ] Prueba de resiliencia: `SIGKILL` al proceso ⇒ resurrección en <5 s; bloqueo de sesión sin caída del servicio.
- [ ] Prueba de idempotencia: reinicio del Centinela sin reemisión de correo ya procesado.
- [ ] Prueba end-to-end: correo real → `Email_Received` → veredicto → visible en la WUI sin intervención en terminal.
- [ ] Registro en `SddIA/evolution/` vinculando el UUID de la feature.
- [ ] Cierre documental en la rama: PBI a `docs/todos/done/`, `validacion.md` con `global: APTO` y `pbi_archived: true`, todo en el mismo PR.

**Gate G9:** los 11 criterios de aceptación del PBI verificados con evidencia, no por declaración.

## Touchpoints

| # | Ruta | Naturaleza | Fase |
|---|------|-----------|------|
| 1 | `SddIA/library/codexes/codex-contract.md` | mutación contrato (v1.2.0) | T1 |
| 2 | `SddIA/core/cumulo.paths.json` | mutación SSOT topología | T1 |
| 3 | `SddIA/library/norms/email-triage-matrix.md` | alta | T2 |
| 4 | `SddIA/library/norms/index.md` | reconciliación | T2 |
| 5 | `SddIA/library/codexes/codex-kalma2-assistant.md` | alta | T2 |
| 6 | `SddIA/library/codexes/index.md` | reconciliación | T2 |
| 7 | `SddIA/events/domain/email-received.md` | alta | T3 |
| 8 | `SddIA/events/domain/email-triaged.md` | alta | T3 |
| 9 | `SddIA/events/domain/index.md` | reconciliación | T3 |
| 10 | `SddIA/core/event-domain-subscriptions.json` | mutación SSOT EDA | T3 |
| 11 | `SddIA/daemons/email-watcher.md` | alta | T4 |
| 12 | `SddIA/daemons/email-watcher/` + `email-watcher.sh` | alta (cápsula Rust) | T4 |
| 13 | `SddIA/daemons/index.md` | reconciliación | T4 |
| 14 | `SddIA/Cargo.toml` | mutación workspace | T4 |
| 15 | `SddIA/templates/systemd/sddia-email-watcher@.service.template` | alta | T4 |
| 16 | `.dev/.env.example` | documentación de entorno | T4 |
| 17 | `SddIA/library/codexes/codex-kalma2-assistant/process/email-triage-gateway.md` | alta | T5 |
| 18 | `SddIA/library/codexes/codex-kalma2-assistant/process/index.md` | alta | T5 |
| 19 | `SddIA/skills/agenda-manager.md` + cápsula | alta | T5 |
| 20 | `SddIA/skills/index.md` | reconciliación | T5 |
| 21 | `SddIA/tools/github-raw-fetcher.md` + cápsula | alta | T6 |
| 22 | `SddIA/tools/index.md` | reconciliación | T6 |
| 23 | `SddIA/core/capability-bindings.md` | mutación SSOT DI (v1.5.0) | T5, T6 |
| 24 | `SddIA/actions/download-remote-asset.md` | alta | T7 |
| 25 | `SddIA/actions/index.md` | reconciliación | T7 |
| 26 | `SddIA/process/sync-client-assets.md` | alta | T7 |
| 27 | `SddIA/process/index.md` | reconciliación | T7 |
| 28 | `SddIA/interfaces/kalma2-bridge/src/main.rs` | mutación (ruta nueva) | T8 |
| 29 | `interfaces/kalma2/{index.html,app.js,style.css}` | mutación WUI | T8 |
| 30 | `SddIA/evolution/` | registro de hito | T9 |

## Riesgos y contención

| Riesgo | Contención |
|--------|-----------|
| Falso positivo `noise` sobre correo crítico | MVP read-only: nada se pierde. Todo veredicto queda auditable en `Email_Triaged` para calibrar antes de habilitar escritura IMAP |
| Ráfaga de correo saturando el bus | Payload ligero + salida temprana determinista. Si persiste, redirigir telemetría a `eda_instance.customization` según `daemons-contract` §6.1 |
| Credencial IMAP filtrada al genoma | `policy-validator` en T9 audita declaración de secretos; `.SddIA/` fuera de Git |
| `codex-contract` v1.2.0 rompe códices existentes | Bloque `dlt` opcional; verificación explícita de los 4 códices en T1 |
| Ola B arrastrando el cierre de Ola A | Recomendación de entrega en dos PBIs secuenciales |

## Entrega ratificada

30 touchpoints, 11 entidades y 4 mutaciones de SSOT en un único PR producen una aduana de revisión inauditable. Entrega en **dos PBIs secuenciales**, ratificada el 2026-08-17:

| PBI | Fases | Feature | Criterio de Done |
|-----|-------|---------|------------------|
| `PBI-KALMA2-MVP-01A` | T0–T5 + T9 sensorial (G0–G5, G9a) | `kalma2-mvp-sensorial-email` | Circuito sensorial completo: correo → veredicto → WUI |
| `PBI-KALMA2-MVP-01B` | T6–T8 + T9 sincronización (G6–G8, G9b) | `kalma2-mvp-sync-activos` | Sincronización de activos con pivote DLT verificado |

Reparto de T9: **G9a** (bus, heartbeat, telemetría, resiliencia, idempotencia, e2e sensorial) cierra 01A; **G9b** (`policy-validator` sobre las entidades de Ola B, e2e de sincronización, registro en `SddIA/evolution/`) cierra 01B.

Regla de secuencia: 01B no arranca hasta que 01A esté mergeado, porque su carga a sincronizar es el códice forjado en T2.

Cada PBI cumple `task-closure-documental` por separado: un PR, `validacion.md` con `global: APTO` y `pbi_archived: true`, y el PBI movido a `docs/todos/done/` en la misma rama. El PBI paraguas `PBI-KALMA2-MVP-01` se archiva cuando ambos cierran.
