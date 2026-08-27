---
document_id: PBI-OPER-LATIDO-ONTOLOGICO-001
title: "[OPERATIVO] Latido Ontológico (System Heartbeat)"
format: markdown
version: "2.0.0"
created: "2026-08-27"
updated: "2026-08-27"
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
    rol: "Hermano en vuelo; este PBI le aporta el hallazgo bloqueante del genoma ausente. Sin dependencia de orden."
  - id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
    rol: "Consumidor: este PBI es su proveedor de señal (System_Vitality_Probed)."
friction_ids:
  - F-AUDITORIA-CENSO-GENOMA
  - F-ORGANO-SIN-AUTOREPORTE
  - F-CONTRATO-HEARTBEAT-DRIFT
  - F-VITALIDAD-NO-PROCESO-INVISIBLE
related:
  - SddIA/events/telemetry/daemon-heartbeat.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/core/event-telemetry-subscriptions.json
  - SddIA/daemons/index.md
  - SddIA/interfaces/kalma2-bridge/
  - start-sddia.sh
---

# [OPERATIVO] Latido Ontológico (System Heartbeat)

> **Refinamiento v2.0.0.** El borrador v1 proponía crear un evento `System_Heartbeat_Emitted`, un centinela nuevo y un `System_Degraded` gobernado por Radamanto. La auditoría del genoma demuestra que **la mitad de eso ya existe y la otra mitad colisiona con jurisdicciones vigentes**. El déficit real es distinto y está acotado en §1. Las afirmaciones descartadas quedan registradas en §7 para evitar reincidencia.

## 1. Falla Estructural y Contexto

El latido de centinelas **ya está implementado y auditado**: `Daemon_Heartbeat` (familia `telemetry`), emitido por `DaemonRuntime::emit_heartbeat`, consumido por Argos vía `daemon-heartbeat-audit`, con fractura automática a los 3 ciclos omitidos.

### Lo que ya existe (no se reconstruye)

| Órgano | Artefacto | Referencia |
|--------|-----------|------------|
| Clase de evento | `Daemon_Heartbeat`, `event_family: telemetry`, uuid `9c5190ac-ac8a-46b6-b61d-67d45ff7caf1` | `SddIA/events/telemetry/daemon-heartbeat.md` |
| Emisión | Lock PID + side-channel + evento fractal en `eda_fractal.telemetry` | `SddIA/sddia-daemon-runtime/src/lib.rs:328-386` |
| SSOT del intervalo | `execution.heartbeat_interval_seconds` en `SddIA/daemons/{name}.md` (default 30 s, piso 5 s) | `sddia-daemon-runtime/src/lib.rs:222-253` |
| Auditoría | `missed_cycles = floor(elapsed / heartbeat_interval_seconds)`; fractura si `>= 3` | `SddIA/process/daemon-heartbeat-audit.md`; `handlers/daemon_heartbeat.rs:15,275-281` |
| Censo auditado | `audit_staleness` itera **`list_indexed_daemon_ids`** = ficheros `{name}.md` presentes en `SddIA/daemons/` | `handlers/daemon_heartbeat.rs:303-314`; `engine/daemons.rs:55-75` |
| Suscripción | `Daemon_Heartbeat` → `argos` / `daemon-heartbeat-audit` | `SddIA/core/event-telemetry-subscriptions.json:14-19` |
| Tick del sweep | 30 s desde `event-sweeper` (`HEARTBEAT_AUDIT_SWEEP_SECONDS`) | `SddIA/daemons/event-sweeper/src/main.rs:11` |
| Gate de ignición | `_wait_required_heartbeats` sobre `REQUIRED_DAEMONS=(event-watcher event-sweeper)` | `start-sddia.sh:19,390-411` |
| Señal de degradación | `System_Fracture_Detected` → `eda_bus.pending` | `SddIA/events/domain/system-fracture-detected.md` |

### El déficit real: el genoma es la puerta de la auditoría

`audit_staleness` no recorre los locks vivos ni los procesos del sistema: recorre los ficheros `{name}.md` de `SddIA/daemons/` (`daemons.rs:55-75`). **La existencia de la definición de genoma es la condición habilitante de la auditoría.** Un proceso puede tener launcher, lock, `DaemonRuntime` y emitir latido perfectamente, y aun así **jamás producir fractura al morir**, porque no entra en el censo iterado.

| Órgano vivo | `{name}.md` en `SddIA/daemons/` | Emite `Daemon_Heartbeat` | Auditado por `audit_staleness` |
|-------------|--------------------------------|--------------------------|-------------------------------|
| `event-watcher`, `event-sweeper`, `telegram-watcher`, `github-bridge-watcher`, `email-watcher` | Sí (5) | Sí | **Sí** |
| `iota-publish-relay` — crate Rust con `DaemonRuntime`, `/health`, launcher y `REQUIRED_DAEMONS+=` condicional (trabajo **en vuelo**, sin commit) | **No** | Sí (`iota-publish-relay/src/main.rs:158,214`) | **No** |
| `kalma2-bridge` — servidor HTTP `SDDIA_CLIENT_PORT`, launcher `SddIA/scripts/daemons/kalma2-bridge.sh` | **No** | **No** (cero referencias a `sddia_daemon_runtime` en su crate) | **No** |
| Integridad de índices de Cúmulo | N/A (no es proceso) | Imposible por diseño | **No** |
| Configuración de la aduana Cerbero (`execution-contexts.md`, `revoked_entities.json`) | N/A (no es proceso) | Imposible por diseño | **No** |

Dos consecuencias verificadas y accionables:

1. **El relay IOTA sigue siendo invisible pese a estar supervisado.** El trabajo en vuelo de `PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO` ya le dio `DaemonRuntime`, sonda `/health`, reinicio del hijo Node y promoción a `REQUIRED_DAEMONS` (`start-sddia.sh:70-73,347-354`). Su latido se **registra** vía `ingest_regime` (que sí barre todo el side-channel, `daemon_heartbeat.rs:146-176`) pero **no se audita**, porque falta `SddIA/daemons/iota-publish-relay.md`. Forjar el binario sin forjar el genoma deja la aduana DLT a un `.md` de distancia de repetir la ceguera.
2. **`start-sddia.sh:168` intenta detener un lock que nadie escribe.** `_sddia_stop_lock_pid "${STATUS_DIR}/kalma2-bridge.lock"` es la única aparición de ese fichero en todo el repo: sin `DaemonRuntime` no hay `write_lock`, luego el puente ni se detiene por esa vía ni se audita por ninguna.

Este patrón **ya causó una ceguera de 3 días** (relay muerto el 24-08, incidente en el PBI DLT). La lección no fue "falta un latido más", sino **"falta obligar a cada órgano a existir en el genoma y latir, y verificar activamente lo que no puede latir"**.

### Drift secundario detectado (contrato vs realidad)

`SddIA/events/telemetry/daemon-heartbeat.md:33-37` autoriza como emisores únicamente a `event-watcher`, `telegram-watcher` y `github-bridge-watcher`. `event-sweeper` y `email-watcher` figuran en `SddIA/daemons/index.md` con `heartbeat_interval_seconds` y **emiten latido en producción sin figurar como emisores autorizados**. Peor: `event-sweeper` es `REQUIRED` en la ignición. El contrato está desincronizado del territorio.

## 2. Objetivo Medible

Que **ningún órgano vital pueda morir sin señal**, y que la vitalidad de lo que no es un proceso (índices, configuración de aduana) sea verificada activamente en lugar de asumida.

Éxito si:
1. Todo proceso persistente del ecosistema es un centinela forjado que late bajo `DaemonRuntime` y es auditable por `daemon-heartbeat-audit`. Cero procesos huérfanos de terminal.
2. Existe una sonda periódica que verifica invariantes **no-proceso** y emite un hecho auditable con su veredicto.
3. `daemon-heartbeat.md` declara exactamente los emisores que emiten — sin drift.
4. La ignición no declara grado operativo con un órgano vital sin verificar.
5. Todo veredicto negativo desemboca en `System_Fracture_Detected`, nunca en un log.

## 3. Decisiones Arquitectónicas Obligatorias

### 3.1. Prohibido inventar ontología paralela al latido existente
Se **descarta** crear `System_Heartbeat_Emitted`. `Daemon_Heartbeat` cubre la liveness de procesos, tiene contrato, emisor, suscriptor y auditor. Una segunda clase de latido duplicaría el auditor y fragmentaría el cómputo de `missed_cycles`. Se **descarta** igualmente `System_Degraded`: `System_Fracture_Detected` (Argos, liveness/infraestructura) y `Domain_Entity_Degraded` (Radamanto, termodinámica de entidades) ya cubren el espectro; un tercer nombre sería sinónimo sin jurisdicción propia.

### 3.2. Sin `{name}.md` no hay centinela, aunque haya binario
Un proceso que sobrevive a la sesión y del que depende una capacidad del ecosistema **es un centinela**, no un hijo de terminal. Y un centinela sin definición de genoma **no existe para el auditor** (§1). Por tanto el criterio de "supervisado" se endurece:

```text
Supervisado = {name}.md en SddIA/daemons/ + fila en index.md
            + DaemonRuntime (lock + side-channel + Daemon_Heartbeat)
            + launcher en SddIA/scripts/daemons/
            + emisor declarado en events/telemetry/daemon-heartbeat.md
```

Forja obligatoria vía `daemon-creator` / `entity-manager` (DA-2: prohibida la mutación manual del genoma). Aplica a `kalma2-bridge` (hoy sin genoma, sin `DaemonRuntime` y sin latido) y a `iota-publish-relay` (hoy con binario y latido, sin genoma).

`iota-publish-relay` es **jurisdicción de `PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO` §3.1**: este PBI no lo implementa, pero **aporta el hallazgo bloqueante** de §1 — sin `iota-publish-relay.md` la supervisión que ese PBI está construyendo no llega a producir fractura. Si el PBI DLT cierra antes, este PBI lo verifica; si no, lo desbloquea. No hay dependencia de orden: las Fases 1, 3 y 4 son independientes de la aduana DLT.

### 3.3. Lo que no es proceso se verifica, no se supone
Para invariantes sin PID el latido es imposible. Se forja el proceso **`system-vitality-probe`** (agente `argos`, misma jurisdicción que `daemon-heartbeat-audit`) que ejecuta sondas deterministas y emite **`System_Vitality_Probed`** en `eda_fractal.telemetry` con el censo de sondas y su veredicto. Es la **única** clase de evento nueva autorizada por este PBI.

Sondas de la Fase 3, todas verificables hoy sin capacidades nuevas:

| Sonda | Verificación física | Base existente |
|-------|--------------------|----------------|
| `bus.topology` | `cumulo.paths.json` parseable + overlay `.SddIA/local.paths.json` + existencia de las hojas `eda_fractal.*` | `SddIA/core/cumulo.paths.json` (v1.6.4) |
| `cumulo.tools_index` | Índice de tools sincronizado con el YAML fuente | `verify-tools-index` (`sddia-qa/src/main.rs:146`) |
| `cerbero.config` | `execution-contexts.md` parseable y `.SddIA/cerbero/revoked_entities.json` JSON válido | `cerbero_di_rbac.rs` (`CERBERO_CONFIG_ERROR`) |
| `kalma2.bridge` | `GET /api/runtime-profile` responde JSON | `kalma2-bridge/src/main.rs:75-88,1856` |

**Nombrar con honestidad:** la sonda de Cerbero mide **integridad de configuración**, no "disponibilidad". Cerbero es un gate en proceso dentro de `execute-process` (`cerbero_di_rbac.rs`), no un servicio con endpoint. Declarar que se audita su "disponibilidad" describiría un órgano que no existe.

### 3.4. La sonda reutiliza el tick existente
Se **descarta** forjar un centinela nuevo sólo para producir un tick. `event-sweeper` ya posee jurisdicción de barrido periódico y ya invoca `daemon-heartbeat-audit` cada 30 s. Se extiende con capacidad `vitality-probe-sweep` (versión y capabilities actualizadas vía `entity-manager`), con cadencia propia `SDDIA_VITALITY_PROBE_SECONDS` (default 300, piso 30). Se **descarta** `cron`: la jurisdicción de ciclo de vida es `systemd` / launchers, y `cron` quedaría fuera del contrato de daemons.

### 3.5. El veredicto negativo es fractura, no log
Sonda en rojo → `System_Fracture_Detected` en `eda_bus.pending` con `friction_id` y la causa física exacta, idempotente por incidente (mismo criterio de reset que `daemon-heartbeat-audit`: una fractura hasta que la sonda vuelva a verde). Prohibida la rama silenciosa: toda sonda tiene `else` y toda `else` registra.

### 3.6. Un intervalo, un SSOT
El intervalo de latido vive **exclusivamente** en el genoma (`SddIA/daemons/{name}.md` → `execution.heartbeat_interval_seconds`) y viaja replicado en el lock y en el side-channel, porque `missed_cycles` lo divide (`daemon_heartbeat.rs:253,275`). Prohibido introducir una variable de bóveda que declare el intervalo de latido: crearía doble SSOT y un actor externo podría desincronizar el divisor del auditor, **suprimiendo fracturas en silencio**. Nótese además que `daemon_interval` degrada silenciosamente a 30 s cuando falta el `.md` (`daemons.rs:150-165`): otra razón para que el genoma sea obligatorio y no opcional. Las variables de bóveda quedan reservadas a cadencias que **no** participan del cómputo de fractura (p. ej. `SDDIA_VITALITY_PROBE_SECONDS`), siguiendo la convención vigente en segundos (`SDDIA_EMAIL_POLL_SECONDS`, `SDDIA_GITHUB_BRIDGE_POLL_SECONDS`).

### 3.7. Metabolismo Adaptativo — condicionado a evidencia (Fase 4)
La directriz de pulso variable se conserva como intención, **no** como diseño aprobado. Objeciones bloqueantes en §7.B. Si se implementa, sólo bajo estas cuatro restricciones:

1. **Auto-declaración.** El intervalo efectivo lo decide el propio centinela y viaja en lock + side-channel de forma atómica junto al latido. El auditor divide siempre por el valor que el daemon declaró. Ningún actor externo muta el divisor.
2. **Radamanto observa, no gobierna.** Radamanto no puede mutar entorno ni configuración en runtime (verificado: sin `set_var`, sólo lectura). Su rol se limita a observar bandas patológicas y emitir `Kaizen_Alert_Required`. Asignarle el gobierno del pulso invadiría la jurisdicción de Argos.
3. **SLO de detección acotado.** `3 × intervalo_máximo` es el peor tiempo de detección de una muerte. Debe declararse como SLO explícito y quedar por debajo del techo tolerable; relajar el pulso alarga la ceguera.
4. **Peaje medido antes de optimizar.** Requisito de entrada: medir el coste real de la emisión (una escritura JSON atómica cada 30 s) sobre el ciclo termodinámico total. Si es < 1 %, la fase se **cierra como no-mejora** y se archiva. Optimizar sin medición previa es entropía.

## 4. Alcance

### Dentro
- Sincronización del contrato `daemon-heartbeat.md`: emisores autorizados = emisores reales (`event-sweeper`, `email-watcher`).
- **Aduana de censo:** verificación de que todo proceso persistente vivo tiene `{name}.md` en `SddIA/daemons/`, de modo que `audit_staleness` lo alcance. Incluye señalar (no forjar) el `iota-publish-relay.md` ausente al PBI DLT.
- Forja de `kalma2-bridge` como centinela catalogado con `DaemonRuntime`, lock, latido y auditoría; corrección del `_sddia_stop_lock_pid` sobre un lock inexistente (`start-sddia.sh:168`).
- Forja del proceso `system-vitality-probe` y de la clase `System_Vitality_Probed` (familia `telemetry`) + fila en `SddIA/events/telemetry/index.md`.
- Extensión de `event-sweeper` con `vitality-probe-sweep` y su cadencia de bóveda.
- Emisión de `System_Fracture_Detected` ante sonda en rojo.
- Gate de ignición: promoción a `REQUIRED` de los órganos vitales verificados, y bloqueo del grado operativo si una sonda vital está en rojo.

### Fuera
- **Panel visual de salud** — jurisdicción de `PBI-KAIZEN-ESPEJO-CONSCIENCIA-001`. Este PBI es su **proveedor de señal**: `System_Vitality_Probed` es precisamente el Read Model que ese panel necesita. No se renderiza nada aquí.
- **Supervisión del relay IOTA** — jurisdicción de `PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO`. Dependencia declarada, no duplicada.
- **Validador universal de índices de Cúmulo.** Hoy sólo existen verificadores parciales (`verify-tools-index`, `verify-process-integrity`). Un `verify-cumulo-indices` que cubra los 7+ catálogos de `directories.*` es deuda legítima (`DT-CUMULO-INDEX-AUDIT`) y PBI aparte: la sonda de este PBI consume lo que exista, no lo construye.
- **Reinicio automático de órganos.** `daemon-heartbeat-audit` no arranca ni mata centinelas (`daemon-heartbeat-audit.md:70`); esa jurisdicción es `governance-daemon-manager` / `daemon-kill-switch`. La sonda percibe, no interviene.
- **Métricas históricas y series temporales.**

### Entropía detectada
- `SddIA/agents/` no declara `type:` en ningún frontmatter (0 coincidencias), incumpliendo el Estándar Atómico. Fuera de alcance aquí; registrar como deuda de genoma.
- `SddIA/core/event-subscriptions.json` coexiste con `event-domain-subscriptions.json` con contenidos divergentes. Doble SSOT de suscripciones; auditar y extirpar en PBI aparte.

## 5. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
|----|----------|--------------|
| VIT-CA1 | `daemon-heartbeat.md` lista como emisores autorizados exactamente los centinelas que emiten latido en `SddIA/daemons/index.md`. | Diff contrato vs índice; `hash_signature` recalculado. |
| VIT-CA2 | `kalma2-bridge` existe como centinela forjado: `{name}.md` con uuid, fila en índice, launcher, lock y side-channel. | `ls .SddIA/daemons/status/`, `.SddIA/daemons/state/heartbeats/kalma2-bridge.json`. |
| VIT-CA2b | Todo proceso persistente vivo aparece en el censo que itera `audit_staleness`; cero locks o side-channels sin `{name}.md` correspondiente. | Diff: ficheros de `.SddIA/daemons/state/heartbeats/` y `status/` vs `ls SddIA/daemons/*.md`. |
| VIT-CA3 | Matar `kalma2-bridge` produce `System_Fracture_Detected` tras 3 ciclos omitidos, sin intervención humana. | `kill` + `daemon-heartbeat-audit --sweep` + conteo en `eda_bus.pending`. |
| VIT-CA4 | `System_Vitality_Probed` existe como clase en `SddIA/events/telemetry/` con uuid, `event_family: telemetry` y payload contractual. | Lectura del `{name}.md` + fila en `telemetry/index.md`. |
| VIT-CA5 | `system-vitality-probe` ejecuta las 4 sondas de §3.3 y devuelve veredicto por sonda con causa física. | `./sddia-run.sh --process system-vitality-probe` → JSON con censo completo. |
| VIT-CA6 | Sabotear un invariante (renombrar `execution-contexts.md`) pone su sonda en rojo con causa exacta, no con mensaje genérico. | Smoke de sabotaje; `error_trace` contiene el fichero ausente. |
| VIT-CA7 | Sonda en rojo emite `System_Fracture_Detected`; sonda que vuelve a verde permite una fractura nueva y no repite la anterior. | Sabotaje → reparación → sabotaje; conteo = 2, no 3+. |
| VIT-CA8 | `event-sweeper` invoca la sonda según `SDDIA_VITALITY_PROBE_SECONDS` y respeta el piso de 30 s ante valores inválidos o ausentes. | Test unitario del parseo + observación de cadencia. |
| VIT-CA9 | Ningún proceso persistente queda fuera del censo que habilita la auditoría. | Censo: procesos vivos post-ignición vs `ls SddIA/daemons/*.md`, y coherencia de éste con las filas de `index.md`. |
| VIT-CA10 | La ignición no declara grado operativo con una sonda vital en rojo. | Smoke: arrancar con `kalma2-bridge` saboteado; la ignición marca error. |
| VIT-CA11 | El intervalo de latido sigue teniendo un único SSOT: no se introduce variable de bóveda que lo declare. | `rg 'SDDIA_HEARTBEAT'` = 0 coincidencias en código y en `.dev/.env.example`. |

## 6. Orden de ejecución

### Fase 1 — Verdad del contrato (barata, inmediata)
Sincronizar `daemon-heartbeat.md` con los emisores reales (§1 drift). Sin esto, cualquier auditoría posterior valida contra un contrato falso.

### Fase 2 — Cerrar el censo de órganos
Forja de `kalma2-bridge` como centinela y aduana de censo (§3.2). Al terminar, todo proceso persistente tiene genoma, late y es alcanzado por `audit_staleness`. `iota-publish-relay.md` entra por su PBI, con el hallazgo de §1 trasladado como bloqueante.

### Fase 3 — Verificar lo no-proceso
Clase `System_Vitality_Probed`, proceso `system-vitality-probe`, las cuatro sondas de §3.3, extensión de `event-sweeper`, emisión de fractura y gate de ignición (§3.3–§3.5).

### Fase 4 — Metabolismo Adaptativo (condicionada)
No se abre sin la medición del peaje exigida en §3.7.4. Si la emisión de latido resulta termodinámicamente insignificante, la fase se cierra como no-mejora y se archiva con acta. Título de la directriz original, conservado por trazabilidad: *[OPERATIVO] Metabolismo Adaptativo: Rango de Latido y Gobernanza de Radamanto*.

## 7. Refutación del borrador v1

Registrado para evitar reincidencia diagnóstica. Cada afirmación fue contrastada contra el genoma.

### A. Especificación y clarificación

| Afirmación del borrador | Veredicto | Evidencia |
|-------------------------|-----------|-----------|
| "Creación de un evento `System_Heartbeat_Emitted` en la familia `telemetry`" | **Redundante** | `Daemon_Heartbeat` ya ocupa ese rol con contrato, emisor, suscriptor y auditor (`SddIA/events/telemetry/daemon-heartbeat.md`). Duplicar la clase fragmentaría `missed_cycles`. |
| "El evento auditará la disponibilidad de los nodos vitales" | **Error ontológico** | Un evento ECST es un hecho inmutable; no ejecuta lógica. La auditoría reside en un proceso (`daemon-heartbeat-audit`). |
| "Auditará la aduana RBAC de Cerbero" | **Inexacto** | Cerbero es un gate en proceso dentro de `execute-process` (`cerbero_di_rbac.rs`), sin endpoint ni PID propio. No existe `/health` ni `cerbero-status`. Reformulado a integridad de configuración (§3.3). |
| "Auditará la integridad de los índices de Cúmulo" | **Capacidad inexistente** | No existe `verify-cumulo-indices`. Sólo verificadores parciales: `verify-tools-index`, `verify-process-integrity` (`sddia-qa/src/main.rs:146-147`). Acotado en §3.3 y §4. |
| "Auditará la conexión del puente físico de Kalma2" | **Confirmado y es el hallazgo central** | `kalma2-bridge` corre con launcher pero sin `{name}.md`, sin `DaemonRuntime` y sin latido; su lock nunca se escribe pese a que `start-sddia.sh:168` intenta detenerlo por él. Mismo patrón que mató al relay IOTA. |
| "Chispazo de Nivel 1 (Infraestructura)" | **Vocabulario inexistente** | No hay taxonomía de "niveles" en `events-contract`. La clasificación real es `event_family`: `telemetry` / `orchestration` / `domain` / `progress`. |
| "Radamanto será el actuario suscrito a este evento" | **Colisión de jurisdicción** | El suscriptor de `Daemon_Heartbeat` es **Argos** vía `daemon-heartbeat-audit` (`event-telemetry-subscriptions.json:14-19`). Radamanto consume exclusivamente `Raw_Execution_Finished` (`:2-7`). |
| "Radamanto emitirá un `System_Degraded`" | **Clase inexistente** | No hay `System_Degraded` en el genoma. Existen `System_Fracture_Detected` (Argos, infraestructura) y `Domain_Entity_Degraded` (Radamanto, termodinámica de entidades). Un tercer nombre sería sinónimo. |
| "Alertando antes de que colapse una operación de negocio" | **Confirmado como objetivo** | Es la intención correcta y se conserva íntegra en §2. |
| "Definir el contrato del evento en `SddIA/events/events-contract.md`" | **Ruta equivocada** | `events-contract.md` es el contrato de familia. Cada clase vive en `SddIA/events/{family}/{name}.md` con uuid y `hash_signature` propios, más fila en el índice de familia. Sólo se versiona el contrato si cambia el contrato. |
| "Implementar un centinela ligero (cron/daemon en Rust) que inyecte el evento a intervalos regulares" | **Redundante y fuera de jurisdicción** | `event-sweeper` ya tiene tick de 30 s e invoca la auditoría (`event-sweeper/src/main.rs:11`). `cron` está fuera del contrato de daemons (jurisdicción `systemd`/launchers). Y todo centinela se forja vía `daemon-creator`, jamás a mano (DA-2). |

### B. Actualización Táctica: Metabolismo Adaptativo

| Afirmación del borrador | Veredicto | Evidencia |
|-------------------------|-----------|-----------|
| `SDDIA_HEARTBEAT_MIN_MS` / `SDDIA_HEARTBEAT_MAX_MS` | **Unidad incoherente** | Todo el mecanismo opera en **segundos**: `heartbeat_interval_seconds` en genoma, lock, side-channel y divisor de `missed_cycles`; piso de 5 s. Milisegundos exigirían conversión en cada frontera sin ganancia. |
| Alojarlas "en `.dev/.env`" | **Bóveda equivocada** | `env_hierarchy` = global `.dev/.env` (Core) → instancia `.SddIA/.dev/.env` (`cumulo.paths.json:104-107`). Una cadencia de instancia no pertenece a la bóveda global; además `.dev/.env` está gitignored y la plantilla versionada es `.dev/.env.example`. |
| El intervalo de latido pasa a variable de entorno | **Rechazado: doble SSOT peligroso** | El intervalo es el **divisor** de `missed_cycles`. Si un actor externo lo muta sin actualizar genoma, lock y side-channel, el umbral de fractura se vuelve indeterminado y **una muerte real puede dejar de producir fractura**. Reintroduciría el fallo silencioso del incidente DLT. |
| Radamanto "relajará" o "acelerará" el pulso | **Capacidad inexistente + riesgo** | Radamanto no muta entorno ni configuración en runtime (sin `set_var`; sólo lectura de `radamanto.thresholds.json` y Cúmulo). Y relajar el pulso alarga el peor tiempo de detección a `3 × intervalo_máximo`: la optimización compra ciclos vendiendo ceguera. |
| "Acelerar el pulso garantiza respuesta inmediata de Kalma2" | **Premisa falsa** | La latencia de estímulo depende del **poll del bus** (`event-watcher`: 2 s) y del long-poll sensorial, no de la emisión de latido. El latido es telemetría diagnóstica, no ruta de estímulo. Acelerarlo no mejora la respuesta de Kalma2 en absoluto. |
| "Relajar el pulso ahorra ciclos termodinámicos bajo carga de Tekton" | **No cuantificado** | El coste es una escritura JSON atómica cada 30 s por centinela. Nunca se midió contra el ciclo total. Requisito de entrada en §3.7.4: medir antes de optimizar. |
| Ticks configurables por bóveda | **Inexistentes hoy** | `HEARTBEAT_TICK_SECONDS = 10` y `HEARTBEAT_EMIT_FAIL_BUDGET = 5` son constantes Rust compiladas, no variables de entorno. |
| "Activo el Protocolo de Indexación de Supervivencia…" | **Metadiscurso, no requisito** | Es narración de sesión. Absorbido como Fase 4 condicionada; el título propuesto se conserva en §6 sólo por trazabilidad. |

**Conclusión del refinamiento:** no falta un latido. Falta **cerrar el censo del genoma** —porque el auditor sólo ve lo que tiene `{name}.md`— y **verificar activamente lo que no puede latir**. El borrador diagnosticó bien el síntoma (ceguera multiorgánica) y erró en el remedio: propuso ontología nueva sobre una jurisdicción ya ocupada, cuando el eslabón roto es documental y está a un fichero de genoma de distancia.
