---
feature_name: inyeccion-dependencias-capacidades
created: "2026-07-21"
process: feature
branch_name: feat/inyeccion-dependencias-capacidades
persist_ref: docs/features/inyeccion-dependencias-capacidades
document_id: PBI-042-INYECCION-DEPENDENCIAS-CAPACIDADES
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
execution_id: 9120e3da-6ba9-4a93-9735-34486383c7de
status: closed
verdict: ok
phase_gate: accept-pr-merged
---

# Objetivos — inyeccion-dependencias-capacidades

## Misión

Materializar el MVP de DI por capacidades semánticas: **Metadatos Activos** + **Códice de la Lengua** + **Aduana Temprana** síncrona en `execute-process`, alineado al SSOT `{name}.md` (sin `spec.json`).

## Estado

Ciclo feature **cerrado** en `main` (merge `d887d7b`, PR #126). PBI-042 permanece en pending (R1–R8).

## Vectores soberanos (Racso)

1. Metadatos Activos — `provides` / `requires_capability` en genoma `.md`
2. Códice de la Lengua — Taxonomía Universal (norma `capability-taxonomy`)
3. Aduana Temprana — validación contractual pre-ignición en `execute-process`
4. MVP OK (path síncrono)

## Criterios de aceptación — producto MVP

| ID | Criterio |
|----|----------|
| **AC-P1** | Inject/homologación OK → ignición permitida |
| **AC-P2** | Incumplimiento contrato → abort + DLQ |
| **AC-P3** | Capacidad no indexada → abort limpio |
| **AC-M1** | Contratos ED documentan metadatos DI |
| **AC-M2** | Taxonomía forjada con `doc:closure` |
| **AC-M3** | Tests del gate en `execute-process` |

## Fuera de alcance (MVP) — residual en PBI pending

GesFer; DI vía Library_Codex; Cerbero como aduana DI; EDA-only §2.6; migración masiva de EDs; resolución ciega sin `delegates_to`.

Detalle R1–R8: `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` §3.

## Ley aplicada

- `features-documentation-pattern` · `external-ai-constraints` · `entidades-dominio-ecosistema-sddia.md`
- `cumulo.paths.json` · `CONSTITUTION_CORE.md` Filtro A
- SSOT diseño: `spec.md` / `plan.md`
