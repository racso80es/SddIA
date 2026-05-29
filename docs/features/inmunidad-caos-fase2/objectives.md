---
feature_name: inmunidad-caos-fase2
created: "2026-05-29"
process: feature
branch_name: feat/inmunidad-caos-fase2
persist_ref: docs/features/inmunidad-caos-fase2
master_pbi_ref: docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md
master_pbi_id: PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO
phase: 2
pbi_archived_at_close: false
status: validacion_apto
depends_on:
  - docs/features/inmunidad-caos-fase1
gate_ref: docs/features/inmunidad-caos-fase1/validacion.md
---

# Objetivos — Inmunidad, Caos S+ Grade · Fase 2 (Nodos de Diagnóstico)

## Misión

Ejecutar la **Fase 2** del PBI maestro `PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO` como **feature independiente**: forjar tres **procesos de auditoría atómicos** (un vector de ataque = un proceso), cada uno con `workspace_template` propio, invocación Tekton → tool ofensiva Fase 1 → certificación Argos sobre la reacción del ecosistema.

El PBI maestro permanece en `docs/todos/pending/`. Esta feature **no** archiva el PBI al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature | Estado |
|----------|---------|--------|
| 0 | `inmunidad-caos-fase0` | ✅ Cerrada |
| 1 | `inmunidad-caos-fase1` | ✅ Cerrada — Arsenal Entropía |
| **2** | **`inmunidad-caos-fase2` (esta)** | Procesos audit atómicos |
| 3 | `inmunidad-caos-fase3` (futura) | ED `Suite` + `execute-suite` |
| 4–5 | features independientes | Según PBI |

## Contexto heredado (Fase 1)

| Activo / decisión | Implicación Fase 2 |
|-------------------|-------------------|
| Tools `io-choke`, `schema-corruptor`, `sandbox-breacher` | Vectores únicos por proceso audit |
| Contexto `chaos-engineering` (D0.1) | Procesos audit declaran contexto + políticas Tekton ampliadas (D1.6) |
| `assert_workspace_bound` (D0.3) | Proceso `audit-sandbox-isolation-rbac` certifica envelope de `sandbox-breacher` |
| `run_thermodynamic_toll` fail-soft D3.13 (H22) | Proceso `audit-thermodynamic-toll-failsoft` valida exit 0 pese a estrés E/S |
| Fan-out `telemetry-compliance-audit` (H23) | Proceso `audit-telemetry-compliance-breach` valida `Telemetry_Compliance_Breached` |
| Cerbero stub en lab (H25) | Certificación vía Argos + envelope tool; no gate Cerbero determinista global |

## Objetivos medibles (Fase 2)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F2-O1** | **`audit-thermodynamic-toll-failsoft`** | Tekton → `io-choke`; Argos certifica exit 0 del proceso pese a fallo E/S telemetría | AC2.1, AC2.2 |
| **F2-O2** | **`audit-telemetry-compliance-breach`** | Tekton → `schema-corruptor`; Argos verifica JSON `Telemetry_Compliance_Breached` en `./.events/domain/` | AC2.1, AC2.2 |
| **F2-O3** | **`audit-sandbox-isolation-rbac`** | Tekton → `sandbox-breacher`; Argos certifica bloqueo (envelope error / sin escritura fuera workspace) | AC2.1, AC2.2 |
| **F2-O4** | **Atomicidad Diagnóstica** | Cada proceso invoca **exactamente una** tool ofensiva | AC2.3 |
| **F2-O5** | **Handlers lab + smoke** | `execute_process_capsules` + tests `execute-process` por proceso | AC2.2 |
| **F2-O6** | **Índice procesos** | Tres filas en `SddIA/process/index.md` | AC2.1 |

## No objetivos (esta feature)

- ED `Suite`, `execute-suite`, `core-full-stress.md` (Fase 3).
- Eventos ECST `Suite_Execution_Requested` / `System_Immunity_Certified` (Fase 4).
- README raíz y cierre global PBI (Fase 5).
- Cerbero gate determinista en todo `execute-process` (Kaizen H25).
- Alinear `policy-validator` con los 8 contextos SSOT (Kaizen Fase 0).

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro § Fase 2; gate: `inmunidad-caos-fase1/validacion.md`
- Axioma **Atomicidad Diagnóstica** (1 vector = 1 proceso)

## Artefactos previstos

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | ✅ Este documento |
| `clarify.md` | ✅ |
| `spec.md` | ✅ |
| `plan.md` | ✅ |
| `implementation.md` / `execution.md` | ✅ |
| `validacion.md` | ✅ APTO; `pbi_archived: false` |

## Estado del proceso feature

| Fase proceso | Estado |
|--------------|--------|
| Inicialización (`workspace-init` / rama) | ⏳ `feat/inmunidad-caos-fase2` |
| Estabilización (Mayeuta) | ✅ `objectives.md` + `clarify.md` |
| Diseño (Dedalo) | ✅ `spec.md` + `plan.md` |
| Ejecución (Tekton) | ✅ `implementation.md` + `execution.md` |
| Verificación (Argos) | ✅ `validacion.md` APTO |
| Cierre entrega (PR) | ⏳ |
