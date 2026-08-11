---
feature_name: evolution-contract-index-v11
created: "2026-08-11"
process: feature
branch_name: feat/evolution-contract-index-v11
persist_ref: docs/features/evolution-contract-index-v11
document_id: 4feb4ea2-b1ca-41c6-bc57-75457840eabf
execution_id: c906d516-f708-48bc-87b3-19980a9a11ab
phase: blueprint
agents: dedalo
phases: "contract → index → validator → qa-evidence → docs → argos"
---

# Plan — evolution-contract-index-v11

## Fases

| # | Fase | Acciones | Done |
|---|------|----------|------|
| 1 | Contrato | Escribir `normative_documents.evolution_contract` v1.1.0 (esquema, alias, L-JURISDICTION) | [x] |
| 2 | Índice | Generar `Evolution_log.md` con 61 filas desde inventario audit 2026-08-11; documentar delta post-corte | [x] |
| 3 | Validador | Módulo Rust + CLI `sddia-qa validate-evolution-contract` (solo lectura, `--json`, `--universe audit-cut`) | [x] |
| 4 | QA evidencia | Ejecutar validador; persistir `_qa-validate-evolution.json`; assert `classified_total==61` y sin mutación de detalles | [x] |
| 5 | Docs Tekton | `implementation.md`, `execution.md`, nota migración/compatibilidad en persist_ref | [x] |
| 6 | Evolution ciclo | Registro `{execution_id}.md` del hito **sin** alterar las 61 filas del corte; opcional fila aparte solo si se decide indexar post-corte en ciclo futuro | [x] |
| 7 | Argos + cierre | `validacion.md` APTO, PBI → `docs/todos/done/`, `pbi_archived: true` | [x] |

## Orden de mutación

1. **`directories.evolution`:** `evolution_contract.md` → `Evolution_log.md` (excepción EDA).
2. **`directories.tools` / crate `sddia-qa`:** módulo `validate_evolution_contract.rs` + wire en `main.rs` (DA-4 feature activa).
3. **`persist_ref`:** evidencia QA + cascada documental.
4. **Evolution del hito:** detalle UUID = `execution_id` (o nuevo UUID si colisión); no reescribir históricos.

## Delegaciones (blueprint ejecutable)

| Fase plan | Cápsula / agente |
|-----------|------------------|
| 1–2, 5 | Tekton + escritura directa evolution/docs (jurisdicción permitida) |
| 3–4 | Tekton + `cargo build -p sddia-qa` / invocación binario |
| 6 | filesystem bajo `directories.evolution` |
| 7 | agent:argos + `doc:closure` / move PBI |

RBAC objetivo: `ecosystem-evolution`, `filesystem-ops`, `quality-assurance`.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Drift 61 vs disco | Anclar nombres al informe; fallar si falta archivo del corte |
| Delta post-corte (3 archivos) | Documentar en `migration-notes.md`; fuera del índice de este PR |
| Validador usado como gate CI | Exit 0 con legacy; no añadir workflow |
| Paths hardcodeados | Resolver claves Cúmulo en el binario vía `find_repo_root` + parse `cumulo.paths.json` |

## Handoff Tekton

Ejecutar §1–6; no normalizar frontmatter de históricos; no tocar `cumulo.paths.json`.
