---
context:
- quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:2450e36cabfe73ebd61ba86f8b7971da9e1d921d212a08eb6d822fd06568ac4b"
inputs: []
minteo_maximo: null
name: workspace-smoke
outputs:
- workspace_verified: Indica que el marker fue escrito en el workspace inyectado
phases:
- intent: Escribir marker `.workspace_ok` en workspace_path inyectado por CLI (proceso no ligado a feature/fix).
  name: Verificación de workspace
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
  id: verificacion_de_workspace
porcentaje_de_exito: null
uuid: c4e8a1b2-3f5d-4a9c-8e7b-2d1f0a9b6c3e
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# workspace-smoke

## Propósito

Smoke de **AC2.1** (telemetría reactiva EDA, fase 2): demostrar que `execute-process` instancia `workspace_template` bajo `.SddIA/workspaces/{process_name}/{execution_id}/` para procesos que no son `feature`/`bug-fix`, sin acoplar rutas `docs/features/{slug}`.

## Orquestación

- Fase `Verificación de workspace` (`id: verificacion_de_workspace`): escribe el marcador `.workspace_ok` en el `workspace_path` inyectado; declara `requires_capability` `fs:persist` (`qa` / persistencia atómica vía runtime).
- Entradas: ninguna (`inputs: []`); salida de negocio: `workspace_verified`.

## Invocación de laboratorio

Documentado en `SddIA/scripts/build-release-bundle.sh` y features `telemetria-reactiva-eda-fase2`: `./sddia-run.sh --process workspace-smoke --inputs '{}'`. Cada ejecución CLI emite peaje termodinámico (`Raw_Execution_Finished`) coherente con AC3.1.
