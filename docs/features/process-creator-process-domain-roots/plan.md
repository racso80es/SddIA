---
feature_name: process-creator-process-domain-roots
created: "2026-08-10"
process: feature
branch_name: feat/process-creator-process-domain-roots
persist_ref: docs/features/process-creator-process-domain-roots
pbi_ref: docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
phases: "T0-factory-juris T1-genome-creator T2-contract-norms T3-verify T4-docs T5-argos"
agents: dedalo
laudo: L-JURIS-MEMBERSHIP-PLUS-FLAG
---

# Plan — process-creator-process-domain-roots

Blueprint de ejecución Dedalo. No instancia process nuevo: orquesta Tekton sobre forja nativa + genoma `process-creator` (L-KEEP-CORE).

| ID | Fase | Touchpoints | Done |
|----|------|-------------|------|
| **T0** | Forja multi-root (factory) | `forges/factory.rs` `run_process_forge`: `load_paths_config` + classifier D1–D2; L-UNIQ-MULTI; L-INDEX-TARGET; update via `process_search_roots`; tests fixture tempdir | **AC-JURIS / AC-INDEX / AC-UNIQ** (unit) |
| **T1** | Genoma creator | `SddIA/process/process-creator.md`: inputs/outputs/fases; bump version; alineado a outputs `resolved_process_root` | contrato obrero = factory |
| **T2** | Contrato / aduana | `process-contract.md` nota multi-root; `external-ai-constraints.md` si lectura-only aún; evolution `7c2d9e41-…` + PBI uuid | D8 |
| **T3** | Verificación | `cargo test`/`build -p execute-process`; smoke AC-SMOKE (lab o test); resolve regresión 6 packing + Core | **AC-SMOKE / AC-RESOLVE-COMPAT / AC-BUILD** |
| **T4** | Docs tarea | `implementation.md`, `execution.md`; AC-OVERLAY N/A documentado | cascada |
| **T5** | Argos + cierre | `validacion.md`; PBI → `docs/todos/done/`; `pbi_archived: true`; un PR | **AC-DOC / AC-NONSCOPE** |

## Orden Tekton (estricto)

1. **T0** primero: evidencia ejecutable en factory + tests verdes.
2. **T1** alinea genoma al comportamiento real (prohibido genoma “domain” con factory aún Core-only).
3. **T2** notas normativas mínimas + evolution.
4. **T3** smoke materializado; no inventar APTO.
5. **T4–T5** documentación y cierre en rama (task-closure-documental).

## Delegaciones

| Necesidad | Vía |
|-----------|-----|
| Mutación Rust factory/tests | Tekton directo `SddIA/engine/execute-process` (motor L-GENOME) |
| Mutación genoma `process-creator` | Forja gobernada / `entity-manager` preferente (L-FORGE); si Raw Kernel DA-4: topología `docs/features/` ya instanciada |
| Git | `skill:git-manager` / `./sddia-run.sh --tool git-manager` — sin bypass raw destructivo |
| Semillas Kaizen | Solo `agent:cumulo` / `Kaizen_Alert_Required` — Tekton **no** escribe `docs/todos/` |

## Gate anti-entropía

```text
si factory sigue hardcode SddIA/process en create domain → FAIL AC-JURIS
si alta domain añade fila a SddIA/process/index.md → FAIL AC-INDEX
si create homónimo de packing en Core no aborta → FAIL AC-UNIQ
si diff mueve/re-forja los 6 packing → FAIL AC-NONSCOPE
si AC-SMOKE sin artefacto/comando reproducible → no declarar éxito
```

## Classifier (resumen operativo)

```text
process_jurisdiction=domain | (ausente ∧ name ∈ process_membership códice SE)
  → dest = process_domain_roots[0]  (o process_domain_root si multi)
else → dest = directories.process
```

## Handoff Argos

Consumir `objectives.md` § AC + `spec.md` §6. Verificar D3 (factory+genoma), D5 (sin fantasma Core), D7 (overlay N/A), D10 (L-NO-REMOVE), L-DEDUP-136.
