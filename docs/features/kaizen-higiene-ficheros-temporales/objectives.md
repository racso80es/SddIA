---
feature_name: kaizen-higiene-ficheros-temporales
created: "2026-05-25"
process: feature
branch_name: feat/kaizen-higiene-ficheros-temporales
persist_ref: docs/features/kaizen-higiene-ficheros-temporales
pbi_ref: docs/todos/pending/[Kaizen] higiene ficheros temporales — .tmp fuera de control y limpieza post-uso.md
---

# Objetivos — Kaizen higiene ficheros temporales

## Misión

Reducir el **ruido en el working tree** causado por artefactos efímeros de laboratorio, hooks y cierres de ciclo: centralizarlos en `.tmp/` (fuera de control de versiones) o eliminarlos automáticamente al cumplir su finalidad.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| `.gitignore` ya incluye `/.tmp` y `/tmp` | Existe barrera parcial; uso inconsistente entre scripts |
| `hook_common.py` escribe en `tmp/` (sin punto) | Funciona con `/tmp` ignorado, pero diverge de runbooks que citan `.tmp/` |
| `run-eda-e2e-lab.py` forja entidades en Core | Genera `SddIA/tools/eda-e2e-tool-*.md` huérfanos visibles en `git status` |
| Smokes documentados con `--inputs-file docs/features/.../_close-cycle-*.json` | Inputs one-shot quedan en carpetas de feature y aparecen como untracked |
| `run-iota-ci-smoke.py` ya limpia bus post-smoke | Patrón de referencia para otros scripts QA |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | **Norma SSOT `.tmp/`** | Sección en norma táctica (nueva o `git-operations.md`): clasificación efímero vs fixture; ruta canónica `.tmp/` |
| **O2** | **E2E lab local + teardown** | `run-eda-e2e-lab.py`: `semantic_seed.scope: local`; bloque `finally` elimina `.SddIA/tools/eda-e2e-*`, eventos bus y fila índice si aplica |
| **O3** | **Hooks autolimpiantes** | `hook_common.invoke_process`: borrar payload tras ejecución (salvo `SDDIA_KEEP_TMP=1`) |
| **O4** | **Helper compartido** | `tmp_paths.py` (o módulo en `eda_bus_utils`): `write_ephemeral_json()`, `repo_tmp_dir()` usado por hooks y smokes |
| **O5** | **Patrón documental** | `features-documentation-pattern`: § inputs efímeros; runbooks forward usan `.tmp/` para one-shot |
| **O6** | **Barrido inicial** | Working tree sin los `eda-e2e-tool-*.md` y JSON one-shot del incidente (vanguardia `_close-cycle-accept-pr.json`, etc.) |

## No objetivos

- Reescribir todos los `execution.md` históricos de features cerradas.
- Prohibir fixtures `_smoke-*` versionados cuando son plantillas reproducibles.
- Gate Argos en `delivery-close-cycle` por ficheros `.tmp/` (carpeta ya ignorada).

## Ley aplicada

- `features-documentation-pattern` v1.2.0
- Proceso `feature` v1.3.0
- Principio Kaizen / Cúmulo: deuda operativa → norma + código mínimo
