---
feature_name: pbi-005-hito3-ola-b
created: "2026-05-20"
process: feature
items_applied:
  - hook_common.py
  - pre_push_gate.py
  - post_merge_gate.py
  - pre-push
  - post-merge
  - install-hooks.ps1
  - install-hooks.sh
  - execute_process_capsules accept-pr capsule
  - execute-action traceability_anomaly
  - evolution git-hooks-ca3-ola-b-contract.md
---

# Ejecución — Ola B

## Fase implementación (2026-05-20)

| # | Ítem | Estado |
|---|------|--------|
| 1 | Evolution H3.1 | ✅ |
| 2 | `hook_common.py` + gates + shells | ✅ |
| 3 | Instalador dinámico O5 | ✅ |
| 4 | Cápsula `accept-pr` + O4 | ✅ |
| 5 | Smoke local (sin push remoto) | ✅ `validacion.md` V-B0–V-B8 |

## Comandos smoke local

```powershell
# O3 — guarda main
@("refs/heads/main deadbeef refs/heads/main cafebabe") -join "`n" | python SddIA/scripts/qa/git-hooks/pre_push_gate.py
# exit 1 + mensaje soberanía

# O2 — persist_ref heurística
python -c "import sys; sys.path.insert(0,'SddIA/scripts/qa/git-hooks'); import hook_common as h; print(h.resolve_persist_ref('feat/pbi-005-hito3-ola-b'))"
# docs/features/pbi-005-hito3-ola-b

# accept-pr cápsula (sello simulado — requiere HEAD en main con merge)
# $env:SDDIA_LAB_SKIP_GIT_PUSH='1'
# python SddIA/scripts/qa/execute-process.py --process accept-pr --inputs-file tmp/test-accept-pr.json
```

## Pendiente cierre

- Smoke H3.5 con push/merge real y `event_ids` en `validacion.md`
- PBI operativo v1.5.0 tras APTO Argos
- PR vía `delivery-close-cycle` + merge `accept-pr`
