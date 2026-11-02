---
feature_name: aut-52-refine-fracture-route
created: 2026-11-02
updated: 2026-11-02T16:50:00Z
process: bug-fix
phase: Veredicto
agent: tekton
branch: Fix_Events_Multiple_02
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-52-refine-fracture-route
tracker_ref: AUT-52
execution_id: f2880068-385c-4403-8c31-9d11bd4b08a1
code_commit: 49e02b5c
global: APTO
pbi_archived: false
source_process: bug-fix
scope: AUT-52 C1–C5 — código 49e02b5c; doc alineado post re-refino ca843b5c / diseño f2880068
checks:
  DOC_OBJECTIVES: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_EVOLUTION: APTO
  AUT52_CA1: APTO
  AUT52_CA2: APTO
  AUT52_CA3: APTO
  AUT52_CA4: NO_APTO
  AUT52_CA5: APTO
  AUT52_CA6: APTO
  AUT52_CA7: APTO
  AUT52_CA8: NO_APTO
  TECH_CARGO_AUT52: APTO
fracture_corpus:
  artifact: SddIA/engine/execute-process/tests/fixtures/fracture-corpus/684ebbf7-refine-route-redb.json
  status: anchored
validated_base_sha: c86fec895b9878cd6d80007553c161789561a93c
validated_head_sha: 415285837fa6c322c26bf24a24891ba88f970406
validated_tree_sha: 25576cd2ada2512ac9846a098f3151efa50803f5
diff_digest: sha256:637c4186c73dff6285d4837c3e05d2bb7daaed7d66a51e55d9d165012f2708d5
---

# Validación — aut-52-refine-fracture-route

## Veredicto

**APTO** para entrega en rama con salvedades: AUT52-CA4 (re-route integración evento `684ebbf7-…`) y AUT52-CA8 (Linear Done) quedan fuera de este commit. Compilación y **`cargo test aut52 --lib`**: 3/3 OK (capsules + route redb).

## Evidencia por corte

| Criterio | Estado | Nota |
|----------|--------|------|
| AUT52-CA1 | APTO | Sidechannel FS + `friction_id` en payload lock |
| AUT52-CA2 | APTO | Path virtual `{event_id}.json` en redb-only |
| AUT52-CA3 | APTO | Rechazo `.redb`; batch `success: false` |
| AUT52-CA4 | NO_APTO | Requiere bus + evento en store (lab manual post-merge) |
| AUT52-CA5 | APTO | Test `aut52_tool_invoke_error_message_includes_linear_transport_detail` |
| AUT52-CA6 | APTO | Firma `F-REFINE-ITEM-REDB-LOCK` + corpus `684ebbf7-refine-route-redb.json` |
| AUT52-CA7 | APTO | `materialize_fracture_pbi::run_remote` sin regresión (build_pbi_body + labels) |
| AUT52-CA8 | NO_APTO | Tracker Linear Done pendiente laudo humano |

## Evolution

- `SddIA/evolution/c55f04de-f956-4f1e-89ab-580b2c5bab34.md` (`hash_integrity` canónico).

## Prohibiciones respetadas

- Sin ficheros nuevos bajo `docs/todos/pending|done/`.
- Genoma `fracture-signatures.json` acotado a firma AUT-52.