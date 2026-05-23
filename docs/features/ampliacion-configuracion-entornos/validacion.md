---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
branch: feat/ampliacion-configuracion-entornos
pr_url: https://github.com/racso80es/SddIA/pull/20
global: APTO
pbi_archived: true
merged_pr: 20
merge_commit: f0ef7bf
closed: "2026-05-23"
checks:
  CA-1: pass
  CA-2: pass
  CA-3: pass
  CA-4: pass
  CA-5: pass
  CA-6: pass
  CA-7: pass
  CA-8: pass
  CA-9: pass
  CA-10: pass
  verify-process-integrity: pass
  eda-orphan-scan: pass
git_changes:
  - SddIA/scripts/qa/env_loader.py
  - SddIA/scripts/qa/execute-process.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/daemons/event-watcher.py
  - SddIA/scripts/tools/iota-immutable-publisher/index.ts
  - SddIA/scripts/tools/iota-immutable-publisher/package.json
  - SddIA/core/cumulo.paths.json
  - .gitignore
  - .dev/.env.example
  - README.md
  - SddIA/tools/iota-immutable-publisher.md
  - docs/features/ampliacion-configuracion-entornos/
  - docs/todos/done/AmpliacionConfiguracionEntornos.md
---

# Validación — Jerarquía de Bóvedas (Argos)

**Veredicto global: APTO**

Perfil: `SDDIA_ENV=production` (bóveda global `.dev/.env` + instancia `.SddIA/.dev/.env`).

## Criterios spec §7

| ID | Check | Resultado | Evidencia |
|----|-------|-----------|-----------|
| CA-1 | Merge local > global en dict | ✅ | Smoke `env_loader` temp dir: `A=local` |
| CA-2 | SO no sobrescrito | ✅ | `A=from_so` intacto tras carga |
| CA-3 | Log exacto ambas bóvedas | ✅ | `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env` |
| CA-4 | Arranque sin bóvedas | ✅ | `load_hierarchical_env(tmp)` → `{}` sin error |
| CA-5 | `run_process()` carga env | ✅ | `success: true`; log jerarquía en stderr |
| CA-6 | IOTA vía bóveda instancia | ✅ | Claves en `.SddIA/.dev/.env`; cápsula sin `dotenv`; IOTA físico no forzado en aduana |
| CA-7 | Cero dotenv en tools | ✅ | `rg dotenv.config` → 0 en `SddIA/scripts/tools/` |
| CA-8 | `.gitignore` bóvedas | ✅ | `.dev/`, `.SddIA/.dev/` presentes |
| CA-9 | Cúmulo `env_hierarchy` | ✅ | `python -m json.tool cumulo.paths.json` OK |
| CA-10 | README Jerarquía de Bóvedas | ✅ | Sección § Configuración + desacoplamiento |

## Aduanas transversales

| Check | Resultado |
|-------|-----------|
| `verify-process-integrity.py` | ✅ OK (`PYTHONUTF8=1`) |
| `audit-entity-eda-coverage.py --scan` | ✅ `orphan_count: 0` |
| `execute-action.py` + vault | ✅ `emit-pr-presented-event` → `success: true` |

## Gate Ola A

Hitos **0.1–0.3** APTO → desbloquea pasivos técnicos Ola A dependientes de env.

## Cierre documental

PBI `PBI-AMPLIACION-CONFIGURACION-ENTORNOS` archivado en `docs/todos/done/AmpliacionConfiguracionEntornos.md`; retirado de `pending/` (2026-05-23). PR #20 mergeado en `main`.

## Comandos reproducibles

```powershell
$env:PYTHONUTF8='1'
python SddIA/scripts/qa/verify-process-integrity.py
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
rg "dotenv\.config" SddIA/scripts/tools/
python -m json.tool SddIA/core/cumulo.paths.json
```
