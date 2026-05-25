---
feature_name: kaizen-higiene-ficheros-temporales
created: "2026-05-25"
process: feature
base: main
scope: kaizen-higiene-ficheros-temporales
version_spec: "1.0.0"
---

# Especificación — Kaizen higiene ficheros temporales

## Hito 1 — Norma táctica (§ Efímeros)

Añadir sección **«Artefactos efímeros y fixtures»** en `SddIA/norms/git-operations.md` (o norma dedicada si Argos lo exige):

- Ruta canónica: `.tmp/` en raíz del workspace.
- Prohibido escribir inputs JSON one-shot bajo `docs/features/<persist_ref>/` salvo fixtures `_smoke-*` plantilla.
- Prohibido forja lab en `SddIA/` (Core) sin evento productivo y cierre documentado.
- Referencia a `SDDIA_KEEP_TMP` para depuración.

## Hito 2 — Módulo helper `tmp_paths.py`

| Función | Contrato |
|---------|----------|
| `repo_tmp_dir(repo: Path) -> Path` | Crea `.tmp/` si no existe; retorna path absoluto |
| `write_ephemeral_json(repo, prefix, payload) -> Path` | `{prefix}-{uuid12}.json` bajo `.tmp/` |
| `cleanup_path(path, *, keep: bool)` | `unlink(missing_ok=True)` salvo `keep=True` |

Consumidores iniciales: `hook_common.py`, `run-eda-e2e-lab.py`.

## Hito 3 — `run-eda-e2e-lab.py`

| Cambio | Detalle |
|--------|---------|
| `semantic_seed.scope` | `"local"` en payload `entity-manager` |
| Prefijo entidad | Mantener `eda-e2e-{class}-{hex8}` |
| Teardown `finally` | Eliminar `.SddIA/tools/{name}.md`, fila en `.SddIA/tools/index.md` si insertada, artefactos bus del `event_id`, witnesses |
| Report | Campo `cleaned: true/false` en JSON de salida |

Regresión: exit 0 con `--json`; `git status` sin nuevos paths bajo `SddIA/tools/` tras ejecución.

## Hito 4 — `hook_common.py`

| Cambio | Detalle |
|--------|---------|
| `TMP_DIR` | `REPO / ".tmp"` |
| `invoke_process` | `try/finally`: `cleanup_path(payload_path)` salvo `SDDIA_KEEP_TMP` |
| Compat | `/tmp` permanece en `.gitignore` una release |

## Hito 5 — `features-documentation-pattern`

Añadir fila o § breve:

| Tipo | Convención |
|------|------------|
| Fixture smoke | `_smoke-<escenario>.json` en `persist_ref` — plantilla versionada |
| Input operativo | `.tmp/<proceso>-<id>.json` — prohibido commitear |
| Cierre ciclo | Documentar comando con copia a `.tmp/`, no path bajo `persist_ref` |

## Hito 6 — Barrido y documentación de feature

1. Eliminar del disco (no commitear):
   - `SddIA/tools/eda-e2e-tool-*.md` (5 ficheros conocidos)
   - `docs/features/vanguardia-soberania-local/_close-cycle-accept-pr.json`
   - Otros JSON one-shot untracked listados en `execution.md` de features activas (evaluar caso a caso)
2. Entregar `implementation.md`, `execution.md`, `validacion.md` en este kaizen.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Norma publicada con taxonomía efímero/fixture |
| CA2 | `run-eda-e2e-lab.py` deja working tree limpio de forges Core |
| CA3 | Hook borra payload `.tmp/` tras invocación |
| CA4 | Helper reutilizable en `SddIA/scripts/qa/` |
| CA5 | Barrido inicial ejecutado; evidencia en `validacion.md` |
| CA6 | PBI en `done/` + `validacion.md` APTO en un único PR |
