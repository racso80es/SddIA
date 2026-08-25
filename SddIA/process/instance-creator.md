---
context:
- ecosystem-evolution
- filesystem-ops
- system-operations
contract: process-contract v1.4.0
hash_signature: sha256:4eccbf39f5dacc7c4c02e6b111473d18394e3e3394fc121c6beef9e3c9297a5d
inputs:
- instance_root: Ruta absoluta o relativa (al repo) de la carpeta instancia objetivo
name: instance-creator
outputs:
- instance_root: Ruta resuelta de la instancia
- smoke: Resultado del gate post-ignición
- runtime_profile: Perfil aplicado
phases:
- intent: Crear/verificar arbol .SddIA/; overlay starter-kit; sustituir local.paths.json ausente, vacio o {}
  name: Topologia
- intent: Inyectar secretos desde vault/plantilla sin filtrar a logs
  name: Vault
- intent: Registrar unidades hermeticas WorkingDirectory=%f; @@SDDIA_CORE_ROOT@@ = instance_root no repo CLI
  name: Systemd
- intent: Arrancar daemons segun SDDIA_RUNTIME_PROFILE y jurisdiccion sensorial R-07
  name: Ignicion
- intent: Preflight topologia (overlay no vacio); no emitir Local_QA_Requested; si skip_ignition no exigir route-domain*; si ignicion no skipped, route-domain-event success:true
  name: Smoke
uuid: dead5ca7-c0b9-42ef-aad6-171991fb524f
version: 1.2.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# instance-creator

Despliegue hermético de instancia consumidor: topología `.SddIA/` (starter-kit `local.paths.json`; sustituye stub `{}`), vault, systemd `%f` con `CORE_ROOT=instance_root`, ignición perfilada y smoke de topología **sin** `Local_QA_Requested` (`route-domain-event` si ignición no skipped). Complementa `sync-client-assets`; no inventa CLI `sddia`.

## Invocación

```bash
./sddia-run.sh --process instance-creator --inputs '{
  "instance_root": "/path/to/cliente",
  "runtime_profile": "consumer",
  "vault_source": "/path/to/preprod-vault",
  "skip_ignition": true,
  "skip_smoke": false
}'
```

Inputs opcionales (no en frontmatter obligatorio): `runtime_profile` (default `consumer`), `vault_source`, `skip_smoke`, `skip_ignition`, `correlation_id`.

Handler nativo: `engine/execute-process` → `handlers/instance_creator.rs`.

## Norma

`SddIA/norms/sddia-distribution-protocol.md` v1.2.1 (Vía C + bundle + este proceso).
