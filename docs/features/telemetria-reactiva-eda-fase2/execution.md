---
feature_name: telemetria-reactiva-eda-fase2
created: "2026-05-27"
process: feature
items_applied:
  - "2.A process-contract v1.4.0 + workspace_template en procesos"
  - "2.B bootstrap_process_workspace en CLI"
  - "2.C sync_workspace_context agentes"
  - "2.D cumulo.paths v1.1.0 + migración scripts/normas"
  - "2.E workspace-smoke + regresión test_eda_bus_v3plus"
---

# Ejecución — Fase 2

## Aplicado

- `process-contract.md` → v1.4.0 con § Workspace operativo.
- `workspace_template` en 18 procesos Core + nuevo `workspace-smoke.md`.
- `workspace_utils.py`: `load_paths_config`, `materialize_workspace`, `bootstrap_process_workspace`.
- `execute_process_capsules.py`: materializa workspace antes del bucle de fases; expone `workspace_path` y `execution_id` en `data`.
- `cumulo.paths.json` v1.1.0: bloque `paths` (`workspacesRoot`, `featurePath`, `fixPath`).
- `eda_bus_utils.infer_persist_ref_from_branch`: prefijos vía Cúmulo.
- `route_domain_event_core.py`: eliminado fallback `docs/features/remove-cli-legacy-compat`.
- Normas: `paths-via-cumulo.md`, `entidades-dominio-ecosistema-sddia.md`, `touchpoints-ia.md`.
- `.gitignore`: `.SddIA/workspaces/`.

## Smoke AC2.1 / AC2.2

```text
python SddIA/scripts/qa/execute-process.py --process workspace-smoke --inputs "{}"
```

| Ejecución | `execution_id` | Carpeta workspace |
|-----------|----------------|-------------------|
| 1 | `d0aebedd-0800-4d52-9a36-f35137053368` | `.SddIA/workspaces/workspace-smoke/d0aebedd-.../` |
| 2 | `f9fd3542-5cbe-4628-a4f0-eb2c60ca0437` | `.SddIA/workspaces/workspace-smoke/f9fd3542-.../` |

Marker `.workspace_ok` escrito en cada workspace. Sin dependencia de `docs/features/{slug}`.

## AC2.3 (contexto agente)

`sync_workspace_context` propaga `workspace_path` a `process_inputs` en cada fase. Normas actualizadas: agentes limitados al workspace inyectado; emisión ECST formal → Fase 3.

## Regresión

- `test_eda_bus_v3plus.py`: 14 tests OK.

## Pendiente operador

- PR vía `delivery-close-cycle` (Fase 7 proceso feature).
