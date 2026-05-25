---
document_id: PBI-KAIZEN-TMP-HIGIENE
title: "[Kaizen] Higiene ficheros temporales — .tmp fuera de control y limpieza post-uso"
format: markdown
version: "1.0.0"
created: "2026-05-25"
status: listo_para_merge
priority: alta
process: feature
feature_ref_target: docs/features/kaizen-higiene-ficheros-temporales
related:
  - SddIA/scripts/qa/run-eda-e2e-lab.py
  - SddIA/scripts/qa/run-iota-ci-smoke.py
  - SddIA/scripts/qa/git-hooks/hook_common.py
  - SddIA/scripts/qa/tmp_paths.py
  - .gitignore
  - SddIA/library/norms/features-documentation-pattern.md
incident_ref: "Ruido git: eda-e2e-tool-*.md en SddIA/tools/; _close-cycle-*.json sueltos en docs/features/"
---

# [Kaizen] Higiene ficheros temporales

## 0. Mandato

Iniciar como **`feature`** bajo `docs/features/kaizen-higiene-ficheros-temporales/`.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **SSOT carpeta efímera** | Norma + Cúmulo: artefactos runtime solo bajo `.tmp/` (gitignored); prohibido inputs efímeros bajo `docs/features/` o forja lab en Core |
| **O2** | **E2E lab sin ruido genómico** | `run-eda-e2e-lab.py` forja con `scope: local` y limpia entidad + bus en `finally` |
| **O3** | **Hooks y smokes autolimpiantes** | Payload JSON de hooks borrado tras `invoke_process`; patrón `run-iota-ci-smoke.py` |
| **O4** | **Distinción fixture vs runtime** | Fixtures `_smoke-*` versionados vs one-shot en `.tmp/` |
| **O5** | **Barrido inicial** | Working tree sin artefactos huérfanos del incidente |

## 1. Incidente

| Campo | Valor |
|-------|--------|
| Síntoma | `git status` muestra decenas de paths untracked tras smokes y cierres de ciclo |
| Ejemplos | `SddIA/tools/eda-e2e-tool-3c73ab33.md`, `docs/features/vanguardia-soberania-local/_close-cycle-accept-pr.json` |
| Causa raíz | E2E forja Core sin teardown; inputs JSON en `persist_ref`; inconsistencia `tmp/` vs `.tmp/` |

## 2. Diseño objetivo (laudo)

```text
Efímero = .tmp/<contexto>-<uuid>.json  → gitignored → borrado en finally
Fixture = docs/features/<feat>/_smoke-*.json  → plantilla versionada
Lab forge = scope local (.SddIA/) + teardown
```

## 3. Entrega

Ver `docs/features/kaizen-higiene-ficheros-temporales/validacion.md` — **APTO**.
