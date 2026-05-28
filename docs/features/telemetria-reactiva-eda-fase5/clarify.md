---
feature_name: telemetria-reactiva-eda-fase5
created: "2026-05-28"
purpose: Decisiones Fase 5 y herencia Fases 0–4
---

# Clarificación — Fase 5

## Precondición (gate Fase 4)

- **Fase 4** cerrada con `validacion.md` APTO (PR #55): agente Radamanto, `radamanto-batch`, bucle Self-Healing, eventos dominio `Tool_*`, sandbox estricto, handoff DLT dual D0.1.
- **Peaje Termodinámico** operativo: `Raw_Execution_Finished` en `./.events/telemetry/` con payload físico (`asset_id`, `exit_code`, `duration_ms`, `process_name`); campo genómico `telemetry_receipt` declarado OPTIONAL pero **no parseado** aún.
- **Suscripción telemetría** apunta a `radamanto-batch` (estadística + Self-Healing); no mezclar auditoría de contratos en el mismo handler.
- No se reabren Fases 1–4 salvo hallazgo bloqueante durante Tekton.

## Decisiones heredadas (aplican en Fase 5)

| ID | Resolución | Uso en Fase 5 |
|----|------------|---------------|
| D0.5 | Peaje Termodinámico solo CLI | Extracción `telemetry_receipt` en `run_thermodynamic_toll`; ED no auto-reportan al bus |
| D3.13 | Fail-soft E/S telemetría | Omisión o parseo inválido del recibo **no** altera `exit_code` del proceso de negocio |
| D4.4 | `capsule_id` opcional en telemetría | Resolver contrato ED invocada para cruce compliance |
| Axioma §0.3 | Interceptación central CLI | Recibo via stdout cápsula delegada, no escritura directa ED al bus |
| PBI §5.D | Gobernanza post-breach | **Placeholder** — sin degradación RBAC ni Self-Healing en esta feature |

## Decisiones cerradas — Fase 5

| ID | Pregunta | Resolución |
|----|----------|------------|
| D5.1 | ¿Dónde vive `telemetry_receipt` en stdout? | Clave **`telemetry_receipt`** en el envelope JSON de la **última cápsula delegada** de la ejecución (skills/actions). Orden de búsqueda: raíz del objeto parseado → `data.telemetry_receipt` → `result.telemetry_receipt` (compat envelope v2 / skills-contract) |
| D5.2 | ¿Forma del recibo? | Objeto JSON con al menos un campo numérico de consumo; schema mínimo canónico: `{ "prompt_tokens": int, "completion_tokens": int }`; campos extra permitidos (`total_tokens`, `model`, `provider`) |
| D5.3 | ¿Declaración en contratos ED? | Bump **`skills-contract` v1.2.0** y **`actions-contract` v1.3.0**: campos opcionales en frontmatter de cada `{name}.md` — `telemetry_provided: false` (default implícito) y `telemetry_schema` (array de claves obligatorias en recibo cuando `telemetry_provided: true`) |
| D5.4 | ¿Obligatoriedad retroactiva? | **No.** Entidades existentes sin frontmatter → `telemetry_provided: false`; solo ED que declaren `true` entran en auditoría de incumplimiento |
| D5.5 | ¿Quién audita contrato vs recibo? | Proceso lab dedicado **`telemetry-compliance-audit`** suscrito en paralelo a `Raw_Execution_Finished` en `event-telemetry-subscriptions.json` — **no** extender `radamanto-batch` (separación actuario estadístico vs juez de contratos) |
| D5.6 | ¿Quién emite `Telemetry_Compliance_Breached`? | Handler `telemetry-compliance-audit` vía `write_fractal_event(..., "domain")`; emisor lógico `telemetry-compliance-audit`; familia `domain` |
| D5.7 | ¿Condición de breach? | `telemetry_provided: true` en spec ED resuelta por `capsule_id` (fallback `process_name` solo si ED es proceso) **y** (`telemetry_receipt` ausente en payload telemetría **o** recibo no satisface `telemetry_schema`) |
| D5.8 | ¿Enriquecimiento payload telemetría? | Añadir `capsule_id` y `telemetry_receipt` (objeto o `null`) al payload `Raw_Execution_Finished`; persistir en genoma v1.1.0 sin bump de contrato events |
| D5.9 | ¿Captura en runtime CLI? | Durante ejecución de fases, registrar en `state["last_capsule_envelope"]` el JSON parseado de la última cápsula skill/action ejecutada; Peaje lee al final |
| D5.10 | ¿Idempotencia breach? | Una emisión `Telemetry_Compliance_Breached` por `asset_id` de telemetría; registro local `.SddIA/telemetry-compliance/emitted.json` (gitignored) |
| D5.11 | ¿Suscripción dominio post-breach? | **Ninguna** en Fase 5 — evento queda disponible en `./.events/domain/` para Fase 6 README y gobernanza futura (§5.D) |
| D5.12 | ¿Skill de referencia con recibo? | Marcar **`text-metrics`** como `telemetry_provided: true` con schema mínimo para smoke lab (tokens simulados en envelope) |
| D5.13 | ¿Fan-Out telemetría — quién purga? | **Inmunidad de Fan-Out (T5.6):** `Raw_Execution_Finished` tiene dos suscriptores paralelos (`radamanto-batch`, `telemetry-compliance-audit`). **Ningún consumidor** ejecuta `os.remove()` / `unlink()` sobre el JSON fuente. Cada uno **sella** `delivery_state[{subscriber_id}]` (patrón DLQ + Ledger). La aniquilación física pertenece **exclusivamente** a infraestructura: `route-telemetry` tras consenso de sellos, o `event-sweeper` sobre `./.events/telemetry/` si el enrutador no purga aún |
| D5.14 | ¿Deuda F4 `radamanto-batch` purga directa? | **Retrofix obligatorio en F5:** retirar `event_path.unlink()` de `radamanto_batch_core.process_telemetry_file` (introducido F4 pre-Fan-Out). Sustituir por sello `delivery_state` + purga centralizada |

## Payload Peaje ampliado (`Raw_Execution_Finished`)

| Campo | Origen | Obligatorio F5 |
|-------|--------|:--------------:|
| `asset_id` | CLI | Sí |
| `exit_code` | CLI | Sí |
| `duration_ms` | CLI | Sí |
| `process_name` | CLI | Sí |
| `execution_id` | Herencia F2 | Recomendado |
| `workspace_path` | Herencia F2 | Recomendado |
| `capsule_id` | Última cápsula delegada (skill/action name) | Recomendado |
| `telemetry_receipt` | Envelope cápsula (D5.1) | No — omitido si ausente |

## Reglas de auditoría compliance

| Regla | Condición | Acción |
|-------|-----------|--------|
| **R5.1 Tolerancia** | `telemetry_provided: false` o ausente | No-op; telemetría física suficiente |
| **R5.2 Recibo presente** | `telemetry_provided: true` + recibo válido vs schema | No-op |
| **R5.3 Breach recibo vacío** | `telemetry_provided: true` + recibo ausente/`null` | Emitir `Telemetry_Compliance_Breached` |
| **R5.4 Breach schema** | Recibo presente pero falta clave de `telemetry_schema` | Emitir `Telemetry_Compliance_Breached` (motivo `schema_mismatch`) |
| **R5.5 Fail-soft parseo** | JSON stdout ilegible o sin envelope | Tratar como recibo vacío; **no** fallar ejecución CLI (AC5.1) |

## Jurisdicciones (Panteón — Fase 5)

| Agente / Proceso | Rol Fase 5 | Alcance esta feature |
|------------------|------------|----------------------|
| **CLI (Peaje)** | Interceptor pasivo | Extrae recibo; anexa a telemetría; fail-soft |
| **telemetry-compliance-audit** | Juez de contratos | Cruce recibo vs spec ED; emite dominio breach |
| **Radamanto-batch** | Actuario estadístico | Sella `delivery_state`; **prohibido** borrar archivo telemetría (D5.13) |
| **route-telemetry / event-sweeper** | Infraestructura inerte | Purga física solo tras consenso de todos los sellos suscriptor |
| **Argos** | Inspector materia | **No** auditor de tokens en Fase 5 |
| **Cerbero / Radamanto DLT** | Gobernanza post-breach | **Fuera de alcance** (§5.D placeholder) |

## Referencias

- Gate Fase 4: `docs/features/telemetria-reactiva-eda-fase4/validacion.md`
- Hallazgo F0: `impact-analysis.md` H18
- PBI: § Fase 5 (5.A–5.D)
- Origen consolidado: `docs/todos/tmp/Ampliacion_Log_Telemetris_Tokens.md`
