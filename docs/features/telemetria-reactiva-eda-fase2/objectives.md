---
feature_name: telemetria-reactiva-eda-fase2
created: "2026-05-27"
process: feature
branch_name: feat/telemetria-reactiva-eda-fase2
persist_ref: docs/features/telemetria-reactiva-eda-fase2
master_pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
phase: 2
pbi_archived_at_close: false
status: validacion_apto
depends_on:
  - docs/features/telemetria-reactiva-eda-fase1
gate_ref: docs/features/telemetria-reactiva-eda-fase1/validacion.md
---

# Objetivos — Telemetría Reactiva EDA · Fase 2 (Workspaces dinámicos)

## Misión

Ejecutar la **Fase 2** del PBI maestro `PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO` como **feature independiente**: abandonar rutas rígidas ligadas a desarrollo de software (`paths.featurePath`, `paths.fixPath`) por **Espacios de Trabajo Aislados (Workspaces)** dinámicos e impermanentes. Cualquier proceso (ingeniería, legal, documental) debe instanciar su territorio operativo sin romper la **Ceguera Espacial** de las Entidades de Dominio.

El PBI unificado permanece en `docs/todos/pending/` como plan de ruta. Esta feature **no** archiva el PBI maestro al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature | Estado |
|----------|---------|--------|
| 0 | `telemetria-reactiva-eda-fase0` | Gate cumplido — `impact-analysis.md` AC0.x |
| 1 | `telemetria-reactiva-eda-fase1` | Cerrada — genoma fractal + `event_family` AC1.x |
| **2** | **`telemetria-reactiva-eda-fase2` (esta)** | Workspaces dinámicos + SSOT `workspacesRoot` |
| 3–6 | features independientes | Según PBI § Fases 3–6 |

## Contexto heredado (Fases 0–1)

| Decisión / hallazgo | Implicación Fase 2 |
|---------------------|-------------------|
| **D0.3** `paths.workspacesRoot` en SSOT | Sustituye dependencia efectiva de `featurePath`/`fixPath` no declarados (H16) |
| **H13** `execute_process_capsules.py` | Literales `docs/features\|fixes`; sin instanciación de workspace |
| **H14** `infer_persist_ref_from_branch` | Inferencia acoplada a slug feat/fix |
| **H15** Normas citan `paths.featurePath` | Actualizar narrativa hacia workspaces + convivencia documental |
| **H17** Ningún proceso declara `workspace_template` | Objetivo directo §2.A |
| **Axioma Ceguera espacial** | ED operan solo con coordenadas inyectadas en payload |

## Objetivos medibles (Fase 2)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F2-O1** | **Contrato `workspace_template`** | `process-contract.md` exige plantilla; procesos forja (`feature`, `bug-fix`, `refactorization` mínimo) la declaran | AC2.1 (precondición) |
| **F2-O2** | **Instanciación CLI** | Orquestador parsea plantilla, genera `execution_id` único y materializa carpeta antes de la primera Acción | AC2.2 |
| **F2-O3** | **Inyección de contexto** | Payload de evento táctico incluye coordenada absoluta del Workspace; instrucciones a agentes limitadas a esa frontera | AC2.3 |
| **F2-O4** | **SSOT `workspacesRoot`** | `cumulo.paths.json` declara `paths.workspacesRoot: ".SddIA/workspaces/"`; claves legacy deprecadas con alias de transición | PBI §2.D |
| **F2-O5** | **Migración scripts QA** | `execute_process_capsules`, `eda_bus_utils`, `route_domain_event_core` resuelven vía Cúmulo + workspace, no literales feat/fix | PBI §2.D |
| **F2-O6** | **Proceso no-SW ejecutable** | Smoke de proceso genérico (p. ej. lab o proceso mínimo) sin errores de ruta | AC2.1 |

## Modelo espacial (SSOT)

```text
Ruta absoluta = resolve(paths.workspacesRoot) + resolve(workspace_template)
```

- **Cúmulo** aporta la raíz base (`paths.workspacesRoot`).
- **Proceso** aporta la parte relativa (`workspace_template` con placeholders `{process_name}`, `{execution_id}`).
- **Convivencia:** features/fixes en curso pueden seguir usando `directories.documentation` + `persist_ref` hasta migración completa (PBI §2.D).

## No objetivos (esta feature)

- Peaje Termodinámico ni emisión `Raw_Execution_Finished` en runtime (Fase 3).
- Split de suscripciones ni rutas `./.events/{telemetry,orchestration,domain}/` (Fase 3).
- Radamanto, Self-Healing, recibos termodinámicos (Fases 4–5).
- Actualización `README.md` raíz (Fase 6).
- Mover el PBI maestro a `docs/todos/done/`.
- Purga total inmediata de `docs/features/` — convivencia temporal explícita.

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro § Fase 2; gate Fase 1: `validacion.md` APTO; gate Fase 0: `impact-analysis.md` (H13–H17, D0.3)

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
| Inicialización (`workspace-init` / rama) | ✅ `feat/telemetria-reactiva-eda-fase2` |
| Estabilización (Mayeuta) | ✅ `objectives.md` + `clarify.md` |
| Diseño (Dedalo) | ✅ `spec.md` + `plan.md` |
| Ejecución (Tekton) | ✅ `implementation.md` + `execution.md` |
| Verificación (Argos) | ✅ `validacion.md` APTO |
| Cierre entrega (PR) | Pendiente `delivery-close-cycle` |
