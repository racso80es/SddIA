---
feature_name: iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
created: "2026-07-20"
process: feature
branch_name: feat/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
persist_ref: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
canonical_feature_name: fractura-core-paciente-0-gesfer
document_id: PBI-ECOSISTEMA-GESFER-PACIENTE-0
pbi_ref: docs/todos/kitchen/Ecosistema SddIA e Inyección Industrial en Paciente 0 (GesFer).md
pbi_status: kitchen
pbi_uuid: null
correlation_id: 4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51
depends_on: []
soft_depends_on:
  - docs/todos/kitchen/[REFACTOR] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md
  - docs/features/monorepo-local-constitutions-setup
status: stabilized
verdict: ok
scope_phase: "1"
---

# Objetivos — Fractura Core (Paciente 0 / GesFer) · Fase 1

## Misión (requisito termodinámico)

Romper el acoplamiento del Core SddIA respecto de cualquier dominio de cliente (GesFer incluido) y dejarlo como **jurisdicción empaquetable, ciega y hermética**, de modo que el Paciente 0 pueda consumirlo después (Fases 2–4, fuera de este ciclo).

Semilla cruda: iniciar feature sobre el plan maestro kitchen `Ecosistema SddIA e Inyección Industrial en Paciente 0 (GesFer).md`. Ese plan **no** cabe en un solo feature; el alcance estable es **solo Fase 1 — Fractura Core**.

## Por qué

Sin Core distribuible y sin tubería JSON obligatoria, la inyección en repos GesFer (Fase 2), el minteo (Fase 3) y el runtime invisible (Fase 4) contaminarían el genoma o violarían ceguera espacial. El propio PBI declara la dependencia F1 → F2 → F3 → F4.

## Alcance (qué)

| ID | Hito PBI | Qué se exige en este ciclo |
|----|----------|----------------------------|
| **F1-1** | 1.1 Jurisdicción Core | Existencia de un artefacto/paquete Core consumible (`@sddia/core` o equivalente declarado) que aísle nodos de control y física de bus/eventos **sin** reglas ni nombres de negocio GesFer |
| **F1-2** | 1.2 Tubería hermética | `capsule-json-io` como contrato innegociable stdin/stdout JSON para skills/tools de bajo nivel; IA obrera sin shell crudo fuera de aduana |
| **F1-3** | 1.3 Cáscaras bifrontales | **Solo esqueletos vacíos** de SddIA Forge y SddIA Portal que declaren dependencia inerte al Shared Kernel; sin UI/producto funcional ni AST compilador |

## Fuera de alcance (explícito)

- Fase 2: `.SddIA/` + despertadores en los 4 microservicios GesFer.
- Fase 3: códices C# / agentes especialista / minteo NFT IOTA.
- Fase 4: wallet, caja de ceguera operativa en GesFer, peaje termodinámico a ledger.
- Absorber el PBI kitchen de separación de dominio (soft-dep paralelo).
- Rediseñar Kalma2, puentes SSE, o procesos `feature`/`bug-fix` existentes.

## Criterios de aceptación (entrada Argos futura)

| ID | Criterio |
|----|----------|
| **AC1** | Core empaquetado o boundary explícito documentado: instancia GesFer no requiere mutar genoma para consumir física de eventos/cápsulas |
| **AC2** | Cero literales de dominio GesFer (nombres de microservicios, BD, rutas de cliente) en el perímetro Core tocado por este feature |
| **AC3** | Norma/contrato `capsule-json-io` referenciada como ley de ejecución; violaciones de E/S fuera de envelope = rechazo documentado en validación |
| **AC4** | Esqueletos Forge y Portal existen como contenedores vacíos con dependencia declarada al Shared Kernel (sin features de producto) |
| **AC5** | Cascada documental `features-documentation-pattern` bajo este `persist_ref`; Git solo vía `skill:git-manager` |
| **AC6** | Fases 2–4 no aparecen como entregables de este PR |

## Restricciones (ley aplicada)

- Norte: `README.md` — activos atómicos, Core/instancia desacoplados, cápsulas, JSON stdin/stdout.
- Constitución: `SddIA/CONSTITUTION_CORE.md` — Triaje; no lobotomía del Core por cliente.
- Blindaje: `SddIA/norms/external-ai-constraints.md` — forja genoma solo vía procesos/entity-manager.
- Documentación: `features-documentation-pattern` v1.2.x — frontmatter obligatorio; sin `spec.json`.
- SSOT rutas: `SddIA/core/cumulo.paths.json` (`paths.featurePath` → `docs/features`).
- PBI origen permanece en `kitchen/` hasta promoción operador (UUID atómico pendiente).

## Handoff a Dedalo

Consumir **este cuerpo** como `refined_requirements`. Diseñar `spec.md` / `plan.md` **únicamente** para F1-1…F1-3. No inventar arquitectura de Fases 2–4. Resolver O1–O3 de `clarify.md` solo en el plano técnico de F1, sin ampliar el qué.
