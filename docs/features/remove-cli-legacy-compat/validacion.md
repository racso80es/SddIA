---
feature_name: remove-cli-legacy-compat
created: "2026-05-22"
process: feature
branch: feat/remove-cli-legacy-compat-12759319319782396173
pr_url: "https://github.com/racso80es/SddIA/pull/14"
global: true
checks:
  A1: pass
  A2: pass
  A3: pass
  A4: pass
  A5: pass
---

# Validación — remove-cli-legacy-compat

## Criterios spec §4

| ID | Check | Estado | Evidencia |
|----|-------|--------|-----------|
| A1 | `--input-file` en execute-process rechazado | ✅ | `unrecognized arguments: --input-file` |
| A2 | `--action` en execute-process rechazado | ✅ | `unrecognized arguments: --action` |
| A3 | `--input-file` en execute-action rechazado | ✅ | `unrecognized arguments: --input-file` |
| A4 | Scripts `SddIA/scripts/**` sin rutas legacy | ✅ | sin coincidencias en scripts activos |
| A5 | Integridad procesos + CLI canónico | ✅ | `verify-process-integrity: OK` |

## Comandos reproducibles

```powershell
cd C:\Proyectos\SddIA
$env:PYTHONIOENCODING = 'utf-8'

# A1–A3 (deben fallar parseo)
python SddIA/scripts/qa/execute-process.py --input-file tmp/x.json 2>&1
python SddIA/scripts/qa/execute-process.py --action emit-pr-presented-event --inputs '{}' 2>&1
python SddIA/scripts/qa/execute-action.py --action sync-entity-index --input-file tmp/x.json 2>&1

# A4
rg "execute-process\.py --(input-file|action)" SddIA/scripts

# A5
python SddIA/scripts/qa/execute-process.py --process feature --inputs '{"feature_name":"smoke-cli","base_branch":"main"}'
python SddIA/scripts/qa/verify-process-integrity.py
```

## Aduana PR (#14)

| Campo | Valor |
|-------|-------|
| Perfil | `SDDIA_ENV=production` |
| Proceso | `pull-request-review` v2 |
| Veredicto | `aprobado` |
| `delivery_state` | `success` |
| Triaje documental | OK (`spec`, `plan`, `implementation`, `objectives`) |
| Triaje técnico | OK (`verify-process-integrity`) |
| Handoff | `accept-pr` ejecutado (merge local `b44402bd`; huérfano sin Presented previo en bus) |
| Evento | `PullRequest_Merged` pending `5f47474f-4699-4afa-aa89-750cc28bf4d8` |
