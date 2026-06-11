---
feature_name: husky-pre-push-blocking-route
process: feature
created: 2026-06-09T00:00:00Z
branch: feat-husky-pre-push-blocking-route-8716941346700891712
---

# Implementación

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `.husky/pre-push` | Despertador inerte: `exec "$PYTHON" "$GATE"` → `pre_push_gate.py` |
| `SddIA/scripts/qa/route_domain_event_core.py` | `__main__` reescrito: `SDDIA_LAB_ROUTE_SYNC=1` + `route_domain_event()` |
| `SddIA/core/event-domain-subscriptions.json` | Entrada `Local_QA_Requested` → argos/pull-request-review |
| `SddIA/events/orchestration/local-qa-requested.md` | Clase ECST nueva |
| `SddIA/events/orchestration/index.md` | Fila añadida al catálogo |

## `.husky/pre-push` (despertador inerte)

Idéntico al canon `SddIA/scripts/qa/git-hooks/pre-push`: `set -eu`, guarda `SDDIA_SKIP_HOOKS`, resuelve `REPO_ROOT`, delega con `exec "$PYTHON" "$GATE"`.

## `route_domain_event_core.py __main__` (modo blocking)

```
__main__ --event <type> [--blocking]
  1. repo = _repo_root()
  2. Si --blocking: os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
  3. branch = git rev-parse --abbrev-ref HEAD (shell=False)
  4. Escribir evento pending/{uuid}.json con payload {"branch": branch}
  5. result = route_domain_event(repo, rel_path)
  6. sys.exit(result["exitCode"])
```

Sin lookup manual de suscriptores. Sin shell=True. Sin resolución de process/action/tool.
