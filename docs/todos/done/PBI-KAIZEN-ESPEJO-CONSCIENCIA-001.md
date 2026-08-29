---
document_id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
title: "[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema"
format: markdown
version: "1.2.0"
created: "2026-08-27"
refined: "2026-08-29"
clarified: "2026-08-29"
status: done
refinement_status: implemented
pbi_archived: true
updated: "2026-08-29T15:30:00+02:00"
persist_ref: docs/features/kaizen-espejo-consciencia-observabilidad
branch_name: feat/kaizen-espejo-consciencia-observabilidad
priority: critica
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
related:
  - SddIA/agents/radamanto.md
  - SddIA/agents/cumulo.md
  - SddIA/agents/argos.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/process/compile-ecosystem-map-snapshot.md
  - SddIA/process/query-ecosystem-health.md
  - SddIA/events/telemetry/daemon-heartbeat.md
  - SddIA/events/domain/domain-entity-degraded.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/ecosystem-health/src/lib.rs
  - interfaces/kalma2/app.js
  - interfaces/kalma2/index.html
  - SddIA/tools/index.md
  - SddIA/core/cumulo.paths.json
  - docs/todos/done/PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2.md
  - docs/todos/done/PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md
---

# [KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema

## 0. Rectificación de Inexactitudes (Triaje Antientrópico previo)

La v1.0.0 contenía código fósil/alucinaciones detectadas al validar contra `cumulo.paths.json`, el genoma real y el precedente ya implementado `PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2` (done). Corregidas aquí:

1. **`related` alucinado.** `SddIA/library/norms/read-models-y-proyecciones/spec.md` **no existe** (verificado por glob: 0 resultados). Retirado del frontmatter y sustituido por entidades reales.
2. **Artefacto de cliente inexistente.** `CA4` apuntaba a `.SddIA/client/`; ese directorio **no existe** y los clientes Python fueron podados (feature `poda-python-rust-clientes`, ya rectificado en el PBI de telemetría). El artefacto material real es `interfaces/kalma2/` (`index.html` + `app.js`, servidos como estáticos por el bridge) y el Puente Físico es el binario **Rust** `SddIA/interfaces/kalma2-bridge/src/main.rs` en `127.0.0.1:8765`. Proponer backend Python viola `.cursorrules` §4/§5.
3. **Bridge Python inexistente.** `CA5` citaba `kalma2-bridge.py/rs`; la variante `.py` no existe. Solo Rust.
4. **Fusión ontológica binaria falsa.** El territorio **no** lo provee Radamanto en solitario. Son **tres** fuentes de instancia distintas:
   - **Vida de Centinelas/daemons:** la audita **Argos** vía proceso `daemon-heartbeat-audit` (consume `Daemon_Heartbeat`, escribe `.SddIA/daemons/state/heartbeat-audit.json`; fractura si `missed_cycles >= 3`). Atribuir heartbeats a Radamanto es incorrecto (verificado en `radamanto.md` y `daemon-heartbeat-audit.md`).
   - **Estatus termodinámico de skills/tools:** lo consolida **Radamanto** vía `radamanto-batch` en `.SddIA/radamanto/stats.json`; emite `Domain_Entity_{Degraded|Restored|Deprecated}`.
   - **Revocación/gobernanza:** Cerbero, en `.SddIA/cerbero/revoked_entities.json`.
5. **"Placeholder" no es un estado termodinámico observable hoy.** Radamanto solo mide `success_rate`/latencia de ejecuciones reales. Un adaptador placeholder (verificado: `lancedb_thought_repo/src/lib.rs` devuelve `Ok(vec![])` / `Ok(None)`) **no falla**: retorna éxito vacío, así que Radamanto lo vería `healthy`. Detectar "placeholder" exige una señal nueva (marca de contrato o heurística), inexistente hoy. Es sub-tarea no trivial (ver §3, DD-4), no un `if` inmediato.
6. **LanceDB no es entidad indexada por Cúmulo.** Los adaptadores viven en `SddIA/infrastructure/adapters/`, ruta **no** presente en `directories` de `cumulo.paths.json`. El "mapa" de conectores de infraestructura requiere una fuente de topología distinta al índice de entidades de Cúmulo (o incorporarla al SSOT). No es derivable del `index.md` de tools/skills.
7. **Duplicación con observabilidad ya viva.** El PBI de telemetría **ya** implementó el patrón: `GET /api/telemetry/cognitive` (pull sobre `.SddIA/radamanto/stats.json` clave `cognitive`), `GET /api/telemetry/stream` (SSE broadcast) y el widget "Pulso cognitivo" en `interfaces/kalma2`. Este PBI **reutiliza** ese patrón y **extiende el mismo dashboard**; no lo reinventa (ver §3, DD-1/DD-3).

## 1. Falla Estructural y Contexto

El Vértice Biológico sufre "Ceguera Espacial" respecto a la materialización física del sistema: SddIA es un motor asíncrono descentralizado con observabilidad parcial. Hoy solo existe el **Pulso Cognitivo** (consumo LLM). El usuario aún no puede ver desde Kalma2 si un adaptador es un *placeholder* (caso LanceDB, confirmado en código), si un Centinela dejó de emitir `Daemon_Heartbeat`, o si Radamanto degradó una entidad (`Domain_Entity_Degraded`). El resto del ecosistema opera como caja negra: obliga a auditar logs crudos o leer código fuente.

## 2. Objetivo Medible

Erradicar la opacidad estructural extendiendo el dashboard de observabilidad de Kalma2 con un **Espejo de Consciencia**: una proyección (Read Model) que cruza la **topología esperada** con el **estado físico real de instancia**, mostrando el estatus de la trinchera en un vistazo, junto al Pulso Cognitivo ya existente.

Flujo exitoso: el usuario abre Kalma2 (o pide el estado) y obtiene una matriz visual con:
1. Estatus de vida de los Centinelas/daemons (derivado de `heartbeat-audit.json` / sweep de Argos).
2. Estatus termodinámico de skills y tools (Activa, Degradada, Deprecada) desde `stats.json` de Radamanto + revocación de Cerbero.

**Conectores de infraestructura (LanceDB, IOTA) quedan FUERA del MVP** (ver DD-7): sus adaptadores viven en `SddIA/infrastructure/adapters/`, ruta no gobernada por Cúmulo en `cumulo.paths.json`. Mapearlos "a ciegas" violaría el SSOT y la Anti-Alucinación Espacial de Cúmulo. Se incorporan solo tras mandato de indexación (Fase 2).

## 3. Decisiones de Diseño (Clarificaciones)

- **DD-1 · Reutilizar el patrón de telemetría, no reinventar.** El consumo material replica el precedente vivo: un endpoint **pull** `GET /api/system-health` en el bridge Rust (espejo de `GET /api/telemetry/cognitive`), consumido por un panel nuevo en `interfaces/kalma2` que convive con "Pulso cognitivo". Nada de Python.
- **DD-2 · Fusión de tres vectores de instancia.** El Read Model cruza: **Mapa** (índices de Cúmulo: `SddIA/tools/index.md`, `skills/index.md`, `daemons/index.md`, `cumulo.paths.json`) contra **Territorio** compuesto por (a) `.SddIA/daemons/state/heartbeat-audit.json` (Argos), (b) `.SddIA/radamanto/stats.json` (Radamanto) y (c) `.SddIA/cerbero/revoked_entities.json` (Cerbero).
- **DD-3 · Ceguera Espacial precisada + arquitectura de dos capas.** El bridge/Kalma2 **no parsean el genoma del Core** (`SddIA/**`, contratos, `.md` de entidades). Sí pueden leer **read models de instancia** bajo `.SddIA/**` y `./.events/**` (el bridge ya lo hace con `stats.json` y `./.events/telemetry/`). Para respetar esto sin acoplar lectura del Core al vivo:
  - **Capa Mapa (snapshot):** Cúmulo —único agente con licencia para leer el genoma— precompila el inventario de entidades esperadas a un artefacto de instancia (`map-snapshot`). Se refresca en cambios de genoma (`Domain_Entity_{Created|Updated|Deleted}`) o en corridas de indexación; **nunca** en el camino caliente de los batches.
  - **Capa Territorio:** ya la mantienen sus dueños (`heartbeat-audit.json` de Argos, `stats.json` de Radamanto, `revoked_entities.json` de Cerbero).
  - **Fusión:** merge barato entre artefactos de **instancia** (map-snapshot × territorio). Es lícito computarlo on-demand en el bridge (todo son lecturas de instancia) o cachearlo con TTL corto. No requiere walk del Core en caliente.
- **DD-4 · Detección de "placeholder" acotada.** Como no hay señal termodinámica de placeholder (Rectificación #5), el MVP marca **GRIS (Letargo/Teórico)** las entidades sin ejecuciones registradas en `stats.json`, y reserva **ROJO** solo para señales duras (daemon con `missed_cycles>=3`, entidad `Domain_Entity_Degraded`/revocada). Una detección explícita de placeholder (marca de contrato en el `{name}.md` del adaptador) queda como sub-tarea opcional / Fase 2.
- **DD-5 · Quién computa la fusión.** La lógica de fusión `query-ecosystem-health` admite dos encarnaciones válidas; se decide en implementación según coste de anidamiento:
  - (a) **Core Rust** (módulo del engine, precedente `radamanto_batch_core.rs`/`telemetry_compliance_core.rs`), invocable por el bridge sin nueva orquestación CLI; o
  - (b) **cápsula atómica** (`{name}.md` + `uuid` + `type`, creada vía `entity-manager`) si se requiere entidad gobernable e indexable.
  En ambos casos persiste/expone el Read Model como artefacto de instancia (candidato: `.SddIA/observability/ecosystem-health.json`, a declarar en `cumulo.paths.json`).
- **DD-6 · Orquestación desacoplada (anti-anidamiento).** **Prohibido** invocar `query-ecosystem-health` de forma **síncrona** vía `execute-process` desde dentro de los procesos críticos `daemon-heartbeat-audit`/`radamanto-batch` (son procesos con core Rust, no daemons): anidar orquestación introduce latencia y puntos de fallo, y podría disparar `System_Fracture_Detected` en cascada. El refresco del Read Model se dispara **de forma asíncrona** reaccionando a eventos ya emitidos (`Daemon_Heartbeat`, `Domain_Entity_{Degraded|Restored|Deprecated}`, `Domain_Entity_{Created|Updated|Deleted}`). Si se necesita un disparador explícito, se define el evento `Ecosystem_State_Changed` con su **contrato atómico** `SddIA/events/**/ecosystem-state-changed.md` (uuid, familia, payload) consumido por un proceso suscriptor ligero — nunca un acoplamiento síncrono dentro del batch. Respeta DA-5 (fire-and-forget, sin polling).
- **DD-7 · Exclusión de infraestructura (SSOT primero).** Los conectores `SddIA/infrastructure/adapters/**` quedaron **fuera del MVP** del Espejo hasta gobernanza SSOT. **Deuda cerrada:** `PBI-ARCH-INFRA-ADAPTERS-SSOT-001` (`docs/todos/done/`) — `cumulo.paths.json` v1.7.0 + `index.md` + fichas con `status`. La **Fase 2** del Espejo puede consumir el censo vía `directories.infrastructure_adapters` (sin glob de `src/`). IOTA sigue en `tools/index.md`.

## 4. Alcance

### Dentro
- Lógica de fusión `query-ecosystem-health` (Core Rust o cápsula atómica, ver DD-5), que cruza map-snapshot × territorio y persiste el Read Model como artefacto de instancia declarado en `cumulo.paths.json`.
- Capa Mapa: snapshot del inventario esperado precompilado por Cúmulo (artefacto de instancia), refrescado en cambios de genoma / indexación (DD-3).
- Endpoint `GET /api/system-health` (pull) en `SddIA/interfaces/kalma2-bridge/src/main.rs`, reutilizando helpers existentes (`load_*`, `read_json_file`) y test estático de ruta en `dispatch` (patrón `telemetry_routes_exist_in_dispatch`).
- Panel nuevo en `interfaces/kalma2/index.html` + `interfaces/kalma2/app.js`, contiguo al widget "Pulso cognitivo", con semántica de colores estricta.
- Refresco **asíncrono** del Read Model, dirigido por eventos ya emitidos (DD-6). Prohibido acoplarlo síncronamente a los batches críticos.

### Fuera
- Gráficos históricos de rendimiento a largo plazo.
- Intervención manual desde la UI (reiniciar daemon, etc.): Kalma2 sigue siendo "Despertador Inerte" de lectura/inyección. El reinicio pertenece a Self-Healing (`governance-daemon-manager` / `daemon-kill-switch`).
- **Conectores de infraestructura `SddIA/infrastructure/adapters/**` (LanceDB, etc.):** excluidos hasta mandato de indexación de Cúmulo en `cumulo.paths.json` (DD-7). Diferido a Fase 2.
- Detección semántica de placeholder de infraestructura (Fase 2).
- Anidar `execute-process` de forma síncrona dentro de `daemon-heartbeat-audit`/`radamanto-batch` (DD-6).

## 5. Representación Visual (semántica de estados)
- **VERDE (S+):** Activa, cápsula con ejecuciones OK; daemon con heartbeat vigente.
- **AMARILLO (Fricción):** Degradada (Self-Healing en curso), latencia alta o advertencia de cumplimiento.
- **ROJO (Entropía):** Caído, Daemon muerto (`missed_cycles>=3`), entidad revocada/deprecada.
- **GRIS (Letargo):** Teórica / sin ejecuciones registradas / deshabilitada. (Incluye placeholders no detectables como fallo en MVP — ver DD-4.)

## 6. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
|----|----------|--------------|
| OBS-CA1 | `query-ecosystem-health` (Core Rust o cápsula atómica, DD-5) retorna un JSON que cruza el map-snapshot con el territorio de instancia (Argos/Radamanto/Cerbero). | JSON válido; si es cápsula, entidad con uuid indexada por Cúmulo. |
| OBS-CA2 | Un Centinela detenido (p. ej. `event-watcher`) aparece **ROJO** tras `missed_cycles>=3`, coherente con `heartbeat-audit.json`. | E2E Lab Smoke Test cruzando sweep de `daemon-heartbeat-audit`. |
| OBS-CA3 | Una entidad con `Domain_Entity_Degraded`/revocada se reporta diferenciada; entidades sin ejecuciones aparecen **GRIS** (no falso ROJO). | Validación de payload contra `stats.json` + `revoked_entities.json`. |
| OBS-CA4 | El panel se renderiza en `interfaces/kalma2` junto al "Pulso cognitivo", sin dependencias externas pesadas, preservando la inercia de diseño. | Revisión de `index.html` + `app.js`. |
| OBS-CA5 | El bridge y Kalma2 no parsean el genoma del Core: consumen `GET /api/system-health`, que fusiona artefactos de instancia (map-snapshot × territorio); el map-snapshot lo precompila Cúmulo. | Auditoría de `app.js` y `kalma2-bridge/src/main.rs` (+ test estático de ruta en `dispatch`). |
| OBS-CA6 | El refresco del Read Model es **asíncrono**, dirigido por eventos ya emitidos; **no** hay invocación síncrona de `query-ecosystem-health` dentro de `daemon-heartbeat-audit`/`radamanto-batch`, ni nuevos daemons, ni polling castrado (DA-5). | Auditoría de la línea de montaje: ausencia de anidamiento síncrono; disparador vía suscripción a eventos (o `Ecosystem_State_Changed` con contrato). |
| OBS-CA7 | Los conectores `SddIA/infrastructure/adapters/**` no aparecen en el panel mientras no estén gobernados en `cumulo.paths.json` (SSOT). | Revisión: ausencia de filas de infra no indexada; sin walk "a ciegas". |
