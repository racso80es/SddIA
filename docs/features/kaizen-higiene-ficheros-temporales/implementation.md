---
feature_name: kaizen-higiene-ficheros-temporales
created: "2026-05-25"
process: feature
items:
  - id: H1-norma
    touchpoint: SddIA/norms/git-operations.md
    status: done
  - id: H2-tmp-paths
    touchpoint: SddIA/scripts/qa/tmp_paths.py
    status: done
  - id: H2-lab-teardown
    touchpoint: SddIA/scripts/qa/lab_teardown.py
    status: done
  - id: H3-e2e-lab
    touchpoint: SddIA/scripts/qa/run-eda-e2e-lab.py
    status: done
  - id: H4-hooks
    touchpoint: SddIA/scripts/qa/git-hooks/hook_common.py
    status: done
  - id: H5-pattern
    touchpoint: SddIA/library/norms/features-documentation-pattern.md
    status: done
  - id: H6-barrido
    touchpoint: working tree
    status: done
---

# Implementación — Kaizen higiene ficheros temporales

## Touchpoints

| ID | Archivo | Cambio |
|----|---------|--------|
| H1 | `git-operations.md` v1.1.0 | §3 taxonomía efímero/fixture; SSOT `.tmp/` |
| H2 | `tmp_paths.py` | `write_ephemeral_json`, `cleanup_path`, `keep_tmp` |
| H2 | `lab_teardown.py` | Teardown forge local + bus EDA + barrido Core huérfano |
| H3 | `run-eda-e2e-lab.py` | `scope: local`; `finally` teardown; barrido Core legacy |
| H4 | `hook_common.py` | Payload en `.tmp/`; `cleanup_path` post-invoke |
| H5 | `features-documentation-pattern` v1.2.1 | § Artefactos efímeros |
| H6 | Barrido | Eliminados `eda-e2e-tool-*.md` Core y JSON one-shot sueltos |

## Patrón de referencia

`run-iota-ci-smoke.py` — `_cleanup_smoke_artifacts` en `finally` (reutilizado vía `lab_teardown.cleanup_eda_bus_event`).
