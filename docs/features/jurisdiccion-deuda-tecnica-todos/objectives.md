---
feature_name: jurisdiccion-deuda-tecnica-todos
created: "2026-08-28"
process: feature
branch_name: feat/jurisdiccion-deuda-tecnica-todos
persist_ref: docs/features/jurisdiccion-deuda-tecnica-todos
pbi_ref: docs/todos/pending/Registro y Resolución de Deuda Técnica (Kintsugi Ontológico).md
document_id: PBI-OPER-DEUDA-TECNICA-KINTSUGI-001
uuid: 4be8aeee-896a-4d2f-b2d3-3ee0d05fbd80
execution_id: "a3050468-df71-4922-bac9-3743bef2e54d"
---

# Objetivos — jurisdiccion-deuda-tecnica-todos

## Misión

Declarar jurisdicción normativa de `docs/todos/` (qué bucket es despachable, archivable o inerte) y convertir la deuda **no-fractura** en PBI visible bajo `paths.todos.pending`, sin reconstruir el fan-out Kintsugi ni el resolutor Core.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| TQM `extract_pbi_path` solo ancla `docs/todos/pending/` y `docs/todos/done/` | Ítem fuera de esos prefijos no es despachable |
| Archivado exige `rel.contains("docs/todos/pending/")` | Ítem en `DeudaTecnica/` no cierra ciclo (`pbi_archived`) |
| Cúmulo ya declara `paths.todos.pending` / `paths.todos.done` | Norma consume esas claves; no duplicar resolutor |
| Fan-out fractura vive en otro PBI | CA6: consumo, cero reimplementación |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Norma táctica de jurisdicción | `norm-creator` vía `entity-manager`; buckets con ciclo de vida explícito |
| **O2** | Done inalterado | Sin tercer estado de cierre; patrón v1.2.1 |
| **O3** | Reclasificación `DeudaTecnica/` | Laudo por documento; semillas vs deuda accionable vs descarte |
| **O4** | Portador no-fractura | `type: deuda` + `tech_debt_ids` en `pending/`; enum de prefijos |
| **O5** | Evidencia física CA5 | CLI/tests sobre rutas migradas; no afirmación de operador |
| **O6** | Cero solape fan-out | No mutar `fracture_pbi` / materialize / enrich |

## Ley aplicada

- Git vía `skill:git-manager`. Genoma vía `entity-manager`. Rutas vía `cumulo.paths.json`.
- `dispatch: false` en toda migración a `pending/` (riesgo §8 del PBI).
