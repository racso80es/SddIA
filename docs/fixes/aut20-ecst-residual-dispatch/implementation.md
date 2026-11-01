---
feature_name: aut20-ecst-residual-dispatch
created: "2026-11-01"
tracker_ref: AUT-58
---

# Implementación — AUT-58

- `assemble_domain_process_subscriber_inputs` + `AssembleProcessInputs` en `route_domain_core.rs`.
- `dispatch_subscriber` delega ensamblado; `pull-request-review` conserva precheck y dedupe post-ensamblado.
- SDLC (`Kalma2_Process_Requested`, `Aiua_Process_Requested`): ECST fusionado en inputs de cola.
- `telegram-fallback-responder`: ECST + `text` / `chat_id`.
