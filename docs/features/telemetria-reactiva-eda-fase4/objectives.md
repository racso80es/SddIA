---
feature_name: telemetria-reactiva-eda-fase4
created: "2026-05-27"
process: feature
branch_name: feat/telemetria-reactiva-eda-fase4
persist_ref: docs/features/telemetria-reactiva-eda-fase4
master_pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
phase: 4
pbi_archived_at_close: false
status: planificacion
depends_on:
  - docs/features/telemetria-reactiva-eda-fase1
  - docs/features/telemetria-reactiva-eda-fase2
  - docs/features/telemetria-reactiva-eda-fase3
gate_ref: docs/features/telemetria-reactiva-eda-fase3/validacion.md
---

# Objetivos — Telemetría Reactiva EDA · Fase 4 (Radamanto + Self-Healing)

## Misión

Ejecutar la **Fase 4** del PBI maestro `PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO` como **feature independiente**: materializar el agente **Radamanto** (Certificador/Actuario), sustituir el stub `telemetry-batch-stub` por un consumidor batch real de telemetría CLI, aplicar **umbrales deterministas** sobre el acumulado estadístico de herramientas/skills, sellar cambios de estatus en IOTA Rebased con **exclusividad DLT** (handoff D0.1), y cerrar el **bucle Self-Healing** (degradación → Cerbero → sandbox → redención / muerte definitiva).

El PBI unificado permanece en `docs/todos/pending/` como plan de ruta. Esta feature **no** archiva el PBI maestro al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature | Estado |
|----------|---------|--------|
| 0 | `telemetria-reactiva-eda-fase0` | Cerrada — `impact-analysis.md` AC0.x |
| 1 | `telemetria-reactiva-eda-fase1` | Cerrada — genoma fractal + `Raw_Execution_Finished` AC1.x |
| 2 | `telemetria-reactiva-eda-fase2` | Cerrada — workspaces dinámicos AC2.x |
| 3 | `telemetria-reactiva-eda-fase3` | Cerrada — Peaje + bus fractal + stub Radamanto AC3.x |
| **4** | **`telemetria-reactiva-eda-fase4` (esta)** | Radamanto + Self-Healing + eventos dominio |
| 5–6 | features independientes | Según PBI § Fases 5–6 |

## Contexto heredado (Fases 0–3)

| Decisión / hallazgo | Implicación Fase 4 |
|---------------------|-------------------|
| **D0.1** Handoff DLT Cúmulo → Radamanto | Radamanto sella `Tool_*` / `Status_Restored`; Cúmulo mantiene `PullRequest_*` / `Domain_Entity_*` hasta acta CI |
| **H10–H12** DLT hoy solo Cúmulo; sin `radamanto.md` | Objetivos directos §4.0 y §4.A |
| **D3.7** Stub `telemetry-batch-stub` | Sustituir por proceso `radamanto-batch` + contrato agente |
| **Peaje Termodinámico** (F3) | Radamanto **solo consume** `Raw_Execution_Finished`; prohibida medición directa (AC4.2) |
| **Bus fractal** `./.events/telemetry/` | Acumulador batch + purga post-consumo |
| **Argos vs Radamanto** (PBI §3.E) | Argos = materia/código; Radamanto = actuario/confianza macroscópica |

## Objetivos medibles (Fase 4)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F4-O1** | **Contrato Radamanto** | `radamanto.md` + `radamanto.instructions.json` con exclusividad DLT y prohibición de medición directa | AC4.1, AC4.2 |
| **F4-O2** | **Handoff DLT documentado** | Acta D0.1 en spec; Cúmulo conserva anclaje PR/ECST; smoke dual CI | PBI §4.0 |
| **F4-O3** | **Umbrales deterministas** | Reglas configurables (ej. `< 85%` éxito → `Tool_Degraded`; latencia media) en SSOT o config Radamanto | AC4.3 |
| **F4-O4** | **Consumidor batch telemetría** | Sustituir stub; acumular métricas por entidad; emitir dominio + DLT por lote o caída abrupta | AC4.2 |
| **F4-O5** | **Eventos dominio Self-Healing** | Forjar `Tool_Degraded`, `Status_Restored`, `Tool_Deprecated` en `SddIA/events/domain/` | PBI §4.E |
| **F4-O6** | **Suscripciones EDA** | Cerbero + `fix-tool-process` reaccionan a eventos de estatus; telemetría → Radamanto | AC4.4 |
| **F4-O7** | **Sandbox estricto reparación** | Dédalo/Tekton sin escritura en `SddIA/tools/`, `SddIA/skills/` durante `fix-tool-process` | AC4.5 |
| **F4-O8** | **Límite de redención** | `max_recovery_attempts` configurable; lógica `Tool_Deprecated` operativa | AC4.6 |

## Modelo de jurisdicción (Panteón)

```text
CLI (Peaje)          →  ./.events/telemetry/  →  Radamanto (batch, solo lectura stats)
Radamanto (DLT)      →  iota-immutable-publisher  (Tool_* / Status_Restored / Tool_Deprecated)
Radamanto (dominio)  →  ./.events/domain/     →  Cerbero + fix-tool-process
Cúmulo (DLT legacy)  →  PullRequest_* / Domain_Entity_*  (sin cambio hasta acta)
Argos                →  pull-request-review + validación post-reparación (materia)
Cerbero              →  RBAC revocación/rehabilitación ante eventos dominio
```

## No objetivos (esta feature)

- Recibos `telemetry_receipt` ni `Telemetry_Compliance_Breached` (Fase 5).
- Actualización `README.md` raíz (Fase 6).
- Retirada total de `telemetry-batch-stub` del historial (puede quedar deprecated).
- Big-bang migración de todos los sellados DLT de Cúmulo a Radamanto (solo gobernanza herramientas).
- Mover el PBI maestro a `docs/todos/done/`.

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro § Fase 4; gate: Fase 3 `validacion.md` APTO (AC3.1–AC3.4, D3.13)

## Artefactos previstos

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | ✅ Este documento |
| `clarify.md` | ✅ |
| `spec.md` | ✅ |
| `plan.md` | ✅ |
| `implementation.md` / `execution.md` | Pendiente (Tekton) |
| `validacion.md` | Pendiente (Argos) |

## Estado del proceso feature

| Fase proceso | Estado |
|--------------|--------|
| Inicialización (`workspace-init` / rama) | ✅ `feat/telemetria-reactiva-eda-fase4` |
| Estabilización (Mayeuta) | ✅ `objectives.md` + `clarify.md` |
| Diseño (Dedalo) | ✅ `spec.md` + `plan.md` |
| Ejecución (Tekton) | ⏸ Detenido — pendiente aprobación plan |
| Verificación (Argos) | Pendiente |
| Cierre entrega (PR) | Pendiente |
