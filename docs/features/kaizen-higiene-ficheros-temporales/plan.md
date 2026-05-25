---
feature_name: kaizen-higiene-ficheros-temporales
created: "2026-05-25"
process: feature
---

# Plan — Kaizen higiene ficheros temporales

| Hito | Entregable | Estado |
|------|------------|--------|
| H0 | PBI + `objectives` / `clarify` / `spec` / `plan` | [x] |
| H1 | § norma efímeros (`git-operations.md`) | [x] |
| H2 | `tmp_paths.py` + `lab_teardown.py` | [x] |
| H3 | `run-eda-e2e-lab.py` scope local + teardown | [x] |
| H4 | `hook_common.py` → `.tmp/` + cleanup | [x] |
| H5 | `features-documentation-pattern` § inputs efímeros | [x] |
| H6 | Barrido artefactos incidente | [x] |
| H7 | `implementation.md` / `execution.md` / `validacion.md` + PBI `done/` | [x] |
| H8 | Un solo PR + `delivery-close-cycle` | [ ] |

## Orden Tekton

H1 → H2 → (H3 ∥ H4) → H5 → H6 → H7 → H8

## Touchpoints código

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/tmp_paths.py` | Nuevo helper |
| `SddIA/scripts/qa/run-eda-e2e-lab.py` | scope + finally |
| `SddIA/scripts/qa/git-hooks/hook_common.py` | `.tmp/` + unlink |
| `SddIA/norms/git-operations.md` | § efímeros |
| `SddIA/library/norms/features-documentation-pattern.md` | § fixtures |
| `.gitignore` | Confirmar `/.tmp`; nota deprecación `/tmp` |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Smokes que dependían de forge Core persistente | Documentar `SDDIA_KEEP_TMP=1`; smokes de aduana usan `--scan` con backfill conocido |
| Pérdida de payload hook en depuración | Flag explícito `SDDIA_KEEP_TMP` |
| Fixtures históricos en `docs/features/` | No migrar masivamente; solo forward en runbooks nuevos |

## Verificación (pre-`validacion.md`)

```powershell
# E2E lab — sin ruido Core
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
git status --short SddIA/tools/

# Hook simulado — payload no persiste (tras H4)
# (smoke dedicado o inspección manual de .tmp/)

# Barrido
git status --short
```
