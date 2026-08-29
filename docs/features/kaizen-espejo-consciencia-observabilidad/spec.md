---
feature_name: kaizen-espejo-consciencia-observabilidad
created: "2026-08-29"
process: feature
base: main
scope: kaizen-espejo-consciencia-observabilidad
version_spec: "1.0.0"
uuid: 97d96117-49cf-4db7-b860-acd65bee216a
status: dedalo_locked
agent: dedalo
branch_name: feat/kaizen-espejo-consciencia-observabilidad
persist_ref: docs/features/kaizen-espejo-consciencia-observabilidad
pbi_ref: docs/todos/pending/[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema.md
document_id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
execution_id: "a15ad28b-27a3-491c-902e-f78c100ffd43"
depends_on:
  - docs/features/telemetria-cognitiva-llm-kalma2
  - docs/features/kalma2-bridge-rust
---

# Especificación — kaizen-espejo-consciencia-observabilidad

## 1. Topología de responsabilidades

```text
Cúmulo + index.md (tools|skills|daemons)     ← lee genoma (licencia)
  process compile-ecosystem-map-snapshot     ← evoluciona (handler nativo)
  → .SddIA/observability/map-snapshot.json   ← instancia

Argos daemon-heartbeat-audit                 ← INTANGIBLE batches
  → .SddIA/daemons/state/heartbeat-audit.json

Radamanto radamanto-batch                    ← INTANGIBLE batches
  → .SddIA/radamanto/stats.json (entities)

Cerbero revoked_entities.json                ← INTANGIBLE

query-ecosystem-health (core Rust)           ← evoluciona
  merge map × territorio → ecosystem-health.json + stdout

kalma2-bridge                                ← evoluciona
  GET /api/system-health                     ← pull (espejo cognitivo)
  GET /api/telemetry/cognitive               ← INTANGIBLE

interfaces/kalma2                            ← evoluciona (panel Espejo)
```

## 2. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | Encarnación DD-5 | Módulo `ecosystem_health_core.rs` en `execute-process` (lib interna) + handler de proceso. **No** cápsula | DD-5(a); anidamiento WASI innecesario |
| **L2** | Dónde fusiona el GET | Bridge llama la **misma** lógica (path-dep crate `execute-process` **prohibido** si arrastra CLI). Extraer lib `sddia-ecosystem-health` **solo si** el path-dep rompe el crate del puente; si no, duplicar el merge mínimo en bridge **está prohibido**. Preferencia: módulo en `SddIA/sddia-core` o fichero compartido compilado por ambos crates | Un SSOT de merge (OBS-CA1) |
| **L3** | On-demand vs persist | GET fusiona siempre desde snapshot + 3 ficheros territorio. Persiste `ecosystem-health.json` best-effort (no falla el GET si el write falla) | DD-3; lectura fresca; cache auditoría |
| **L4** | Map-snapshot | Proceso nativo `compile-ecosystem-map-snapshot` (Cúmulo). Parsea **solo** tablas de `tools/index.md`, `skills/index.md`, `daemons/index.md` (rutas vía `directories.*`). Familias MVP: `daemon`, `skill`, `tool`. **Excluye** `infrastructure_adapters` | Ceguera espacial del bridge; CA7 |
| **L5** | Disparo mapa | Fan-out extra en `event-domain-subscriptions.json` sobre `Domain_Entity_Created/Updated/Deleted` → `process: compile-ecosystem-map-snapshot`. Seed en ejecución Tekton vía `./sddia-run.sh` | Reutiliza eventos; L7 |
| **L6** | Fusión y heartbeats | **No** suscribir `query-ecosystem-health` a `Daemon_Heartbeat` ni a Degraded/Restored/Deprecated | Territorio ya está en disco; evita carga en ruta caliente; DD-6 |
| **L7** | Evento nuevo | **No** `Ecosystem_State_Changed` en MVP | PBI lo marca opcional; CA6 se cubre con L5 + L3 |
| **L8** | Precedencia color | 1 ROJO: revoked **o** `deprecated` **o** daemon `missed_cycles>=3`. 2 AMARILLO: `degraded` \| `pending_redemption` \| latencia sobre umbral Radamanto si el bucket lo expone. 3 VERDE: daemon con heartbeat y `missed_cycles<3`; entidad `healthy` con `samples`/`executions` > 0. 4 GRIS: en mapa sin territorio, o entidad sin ejecuciones, o daemon sin registro de latido | DD-4; PBI §5 |
| **L9** | Snapshot ausente | GET 200 con `map_status: absent`, filas vacías o solo territorio huérfano marcado GRIS + `warning`. No 500. No walk de genoma | Cold start |
| **L10** | Huérfanos territorio | Entidad en `stats`/`revoked`/`heartbeat-audit` **no** listada en snapshot → incluir con `on_map: false`, color según L8, no inventar nombre desde genoma | Anti-alucinación |
| **L11** | Topología Cúmulo | Claves nuevas `observability.map_snapshot`, `observability.ecosystem_health` en `cumulo.paths.json` (bump versión paths) | Como `radamanto.cognitive_inbox` |
| **L12** | Genoma | `query-ecosystem-health.md` + `compile-ecosystem-map-snapshot.md` vía `entity-manager`. Suscripciones JSON = Cúmulo (mismo PR; mutación índice EDA vía cadena emit). Handlers en engine = Tekton | DA-2 |
| **L13** | Bridge | Ruta **antes** de `serve_static`. Test estático `system_health_route_exists_in_dispatch` (paridad `telemetry_routes_exist_in_dispatch`) | OBS-CA5 |
| **L14** | WUI | `<section>` inmediata tras `.cognitive-pulse`. Fetch al cargar; sin EventSource. CSS tokens del shell existente. Cero deps npm | OBS-CA4 |
| **L15** | Fase 2 | Filas `infrastructure_adapters` + heurística placeholder. IOTA tool ya visible como tool | DD-7 cerrado SSOT; panel diferido |

## 3. Contratos de instancia

### 3.1 `map-snapshot.json`

```json
{
  "compiled_at": "ISO-8601",
  "compiler": "compile-ecosystem-map-snapshot",
  "families": {
    "daemon": [{"id": "event-watcher", "uuid": "…"}],
    "skill": [{"id": "git-manager", "uuid": "…"}],
    "tool": [{"id": "iota-immutable-publisher", "uuid": "…"}]
  }
}
```

Sin familia `adapter`. `id` = columna `name` del índice.

### 3.2 `ecosystem-health.json` / GET body

```json
{
  "success": true,
  "exit_code": 0,
  "map_status": "ok|absent|stale",
  "compiled_at": "ISO-8601|null",
  "fused_at": "ISO-8601",
  "rows": [
    {
      "family": "daemon|skill|tool",
      "id": "event-watcher",
      "uuid": "…",
      "on_map": true,
      "color": "green|yellow|red|gray",
      "reason": "heartbeat_ok|missed_cycles|healthy|degraded|deprecated|revoked|no_executions|no_heartbeat|off_map",
      "missed_cycles": 0,
      "thermo_status": "healthy|degraded|pending_redemption|deprecated|null",
      "revoked": false
    }
  ]
}
```

`color` es el único contrato WUI. `reason` para tooltip/auditoría.

### 3.3 Lectura territorio (fail-soft)

Fichero ausente o JSON inválido → objeto vacío / `daemons: {}` / lista revocadas vacía. No tumbar GET.

Claves heartbeat: `daemons.{id}.missed_cycles` (handler existente). Stats: `entities.{id}.status` y presencia de muestras (si no hay campo `samples`, `status` ausente = sin ejecuciones → GRIS). Revocadas: lista o mapa de ids; matching por `id` o `uuid`.

## 4. Proceso `query-ecosystem-health`

Inputs: `persist: bool` (default true), `compile_map: bool` (default false; true solo en seed Cúmulo/CLI, **nunca** desde GET del bridge).

Si `compile_map`: ejecuta la misma rutina que `compile-ecosystem-map-snapshot` (lee índices). El bridge **siempre** pasa implícito `compile_map=false`.

## 5. Anti-anidamiento (OBS-CA6)

Grep de entrega: cero `query-ecosystem-health` / `compile-ecosystem-map-snapshot` / `execute-process` en `daemon_heartbeat.rs` y `radamanto_batch_core.rs`.

## 6. Criterios de aceptación

| ID | Verificación |
|----|----------------|
| OBS-CA1 | JSON L3; proceso indexado con uuid |
| OBS-CA2 | Fixture `heartbeat-audit` con `missed_cycles>=3` → fila daemon ROJO; smoke lab opcional post-código |
| OBS-CA3 | stats `degraded` → AMARILLO; revoked → ROJO; sin samples → GRIS |
| OBS-CA4 | `index.html` + `app.js` + CSS; vecino de pulso cognitivo |
| OBS-CA5 | bridge sin `SddIA/tools/` walk; test ruta dispatch |
| OBS-CA6 | L6 + §5 |
| OBS-CA7 | snapshot sin adapters; UI sin filas `family=adapter` |
