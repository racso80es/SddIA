---
feature_name: pr-presented-orchestration
created: "2026-05-20"
process: feature
---

# Ejecución — Orquestación fractal PR presentado

## Laboratorio (sin `gh` ni push reales)

```powershell
$env:SDDIA_LAB_SKIP_SNAPSHOT = "1"
$env:SDDIA_LAB_SKIP_GIT_PUSH = "1"
$env:SDDIA_LAB_SIMULATE_GH_PR = "1"
$env:SDDIA_LAB_SKIP_HIGIENE = "1"
$env:SDDIA_LAB_SIMULATE_IOTA = "1"

python SddIA/scripts/qa/execute-process.py --process delivery-close-cycle --inputs-file docs/features/pr-presented-orchestration/_smoke-close-cycle-presented.json

python SddIA/scripts/qa/event-watcher.py --once
```

El JSON de smoke fija `pr_url` para la fase «Apertura en forja»; el sello usa `emitter_agent: delivery-close-cycle` y persiste `pr_url` en el payload ECST.

## Producción — PR #11

| Paso | Comando / resultado |
|------|---------------------|
| Push | `git push -u origin feat/pr-presented-orchestration` |
| PR | `gh pr create` → https://github.com/racso80es/SddIA/pull/11 |
| Sello | `execute-process.py --process delivery-close-cycle --inputs-file _delivery-close-pr11.json` |
| Watcher | `event-watcher.py --once` → `processed/` + IOTA |

## Producción (operador)

Orquestar vía proceso padre (`feature` → fase Cierre de entrega → `delivery-close-cycle`) sin `gh` suelto en runbooks. Push y `gh pr create` ocurren dentro del handler del proceso cuando no están activas las variables de simulación del laboratorio.
