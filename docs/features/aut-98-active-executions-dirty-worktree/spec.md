---

feature_name: aut-98-active-executions-dirty-worktree
created: "2026-11-22"
process: feature
base: main
scope: execute-process/workspace_init + .gitignore + fracture-corpus
execution_id: "bf9e18c4-3ef6-490b-852b-fdfb688d9e04"
---
# Especificación técnica — aut-98-active-executions-dirty-worktree

## 1. `SddIA/engine/execute-process/src/engine/workspace_init.rs`

En `path_in_scope(...)`:
- Añadir comprobación explícita antes del `false` final (o al inicio tras normalizar `./`):
  - `.SddIA/state/active-executions.json`
  - `.SddIA/state/active-executions.json.tmp`
- Además, si `git status` reporta el directorio padre `.SddIA/state/` o `.SddIA/` como untracked cuando solo contiene estado runtime permitido, `path_in_scope` no debe bloquear por `.SddIA/state/active-executions.json` ni `.SddIA/state/active-executions.json.tmp`.

## 2. `.gitignore`

En la sección `# Registro saliente anti-eco Linear` / estado `.SddIA/state/`:
```gitignore
.SddIA/state/active-executions.json
.SddIA/state/active-executions.json.tmp
```

## 3. Fixture `fracture-corpus`

Crear `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/5f46810a-workspace-init-active-executions.json` documentando el incidente `AUT-98` (`trace_hash: 5f46810af5ec`, `unclassified` sin firma dedicada previa).
