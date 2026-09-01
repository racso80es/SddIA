---
context: quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:aff5b43eca671f740382195f8eab70cd3b9462cf398f19fbd96d00a9d159854e"
name: system-vitality-probe
phases:
- intent: Correr bus.topology, cumulo.tools_index, cerbero.config, kalma2.http con causa física.
  name: Ejecutar sondas
- intent: Escribir System_Vitality_Probed en eda_fractal.telemetry.
  name: Emitir telemetría
- intent: Sonda roja → System_Fracture_Detected; reset al volver a verde.
  name: Fractura idempotente
uuid: b215b373-f6d3-4fb1-9d55-60eb260df5cc
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# system-vitality-probe

Sondas deterministas de invariantes no-proceso y HTTP de Kalma2 (`bus.topology`, `cumulo.tools_index`, `cerbero.config`, `kalma2.http`). Emite `System_Vitality_Probed` en `eda_fractal.telemetry` y `System_Fracture_Detected` idempotente por `probe_id` (estado `.SddIA/daemons/state/vitality-probe.json`). Cadencia: sweeper `SDDIA_VITALITY_PROBE_SECONDS` (default 300, piso 30). No alimenta el panel Espejo.
