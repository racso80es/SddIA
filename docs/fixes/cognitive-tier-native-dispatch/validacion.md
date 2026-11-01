---
feature_name: cognitive-tier-native-dispatch
created: 2026-10-31
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/cognitive-tier-native-dispatch
tracker_ref: AUT-26
execution_id: 73d65b19-25cb-489c-94e5-abffe60b1e46
global: APTO
pbi_archived: false
branch: Fix_Events_Multiple
approval_status: aprobado
verdict: aprobado
validated_base_sha: "2d14642644f1acd7ebfa1fa32bac0f941f817736"
checks:
  AUT26-T1-NATIVE-PREDICATE-PROCESS: APTO
  AUT26-T2-TIER-DISPATCH-REGRESSION: APTO
  DOC_CASCADE: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/route_domain_core.rs
- docs/fixes/cognitive-tier-native-dispatch/
- docs/audits/AUT-26.md
validated_head_sha: "6ac2b651c54bbbab8f5b14cb1cc4eb6aaabb7fa4"
validated_tree_sha: "520520befed83c2b688eaaf303c1320b1414b6ab"
diff_digest: "sha256:0971d1c7bde471cbf30ba394620c0c3513f6cd6c3ee32f41c29969fe93134121"
---

# Validación — AUT-26

## Veredicto

**APTO** — En despacho store-backed, `subscriber_declares_native` incluye `process`; la cápsula nativa no cede a `AGENT_PHASE` por tier cognitivo.

## Evidencia

| Check | Medio | Resultado |
|-------|--------|-----------|
| T1 | `subscriber_declares_native` incluye `process`, `action`, `tool` | OK |
| T2 | `cargo test -p execute-process tier_dispatch --lib` | 2/2 OK |
| AGENT_PHASE solo agente | `tier_none_does_not_spawn_and_cognitive_receives_ecst_and_workspace` | OK |
| Nativos store | `store_backed_native_capsule_does_not_use_cognitive_runtime` | OK |

## Criterios de cierre (spec)

- Cápsula nativa con ECST solo en store: **APTO** (T1 + T2).
- `AGENT_PHASE` solo sin cápsula: **APTO** (test vigente).
- `notify-humanized-pr-merged` / `tracker-stamp` no sustituidos por runtime: **APTO** (T2).
- Tier `none` sin AGENT_PHASE en fila `process`: **APTO** (T2).

## Cierre tracker

Provider `linear`: **AUT-26** pasa a Done con merge del PR que incluya este diff; sin fichero en `docs/todos/done/`.