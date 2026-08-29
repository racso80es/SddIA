---
context:
- quality-assurance
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:173309c4b6710ab22401ca3f652c8650d7691c6f886da90c3f28e6bb8454e7e0
inputs:
- sweep: Opcional boolean; reservado para paridad con audit (default true)
- apply: Opcional boolean; si true mueve PBIs pending→done (default false; env SDDIA_PHAGOCYTE_APPLY=1)
minteo_maximo: null
name: phagocyte-recovered-fracture-pbis
outputs:
- candidates: Lista de paths pending candidatos a fagocitosis
- applied: Lista aplicada si apply=true
- skipped_count: Contador de omitidos
phases:
- delegates_to:
  - agent:argos
  intent: Escanear paths.todos.pending; predicado trace_before_lock vs lock.started_at vigente.
  name: Predicado documental
- delegates_to:
  - agent:argos
  intent: Registrar ledger phagocytosed-fractures.json bajo daemons_instance.state.
  name: Ledger instancia
- delegates_to:
  - skill:filesystem-manager
  intent: Si apply, mover pending→done, ola B y manifest (sin delivery-close-cycle).
  name: Aplicar documental
porcentaje_de_exito: null
uuid: e8f2a1b3-5c4d-4e6f-9a0b-1c2d3e4f5a7b
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# phagocyte-recovered-fracture-pbis

Auto-poda de PBIs `PBI-FIX-FRACTURE-*` cuya traza `last_heartbeat` sea anterior al `lock.started_at` vigente con PID vivo. Laudo B automático (`B-automatic-phagocyte`).

```bash
# Dry-run (lista candidatos + ledger)
./sddia-run.sh --process phagocyte-recovered-fracture-pbis --inputs '{"apply":false}'

# Aplicar documental (forja)
SDDIA_PHAGOCYTE_APPLY=1 ./sddia-run.sh --process phagocyte-recovered-fracture-pbis --inputs '{"apply":true}'
```

## Predicado

- `document_id` ~ `PBI-FIX-FRACTURE-*`
- `last_heartbeat` parseado de traza (`last_heartbeat=ISO`)
- Lock vivo + `started_at > last_heartbeat`

## Ledger

`.SddIA/daemons/state/phagocytosed-fractures.json` — fuera de git.

## Límites

* No invoca `delivery-close-cycle` ni push a `main`.
* No muta umbrales ni `heartbeat-audit.json` para limpiar trazas.
* Enganche automático desde `daemon-heartbeat-audit` tras sweep sano (`missed_cycles=0`).
