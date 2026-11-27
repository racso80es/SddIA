---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:c31248c5458c4e17535ed3118da553315612e2b8976f5e3003e82e57fc13cb8d
inputs:
- 'event_uri: Ruta de la instancia ECST User_Preference_Change_Requested'
name: user-preference-ingest
outputs:
- recorded: true si se persistió una revisión
phases:
- intent: Validar payload ECST; descartar campos FORBIDDEN.
  name: Gate
  id: gate
- intent: Destilar UserPreference o IGNORE desde hints estructurados.
  name: Destilar
  id: destilar
- intent: Persistir revisión en store local vía memory:pref-write.
  name: Persistir
  requires_capability:
  - contract: memory.pref_write
    id: memory:pref-write
    version: '>=1.0.0'
  id: persistir
- intent: Emitir User_Preference_Changed sin value sensible.
  name: Sellar
  id: sellar
uuid: d4e5f6a7-b8c9-4d0e-1f2a-3b4c5ab005
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# user-preference-ingest

Consume `User_Preference_Change_Requested`. Handler nativo `user-preference-ingest-core` en `execute-process`.

```bash
./sddia-run.sh --process user-preference-ingest \
  --inputs '{"event_uri":"eda://domain/<event_id>"}'
```

Prohibido anclaje DLT. Inferencias quedan `proposed` hasta confirmación explícita.
