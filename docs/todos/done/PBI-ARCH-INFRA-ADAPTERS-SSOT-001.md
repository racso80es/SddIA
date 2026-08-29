---
document_id: PBI-ARCH-INFRA-ADAPTERS-SSOT-001
uuid: b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47
title: "[ARQUITECTURA] Gobernanza SSOT de infrastructure/adapters — indexación Cúmulo y observabilidad"
format: markdown
version: "1.2.0"
created: "2026-08-29"
refined: "2026-08-29"
closed: "2026-08-29"
status: done
refinement_status: implemented
pbi_archived: true
priority: media
process: feature
type: architecture
dispatch: false
branch_name: feat/infra-adapters-ssot-governance
persist_ref: docs/features/infra-adapters-ssot-governance
depends_on: []
blocks:
  - PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
related:
  - SddIA/core/cumulo.paths.json
  - SddIA/agents/cumulo.md
  - SddIA/infrastructure/adapters/
  - docs/features/infra-adapters-ssot-governance/validacion.md
  - docs/todos/pending/[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema.md
  - docs/todos/pending/[ARQUITECTURA] LanceDB — integración física real y memoria vectorial efectiva.md
---

# [ARQUITECTURA] Gobernanza SSOT de `infrastructure/adapters` — indexación Cúmulo y observabilidad

## 0. Cierre (2026-08-29)

Implementado en `feat/infra-adapters-ssot-governance`. `validacion.md` APTO. Deuda DD-7 del Espejo **cerrada** a nivel gobernanza SSOT; el panel Espejo (Fase 2) puede consumir `directories.infrastructure_adapters` + `index.md`.

## 1. Contexto y Fricción (origen de la deuda)

El árbol `SddIA/infrastructure/` no estaba registrado en `directories` de `cumulo.paths.json`. Los adaptadores eran Entropía documental sin censo ni `status` observable.

## 2. Objetivo Medible (cumplido)

`cumulo.paths.json` v1.7.0 declara la topología; `index.md` + fichas catalogan los dos adaptadores LanceDB como `placeholder`.

## 3. Decisiones cerradas (spec L1–L11)

| ID | Laudo |
|----|-------|
| **DA-1** | `directories.infrastructure` + `infrastructure_adapters`; SSOT 1.7.0; sin `execution_capsules` |
| **DA-2** | Censo `index.md` + ficha `{name}.md`; sin `type: adapter` en Constitución |
| **DA-3** | `adapters-contract.md` → `contracts.infrastructure_adapters` |
| **DA-4** | `status: placeholder\|active\|deprecated` en ficha e índice |
| **DA-5** | Escritura directa bajo `SddIA/infrastructure/` (fuera genoma DA-2 protegido) |

## 4. Entregables

- `SddIA/core/cumulo.paths.json` v1.7.0
- `SddIA/infrastructure/adapters/{adapters-contract,index,lancedb-thought-repo,lancedb-evolution-repo}.md`
- `SddIA/evolution/b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47.md`

## 5. Criterios de Aceptación

| ID | Estado |
|----|--------|
| INF-CA1 | APTO |
| INF-CA2 | APTO |
| INF-CA3 | APTO |
| INF-CA4 | APTO |
| INF-CA5 | APTO |
| INF-CA6 | APTO |
