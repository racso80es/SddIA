---
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
title: "[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch"
format: markdown
version: "1.0.0"
created: "2026-08-27"
status: "propuesto"
priority: "critica"
process: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-aduana-dlt-relay-supervisado
incident_ref: "Ceguera DLT 2026-08-25 → 2026-08-27 — 0 anclajes on-chain, 28 eventos en dead-letter"
friction_ids:
  - F-DLT-RELAY-SIN-SUPERVISOR
  - F-DLT-BATCH-ERROR-ENMASCARADO
  - F-IGNITION-ACTIVO-SIN-VERIFICAR
depends_on: []
related:
  - start-sddia.sh
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/tools/iota-immutable-publisher/src/main.rs
  - SddIA/scripts/daemons/
  - .SddIA/services/iota-publish-relay/server.mjs
  - SddIA/process/daemon-creator.md
---

# [KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch

## 1. Falla Estructural y Contexto

Desde el **2026-08-25** ningún evento de dominio se ha sellado en IOTA Testnet. La forja de PBIs concluye con `success: true`, el bus enruta correctamente (`pending/` y `domain/` vacíos, `event-watcher` vivo), pero **cada intento de anclaje termina en `dead-letter/subscribers/` con la traza opaca `batch-missing-merkle-anchor`**.

### Evidencia recogida

| Vector | Observación |
|--------|-------------|
| Puerto 8787 | No escucha. `ss -ltnp` sin binding. |
| `.SddIA/services/iota-publish-relay/relay.log` | Última línea `listening http://127.0.0.1:8787/v1/publish`, mtime **2026-08-24 19:41**. Nunca se reinició. |
| `event-watcher` (PID 57131) | Vivo desde 2026-08-26. El sistema nervioso sobrevivió; la aduana DLT no. |
| `dead-letter/github-bridge-*.json` (27-08 18:59) | `iota-relay-unreachable: http://127.0.0.1:8787/v1/publish: Connection refused (os error 111)` |
| `dead-letter/subscribers/*.cumulo.iota-immutable-publisher.json` | 2 (24-08) · 7 (25-08) · 8 (26-08) · 13 (27-08), **todos** con `error_trace: batch-missing-merkle-anchor`. |
| `processed/*.json` | Ningún evento con `transaction_digest` ni `merkle_anchored` desde el 25-08. |
| Bóveda `.SddIA/.dev/.env` | `IOTA_WALLET_SECRET` ✅ · `IOTA_ANCHOR_PACKAGE_ID` ✅ · `SDDIA_LAB_SIMULATE_IOTA=0` · `SDDIA_LAB_MOCK_IOTA_URL=` vacío · `IOTA_PUBLISH_RELAY_URL` correcto. **La bóveda no es la causa.** |

### Cadena causal

1. **Muerte silenciosa del relay.** `start-sddia.sh:503` levanta la aduana DLT como hijo desnudo del shell interactivo: `(cd "$IOTA_RELAY_DIR" && node server.mjs > relay.log 2>&1) &`. Sin launcher en `SddIA/scripts/daemons/`, sin `.lock` en `.SddIA/daemons/status/`, sin heartbeat, sin reinicio. Al cerrar la terminal el relay recibe SIGHUP y muere; los centinelas (jurisdicción systemd) sobreviven. **Asimetría de ciclo de vida:** los consumidores de la DLT son persistentes, la DLT no.

2. **La ignición miente.** Tras el `sleep 1` (línea 505) el script imprime `-> IOTA Relay: ACTIVO en puerto 8787` de forma **incondicional**, sin comprobar el binding. Además el relay no figura en `REQUIRED_DAEMONS` ni `OPTIONAL_DAEMONS`, y `_wait_required_heartbeats` no lo audita: el ecosistema declara *"S+ Grade operativo"* con la aduana caída. En modo `systemd`, `cleanup()` retorna en la línea 122 **antes** de matar `IOTA_RELAY_PID`, dejando además procesos huérfanos posibles.

3. **El motor traga el error real.** `route_domain_batch` (`route_domain_core.rs:1635-1718`) pre-sella el lote invocando `iota-immutable-publisher` con payload array (Merkle). El resultado se consume con `if let Ok(Ok(result))` y `success == true` — **sin rama `else`, sin log, sin telemetría**. Cuando la cápsula devuelve `iota-relay-unreachable`, el fallo se descarta en silencio: los eventos nunca reciben `merkle_anchored`. Acto seguido, `route_domain_event(repo, path, true)` (línea 1721, `batch_mode_iota` siempre `true`) encuentra el `delivery_state` vacío y emite `batch-missing-merkle-anchor` (líneas 842/850). **El síntoma sustituye a la causa**: el operador lee "falta ancla Merkle" cuando el hecho físico es "el relay no responde".

4. **Nadie se entera.** El evento muere en `dead-letter/subscribers/` sin `System_Fracture_Detected`, sin notificación y sin métrica. La ceguera duró 3 días.

## 2. Objetivo Medible

Que la ausencia de anclaje DLT sea **imposible de sufrir en silencio**: o el relay está vivo y supervisado, o el sistema lo grita con la causa física exacta en el primer intento fallido.

Éxito si:
1. El relay IOTA se levanta y se supervisa con el mismo contrato que el resto de centinelas (lock, log, heartbeat, reinicio).
2. La ignición no declara `ACTIVO` sin verificación real del puerto, y falla ruidosamente si la aduana DLT no responde.
3. Un fallo de pre-sellado Merkle propaga la causa real (`iota-relay-unreachable`, `config-missing: IOTA_WALLET_SECRET`, …) hasta el `error_trace` del dead-letter.
4. Los 28 eventos huérfanos de la ventana 25-08 → 27-08 quedan anclados o formalmente amnistiados con acta.

## 3. Decisiones Arquitectónicas Obligatorias

### 3.1. El relay es un centinela, no un hijo de terminal
Forjar `iota-publish-relay` como daemon de primera clase **vía `daemon-creator` / `entity-manager`** (DA-2: prohibida la mutación manual del genoma). Debe obtener launcher en `SddIA/scripts/daemons/`, `.lock` en `.SddIA/daemons/status/`, log propio y latido auditable por `daemon-heartbeat-audit`. `start-sddia.sh` deja de instanciarlo inline y lo trata como al resto: `_start_daemon iota-publish-relay`.

Clasificación: **REQUIRED** cuando `SDDIA_LAB_SIMULATE_IOTA=0` y `IOTA_PUBLISH_RELAY_URL` apunta a loopback; irrelevante en perfil `consumer` o con simulación activa. La aduana DLT es infraestructura de instancia, no del Core: su ruta se resuelve por Cúmulo, nunca cableada.

### 3.2. Prohibido el `ACTIVO` no verificado
Sustituir `sleep 1 && echo ACTIVO` por sondeo real del endpoint (`_wait_http` ya existe en el script). Sin binding confirmado → `[ERROR]` y contabilización explícita en el resumen de ignición. El mensaje de estado debe reflejar el territorio, nunca la intención.

### 3.3. El fallo de pre-sellado se propaga, no se traga
En `route_domain_batch`, capturar el envelope de error de `iota-immutable-publisher` y arrastrarlo hasta el dispatch. `batch-missing-merkle-anchor` deja de ser una cadena literal y pasa a ser un prefijo con causa: `batch-anchor-failed: iota-relay-unreachable: …`. La rama `else` debe existir y debe registrar.

### 3.4. El silencio se convierte en fractura
Un fallo de pre-sellado emite `System_Fracture_Detected` con `friction_id: F-DLT-RELAY-SIN-SUPERVISOR`. La ceguera DLT es una fractura de dominio, no un log de depuración.

### 3.5. Peaje Termodinámico coherente
Decidir y documentar explícitamente el régimen: **fail-hard** (el ciclo de entrega no cierra sin ancla) o **fail-soft con deuda registrada** (cierra, pero deja el evento en cola de re-anclaje). Hoy el sistema hace fail-soft *de facto* y sin registro, que es el peor de los tres mundos. Se propone fail-soft **con cola de re-anclaje persistente** y visibilidad obligatoria.

## 4. Alcance

### Dentro
- **Rescate del corpus huérfano: re-anclaje de los 28 eventos de `dead-letter/subscribers/` (fase 0, precede a todo lo demás).**
- Forja del daemon `iota-publish-relay` por cadena autorizada.
- Refactor del bloque DLT de `start-sddia.sh` (líneas 496-510) y de `cleanup()`.
- Propagación de causa en `route_domain_batch` + emisión de fractura.
- Cola/acción de re-anclaje permanente para eventos con `delivery_state` sin ancla.
- Backfill con acta Merkle de la ventana 2026-08-25 → 2026-08-27.

### Fuera
- Panel visual de salud del ecosistema — cubierto por `PBI-KAIZEN-ESPEJO-CONSCIENCIA-001`, del que este PBI es **proveedor de señal**, no sustituto.
- Migración del relay Node.js a cápsula Rust nativa. Es deuda legítima (`DT-DLT-RELAY-NODE`) pero ortogonal: primero supervisión, después sustitución.
- Cambios en el contrato de firma o en el paquete ancla de IOTA.

### Código fósil detectado
`invoke_iota_publisher` (`route_domain_core.rs:427`) es la vía no-batch de anclaje. Dado que `route_domain_batch` es el **único** punto de entrada de enrutamiento y siempre invoca con `batch_mode_iota = true`, esta función está muerta salvo en test. Auditar y decidir: rehabilitar como fallback unitario cuando el pre-sellado del lote falle, o extirpar.

## 5. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
|----|----------|--------------|
| DLT-CA1 | `iota-publish-relay` existe como daemon forjado por `daemon-creator`, con launcher, lock y heartbeat. | `ls SddIA/scripts/daemons/`, `.SddIA/daemons/status/iota-publish-relay.lock`, `daemon-heartbeat-audit`. |
| DLT-CA2 | La ignición con el puerto 8787 ocupado por un proceso ajeno o el relay caído **no** imprime `ACTIVO` y marca error. | Smoke: arrancar con relay saboteado. |
| DLT-CA3 | Con el relay detenido, el dead-letter registra `iota-relay-unreachable` en `error_trace`, no `batch-missing-merkle-anchor` a secas. | E2E lab: parar relay, emitir evento de dominio, inspeccionar dead-letter. |
| DLT-CA4 | Un fallo de pre-sellado deposita `System_Fracture_Detected` en el bus. | Conteo en `.events/` tras el smoke de DLT-CA3. |
| DLT-CA5 | Matar el relay y esperar el intervalo de supervisión lo devuelve a vivo sin intervención humana. | `kill` + verificación de binding. |
| DLT-CA6 | Los 28 eventos de la ventana 25-08 → 27-08 quedan anclados en un lote Merkle único y amparados por acta firmada. | `merkle-acta-dlt-backfill-20260827.json` + raíz común + digest real. |
| DLT-CA7 | Ningún evento anclado conserva `transaction_digest: "batched-digest"`. | `is_valid_iota_anchor` sobre el corpus reprocesado. |
| DLT-CA8 | Todo evento rescatado lleva `anchored_retroactively: true` y no simula contemporaneidad con el hecho original. | Auditoría del censo del acta. |
| DLT-CA9 | El rescate no reinyecta eventos en `pending/`: cero subscribers re-disparados, cero efectos de dominio duplicados. | Diff de `.events/` y conteo de notificaciones antes/después. |
| DLT-CA10 | Tras la Fase 2, un relay caído genera cola de re-anclaje automática; la Fase 0 deja de requerir intervención manual. | E2E: parar relay, emitir evento, arrancar relay, verificar sellado sin comandos humanos. |

## 6. Orden de ejecución

La secuencia no es negociable: **el rescate del corpus precede a la reforma estructural**. Cada hora que el relay lleva caído amplía la ventana de eventos sin sellar, y el corpus huérfano es el único artefacto irrecuperable si se pierde el bus.

### Fase 0 — Rescate (inmediata)
1. Relay vivo y verificado en 8787.
2. Inventario de los 28 eventos de `dead-letter/subscribers/*.cumulo.iota-immutable-publisher.json` con `error_trace: batch-missing-merkle-anchor`, resolviendo cada `event_uuid` a su envelope real.
3. Re-anclaje en **un único lote Merkle** — no 28 transacciones sueltas: preserva el peaje termodinámico y produce una raíz común auditable para toda la ventana de ceguera.
4. Persistir pruebas por evento en `eda_instance.proofs` (`.SddIA/proofs`, resuelto vía Cúmulo) y sellar `delivery_state` con `merkle_anchored: true` + `transaction_digest` real.
5. Reintegrar los eventos rescatados desde `dead-letter/` a `processed/`, jamás a `pending/`: reinyectarlos en la cola re-dispararía subscribers ya ejecutados (notificaciones, materializaciones) y duplicaría efectos de dominio.
6. Emitir `merkle-acta-dlt-backfill-20260827.json` con raíz, digest, censo de UUIDs y ventana temporal.

**Restricción de honestidad:** los eventos rescatados quedan marcados con `anchored_retroactively: true` y la marca temporal del anclaje real. Un sello del 27-08 sobre un hecho del 25-08 no puede presentarse como contemporáneo; la DLT prueba existencia posterior a la fecha del anclaje, no anterior. Falsear esa distancia corrompería la única garantía que la cadena aporta.

**Criterio de parada:** si el re-anclaje del lote falla, la Fase 0 se detiene y escala. Prohibido pasar a Fase 1 dejando el corpus a medias — un backfill parcial sin acta es peor que ninguno, porque genera la ilusión de trazabilidad.

### Fase 1 — Supervisión
Forja del daemon, refactor de la ignición y de `cleanup()` (§3.1, §3.2).

### Fase 2 — Percepción
Propagación de causa real y emisión de fractura (§3.3, §3.4), más la cola de re-anclaje permanente que hace de la Fase 0 un procedimiento rutinario y no un rescate manual.

### Fase 3 — Régimen
Decisión documentada del Peaje Termodinámico (§3.5) y resolución del código fósil `invoke_iota_publisher`.

## 7. Refutación de hipótesis previas

Registrado para evitar reincidencia diagnóstica:

| Hipótesis | Veredicto | Evidencia |
|-----------|-----------|-----------|
| Ausencia del evento `PullRequest_Merged` por desacople EDA | **Refutada** | `dead-letter/91c28717-….json` es un `PullRequest_Merged` emitido por `accept-pr` el 27-08 18:30. Llegó al router y disparó el subscriber. |
| Extirpación de secretos de la bóveda tras la purga ontológica | **Refutada** | `.SddIA/.dev/.env` conserva `IOTA_WALLET_SECRET` e `IOTA_ANCHOR_PACKAGE_ID`. La cápsula nunca llegó a validar config: falló antes, en el transporte HTTP. |
| Centinela `event-watcher` inactivo | **Refutada** | Proceso vivo desde el 26-08; `pending/` y `domain/` vacíos. El enrutado funciona. |
| Modo laboratorio `SDDIA_LAB_SIMULATE_IOTA=1` | **Refutada** | Vale `0` en ambas bóvedas; `SDDIA_LAB_MOCK_IOTA_URL` vacío. Configuración de producción correcta. |
| Fail-soft devolviendo `success: true` con aborto silencioso | **Parcial** | El silencio es real, pero no vive en el Peaje Termodinámico: vive en el `if let Ok(Ok(_))` sin `else` de `route_domain_batch`. El evento no se "aborta": muere en dead-letter. |

La causa raíz no fue arquitectónica ni de secretos: **un proceso Node.js sin supervisor murió el 24-08 y el sistema carecía de órgano para percibirlo.**
