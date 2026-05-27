---
feature_name: telemetria-reactiva-eda-fase3
created: "2026-05-27"
process: feature
branch_name: feat/telemetria-reactiva-eda-fase3
persist_ref: docs/features/telemetria-reactiva-eda-fase3
master_pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
phase: 3
pbi_archived_at_close: false
status: planificacion
depends_on:
  - docs/features/telemetria-reactiva-eda-fase1
  - docs/features/telemetria-reactiva-eda-fase2
gate_ref: docs/features/telemetria-reactiva-eda-fase2/validacion.md
---

# Objetivos — Telemetría Reactiva EDA · Fase 3 (Aduana Universal + runtime fractal)

## Misión

Ejecutar la **Fase 3** del PBI maestro `PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO` como **feature independiente**: materializar el **Peaje Termodinámico** en el CLI, fragmentar el bus runtime en tres rutas especializadas (`./.events/{telemetry,orchestration,domain}/`), colapsar `event-subscriptions.json` en tres configuraciones homólogas con sus procesos enrutadores dedicados, y cablear la suscripción de telemetría hacia **Radamanto** (stub hasta Fase 4).

El PBI unificado permanece en `docs/todos/pending/` como plan de ruta. Esta feature **no** archiva el PBI maestro al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature | Estado |
|----------|---------|--------|
| 0 | `telemetria-reactiva-eda-fase0` | Cerrada — `impact-analysis.md` AC0.x |
| 1 | `telemetria-reactiva-eda-fase1` | Cerrada — genoma fractal + `Raw_Execution_Finished` AC1.x |
| 2 | `telemetria-reactiva-eda-fase2` | Cerrada — workspaces dinámicos AC2.x (PR #53) |
| **3** | **`telemetria-reactiva-eda-fase3` (esta)** | Aduana Universal + bus runtime + enrutadores |
| 4–6 | features independientes | Según PBI § Fases 4–6 |

## Contexto heredado (Fases 0–2)

| Decisión / hallazgo | Implicación Fase 3 |
|---------------------|-------------------|
| **D0.2** Coexistencia V3+ + bus fractal | `eda_bus.pending` sigue activo para dominio legacy; nuevas familias usan rutas fractales |
| **D0.4** `event-watcher` multi-ruta | Evolucionar watcher sin apagar flujo `PullRequest_Presented` → `pull-request-review` |
| **D0.5** Peaje Termodinámico en cápsulas CLI | Cronómetro + emisión solo CLI; extensión `execute_process_capsules` |
| **D2.7** `workspace_path` en contexto CLI | Formalizar en envelope ECST de orquestación (Fase 3) |
| **H04–H08** Sin rutas fractales runtime ni enrutadores dedicados | Objetivos directos §3.B–3.C |
| **H13** CLI sin cronómetro ni telemetría | Objetivo directo §3.A |
| **H05–H06** Suscripción monolítica + watcher solo `pending/` | Split §3.C + migración watcher §3.C.1 |
| **Genoma** `Raw_Execution_Finished` en `telemetry/` | Emisor autorizado: CLI; destino `./.events/telemetry/` |

## Objetivos medibles (Fase 3)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F3-O1** | **Peaje Termodinámico** | CLI activa cronómetro, captura `exit_code`/`duration_ms`/`asset_id` y emite `Raw_Execution_Finished` en `./.events/telemetry/` | AC3.1 |
| **F3-O2** | **Topología runtime fractal** | Existen `./.events/telemetry/`, `orchestration/`, `domain/` declarados en SSOT y materializados idempotentemente | AC3.2 (infra) |
| **F3-O3** | **Split suscripciones** | Tres archivos `event-*-subscriptions.json` + tres procesos enrutadores operativos | AC3.2 |
| **F3-O4** | **Aislamiento de familias** | Telemetría, orquestación y dominio no comparten ruta de escritura ni suscripción cruzada | AC3.3 |
| **F3-O5** | **Suscripción Radamanto (stub)** | `event-telemetry-subscriptions.json` cablea consumidor telemetría; agente Radamanto no implementado (no-op documentado) | AC3.4 |
| **F3-O6** | **Coexistencia V3+** | `event-watcher` + `route-domain-event` siguen operativos sobre `pending/` para eventos legacy | PBI §3.C.1, D0.2 |
| **F3-O7** | **Orquestación post-éxito** | Tras `status: success`, CLI emite evento de familia `orchestration` con blueprint del proceso | PBI §3.A |
| **F3-O8** | **Envelope ECST con workspace** | Eventos tácticos incluyen `workspace_path` inyectado (herencia D2.7) | PBI §3.F |

## Modelo runtime (Simetría Fractal)

```text
Genoma (SSOT)                    Runtime (instancia)
SddIA/events/telemetry/    ↔    ./.events/telemetry/
SddIA/events/orchestration/ ↔   ./.events/orchestration/
SddIA/events/domain/        ↔    ./.events/domain/
eda_bus.pending/ (V3+)      ↔    coexistencia legacy (D0.2)
```

## No objetivos (esta feature)

- Implementación del agente **Radamanto** ni bucle Self-Healing (Fase 4).
- Recibos termodinámicos `telemetry_receipt` ni `Telemetry_Compliance_Breached` (Fase 5).
- Actualización `README.md` raíz (Fase 6).
- Big-bang retirada del pipeline V3+ (`pending/`/`processing/`/`processed/`).
- Mover el PBI maestro a `docs/todos/done/`.
- GC/purge automática de `./.events/telemetry/` (Radamanto batch → Fase 4).

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro § Fase 3; gates: Fase 1 `validacion.md` APTO, Fase 2 `validacion.md` APTO (H04–H13, D0.2, D0.4, D0.5, D2.7)

## Artefactos previstos

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | ✅ Este documento |
| `clarify.md` | ✅ |
| `spec.md` | ✅ |
| `plan.md` | ✅ |
| `implementation.md` / `execution.md` | Pendiente (Tekton) |
| `validacion.md` | Pendiente (Argos); `pbi_archived: false` |

## Estado del proceso feature

| Fase proceso | Estado |
|--------------|--------|
| Inicialización (`workspace-init` / rama) | ✅ `feat/telemetria-reactiva-eda-fase3` |
| Estabilización (Mayeuta) | ✅ `objectives.md` + `clarify.md` |
| Diseño (Dedalo) | ✅ `spec.md` + `plan.md` |
| Ejecución (Tekton) | ⏸ Detenido — pendiente de arranque |
| Verificación (Argos) | Pendiente |
| Cierre entrega (PR) | Pendiente `delivery-close-cycle` |
