---
feature_name: inmunidad-caos-fase0
created: "2026-05-28"
process: feature
branch_name: feat/inmunidad-caos-fase0
persist_ref: docs/features/inmunidad-caos-fase0
master_pbi_ref: docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md
master_pbi_id: PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO
phase: 0
pbi_archived_at_close: false
status: cerrada
---

# Objetivos — Inmunidad, Caos S+ Grade · Fase 0 (análisis de implicaciones)

## Misión

Ejecutar la **Fase 0** del PBI maestro `PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO` como **feature independiente**: recorrer `SddIA/` y acoplamientos (genoma, runtime EDA, sandbox, entity-manager, tools, procesos, Radamanto, Cerbero, Argos) para detectar **impactos, deudas y puntos ciegos no contemplados** en las Fases 1–5 **antes** de forjar tools ofensivas, procesos de auditoría atómicos, la ED `Suite` ni eventos de certificación DLT.

El PBI maestro permanece en `docs/todos/pending/` como **plan de ruta y control** hasta que todas las fases cumplan sus criterios globales. Esta feature **no** archiva el PBI al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature prevista (convención) | Entregable clave |
|----------|-------------------------------|------------------|
| **0** | `inmunidad-caos-fase0` (esta) | `impact-analysis.md` |
| 1 | `inmunidad-caos-fase1` (futura) | Tools ofensivas (`io-choke`, `schema-corruptor`, `sandbox-breacher`) |
| 2 | `inmunidad-caos-fase2` (futura) | Procesos audit atómicos (3 vectores) |
| 3 | `inmunidad-caos-fase3` (futura) | ED `Suite`, `entity-manager`, `execute-suite`, `core-full-stress.md` |
| 4 | `inmunidad-caos-fase4` (futura) | Estímulo EDA + gobernanza autónoma |
| 5 | `inmunidad-caos-fase5` (futura) | README y documentación transversal |

## Contexto operativo

| Hecho | Implicación para el análisis |
|-------|------------------------------|
| Programa Telemetría Reactiva **Done** (Fases 0–6) | Peaje Termodinámico, bus fractal, `telemetry-compliance-audit`, workspaces dinámicos y Radamanto ya existen — el caos debe **reutilizar** infraestructura, no duplicarla |
| `entity-manager` reconoce 8 clases; **no** incluye `suite` | Fase 3 requerirá extensión genómica; analizar patrón `*-creator` y contratos |
| Solo 5 tools en catálogo Core | Fase 1 introduce tools **ofensivas** con contexto RBAC nuevo; evaluar `tools-contract`, `scope` y Candado Semántico Cúmulo |
| `filesystem-manager` confina al workspace del proyecto | Axioma **Inocuidad del Caos** exige límites explícitos sobre `workspace_path` inyectado vs. raíz del repo |
| Fase 5 PBI prevé `System_Immunity_Certified` + Radamanto DLT | Jurisdicción DLT y coexistencia con sellos existentes deben explicitarse en matriz de gaps |
| **Atomicidad Diagnóstica** (1 vector por proceso) | Orquestador `execute-suite` (Fase 3) debe diseñarse sin violar SRP de procesos Fase 2 |

## Objetivos medibles (Fase 0)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F0-O1** | **Inventario de acoplamientos** | Barrido sobre genoma ED, `entity-manager`, tools/skills, sandbox, `workspace_path`, Cerbero/RBAC, bus `.events/domain/`, Radamanto, `telemetry-compliance-audit`; tabla ubicación × fase × severidad |
| **F0-O2** | **Matriz de gaps** | Hallazgos clasificados: cubierto / ampliar tarea / nueva subtarea / fuera de alcance |
| **F0-O3** | **Decisiones bloqueantes** | Ítems bloqueantes con decisión o subtarea asignada a Fase 1–5 antes de abrir Fase 1 |
| **F0-O4** | **Entregable `impact-analysis.md`** | Resumen ejecutivo + hallazgos + decisiones + backlog residual en `persist_ref` |
| **F0-O5** | **Refinamiento PBI maestro** | Subtareas aprobadas incorporadas al PBI pending (commit en rama de esta feature) |
| **F0-O6** | **Validación ejecutabilidad** | Clarificación: Fases 1–5 refinadas sin ambigüedad bloqueante (AC0.5) |

## Ámbito de exploración (checklist PBI § Fase 0)

- **Nueva familia ED `Suite`:** ausencia en `cumulo.paths.json`, `entity-manager`, contratos, índices, `Domain_Entity_*`
- **Tools ofensivas:** contexto RBAC, `scope` core/local, cápsulas en `scripts/tools/`, integración Peaje Termodinámico
- **Sandbox e inocuidad:** `workspace_path`, `filesystem-manager`, path traversal, `.SddIA/sandbox/` vs. workspaces dinámicos
- **Procesos audit atómicos:** patrón delegación Tekton → tool → Argos; `survival-manifest.md` (inexistente hoy)
- **Orquestador `execute-suite`:** sub-workspaces paralelos/secuenciales, `execute-process` anidado, timeouts, `fail_fast`/`run_all`
- **Eventos ECST:** `Suite_Execution_Requested`, `System_Immunity_Certified`; suscripciones y emisores autorizados
- **Radamanto / DLT:** anclaje IOTA Rebased vs. sellos PR existentes; reacción a certificación de inmunidad
- **Integración caos ↔ telemetría:** `schema-corruptor` vs. `Telemetry_Compliance_Breached`; fail-soft Peaje vs. `io-choke`
- **Cerbero:** bloqueo de escalada (`sandbox-breacher`); políticas agentes auditores vs. tools ofensivas
- **Laboratorio QA:** handlers, flags `SDDIA_LAB_*`, tests de regresión para suites de caos

## No objetivos (esta feature)

- Forjar tools ofensivas, procesos audit, ED `Suite`, orquestador ni eventos ECST (Fases 1–4).
- Actualizar `README.md` raíz (Fase 5).
- Mover el PBI maestro a `docs/todos/done/` ni declarar Done global del programa.
- Cambios de código productivo salvo documentación de análisis y refinamiento del PBI maestro.

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro: `docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md` § Fase 0
- Axiomas PBI: Inocuidad del Caos, Identidad Ontológica (`Suite` como ED), Atomicidad Diagnóstica

## Artefactos previstos

| Artefacto | Propósito |
|-----------|-----------|
| `clarify.md` | Decisiones y escalados (0.C) |
| `spec.md` | Metodología de barrido y plantillas de hallazgo |
| `plan.md` | Secuencia 0.A → 0.D |
| `impact-analysis.md` | Entregable principal (AC0.1–AC0.4) |
| `implementation.md` / `execution.md` | Trazabilidad del barrido |
| `validacion.md` | Argos; `pbi_archived: false` |

## Estado

| Fase feature | Estado |
|--------------|--------|
| Objetivos | ✅ Este documento |
| Inicialización Git (`workspace-init`) | ✅ 2026-05-28 |
| Clarificación | ✅ `clarify.md` |
| PBI refinado v2.1.0 | ✅ |
| Especificación (Dedalo) | ✅ `spec.md` |
| Planificación | ✅ `plan.md` |
| Análisis / `impact-analysis.md` | ✅ 2026-05-28 |
| Validación Argos | ✅ `validacion.md` APTO |
