---
feature_name: telemetria-reactiva-eda-fase5
created: "2026-05-28"
process: feature
base: main
scope: skills-contract, actions-contract, execute_process_capsules, eda_bus_utils, telemetry-compliance-audit, Telemetry_Compliance_Breached, event-telemetry-subscriptions, cumulo.paths, tests QA
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
---

# Especificación técnica — Fase 5 · Cumplimiento termodinámico (recibos de tokens)

## 1. Contexto

Estado actual (post Fases 1–4):

- **Peaje Termodinámico** emite `Raw_Execution_Finished` con métricas físicas; genoma declara `telemetry_receipt` OPTIONAL pero CLI **no lo parsea** (reservado F3).
- **Contratos ED** (`skills-contract` v1.1.0, `actions-contract` v1.2.0) carecen de `telemetry_provided` / `telemetry_schema` (H18).
- **radamanto-batch** consume telemetría para estadística Self-Healing; no evalúa promesas contractuales de tokens.
- **Evento** `Telemetry_Compliance_Breached` **no existe** en genoma dominio.
- **Hallazgo F0 H18** clasificado como cubierto por Fase 5.

Objetivo: extender la Física del Valor con recibos opcionales y auditoría asíncrona de cumplimiento, preservando fail-soft D3.13 y sin bloquear Self-Healing.

## 2. Arquitectura objetivo

```text
execute_process_capsules
  │ fases skill/action → state.last_capsule_envelope
  └─ run_thermodynamic_toll
        ├─ extract_telemetry_receipt(envelope)
        ├─ build_raw_execution_finished_event(+ receipt, capsule_id)
        └─ write_fractal_event → ./.events/telemetry/

./.events/telemetry/Raw_Execution_Finished
        ├─► radamanto-batch          (sin cambio funcional F5)
        └─► telemetry-compliance-audit (nuevo)
                  ├─ resolve_ed_spec(capsule_id)
                  ├─ if telemetry_provided && !receipt_ok → breach
                  └─ write_fractal_event → ./.events/domain/
                        Telemetry_Compliance_Breached
```

## 3. Contratos ED — §5.A

### 3.1 Bump `skills-contract` → v1.2.0

Añadir sección **§6 Termodinámica declarativa (Fase 5)**:

| Campo frontmatter | Tipo | Default | Descripción |
|-------------------|------|---------|-------------|
| `telemetry_provided` | boolean | `false` (implícito) | La cápsula **promete** devolver recibo en stdout |
| `telemetry_schema` | string[] | `["prompt_tokens", "completion_tokens"]` cuando `telemetry_provided: true` | Claves obligatorias en `telemetry_receipt` |

Reglas:

- Ausencia de ambos campos → ED no auditada por compliance (R5.1).
- `telemetry_provided: true` sin `telemetry_schema` → usar schema mínimo canónico D5.2.
- Procesos (`process-contract`) **fuera de alcance** salvo como `process_name` fallback cuando no hay `capsule_id`.

### 3.2 Bump `actions-contract` → v1.3.0

Misma sección §6 simétrica a skills.

### 3.3 ED smoke de referencia

| Entidad | Cambio |
|---------|--------|
| `SddIA/skills/text-metrics.md` | `telemetry_provided: true`; `telemetry_schema: [prompt_tokens, completion_tokens]` |
| Handler lab `text-metrics` (si existe) o extensión mínima en `execute_process_capsules` | Devolver envelope con `telemetry_receipt` simulado en smoke |

### 3.4 Resolución de spec ED

Nueva función `resolve_ed_telemetry_contract(repo, capsule_id) -> dict`:

1. Buscar `SddIA/skills/{capsule_id}.md` → parse frontmatter.
2. Si no existe, buscar `SddIA/actions/{capsule_id}.md`.
3. Retornar `{ "telemetry_provided": bool, "telemetry_schema": list[str] | None, "entity_kind": "skill"|"action"|None }`.
4. Si `capsule_id` ausente → `{ "telemetry_provided": false }` (no breach por ambigüedad).

## 4. Peaje Termodinámico ampliado — §5.B

### 4.1 Captura de envelope cápsula

En handlers que invocan cápsulas skill/action (`invoke_capsule_action`, delegaciones fase, subprocess wrappers):

```python
state["last_capsule_id"] = action_name  # o skill_id
state["last_capsule_envelope"] = parsed_json  # última escritura gana
```

### 4.2 Extracción `telemetry_receipt`

Nueva función `extract_telemetry_receipt(envelope: dict | None) -> dict | None`:

| Paso | Acción |
|------|--------|
| 1 | Si `envelope` es `None` → `None` |
| 2 | Si `envelope.get("telemetry_receipt")` es dict no vacío → retornar |
| 3 | Si `envelope.get("data", {}).get("telemetry_receipt")` es dict → retornar |
| 4 | Si `envelope.get("result", {}).get("telemetry_receipt")` es dict → retornar |
| 5 | Cualquier excepción → `None` (fail-soft) |

### 4.3 Extensión `build_raw_execution_finished_event`

Parámetros nuevos opcionales:

- `capsule_id: str | None`
- `telemetry_receipt: dict | None`

Incluir en `payload` solo si presentes (no serializar `null` obligatorio).

### 4.4 Extensión `run_thermodynamic_toll`

Antes de `write_fractal_event`:

```python
receipt = extract_telemetry_receipt(state.get("last_capsule_envelope"))
capsule_id = state.get("last_capsule_id")
# build event con receipt + capsule_id
```

**Invariante AC5.1:** excepciones en extracción → log `[THERMODYNAMIC-TOLL-EMERGENCY]` channel=`receipt-parse`; telemetría física igualmente emitida sin recibo.

### 4.5 Formato recibo canónico (documentación)

```json
{
  "prompt_tokens": 120,
  "completion_tokens": 45,
  "total_tokens": 165,
  "model": "smoke-model"
}
```

Campos extra permitidos; validación compliance solo sobre claves de `telemetry_schema`.

## 5. Auditoría asíncrona — §5.C

### 5.1 Proceso `telemetry-compliance-audit`

| Campo | Valor |
|-------|-------|
| `name` | `telemetry-compliance-audit` |
| `context` | `event-routing`, `quality-assurance` |
| Input | `event_file_path` — JSON telemetría consumido |
| Fase única | `Auditoría cumplimiento termodinámico` |

Handler lab: `telemetry_compliance_audit_core.py`

### 5.2 Algoritmo `audit_telemetry_compliance(repo, event_path)`

1. Cargar evento; verificar `event_type == Raw_Execution_Finished`.
2. Extraer `payload.capsule_id`, `payload.telemetry_receipt`, `payload.asset_id`.
3. `contract = resolve_ed_telemetry_contract(repo, capsule_id)`.
4. Si not `contract.telemetry_provided` → `{ "status": "skipped", "reason": "not_required" }`.
5. Validar recibo vs schema (`all keys present and numeric`).
6. Si inválido → `emit_telemetry_compliance_breached(...)` si `asset_id` no en idempotencia.
7. **Sellar éxito** en `delivery_state[{subscriber_id}]` vía `stamp_fractal_delivery_state()` — valor `success` | `failed` | `skipped`.
8. **Prohibido** `os.remove()` / `unlink()` sobre `event_path` (T5.6 / D5.13).

### 5.2bis Algoritmo `process_telemetry_file` (retrofix Radamanto)

Misma regla T5.6 aplicada al consumidor existente:

1. … lógica estadística batch (sin cambio funcional) …
2. Al finalizar (éxito o skip idempotente): **sellar** `delivery_state["radamanto.radamanto-batch"]` — **no** purgar archivo.
3. Retirar **todas** las llamadas `event_path.unlink()` introducidas en Fase 4 (deuda pre-Fan-Out).

### 5.3 Función `receipt_satisfies_schema(receipt, schema) -> bool`

- `receipt` debe ser `dict`.
- Cada clave en `schema` presente con valor numérico (`int`/`float`) ≥ 0.

### 5.4 Suscripción telemetría

Actualizar `SddIA/core/event-telemetry-subscriptions.json`:

```json
{
  "Raw_Execution_Finished": [
    { "agent": "radamanto", "process": "radamanto-batch", "intent": "..." },
    {
      "agent": "argos",
      "process": "telemetry-compliance-audit",
      "intent": "Cruce recibo vs contrato ED; emisión dominio breach."
    }
  ]
}
```

> `agent: argos` es placeholder de suscripción EDA (patrón existente); emisor dominio real = proceso compliance, no agente Argos interpretativo.

### 5.5 Idempotencia

- SSOT: `.SddIA/telemetry-compliance/emitted.json` referenciado en `cumulo.paths.json` bloque `telemetry_compliance`.
- Estructura: `{ "breach_asset_ids": ["uuid", ...] }`.
- `.gitignore`: `.SddIA/telemetry-compliance/`.

### 5.6 Inmunidad Fan-Out — T5.6 (Directriz de Acero)

#### 5.6.1 Fractura detectada

`Raw_Execution_Finished` en `./.events/telemetry/` tiene **fan-out** con ≥2 suscriptores (`radamanto-batch`, `telemetry-compliance-audit`). Si cada consumidor purga el JSON tras leerlo, el segundo suscriptor encuentra `FileNotFoundError` o pierde la materia antes de procesar — **condición de carrera letal** que viola la Táctica de Inmunidad (DLQ + Ledger).

#### 5.6.2 Regla normativa

| Actor | Permitido | Prohibido |
|-------|-----------|-----------|
| `radamanto-batch` | Leer evento; mutar stats; sellar `delivery_state` | `os.remove()`, `Path.unlink()` sobre JSON fuente |
| `telemetry-compliance-audit` | Leer evento; emitir dominio breach; sellar `delivery_state` | Borrado físico del JSON fuente |
| `telemetry-batch-stub` (legacy) | Deprecar purga directa en mismo diff o marcar exempt | Nuevo borrado competitivo |
| **`route-telemetry`** | Tras dispatch: persistir sellos; purgar si consenso | Purga antes de todos los sellos |
| **`event-sweeper`** | Purga telemetría stale con todos los sellos terminales | — |

#### 5.6.3 Helper `stamp_fractal_delivery_state`

```python
def stamp_fractal_delivery_state(
    repo: Path,
    event_path: Path,
    subscriber_key: str,  # subscriber_id(subscriber)
    status: str,          # success | failed | skipped
) -> None:
    """Read-modify-write atómico del bloque delivery_state en el JSON del evento."""
```

- Usar `_write_json_atomic` (paridad `eda_bus_utils`).
- Clave canónica: `subscriber_id(subscriber)` → ej. `radamanto.radamanto-batch`, `argos.telemetry-compliance-audit`.

#### 5.6.4 Purga centralizada (infraestructura)

Nueva función `maybe_purge_fractal_telemetry_when_terminal`:

1. Cargar registry `event-telemetry-subscriptions.json`.
2. `required = required_subscriber_ids(registry, "Raw_Execution_Finished")`.
3. Leer `delivery_state` del evento en disco.
4. Si `set(required) ⊆ {k for k,v in delivery_state.items() if v in terminal_ok}` → `event_path.unlink(missing_ok=True)`.

Invocación preferente: al final de `route_fractal_event` cuando `subscriptions_rel` apunta a telemetría (`route_telemetry_event`).

**Fallback (deuda acotada):** si `maybe_purge_fractal_telemetry_when_terminal` no puede implementarse en el enrutador en el mismo PR, extender `event-sweeper.py` para escanear `./.events/telemetry/*.json` con todos los sellos terminales y purgar — **documentar en `execution.md`**; en ningún caso reintroducir borrado en consumidores.

#### 5.6.5 Esquema `delivery_state` en instancia telemetría

```json
{
  "event_id": "...",
  "event_type": "Raw_Execution_Finished",
  "delivery_state": {
    "radamanto.radamanto-batch": "success",
    "argos.telemetry-compliance-audit": "success"
  },
  "payload": { ... }
}
```

Estados terminales OK (paridad V3+ `_status_is_terminal_ok`): `success`, `skipped`, prefijos `skipped-*`.

## 6. Evento dominio — §5.C / AC5.3

### 6.1 Forjar Clase `Telemetry_Compliance_Breached`

Vía `event-creator`:

| Campo | Valor |
|-------|-------|
| `event_family` | `domain` |
| `event_type` | `Telemetry_Compliance_Breached` |
| Emisor autorizado | `telemetry-compliance-audit` |
| Payload mínimo | `asset_id`, `capsule_id`, `breach_reason` (`missing_receipt` \| `schema_mismatch`), `expected_schema`, `process_name` |

Actualizar `SddIA/events/domain/index.md` y `eda-coverage.json`.

### 6.2 Builder `build_telemetry_compliance_breached_event`

En `eda_bus_utils.py`, patrón homólogo a eventos Self-Healing.

### 6.3 Destino runtime

`./.events/domain/` — **no** `./.SddIA/events/domain/` (corrección respecto al PBI origen archivado).

## 7. SSOT `cumulo.paths.json`

Bump v1.4.0 (o parche menor coherente con última versión en main):

```json
"telemetry_compliance": {
  "emitted_registry": ".SddIA/telemetry-compliance/emitted.json"
}
```

Referencias contractuales:

- `contracts.skills` → skills-contract v1.2.0
- `contracts.actions` → actions-contract v1.3.0

## 8. Wire runtime lab

| Componente | Cambio |
|------------|--------|
| `execute_process_capsules.py` | `execute_telemetry_compliance_audit_phase`, captura envelope, extensión Peaje |
| `route_fractal_event_core.py` | Sin cambio si dispatch genérico por suscripción |
| `SddIA/process/index.md` | Fila `telemetry-compliance-audit` |
| `SddIA/process/telemetry-compliance-audit.md` | Nuevo proceso |

## 9. Tests QA

| Test | Verifica |
|------|----------|
| `test_thermodynamic_receipt_attached` | Cápsula mock con recibo → payload telemetría incluye `telemetry_receipt` |
| `test_thermodynamic_no_receipt_success` | Sin recibo → `success` negocio true; telemetría sin recibo |
| `test_telemetry_compliance_breach_missing` | ED `telemetry_provided: true` + recibo ausente → dominio breach |
| `test_telemetry_compliance_no_breach_when_false` | ED sin flag → sin evento dominio |
| `test_telemetry_compliance_schema_mismatch` | Recibo incompleto → breach `schema_mismatch` |
| `test_telemetry_fan_out_no_competitive_purge` | Dos suscriptores procesan mismo JSON; ninguno borra; archivo persiste hasta purga infra |
| `test_telemetry_purge_after_all_delivery_stamps` | Tras sellos terminales de todos los suscriptores, infra purga |
| Regresión `test_eda_fractal_bus.py` | Peaje + fan-out verdes (actualizar `test_telemetry_route_and_radamanto_purge` → no assert purga en consumidor) |
| Regresión `test_radamanto_*.py` | Self-Healing intacto |

Fixtures: ED lab temporal o uso de `text-metrics` con flag.

## 10. Gobernanza futura — §5.D (placeholder)

Documentar en `clarify.md` y `execution.md` (Tekton):

- **Sin suscripción** Cerbero/Radamanto a `Telemetry_Compliance_Breached`.
- Backlog Kaizen sugerido: contador infracciones, enlace con `Tool_Degraded`, política Cerbero.

## 11. Criterios de aceptación (PBI)

| AC | Verificación spec |
|----|-------------------|
| **AC5.1** | §4 fail-soft; tests `no_receipt_success` |
| **AC5.2** | §3 contratos + ED smoke |
| **AC5.3** | §5–6 evento dominio + test breach |

## 12. Fuera de alcance (recordatorio)

- README raíz (Fase 6).
- Reacción automática post-breach (§5.D).
- Cambios Radamanto umbrales por tokens.
- Obligatoriedad universal de recibos.

## 13. Touchpoints resumidos

| Archivo | Tipo cambio |
|---------|-------------|
| `SddIA/skills/skills-contract.md` | Bump + §6 |
| `SddIA/actions/actions-contract.md` | Bump + §6 |
| `SddIA/skills/text-metrics.md` | Frontmatter telemetry |
| `SddIA/events/domain/telemetry-compliance-breached.md` | Nuevo ECST |
| `SddIA/events/domain/index.md` | Catálogo |
| `SddIA/process/telemetry-compliance-audit.md` | Nuevo proceso |
| `SddIA/core/event-telemetry-subscriptions.json` | Suscripción dual |
| `SddIA/core/cumulo.paths.json` | Bloque compliance |
| `SddIA/scripts/qa/eda_bus_utils.py` | Builders + resolve + validate + `stamp_fractal_delivery_state` + `maybe_purge_fractal_telemetry_when_terminal` |
| `SddIA/scripts/qa/telemetry_compliance_audit_core.py` | Nuevo core (sin unlink) |
| `SddIA/scripts/qa/radamanto_batch_core.py` | **Retrofix:** retirar unlink; sello delivery_state |
| `SddIA/scripts/qa/route_fractal_event_core.py` | Purga post-consenso telemetría |
| `SddIA/scripts/qa/execute_process_capsules.py` | Peaje + handler fase |
| `SddIA/scripts/qa/test_telemetry_compliance*.py` | Tests nuevos |
| `.gitignore` | `.SddIA/telemetry-compliance/` |
