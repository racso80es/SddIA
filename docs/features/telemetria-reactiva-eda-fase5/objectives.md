---
feature_name: telemetria-reactiva-eda-fase5
created: "2026-05-28"
process: feature
branch_name: feat/telemetria-reactiva-eda-fase5
persist_ref: docs/features/telemetria-reactiva-eda-fase5
master_pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
phase: 5
pbi_archived_at_close: false
status: validacion_apto
depends_on:
  - docs/features/telemetria-reactiva-eda-fase1
  - docs/features/telemetria-reactiva-eda-fase2
  - docs/features/telemetria-reactiva-eda-fase3
  - docs/features/telemetria-reactiva-eda-fase4
gate_ref: docs/features/telemetria-reactiva-eda-fase4/validacion.md
---

# Objetivos — Telemetría Reactiva EDA · Fase 5 (Cumplimiento termodinámico)

## Calificación de densidad

Fase **evolutiva** (prioridad media): extiende el Peaje Termodinámico y el genoma contractual sin alterar el bucle Self-Healing ni la jurisdicción DLT de Radamanto. El riesgo principal es **scope creep** hacia gobernanza reactiva post-breach (§5.D), explícitamente **excluido** de esta entrega.

## Misión

Ejecutar la **Fase 5** del PBI maestro `PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO` como **feature independiente**: auditar de forma **asíncrona** si una Entidad de Dominio cumple su promesa de entregar métricas de consumo (tokens LLM u homólogos) tras ejecución, **sin bloquear** la Línea de Montaje.

Materializar recibos termodinámicos opcionales en `Raw_Execution_Finished`, declaración explícita en contratos ED, y emisión de `Telemetry_Compliance_Breached` ante incumplimiento detectado.

El PBI unificado permanece en `docs/todos/pending/` como plan de ruta. Esta feature **no** archiva el PBI maestro al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature | Estado |
|----------|---------|--------|
| 0 | `telemetria-reactiva-eda-fase0` | Cerrada — `impact-analysis.md` AC0.x |
| 1 | `telemetria-reactiva-eda-fase1` | Cerrada — genoma fractal + `Raw_Execution_Finished` AC1.x |
| 2 | `telemetria-reactiva-eda-fase2` | Cerrada — workspaces dinámicos AC2.x |
| 3 | `telemetria-reactiva-eda-fase3` | Cerrada — Peaje + bus fractal AC3.x |
| 4 | `telemetria-reactiva-eda-fase4` | Cerrada — Radamanto + Self-Healing AC4.x |
| **5** | **`telemetria-reactiva-eda-fase5` (esta)** | Cumplimiento termodinámico tokens |
| 6 | `telemetria-reactiva-eda-fase6` | Pendiente — README raíz |

## Contexto heredado (Fases 0–4)

| Decisión / hallazgo | Implicación Fase 5 |
|---------------------|-------------------|
| **H18** (F0) | Contratos skills/actions sin `telemetry_provided` — objetivo §5.A |
| **D3.13** Fail-soft Peaje | Omisión de recibo no detiene negocio — AC5.1 |
| **Genoma** `telemetry_receipt` OPTIONAL | Activar parseo CLI + documentar schema en spec |
| **D4.4** `capsule_id` en telemetría | Resolver spec ED para auditoría compliance |
| **radamanto-batch** consumidor telemetría | Suscripción paralela `telemetry-compliance-audit` — no fusionar handlers |
| **PBI §5.D** | Reacción ecosystem post-breach — **placeholder** documental |

## Objetivos medibles (Fase 5)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F5-O1** | **Tolerancia CLI** | Herramientas sin recibo ejecutan con éxito de negocio intacto; telemetría física emitida | AC5.1 |
| **F5-O2** | **Contrato ED declarativo** | `skills-contract` / `actions-contract` documentan `telemetry_provided` y `telemetry_schema`; al menos una ED smoke con `true` | AC5.2 |
| **F5-O3** | **Extracción recibo Peaje** | CLI anexa `telemetry_receipt` a `Raw_Execution_Finished` cuando la cápsula lo devuelve | AC5.1 (extensión) |
| **F5-O4** | **Auditoría asíncrona** | Proceso `telemetry-compliance-audit` cruza recibo vs contrato ED | AC5.3 |
| **F5-O5** | **Evento dominio breach** | `Telemetry_Compliance_Breached` forjado en genoma e instanciado en `./.events/domain/` | AC5.3 |
| **F5-O6** | **Separación jurisdiccional** | Radamanto-batch no emite breach; Argos no audita tokens | D5.5, D5.11 |
| **F5-O7** | **Inmunidad Fan-Out telemetría** | Ningún consumidor borra el JSON fuente; sellos `delivery_state`; purga solo infraestructura | D5.13, T5.6 |

## Modelo de flujo

```text
Cápsula ED (stdout) ──► CLI Peaje ──► ./.events/telemetry/ (Raw_Execution_Finished + receipt?)
                                              │
                              route-telemetry (fan-out, sin borrado competitivo)
                    ┌─────────────────────────┴─────────────────────────┐
                    ▼                                                   ▼
           radamanto-batch                         telemetry-compliance-audit
           stats + sello delivery_state            cruce spec + sello delivery_state
                    └─────────────────────────┬─────────────────────────┘
                                              ▼
                              infra: route-telemetry purge | event-sweeper
                              (solo si todos los sellos terminales OK)
```

## Directriz de Control Tekton

| Gate | Condición |
|------|-----------|
| **Apertura feature** | Inputs `_init-feature-fase5.json`; gate Fase 4 `validacion.md` APTO — **aprobada** con refinamiento T5.6 |
| **Fail-soft obligatorio** | Parseo recibo nunca eleva `exit_code` del proceso invocador |
| **T5.6 Inmunidad Fan-Out** | Prohibido `os.remove()`/`unlink()` en consumidores telemetría; solo `delivery_state`; purga infraestructura |
| **No gobernanza §5.D** | Prohibido cablear Cerbero/Radamanto a `Telemetry_Compliance_Breached` en Tekton F5 |

## No objetivos (esta feature)

- Degradación reputación, bloqueo tras N infracciones, auto-reparación Tekton ante breach (PBI §5.D).
- Actualización `README.md` raíz (Fase 6).
- Obligar recibos en todas las ED existentes (solo declaración opt-in).
- Integrar tokens reales de proveedores LLM externos (solo contrato + smoke simulado).
- Mover el PBI maestro a `docs/todos/done/`.
- Modificar umbrales Self-Healing Radamanto por tokens.

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro § Fase 5; gate: Fase 4 `validacion.md` APTO (AC4.1–AC4.6)

## Artefactos previstos

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | ✅ Este documento |
| `clarify.md` | ✅ |
| `spec.md` | ✅ |
| `plan.md` | ✅ |
| `implementation.md` / `execution.md` | ⏳ Tekton |
| `validacion.md` | ⏳ Argos |

## Estado del proceso feature

| Fase proceso | Estado |
|--------------|--------|
| Inicialización (`workspace-init` / rama) | ✅ `feat/telemetria-reactiva-eda-fase5` |
| Estabilización (Mayeuta) | ✅ `objectives.md` + `clarify.md` |
| Diseño (Dedalo) | ✅ `spec.md` + `plan.md` (refinado T5.6) |
| Ejecución (Tekton) | ✅ `implementation.md` + código |
| Verificación (Argos) | ✅ `validacion.md` APTO |
| Cierre entrega (PR) | Pendiente `delivery-close-cycle` |
