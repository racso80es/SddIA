---
feature_name: route-domain-event-pr-merged-resilience
created: "2026-05-25"
process: bug-fix
items:
  - resolve_pull_request_lifecycle en eda_bus_utils.py
  - gh_executable + SDDIA_GH_EXECUTABLE
  - _pull_request_review_precheck en route_domain_event_core.py
  - tests TestPullRequestLifecycle
  - events-contract.md §4 paso 8
---

# Implementación — route-domain-event PR merged resilience

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/eda_bus_utils.py` | `gh_executable`, `parse_pr_number`, `resolve_pull_request_lifecycle`, refactor `github_pr_merged` → `_gh_pr_state` |
| `SddIA/scripts/qa/route_domain_event_core.py` | `_pull_request_review_precheck`; lifecycle antes de subprocess aduana |
| `SddIA/scripts/qa/test_eda_bus_v3plus.py` | Clase `TestPullRequestLifecycle` (6 casos) |
| `SddIA/events/events-contract.md` | Paso 8 ciclo V3+ |
| `.dev/.env.example` | Comentario `SDDIA_GH_EXECUTABLE` |

## Cadena implementada

1. **gh** — `_gh_pr_state` vía `gh_executable()` (override env).
2. **Rama remota** — `_branch_exists_on_remote` tras `fetch --prune`.
3. **pull/N/head** — `_merged_via_pull_ref` + `merge-base --is-ancestor`.
4. Router — fallo explícito si PR abierto sin rama; `merge_already_done` si merge confirmado.
