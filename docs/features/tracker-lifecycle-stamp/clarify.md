---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
process: feature
purpose: Laudos de capacidad, provider resuelto y sumidero file
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
---

# Clarificación — tracker-lifecycle-stamp

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `1d8790f8-abfe-4880-a3a5-ec0c9093812b`. Rama `feat/tracker-lifecycle-stamp`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-PHASE | La fase declara `requires_capability` `tracker:lifecycle` / `tracker.lifecycle` / `>=1.0.0`. No hay `delegates_to`. |
| L-NAME | `lifecycle_tool_name` usa `resolve_phase_bindings` sobre esa capacidad y se queda con el nombre tras `tool:`. |
| L-FILE | Si el índice del `project_slug` apunta a un manifiesto con `tracker.provider: file`, el sello termina en no-op antes de cualquier cápsula. |
| L-CONTEXT | `context` permanece `tracker-operations`. |
| L-FORGE | Update del proceso sin `process_phases`: solo refresca el hash. Versión `1.1.0`. |
