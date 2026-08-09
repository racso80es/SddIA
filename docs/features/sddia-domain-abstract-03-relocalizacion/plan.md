---
feature_name: sddia-domain-abstract-03-relocalizacion
created: "2026-08-09"
process: refactorization
branch_name: feat/sddia-domain-abstract-03-relocalizacion
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
pbi_ref: docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
phases: "T0-resolve T1-move T2-refs T3-smoke T4-docs T5-argos"
agents: dedalo
laudo: L-PACK-MULTIROOT-SIX-MOVE
---

# Plan — sddia-domain-abstract-03-relocalizacion

Blueprint de ejecución (L-RESOLVE-FIRST). No es un process nuevo: orquesta Tekton sobre motor + genoma packing.

| ID | Fase | Touchpoints | Done |
|----|------|-------------|------|
| **T0** | Resolución multi-root | `cumulo.paths.json` (`process_domain_roots` + bump), `resolver.rs` (+ tests fixture), reuso `load_paths_config` | **AC-RESOLVE** |
| **T1** | Move físico + índice | 6× `.md` → packing códice; `SddIA/process/index.md`; `…/process/index.md` dominio; sin stubs Core | **AC-MOVE / AC-INDEX** |
| **T2** | Referencias | Hardcodes tests/reactors/norms/`external-ai-constraints`/nota códice; paths docs PR | refs alineadas |
| **T3** | Compat | Smokes feature|refactorization; gate deny; Kalma2/sddia-run; `cargo build -p execute-process --release` | **AC-RUN / AC-TQM / AC-BUILD** |
| **T4** | Docs tarea | `implementation.md`, `execution.md`, evolution UUID process | cascada |
| **T5** | Argos + cierre | `validacion.md`; PBI kitchen→pending→done según operador; un PR | **AC-DOC** |

## Orden Tekton (estricto)

1. **T0** completo y tests verdes **antes** de cualquier `rm`/`mv` de los 6.
2. **T1** move atómico en el working tree (git vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`).
3. **T2** barrido refs (grep `SddIA/process/(feature|bug-fix|refactorization|pull-request-review|accept-pr|delivery-close-cycle)`).
4. **T3** smokes + build.
5. **T4–T5** documentación y cierre en rama.

## Delegaciones

| Necesidad | Vía |
|-----------|-----|
| Motor Rust (`resolver`, tests) | Tekton directo `SddIA/engine/execute-process` (L-GENOME motor) |
| Move genoma process | `skill:filesystem-manager` + evidencia git `git-manager` — **no** re-crear vía process-creator (preservar UUID) |
| Mutación Cúmulo / normas path | Editable directo documentado en spec; genoma process packing = move, no alta creator |
| Semillas Kaizen residuales (creators→domain) | Solo `agent:cumulo` / `Kaizen_Alert_Required` — Tekton **no** escribe `docs/todos/` |

## Gate anti-entropía

```text
si AC-RESOLVE != verde → STOP (prohibido T1)
si stub en SddIA/process/{los-6} → FAIL AC-MOVE
si fila fantasma índice Core apuntando a path viejo como ejecutable → FAIL AC-INDEX
```

## Handoff Argos

Consumir `objectives.md` § AC + `spec.md` §5. Verificar D2 (6 movidos), D5 (sin stubs), D8 (orden evidenciado en `execution.md`).
