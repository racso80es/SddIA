---
uuid: "a6f5e0a4-ccc5-4076-bf8a-b60cfe1fa684"
name: "switch-llm-provider"
version: "1.0.1"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "switch_llm_provider"
hash_signature: "sha256:d79d9199148c11beafe8eb34ac500961650a855cfa11f2ad6e4a1b986f87a508"
inputs:
  - "provider": "Proveedor a activar; debe ser rol registrado de agent:runtime (alias active_provider)"
  - "unattended_policy": "Opcional: pause_awaiting_agents | fail_fast"
  - "models": "Opcional: mapa de modelos a fusionar en providers.{provider}.models"
  - "endpoint": "Opcional: endpoint del proveedor"
outputs:
  - "active_provider": "Proveedor activo tras la conmutación"
  - "unattended_policy": "Política desatendida efectiva"
  - "config_path": "Ruta relativa del fichero de runtime local escrito"
---

# Acción: switch-llm-provider

## Propósito

Conmuta de forma explícita el proveedor LLM activo de la instancia escribiendo `active_provider` en el fichero de runtime local (`instance.llm_runtime_local` de `cumulo.paths.json`). No existe auto-failover: el cambio solo ocurre cuando se invoca esta acción.

## Orquestación

1. Lee `provider` (alias `active_provider`) de la entrada; sin valor, falla con `provider requerido`.
2. Comprueba que el proveedor figure entre los roles de `agent:runtime` declarados en `SddIA/core/capability-bindings.md`; si no, falla con `CAPABILITY_BINDING_MISSING`.
3. Carga la configuración local vigente; si no existe, parte de `.SddIA/llm-runtime.example.json` o de un esqueleto `schema_version: 1.0.0`.
4. Fija `active_provider` y `unattended_policy` (`pause_awaiting_agents` por defecto o `fail_fast`), fusiona `models` y `endpoint` opcionales bajo `providers.{provider}`.
5. Escribe en un temporal `*.json.tmp` y lo renombra sobre el destino (escritura atómica).

## Límites

- Solo modifica la configuración LLM local de la instancia; no toca genoma ni índices.
- No valida credenciales ni conectividad del proveedor.
- Handler nativo `agent_runtime::switch_llm_provider` en `execute-process`.
