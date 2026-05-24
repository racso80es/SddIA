---
feature_name: laboratorio-handlers-l2-l3
created: "2026-05-24"
process: feature
branch: feat/laboratorio-handlers-l2-l3
---

# Ejecución — Laboratorio handlers L.2 + L.3

## L2 — Impacto skip (`source_process != feature`)

```powershell
$env:SDDIA_LAB_SKIP_SNAPSHOT="1"
$env:SDDIA_LAB_SKIP_GIT_PUSH="1"
$env:SDDIA_LAB_SIMULATE_GH_PR="1"
$env:SDDIA_LAB_SKIP_HIGIENE="1"
Get-Content -Raw docs/features/laboratorio-handlers-l2-l3/_smoke-delivery-close-impact-skip.json | python SddIA/scripts/qa/execute-process.py --process delivery-close-cycle
```

| Campo fase 2 | Valor |
|--------------|-------|
| `handler` | `delivery-impact-assessment` |
| `status` | `skipped` |
| `reason` | `source_process != feature` |

## L2 — Impacto feature (cápsula directa)

```powershell
python -c "import json,sys; sys.path.insert(0,'SddIA/scripts/qa'); from pathlib import Path; from execute_process_capsules import capsule_delivery_impact_assessment; r=capsule_delivery_impact_assessment(Path('.'), {'source_process':'feature','branch_name':'feat/laboratorio-handlers-l2-l3','target_branch':'main'}, {}); print(json.dumps(r))"
```

| Campo | Valor |
|-------|-------|
| `impact` | `none` (rama sin commits delta vs `origin/main`) |
| `handler` | `delivery-impact-assessment` |

## L3 — Proceso `feature` (fases 6–7)

```powershell
$env:SDDIA_LAB_SKIP_DELIVERY_CLOSE="1"
Get-Content -Raw docs/features/laboratorio-handlers-l2-l3/_smoke-feature-lab.json | python SddIA/scripts/qa/execute-process.py --process feature
```

| Fase | `status` | Evidencia |
|------|----------|-----------|
| 2–5 | `simulated` | nota agentes IDE |
| 6 Cierre documental | `executed` | `pbi_path`: `docs/todos/done/_smoke-lab-pbi-archive-test.md` |
| 7 Cierre entrega | `skipped` | `SDDIA_LAB_SKIP_DELIVERY_CLOSE` |

## Integridad genoma

```powershell
python SddIA/scripts/qa/verify-process-integrity.py
```

Resultado: `verify-process-integrity: OK`

## Nota EDA

Smoke `delivery-close-cycle` completo puede bloquear en fase 3 (`orphan_count > 0`) sin manifiesto backfill activo — comportamiento esperado pre-commit.
