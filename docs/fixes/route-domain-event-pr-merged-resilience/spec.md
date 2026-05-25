---
feature_name: route-domain-event-pr-merged-resilience
created: "2026-05-25"
process: bug-fix
base: main
scope: route-domain-event-pr-lifecycle-resolution
version_spec: "1.0.0"
---

# Especificación — Resiliencia merge en route-domain-event

## 1. Problema

El suscriptor `argos.pull-request-review` de `PullRequest_Presented` falla con dead-letter cuando:

1. El evento permanece en `pending/` más allá del merge del PR.
2. La rama remota es podada (`git fetch --prune`).
3. `github_pr_merged(pr_url)` devuelve `False` (gh ausente, error auth, o stdout vacío).

El subprocess `pull-request-review` intenta checkout y propaga el error Git opaco:

```text
error: pathspec '<branch>' did not match any file(s) known to git
```

**Incidente de referencia:** evento `ce5f287e-4e27-4d18-98f6-b9201596ae00` (PR #48).

## 2. Alcance

| Artefacto | Acción |
|-----------|--------|
| `SddIA/scripts/qa/eda_bus_utils.py` | Nueva API `resolve_pull_request_lifecycle`; endurecer `github_pr_merged` |
| `SddIA/scripts/qa/route_domain_event_core.py` | Consumir resolver en `dispatch_subscriber` |
| `SddIA/scripts/qa/test_eda_bus_v3plus.py` | Casos unitarios nuevos |
| `SddIA/events/events-contract.md` | Documentar `skipped-merged-retroactive` |
| `docs/fixes/route-domain-event-pr-merged-resilience/*` | Cascada documental |

**Fuera de scope:** `execute_process_capsules.py` (checkout interno sin cambio en v1).

## 3. API — `resolve_pull_request_lifecycle`

### 3.1 Firma

```python
def resolve_pull_request_lifecycle(
    repo: Path,
    *,
    branch: str,
    pr_url: str | None = None,
    target_branch: str = "main",
) -> dict[str, Any]:
    ...
```

### 3.2 Campos de retorno

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `merged` | `bool \| None` | `True` si PR mergeado; `False` si abierto/cerrado sin merge; `None` indeterminado |
| `source` | `str` | `gh` \| `git-pull-ref` \| `branch-remote` \| `unknown` |
| `branch_on_remote` | `bool` | Tras `fetch --prune`, existe `origin/{branch}` |
| `pr_number` | `int \| None` | Extraído de `pr_url` (`/pull/(\d+)`) |
| `diagnostics` | `list[str]` | Traza corta para logs / error_trace |

### 3.3 Algoritmo (orden estricto)

1. **Normalizar** `branch`, `pr_url`, `target_branch`.
2. **Capa gh** — si `gh_executable()` resuelve:
   - `gh pr view <url> --json state,mergedAt`
   - `state == MERGED` → return `merged=True, source=gh`
   - `state == OPEN` → continuar (puede coexistir con rama podada anómala)
3. **Fetch remoto** — `git fetch origin --prune` (vía git-manager o subprocess acotado).
4. **Rama remota** — `git rev-parse origin/{branch}`:
   - éxito → `branch_on_remote=True`, `merged=False` (PR presumiblemente abierto)
5. **Capa git pull-ref** (solo si `pr_number` conocido y rama ausente):
   - `git fetch origin pull/{N}/head:refs/remotes/origin/.sddia/pr-{N}-head`
   - `git merge-base --is-ancestor origin/.sddia/pr-{N}-head origin/{target_branch}`
   - éxito → `merged=True, source=git-pull-ref`
6. **Default** — `merged=None`, `branch_on_remote=False`, `source=unknown`

### 3.4 Helper `gh_executable()`

```python
def gh_executable() -> str | None:
    override = os.environ.get("SDDIA_GH_EXECUTABLE", "").strip()
    if override:
        return override if Path(override).is_file() else None
    return shutil.which("gh")
```

Reemplazar invocaciones hardcoded `["gh", ...]` en `github_pr_merged` por este helper.

## 4. Cambios en `dispatch_subscriber`

### 4.1 Punto de integración

Antes de construir `process_inputs` para suscriptores con `process`:

```python
if process_name.strip() == "pull-request-review":
    lifecycle = resolve_pull_request_lifecycle(
        repo, branch=branch, pr_url=pr_url if isinstance(pr_url, str) else None
    )
    if lifecycle.get("merged") is True:
        process_inputs["merge_already_done"] = True
    elif lifecycle.get("merged") is False and not lifecycle.get("branch_on_remote"):
        return sid, "failed", (
            "pull-request-review: rama ausente en origin y PR no mergeado "
            f"(branch={branch}, pr={lifecycle.get('pr_number')})"
        ), 1
    elif lifecycle.get("merged") is None and not lifecycle.get("branch_on_remote"):
        # gh indeterminado + sin rama: intentar merge_already_done solo si pull-ref confirmó arriba
        return sid, "failed", (
            "pull-request-review: no se pudo resolver ciclo de vida del PR "
            f"(branch={branch}; diagnostics={lifecycle.get('diagnostics')})"
        ), 1
```

### 4.2 Skip terminal directo (opcional H3)

Si `merged is True` y se desea evitar subprocess de 7 fases:

```python
return sid, "skipped-merged-retroactive", None, 0
```

**Decisión v1:** mantener subprocess con `merge_already_done=True` (menor delta, reutiliza veredicto existente). Evaluar skip directo en H4 si latencia subprocess molesta.

### 4.3 `_status_is_terminal_ok`

Añadir prefijo `skipped-merged-retroactive` a estados OK terminales.

## 5. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| RM-CA1 | gh MERGED → `merge_already_done` | Unit mock gh |
| RM-CA2 | gh ausente + pull-ref ancestor → `merge_already_done` | Unit mock sin gh |
| RM-CA3 | PR OPEN + rama ausente → failed explícito (no pathspec) | Unit |
| RM-CA4 | PR OPEN + rama remota → aduana invocada normal | Integration lab |
| RM-CA5 | Regresión bus V3+ | `pytest test_eda_bus_v3plus.py` |
| RM-CA6 | Regresión E2E lab | `run-eda-e2e-lab.py --json` exit 0 |

## 6. Smoke tests

```powershell
# Unitarios
python -m pytest SddIA/scripts/qa/test_eda_bus_v3plus.py -k "pull_request_lifecycle or pr_merged" -v

# Simulación router (PR #48 payload, gh operativo)
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

# Regresión E2E
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
```

## 7. Matriz de regresión

| Escenario | Antes | Después |
|-----------|-------|---------|
| Presented + PR mergeado + rama podada + gh OK | DL pathspec (si tardío) | success / merge_already_done |
| Presented + PR mergeado + gh KO + pull-ref OK | DL pathspec | success |
| Presented + PR abierto + rama OK | aduana normal | sin cambio |
| Presented + push fallido + rama ausente | pathspec | failed explícito |
