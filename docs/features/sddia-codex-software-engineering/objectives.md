---
feature_name: sddia-codex-software-engineering
created: "2026-08-09"
process: feature
branch_name: feat/sddia-codex-software-engineering
persist_ref: docs/features/sddia-codex-software-engineering
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-SDDIA-DOMAIN-ABSTRACT-02 — Migración process software a códice.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
execution_id: c76c5d95-b066-49ca-834b-78a4f9443a62
phase: mayeuta-stabilization
agents: mayeuta
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
---

# Objetivos — sddia-codex-software-engineering

## Misión

Materializar el **Códice de Dominio `codex-software-engineering`** y enlazarlo al runtime para que los flujos de ingeniería de software (`feature`, `bug-fix`, `refactorization`, y ciclo PR según laudo) exijan autoridad de códice/perfil — heredando el perfil activo de ABSTRACT-01 — en lugar de vivir como supuesto implícito del Core.

La relocalización física de process fuera de `directories.process` queda condicionada a diseño Dedalo (AC-MOVE); si bloquea resolución del orquestador, se difiere a ABSTRACT-03.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Forja `codex-software-engineering` + índice | Migrar `*-creator` / `entity-manager` / routes EDA |
| Membresía process software-lifecycle | `codex-personal-assistant` (L-SPLIT-C padre) |
| Gate runtime sin códice → deny; con códice → allow | GesFer / Paciente 0 |
| Reuso `execution_profile` ABSTRACT-01 | Reabrir gate Git ABSTRACT-01 salvo integración menor |
| Cascada documental + cierre single-PR | Vaciado total de `SddIA/process/` sin path de resolución |

## Hitos

1. **H1 — Códice:** alta indexada bajo `directories.library_codexes`.
2. **H2 — Membresía:** inventario process del dominio en el códice (contrato Dedalo).
3. **H3 — Gate:** orquestador exige códice/perfil software para process miembros.
4. **H4 — AC-MOVE:** move físico **o** defer documentado ABSTRACT-03.

## Criterios de aceptación

- **AC-CODEX / AC-MEMBER / AC-GATE / AC-ALLOW / AC-BUILD / AC-DOC** (ver clarify D4).
- **AC-MOVE:** APTO o diferido explícito.

## Ley aplicada

- Rutas vía `SddIA/core/cumulo.paths.json`.
- Genoma vía `entity-manager` / `./sddia-run.sh`.
- `codex-contract` + `features-documentation-pattern` v1.2.x.
- Filtro C: no mover process sin diseño de resolución.

## Handoff Dedalo

Consumir este cuerpo como `refined_requirements`. Diseñar `spec.md` + `plan.md` bajo L-MVP-A; dictamen AC-MOVE obligatorio.
