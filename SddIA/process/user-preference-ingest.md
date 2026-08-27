---
context:
- ecosystem-evolution
- knowledge-management
contract: process-contract v1.4.0
hash_signature: "sha256:7316a46943d11718206aa26bc5c9228bb4abdacf4e0049fd3e7b8d4868a2e722"
inputs:
- event_file_path: Ruta de la instancia ECST User_Preference_Change_Requested
name: user-preference-ingest
outputs:
- recorded: true si se persistió una revisión
phases:
- intent: Validar payload ECST; descartar campos FORBIDDEN.
  name: Gate
- intent: Destilar UserPreference o IGNORE desde hints estructurados.
  name: Destilar
- intent: Persistir revisión en store local vía memory:pref-write.
  name: Persistir
  requires_capability:
  - contract: memory.pref_write
    id: memory:pref-write
    version: '>=1.0.0'
- intent: Emitir User_Preference_Changed sin value sensible.
  name: Sellar
uuid: d4e5f6a7-b8c9-4d0e-1f2a-3b4c5ab005
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# user-preference-ingest

Consume `User_Preference_Change_Requested`. Handler nativo `user-preference-ingest-core` en `execute-process`.

```bash
./sddia-run.sh --process user-preference-ingest \
  --inputs '{"event_file_path":".events/domain/<event_id>.json"}'
```

Prohibido anclaje DLT. Inferencias quedan `proposed` hasta confirmación explícita.
