---
feature_name: kaizen-higiene-ficheros-temporales
created: "2026-05-25"
process: feature
items_applied:
  - H1-norma-git-operations
  - H2-tmp-paths-lab-teardown
  - H3-run-eda-e2e-lab
  - H4-hook-common
  - H5-features-pattern
  - H6-barrido-artefactos
---

# Ejecución — Kaizen higiene ficheros temporales

## Barrido inicial

Eliminados del working tree (untracked / huérfanos):

- `SddIA/tools/eda-e2e-tool-*.md` (5 ficheros)
- `docs/features/vanguardia-soberania-local/_close-cycle-accept-pr.json`
- `docs/features/pull-request-automation-dlt/_pr36-review-accept.json`

## Verificación E2E lab

```powershell
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
git status --short SddIA/tools/ .SddIA/tools/
```

Esperado: `success: true`, `cleaned: true`, sin nuevos paths bajo `SddIA/tools/`.

## Verificación helper tmp

```powershell
python -c "from pathlib import Path; import sys; sys.path.insert(0,'SddIA/scripts/qa'); from tmp_paths import write_ephemeral_json, cleanup_path, repo_tmp_dir; r=Path('.'); p=write_ephemeral_json(r,'smoke',{'a':1}); print(p); cleanup_path(p); print(repo_tmp_dir(r))"
```

## Cierre de entrega

Inputs efímeros para `delivery-close-cycle` bajo `.tmp/` (no en `persist_ref`).
