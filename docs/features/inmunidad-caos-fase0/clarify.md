---
feature_name: inmunidad-caos-fase0
created: "2026-05-28"
purpose: Cierre de decisiones Fase 0 e incorporación al PBI v2.1.0
---

# Clarificación — Fase 0 (Inmunidad / Caos)

## Decisiones cerradas (incorporadas al PBI maestro v2.1.0)

| ID | Pregunta | Resolución |
|----|----------|------------|
| **D0.1** | ¿Contexto RBAC para tools ofensivas? | Nuevo contexto **`chaos-engineering`** en `execution-contexts.md` (Fase 1.A). Tekton y procesos audit lo incluyen en `allowed_policies`; tools caos declaran `context: chaos-engineering` |
| **D0.2** | ¿`Suite` como 9.ª clase en `entity-manager`? | **Sí** — familia `suite` con `suite-creator`, `directories.suites`, `suites-contract.md`, extensión `sync-entity-index` y enum `entity-manager` (Fase 3) |
| **D0.3** | ¿Límite de Inocuidad del Caos? | Escritura/lectura ofensiva acotada al **`workspace_path` inyectado** (no solo raíz repo). Cápsulas Python de tools caos + helper `assert_workspace_bound` en lab (Fase 1.C) |
| **D0.4** | ¿DLT para `System_Immunity_Certified`? | **Extensión Radamanto** (no Cúmulo): cuarto tipo de sello gobernanza junto a `Tool_Degraded`/`Status_Restored`/`Tool_Deprecated`; acta CI dual en Fase 4 |
| **D0.5** | ¿`telemetry_provided` en tools? | Ampliar `tools-contract` v1.3.0 §6 (paridad con skills/actions) **antes** de forjar `schema-corruptor` (Fase 1.B) |
| **D0.6** | ¿Aislamiento en `execute-suite`? | Cada `atomic_node` → subproceso `execute-process` con **`execution_id` nuevo** + sub-`workspace_path` derivado del template del proceso hijo (Fase 3.C) |
| **D0.7** | ¿Dónde vive `survival-manifest.md`? | Bajo workspace del orquestador: `{workspace_path}/survival-manifest.md`; Argos compila tras nodos; no en genoma Core (Fase 3.D) |
| **D0.8** | ¿Reparto Fases 4–5 del PBI? | **Fase 4** = ECST + suscripciones + emisión certificación; **Fase 5** = README y documentación transversal (corregir numeración original) |
| **D0.9** | ¿Archivar PBI maestro en Fase 0? | **No** — permanece `pending/`; esta feature cierra con `pbi_archived: false` |

## Ejecutabilidad Fases 1–5 (AC0.5)

Tras refinamiento inline (§ 1.A–1.D, 2.A–2.C, 3.A–3.E, 4.A–4.C, 5.A), las fases siguientes son ejecutables sin ambigüedad bloqueante pendiente de Vértice Biológico.

## Escalados no bloqueantes

| Tema | Estado | Fase |
|------|--------|------|
| Cerbero RBAC real en lab (hoy stub solo PR review) | Backlog Kaizen — handler `cerbero-gate` genérico | Post-Fase 2 |
| `policy-validator` enum cerrado vs. 8 contextos SSOT | Deuda preexistente; no bloquea caos si tools usan contextos ya listados en agent-creator | Kaizen |

## Referencia

- Inventario: `impact-analysis.md` (H01–H28)
- PBI: `docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md` v2.1.0
