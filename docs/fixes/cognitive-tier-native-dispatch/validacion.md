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
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
checks:
  AUT26-T1-NATIVE-PREDICATE-PROCESS: APTO
  AUT26-T2-TIER-DISPATCH-REGRESSION: APTO
  DOC_CASCADE: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/route_domain_core.rs
- docs/fixes/cognitive-tier-native-dispatch/
- docs/audits/AUT-26.md
validated_head_sha: 96a622ec05c02f70be009ae08200efcf78bd8b08
validated_tree_sha: 4a6f7d1027ba782957f4e23fb621294a9673ca48
diff_digest: sha256:ea4f358486c5b8f6856c1f2b323241030545cb39c5b7a4540e491c8f97717103
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