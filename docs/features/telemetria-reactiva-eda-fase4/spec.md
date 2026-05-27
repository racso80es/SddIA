---
feature_name: telemetria-reactiva-eda-fase4
created: "2026-05-27"
process: feature
base: main
scope: radamanto agent, radamanto-batch, fix-tool-process, cerbero-governance-react, domain events Tool_*, event-subscriptions, cumulo.paths, execute_process_capsules, route_fractal, tests QA
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
---

# Especificación técnica — Fase 4 · Radamanto + bucle Self-Healing

## 1. Contexto

Estado actual (post Fases 1–3):

- **Telemetría CLI** operativa: `Raw_Execution_Finished` en `./.events/telemetry/` vía Peaje Termodinámico (D3.13 fail-soft).
- **Suscripción telemetría** apunta a `telemetry-batch-stub` — lee, loguea y purga sin estadística ni DLT.
- **Sin agente Radamanto** en `SddIA/agents/` (H12).
- **DLT** (`iota-immutable-publisher`) invocado **solo** por Cúmulo en `PullRequest_*` / `Domain_Entity_*` (`event-domain-subscriptions.json`).
- **Cerbero** valida RBAC en runtime; **no** reacciona a eventos dominio de gobernanza herramientas.
- **Eventos dominio** Self-Healing (`Tool_Degraded`, etc.) **no existen** en genoma.
- **Proceso `fix-tool-process`** inexistente.

Objetivo: materializar Radamanto como actuario determinista, cerrar bucle Self-Healing con sandbox estricto y handoff DLT D0.1 sin romper CI IOTA existente.

## 2. Arquitectura objetivo

```text
.events/telemetry/  ──route-telemetry──►  radamanto-batch
                                              │
                    ┌─────────────────────────┼─────────────────────────┐
                    ▼                         ▼                         ▼
         .SddIA/radamanto/stats.json   iota-immutable-publisher   ./.events/domain/
         (acumulador)                  (Tool_* / Status_*)         Tool_Degraded …
                                                    │
.events/domain/ ──route-domain──► cerbero-governance-react (revocación)
                               └──► fix-tool-process (sandbox Dédalo/Tekton)
```

## 3. Agente Radamanto — §4.A

### 3.1 Artefactos

| Archivo | Propósito |
|---------|-----------|
| `SddIA/agents/radamanto.md` | Carta de naturaleza: actuario, Ceguera Espacial, prohibiciones |
| `SddIA/agents/radamanto.instructions.json` | Reglas batch, umbrales default, transiciones de estado |
| `SddIA/agents/radamanto.thresholds.json` | SSOT configurable (referenciado desde `cumulo.paths.json`) |
| `SddIA/agents/index.md` | Fila catálogo Radamanto |

### 3.2 Frontmatter agente (mínimo)

```yaml
name: radamanto
version: "1.0.0"
contract: agents-contract v1.0.0
allowed_policies:
  - quality-assurance
  - ecosystem-evolution
```

### 3.3 Prohibiciones contractuales (AC4.1, AC4.2)

| Prohibido | Motivo |
|-----------|--------|
| Invocar `skill:shell-executor`, cronómetros, medición directa | Telemetría solo vía bus CLI |
| Modificar `SddIA/tools/`, `SddIA/skills/` directamente | Jurisdicción Tekton en sandbox |
| Sellar `PullRequest_*` / `Domain_Entity_*` | Reservado Cúmulo (D0.1) |
| Interpretar código fuente o intenciones | Genoma determinista — solo stats |

### 3.4 Exclusividad DLT

Radamanto (vía handler batch) es el **único** agente autorizado a invocar `tool:iota-immutable-publisher` para:

- `Tool_Degraded`
- `Status_Restored`
- `Tool_Deprecated`

Implementación: entrada dedicada en `event-domain-subscriptions.json` con `agent: radamanto` + `tool: iota-immutable-publisher` **solo** en fan-out post-emisión dominio desde batch (patrón witness existente en `route_domain_event_core.py`).

## 4. Handoff DLT — §4.0 (D0.1)

### 4.1 Acta de transición (documento en `persist_ref`)

Archivo `dlt-handoff-acta.md` en feature:

| Evento | Suscriptor DLT post-Fase 4 |
|--------|---------------------------|
| `PullRequest_Presented` / `PullRequest_Merged` | **Cúmulo** (sin cambio) |
| `Domain_Entity_Created/Updated/Deleted` | **Cúmulo** (sin cambio) |
| `Tool_Degraded` / `Status_Restored` / `Tool_Deprecated` | **Radamanto** (nuevo) |

### 4.2 Ventana dual CI

- Mantener tests existentes con witness `cumulo.iota-immutable-publisher`.
- Añadir `test_radamanto_dlt_tool_status.py` con `SDDIA_LAB_RADAMANTO_DLT=1`.
- No retirar suscripciones Cúmulo en esta feature.

## 5. Proceso `radamanto-batch` — sustituto del stub

### 5.1 Contrato proceso

| Campo | Valor |
|-------|-------|
| `name` | `radamanto-batch` |
| `context` | `event-routing`, `quality-assurance`, `ecosystem-evolution` |
| Input | `event_file_path` — JSON telemetría |
| Output | `batch_result` — stats actualizados, acciones emitidas |

### 5.2 Flujo handler (`execute_process_capsules.py`)

1. Leer instancia `Raw_Execution_Finished`.
2. Idempotencia: skip si `asset_id` ya en `.SddIA/radamanto/consumed.json`.
3. Actualizar `.SddIA/radamanto/stats.json` por clave `target_entity_id`.
4. Evaluar reglas R4.1–R4.4 (`radamanto.thresholds.json`).
5. Si acción requerida:
   - Forjar instancia dominio vía `write_fractal_event(..., family="domain")`.
   - Invocar fan-out dominio (Cerbero + fix-tool + DLT Radamanto).
6. Purgar archivo telemetría fuente (mismo comportamiento stub).

### 5.3 Suscripción telemetría (reemplazo)

```json
"Raw_Execution_Finished": [
  {
    "agent": "radamanto",
    "process": "radamanto-batch",
    "intent": "Acumulado estadístico batch; umbrales; emisión dominio + DLT."
  }
]
```

Deprecar `telemetry-batch-stub.md` — mantener handler como alias redirect una release o eliminar con test de regresión actualizado.

## 6. Umbrales — §4.B (AC4.3)

### 6.1 SSOT `radamanto.thresholds.json`

```json
{
  "version": "1.0.0",
  "success_rate_min": 0.85,
  "batch_min_events": 10,
  "latency_ms_p95_threshold": 30000,
  "redemption_success_count": 3,
  "max_recovery_attempts": 3,
  "abrupt_drop_min_samples": 3
}
```

### 6.2 Referencia en `cumulo.paths.json`

Bump `version` → **1.3.0**:

```json
"radamanto": {
  "stats": ".SddIA/radamanto/stats.json",
  "consumed": ".SddIA/radamanto/consumed.json",
  "thresholds": "SddIA/agents/radamanto.thresholds.json",
  "sandbox_root": ".SddIA/sandbox/"
}
```

### 6.3 `.gitignore`

```gitignore
.SddIA/radamanto/
.SddIA/sandbox/
.SddIA/cerbero/revoked_entities.json
```

## 7. Eventos dominio — §4.E

Forjar vía `event-creator` (`event_family: domain`):

| Clase | `event_type` | Emisor autorizado | Payload REQUIRED |
|-------|--------------|-------------------|------------------|
| `tool-degraded` | `Tool_Degraded` | `radamanto` | `target_entity_id`, `reason`, `success_rate`, `recovery_attempt` |
| `status-restored` | `Status_Restored` | `radamanto` | `target_entity_id`, `success_rate` |
| `tool-deprecated` | `Tool_Deprecated` | `radamanto` | `target_entity_id`, `recovery_attempts`, `reason` |

Actualizar `SddIA/events/domain/index.md` y `eda-coverage.json`.

## 8. Suscripciones EDA — §4.C (AC4.4)

### 8.1 `event-domain-subscriptions.json` (añadir)

```json
"Tool_Degraded": [
  {
    "agent": "cerbero",
    "process": "cerbero-governance-react",
    "intent": "Revocar RBAC entidad degradada."
  },
  {
    "agent": "dedalo",
    "process": "fix-tool-process",
    "intent": "Iniciar reparación en sandbox."
  }
],
"Status_Restored": [
  {
    "agent": "cerbero",
    "process": "cerbero-governance-react",
    "intent": "Rehabilitar RBAC."
  }
],
"Tool_Deprecated": [
  {
    "agent": "cerbero",
    "process": "cerbero-governance-react",
    "intent": "Bloqueo permanente."
  }
]
```

**Nota D0.1:** el sellado DLT Radamanto ocurre en el handler batch **antes** o **como** suscriptor tool en fan-out — no duplicar con Cúmulo.

## 9. Proceso `fix-tool-process` — §4.D (AC4.5)

### 9.1 Fases

| Fase | Delegado | Intent |
|------|----------|--------|
| Preparación sandbox | `skill:filesystem-manager` | Materializar `.SddIA/sandbox/{entity_id}/{attempt}/` |
| Diseño reparación | `agent:dedalo` | Blueprint en sandbox **solo** |
| Ejecución reparación | `agent:tekton` | Mutación **solo** bajo sandbox |
| Verificación | `agent:argos` | Gate calidad artefacto reparado |

### 9.2 Restricciones sandbox

Handler lab debe:

- Inyectar `writable_root` = sandbox path; rechazar paths bajo `SddIA/tools/`, `SddIA/skills/` en delegaciones.
- Variable `SDDIA_SANDBOX_STRICT=1` (default en lab Self-Healing).
- Test: intento write fuera sandbox → `exit_code != 0`.

### 9.3 Proceso `cerbero-governance-react`

Lab handler: lee payload dominio, actualiza `.SddIA/cerbero/revoked_entities.json`.

Integración Cerbero gate (`execute_process_capsules` § Cerbero): si `target_capsule` ∈ revoked → `exitCode: 1`.

## 10. Límite de redención — §4.E implícito (AC4.6)

Estado por entidad en `stats.json`:

```json
{
  "entities": {
    "skill:filesystem-manager": {
      "samples": [...],
      "status": "healthy|degraded|deprecated",
      "recovery_attempts": 0,
      "degraded_at": null
    }
  }
}
```

Transiciones:

- `healthy` + R4.1/R4.2 → `degraded`, incrementar `recovery_attempts`, emitir `Tool_Degraded`.
- `degraded` + Argos OK + R4.3 → `healthy`, emitir `Status_Restored`.
- `degraded` + fallo reparación + `recovery_attempts >= max` → `deprecated`, emitir `Tool_Deprecated`.

## 11. Módulos código

| Módulo | Responsabilidad |
|--------|-----------------|
| `radamanto_batch_core.py` | Stats, umbrales, emisión dominio |
| `cerbero_governance_react_core.py` | Revocación/rehabilitación JSON |
| `fix_tool_process_core.py` | Orquestación sandbox lab |
| `execute_process_capsules.py` | Handlers + Cerbero check revoked |
| `route_fractal_event_core.py` | Wire nuevos procesos |
| `eda_bus_utils.py` | Helpers load thresholds / stats paths |

## 12. Scripts QA

| Script | Acción |
|--------|--------|
| `test_eda_fractal_bus.py` | Actualizar: stub → radamanto-batch; assert stats |
| Nuevo `test_radamanto_self_healing.py` | Degradación sintética → revocación → sandbox → redención |
| Nuevo `test_radamanto_dlt_tool_status.py` | Witness DLT Radamanto (lab flag) |
| `test_eda_bus_v3plus.py` | Sin regresión Cúmulo DLT |
| `run-iota-ci-smoke.py` | Ventana dual documentada |

## 13. Fuera de alcance (explícito)

- `telemetry_receipt` / `Telemetry_Compliance_Breached` (Fase 5).
- Retirada suscripciones Cúmulo PR/ECST.
- Runtime IDE completo Radamanto (neurona); Fase 4 = handler determinista + contratos.
- `README.md` raíz (Fase 6).
- NFT burn real en red IOTA (simular witness lab).

## 14. Criterios de aceptación (trazabilidad)

| AC PBI | Verificación |
|--------|--------------|
| AC4.1 | `radamanto.md` + exclusividad DLT documentada y cableada |
| AC4.2 | Contrato prohíbe medición; batch solo lee telemetría CLI |
| AC4.3 | `radamanto.thresholds.json` + reglas R4.x en instructions |
| AC4.4 | Suscripciones dominio Cerbero + fix-tool; test Self-Healing |
| AC4.5 | Test sandbox: write producción rechazado |
| AC4.6 | Test `max_recovery_attempts` → `Tool_Deprecated` |
