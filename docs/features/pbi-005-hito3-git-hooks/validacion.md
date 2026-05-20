---
feature_name: pbi-005-hito3-git-hooks
created: "2026-05-20"
process: feature
branch: feat/pbi-005-hito3-git-hooks
global: apto
---

# Validación — Ola A (pre-commit Argos)

## Checks

| ID | Check | Estado | Evidencia |
|----|-------|--------|-----------|
| V-A1 | `git-hooks/pre-commit` + `pre_commit_gate.py` existen | ✅ | `SddIA/scripts/qa/git-hooks/` |
| V-A2 | `verify-process-integrity.py` → OK | ✅ | salida `verify-process-integrity: OK` |
| V-A3 | `audit-entity-eda-coverage --scan` → `orphan_count: 0` | ✅ | JSON 2026-05-20 |
| V-A4 | `pre_commit_gate.py` → exit 0 | ✅ | VPI + audit encadenados |
| V-A5 | Criterio **Existencia en Bus** (Fase 1) | ✅ | `--scan` sin `--require-pending-for-staged` |
| V-A6 | `PYTHONIOENCODING` no altera VPI en gate | ✅ | `isolate_stdio` en `pre_commit_gate.py` |
| V-A7 | Instalador documentado | ✅ | `install-hooks.ps1` + `implementation.md` |

## Comandos reproducibles

```powershell
cd C:\Proyectos\SddIA
python SddIA/scripts/qa/verify-process-integrity.py
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
python SddIA/scripts/qa/git-hooks/pre_commit_gate.py
echo $LASTEXITCODE
```

## Smoke bloqueo (manual)

| Escenario | Resultado esperado | Estado |
|-----------|-------------------|--------|
| Commit con `hash_signature` inválido en proceso | `pre_commit_gate` exit 1 | ⏳ manual opcional |
| `SDDIA_SKIP_HOOKS=1` | exit 0 sin QA | ⏳ manual opcional |

## Veredicto Ola A

**APTO** para uso local del gate y documentación. **CA-3** y Hito 3 global permanecen abiertos hasta Ola B (`pre-push` / `post-merge`) y cierre PBI-005.
