---
feature_name: kaizen-lancedb-ciclo-fricciones
created: "2026-08-31"
process: feature
phase: planning
agents: dedalo
phases:
  - L0-design
  - L1-mayeuta-cubos
  - L2-dcc-workflow-halt
  - L3-relacionado-helper
  - L4-em-norms-ingest-pattern
  - L5-pbi-fracture-archive
  - L6-tests-evolution
  - L7-closure
branch_name: feat/kaizen-lancedb-ciclo-fricciones
persist_ref: docs/features/kaizen-lancedb-ciclo-fricciones
pbi_ref: docs/todos/pending/[KAIZEN] Post-ciclo LanceDB — PAT-workflow, Kintsugi saltado, Mayeuta ciego y correlato evolution.md
document_id: PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES
uuid: "12250eca-49c6-4008-ac50-5c5722a7fe91"
execution_id: "b97c39ce-f5d6-4e26-92c6-68de26eedcf0"
---

# Plan — kaizen-lancedb-ciclo-fricciones

Orden: L1 → L2 → L3 → L4 → L5 → L6 → L7. Este commit sella Diseño (`clarify.md` + `objectives.md` + `spec.md` + `plan.md`). Ejecución Tekton **después** de esta parada.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref` (clarify/objectives/spec/plan). Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. Semilla `.tmp/feature-kaizen-lancedb-ciclo-fricciones.json`.

## Fase L1 — Mayeuta (CA1, CA4)

`SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs`. Tests en el mismo módulo. Fixtures: workflow-scope; specimen `01c9040df256` (Head sha + Apertura en forja).

```text
cd SddIA && cargo test -p execute-process analyze_fracture_kaizen
```

## Fase L2 — DCC envelope + halt (CA2, CA2b)

`SddIA/engine/execute-process/src/engine/delivery_close.rs`. Tests: stamp workflow-scope; suppress fracture; loop halt (phase_reports posteriores `skipped`).

```text
cd SddIA && cargo test -p execute-process dcc_workflow dcc_halt
```

## Fase L3 — Helper relacionado (CA6)

`SddIA/skills/sddia-evolution-register/src/lib.rs` (código de cápsula, no `.md`). Test UNREGISTERED vs OK.

```text
cd SddIA && cargo test -p sddia-evolution-register suggest_relacionado
```

## Fase L4 — Genoma (CA3, CA7, CA8)

Prefijo RAW. Topología `objectives.md` ya en rama.

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-obediencia.json
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-external-ai.json
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-ingest.json
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-features-doc.json
```

Prohibido `Write`/`StrReplace` sobre esos `.md`.

## Fase L5 — Fractura (CA5)

Corregir diagnóstico en `docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (01c9040df256).md`; archivar a `done/` en el mismo PR que el código.

## Fase L6 — Evolution + tests

Alta registro canónico. `relacionado`: uuid PBI `12250eca-49c6-4008-ac50-5c5722a7fe91` + paths tocados (incl. lockfile/manifiestos si el diff los trae). `gate-evolution --json --range` antes de push si toca `directories.evolution`.

```text
cd SddIA && cargo test -p execute-process analyze_fracture_kaizen dcc_
cd SddIA && cargo test -p sddia-evolution-register
SddIA/target/debug/sddia-qa gate-evolution --json --range
```

## Fase L7 — Cierre (fuera de esta parada)

`implementation.md` + `execution.md` → Argos `validacion.md` → ambos PBI a `done/` → `delivery-close-cycle`. Un PR. Tests verdes locales **antes** de DCC. Prohibido raw push si DCC `blocked` por workflow.

## Touchpoints

| Path | Cambio | Vía |
|------|--------|-----|
| `enrich_fracture_pbi_kaizen.rs` | Cubos + tests | IDE |
| `delivery_close.rs` | Stamp + halt | IDE |
| `sddia-evolution-register/src/lib.rs` | Helper | IDE |
| `obediencia-procesos.md` / `external-ai-constraints.md` | CA3 | entity-manager |
| `memory-evolution-ingest.md` | CA7 | entity-manager |
| `features-documentation-pattern.md` | CA8 | entity-manager |
| PBI `01c9040df256` | CA5 | IDE (todos, no genoma) |
| `SddIA/evolution/{uuid}.md` | L6 | cápsula register |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Halt rompe fail-soft post-PR | Solo halt si **no** hay `pr_url` y fase es Publicación remota |
| EM reescribe de más el proceso ingest | `markdown_body_replacements` acotados + phases intent |
| Helper demasiado agresivo | Solo sugiere paths **ya en el diff** |
| DCC de este ciclo choca PAT workflow | Envelope nuevo; no raw; laudo si bloquea |
