---
feature_name: inyeccion-dependencias-resolucion-ciega
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-resolucion-ciega
persist_ref: docs/features/inyeccion-dependencias-resolucion-ciega
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-RESOLUCION-CIEGA-INYECCION
execution_id: a7e3c9f2-4b1d-4e8a-9c5f-2d6b8e1a0f47
phase: accept-pr-merged
status: closed
verdict: ok
execution_id_runtime: 2161b482-7bc6-4cda-a8c7-a70cda8c05b8
merge_commit: 60c4635b351ee78c4f5d1050cc09e4bda3f8c6af
---

# Objetivos — inyeccion-dependencias-resolucion-ciega

## Misión

Materializar el **Hito 2** del residual PBI-042 (post-MVP PR #126): **resolución ciega** de `requires_capability` → artefacto físico e **inyección** del binding resuelto en el `stdin` JSON de la cápsula, sin acoplar por identidad `delegates_to`.

## Estado de partida (innegociable)

| Vector MVP | Estado |
|------------|--------|
| Metadatos Activos `provides` / `requires_capability` | Entregado |
| Códice `capability-taxonomy` (`doc:closure`) | Entregado |
| Aduana Temprana `capability_di_gate` | Entregado — **valida; no sustituye binding** |
| Binding por `delegates_to` | Aún ancla el artefacto físico |

## Vectores soberanos (este ciclo)

1. **R1 — Injector ciego:** resolver `requires_capability` → proveedor homologado (`action`\|`skill` con `provides` compatible) **sin** depender de `delegates_to` por identidad.
2. **R2 — Inject stdin:** wrapper/orquestación CLI que empaqueta en el JSON de `stdin` de la cápsula las rutas/contratos/refs resueltos (ceguera espacial; `capsule-json-io`).
3. **R3 — Binding table:** mapa SSOT capability→artefacto (Cúmulo o entidad dedicada). **Prohibido** sobrecargar Library_Codex de normas (**L-CODEX-ROLE**).
4. **R4 — Piloto ampliado:** anotar `provides`/`requires_capability` en un conjunto **acotado** de EDs adicionales (no migración masiva del catálogo).

## Criterios de aceptación — producto Hito 2

| ID | Criterio |
|----|----------|
| **AC-R1** | Fase/proceso piloto declara solo `requires_capability`; runtime elige proveedor homologado sin `delegates_to` hardcodeado |
| **AC-R2** | Cápsula recibe en `stdin` el binding resuelto (paths/contrato) de forma ciega |
| **AC-R3** | Existe mapa SSOT capability→artefacto distinto de Library_Codex normas |
| **AC-R4** | Piloto ampliado documentado: EDs adicionales anotadas más allá del par MVP `feature`↔`filesystem-manager` |
| **AC-REG** | Regresión MVP: AC-P1, AC-P2, AC-P3 permanecen APTO |

## Restricciones y leyes aplicadas

- Path de composición: **síncrono** en `execute-process` (EDA-only = Hito 3 / R6).
- Gate `capability_di_gate` se **conserva**; el injector alimenta la resolución previa/coherente a la aduana (**L-GATE-PRESERVE**).
- Capacidad ancla: `doc:closure` (taxonomía vigente). Altas nuevas al códice = R7 (fuera), salvo necesidad mínima demostrada por Dedalo vía `entity-manager`.
- Git solo vía `skill:git-manager`. Mutación genoma vía procesos canónicos (`entity-manager` / DA-4).
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.
- SSOT espacial: `cumulo.paths.json` (p. ej. `capability_contracts` ya existe; binding DI pendiente de locus Q1).

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| R5 Cerbero DI / revalidación schema en payload | Hito 3 |
| R6 composición EDA-only §2.6 | Hito 3 |
| R7 expansión códice multi-término | Hito 3 |
| R8 schema runtime de payload de **salida** | Hito 3 |
| GesFer / Paciente 0 | Otro PBI |
| Fractura Core F1 | Otro `persist_ref` |
| Migración masiva de todo el catálogo ED | Post-Hito 2 (R4 rebanado a piloto) |

## Ambigüedades para Dedalo (cerrar en spec/plan)

Ver `clarify.md` §D3: **Q1** locus binding table · **Q2** N proveedores · **Q3** orden injector↔gate · **Q4** forma payload stdin · **Q5** lista piloto R4.

## Handoff

Diseño Dedalo cerrado: `spec.md` + `plan.md` (Q1–Q5 laudeados). Siguiente: Tekton → Ejecución según blueprint.
