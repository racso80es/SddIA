---
feature_name: kaizen-delivery-close-shell-executor-wasm-fallback
created: "2026-06-11"
process: bug-fix
branch_name: fix/delivery-close-shell-executor-wasm-fallback
persist_ref: docs/fixes/kaizen-delivery-close-shell-executor-wasm-fallback
---

# Implementación — fallback shell-executor nativo

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/execute_process_capsules.py` | Fallback nativo en `invoke_shell_executor`; `_git_diff_name_only` usa `git-manager` |
| `scripts/skills/shell-executor.py` | Nueva cápsula Python (allowlist, anti-git, subprocess sin shell) |
| `scripts/skills/git-manager.py` | Operación `diff_name_only` |

## Lógica

```text
invoke_shell_executor(executable, arguments)
  ├─ wasm + wasmtime disponibles → wasmtime run shell-executor.wasm
  │    └─ error cwd/PATH → fallback Python
  └─ else → python scripts/skills/shell-executor.py (stdin JSON)

_git_diff_name_only(base, head)
  └─ invoke_git_manager("diff_name_only", { ref_spec })
```

## Verificación local

```bash
python3 -c "
import sys; sys.path.insert(0,'SddIA/scripts/qa')
from pathlib import Path
from execute_process_capsules import invoke_shell_executor
print(invoke_shell_executor(Path('.'), 'gh', ['--version']))
"

python3 SddIA/scripts/qa/execute-process.py --process delivery-close-cycle --inputs '{
  "source_process":"bug-fix",
  "branch_name":"fix/delivery-close-shell-executor-wasm-fallback",
  "persist_ref":"docs/fixes/kaizen-delivery-close-shell-executor-wasm-fallback",
  "target_branch":"main"
}'
```
