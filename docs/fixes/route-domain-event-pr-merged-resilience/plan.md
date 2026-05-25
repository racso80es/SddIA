---
feature_name: route-domain-event-pr-merged-resilience
created: "2026-05-25"
process: bug-fix
branch_name: fix/route-domain-event-pr-merged-resilience
persist_ref: docs/fixes/route-domain-event-pr-merged-resilience
phases:
  - H1: Documentación y laudo
  - H2: API lifecycle en eda_bus_utils
  - H3: Integración router
  - H4: Tests y regresión
  - H5: Cierre documental
---

# Plan — route-domain-event PR merged resilience

## Estado

| Hito | Estado |
|------|--------|
| Rama `fix/route-domain-event-pr-merged-resilience` | ✅ |
| PBI en `docs/todos/done/` | ✅ |
| `objectives.md` | ✅ |
| `clarify.md` | ✅ |
| `spec.md` | ✅ |
| Implementación | ✅ |
| `validacion.md` APTO | ✅ |

---

## Hito 1 — Documentación y laudo ✅

- [x] PBI Kaizen con incidente `ce5f287e…`
- [x] `objectives.md`, `clarify.md`, `spec.md`, `plan.md`
- [ ] Checkout rama `fix/route-domain-event-pr-merged-resilience` desde `main`

**Commit sugerido:** `docs(fix): iniciar Kaizen resiliencia merge en route-domain-event`

---

## Hito 2 — API `resolve_pull_request_lifecycle`

**Archivo:** `SddIA/scripts/qa/eda_bus_utils.py`

- [ ] `gh_executable()` con `SDDIA_GH_EXECUTABLE`
- [ ] Refactor `github_pr_merged` → usa `gh_executable()`
- [ ] `_parse_pr_number(pr_url) -> int | None`
- [ ] `_branch_exists_on_remote(repo, branch) -> bool`
- [ ] `_merged_via_pull_ref(repo, pr_number, target_branch) -> bool`
- [ ] `resolve_pull_request_lifecycle(...)` según `spec.md` §3

**Estimación:** 1 commit (`fix(eda): resolve_pull_request_lifecycle multicapa`)

---

## Hito 3 — Integración router

**Archivo:** `SddIA/scripts/qa/route_domain_event_core.py`

- [ ] Import `resolve_pull_request_lifecycle`
- [ ] En `dispatch_subscriber`, rama `pull-request-review`: resolver lifecycle **antes** de subprocess
- [ ] Sustituir bloque suelto `if github_pr_merged` por resolver unificado
- [ ] Mensajes `failed` explícitos (§4.1 spec)
- [ ] `_status_is_terminal_ok`: añadir `skipped-merged-retroactive` si se implementa skip directo

**Estimación:** 1 commit (`fix(eda): endurecer dispatch pull-request-review pre-checkout`)

---

## Hito 4 — Tests y regresión

**Archivo:** `SddIA/scripts/qa/test_eda_bus_v3plus.py`

| Caso | Mock |
|------|------|
| gh → MERGED | subprocess gh |
| gh ausente + pull-ref ancestor | git fetch / merge-base |
| rama remota presente | rev-parse OK |
| rama ausente + PR open | failed explícito |

- [ ] Tests unitarios lifecycle
- [ ] Test integración `dispatch_subscriber` con tmp repo / mocks
- [ ] `run-eda-e2e-lab.py --json` exit 0
- [ ] Delta `events-contract.md` §4 si nuevo status terminal

**Estimación:** 1 commit (`test(eda): lifecycle PR mergeado en router`)

---

## Hito 5 — Cierre documental (pre-merge)

- [ ] `implementation.md` + `execution.md`
- [ ] `validacion.md` — `global: APTO`, `pbi_archived: true`
- [ ] Mover PBI → `docs/todos/done/`
- [ ] `delivery-close-cycle` → PR único

**Criterio Done:** regla `task-closure-documental.mdc`

---

## Orden de ejecución recomendado

```text
H1 (docs) → H2 (utils) → H3 (router) → H4 (tests) → H5 (validación + PBI done)
```

## Dependencias

| Upstream | Relación |
|----------|----------|
| PR #48 / `kaizen-alert-required-eda-v2` | Incidente motivador — ya mergeado |
| `revision-gestion-eventos-kaizen` | Patrón terminalización DL — compatible |
| `eda-coverage-ssot-bus-isolation` | Sin conflicto — dominios distintos |

## Riesgo de paralelismo

Si otra rama toca `route_domain_event_core.py` o `eda_bus_utils.py`, rebasar antes de H3.
