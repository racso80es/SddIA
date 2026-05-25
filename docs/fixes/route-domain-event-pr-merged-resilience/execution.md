---
feature_name: route-domain-event-pr-merged-resilience
created: "2026-05-25"
process: bug-fix
items_applied:
  - H2 resolve_pull_request_lifecycle
  - H3 router precheck
  - H4 tests + E2E lab
---

# Ejecución — route-domain-event PR merged resilience

## Tests unitarios

```powershell
python SddIA/scripts/qa/test_eda_bus_v3plus.py -v
```

Resultado: **14 tests OK** (incl. 6 lifecycle).

## Regresión E2E lab

```powershell
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
```

Resultado: `"success": true`, `"parent_purged": true`.

## Smoke router PR #48 (payload incidente)

```powershell
python -c "
from pathlib import Path
import sys, json
sys.path.insert(0,'SddIA/scripts/qa')
from route_domain_event_core import dispatch_subscriber
repo = Path('.').resolve()
event = json.loads(Path('.events/dead-letter/ce5f287e-4e27-4d18-98f6-b9201596ae00.json').read_text())
sub = {'agent':'argos','process':'pull-request-review'}
print(dispatch_subscriber(repo, sub, event))
"
```

Resultado esperado: `('argos.pull-request-review', 'success', None, 0)` con `merge_already_done` vía gh MERGED.
