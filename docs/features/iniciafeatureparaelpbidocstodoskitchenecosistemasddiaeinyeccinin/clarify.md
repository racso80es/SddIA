---
feature_name: iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
created: "2026-07-20"
process: feature
purpose: Estabilización Mayeuta — Plan Maestro Ecosistema SddIA / Paciente 0 GesFer (kitchen → feature lab)
correlation_id: 4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51
branch_name: feat/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
persist_ref: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
pbi_ref: docs/todos/kitchen/Ecosistema SddIA e Inyección Industrial en Paciente 0 (GesFer).md
document_id: PBI-ECOSISTEMA-GESFER-PACIENTE-0
---

# Clarificación — Ecosistema SddIA / Paciente 0 (GesFer)

Transcript Mayeuta (2026-07-20). Semilla: `inicia feature para el pbi docs/todos/kitchen/Ecosistema SddIA e Inyección Industrial en Paciente 0 (GesFer).md`.  
`correlation_id`: `4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51`.

## D0 — Apertura

| Campo | Valor |
|-------|--------|
| Proceso | `feature` v1.3.0 · fase Estabilización |
| `feature_name` (ciclo lab) | `iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin` |
| Alias canónico propuesto | `fractura-core-paciente-0-gesfer` |
| Rama ciclo | `feat/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin` |
| `persist_ref` | `docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin` |
| `persist_ref` en payload runtime | **vacío** → resuelto por stub init + `paths.featurePath` + `branch_name` |
| `pbi_ref` | `docs/todos/kitchen/Ecosistema SddIA e Inyección Industrial en Paciente 0 (GesFer).md` |
| `document_id` (asignado en estabilización) | `PBI-ECOSISTEMA-GESFER-PACIENTE-0` |
| UUID PBI físico | **ausente** (entropía documental kitchen) |

## D1 — Semilla vs realidad documental

| Afirmación de entrada | Hecho verificado | Laudo |
|----------------------|------------------|-------|
| Intención = iniciar feature sobre plan maestro GesFer | PBI existe en `docs/todos/kitchen/` | Fuente válida de `raw_user_intent` |
| PBI listo para forja atómica | Sin frontmatter `{id,uuid,type,version}`; título libre | **Entropía** — no estándar atómico `{name}.md` |
| Alcance = “el PBI” entero | 4 fases encadenadas (Core → inyección → minteo → runtime) | **No termodinámico** como un solo `feature` |
| Dependencias previas | Matriz PBI: F1 → F2 → F3 → F4 | Corte obligatorio en F1 para este ciclo |
| Trabajos colaterales | Kitchen `[REFACTOR] Separación de Dominio…`; feature `monorepo-local-constitutions-setup` (labs GesFer) | Soft-deps; no fusionar sin orden Racso |
| `capsule-json-io` (hito 1.2) | Norma `SddIA/norms/capsule-json-io.md` ya existe (schema 2.0) | 1.2 = **sellar/cumplir**, no inventar contrato desde cero |
| Paquete `@sddia/core` (hito 1.1) | No hay `package.json` `@sddia/*` en repo | 1.1 = **hueco real** de producto |

## D2 — Diálogo interno (preguntas estabilizadas)

| ID | Pregunta | Decisión Mayeuta (sin diseño ejecutable) |
|----|----------|------------------------------------------|
| Q1 | ¿Todo el plan maestro en este PR? | **No.** Solo **Fase 1 — Fractura Core**. Fases 2–4 = features/PBI hijos posteriores. |
| Q2 | ¿Qué cubre Fase 1 en este ciclo? | **1.1** (jurisdicción Core empaquetable/ciega) + **1.2** (tubería hermética `capsule-json-io` como ley innegociable) + **1.3** solo como **esqueletos vacíos** Forge/Portal (Shared Kernel), sin producto UI funcional. |
| Q3 | ¿Promover kitchen → pending antes de Dedalo? | **Recomendado** (añadir UUID + frontmatter atómico). **No bloquea** estabilización: precedente `centinela-soberania-ejecucion` / kitchen. Operador puede promover en paralelo. |
| Q4 | ¿Relación con PBI REFACTOR domain abstraction? | **Paralelo / soft-dep.** Mismo vector “Core agnóstico”; este feature no absorbe ese PBI. Dedalo no los fusiona. |
| Q5 | ¿Renombrar `persist_ref` al alias canónico? | **Diferido.** Este ciclo conserva slug lab; alias humano en frontmatter (`canonical_feature_name`). |

## D3 — Fuera de jurisdicción de este `feature`

| Ítem | Destino |
|------|---------|
| Fase 2 — `.SddIA/` + watchers en 4 repos GesFer | Feature hija post-F1 |
| Fase 3 — códices dominio + minteo IOTA NFT | Feature hija; depende Forge usable + `iota-immutable-publisher` |
| Fase 4 — runtime invisible / peaje ledger | Feature hija post-F2/F3 |
| Mutar genoma Core sin proceso/entity-manager | Prohibido (blindaje IA obrera) |
| Diseñar AST Forge, UX Portal, agentes C# GesFer | Fuera de Mayeuta; fuera de alcance F1.3-esqueleto |

## D4 — Decisiones vinculantes

| ID | Decisión |
|----|----------|
| L-PERSIST | Materializar `clarify.md` + `objectives.md` bajo `docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin` |
| L-F1-ONLY | Alcance termodinámico = **Fase 1** del PBI kitchen; F2–F4 excluidas |
| L-ATOMIC | Criterios medibles de aceptación = hitos **1.1**, **1.2**, **1.3-esqueleto** (ver `objectives.md`) |
| L-AGNOSTIC | Core sin nomenclatura/negocio GesFer; Paciente 0 es consumidor, no genoma |
| L-GIT | Git solo vía `skill:git-manager` |
| L-HANDOFF | Cuerpo de `objectives.md` = `refined_requirements` para Dedalo |
| L-DOCID | `document_id: PBI-ECOSISTEMA-GESFER-PACIENTE-0` hasta que el PBI kitchen reciba UUID atómico |

## D5 — Preguntas abiertas (no bloquean estabilización; sí acotan Dedalo)

| ID | Pregunta | Efecto si Racso contradice |
|----|----------|----------------------------|
| O1 | ¿Empaquetado 1.1 vía cargo workspace, npm, o ambos? | Dedalo elige mecanismo técnico; Mayeuta solo exige **paquete Core distribuible + ciego** |
| O2 | ¿Ubicación física de esqueletos Forge/Portal (mono-repo vs repos nuevos)? | Dedalo propone; no asume path fuera de `cumulo_topology` sin laudo Racso |
| O3 | ¿Promoción kitchen→pending + UUID en este mismo PR? | Operativo documental; no cambia el qué de F1 |

## D6 — Veredicto fase

**ok** — requisito estabilizado para handoff a Dedalo con alcance F1. Sin inventar éxito de forja: código/producto aún no materializado.
