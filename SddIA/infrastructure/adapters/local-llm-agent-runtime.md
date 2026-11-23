---
id: local-llm-agent-runtime
uuid: "7a11c010-2026-4122-9101-000000000003"
type: infrastructure-adapter
version: "1.0.0"
status: active
crate_name: execute-process
impl_dir: local_llm_agent_runtime
contract: "adapters-contract v1.0.0"
port: "AgentRuntimePort"
context: ecosystem-evolution
provider_id: "local-llm"
transport_mode: "headless_api"
supports_unattended: true
---

# Adaptador: local-llm-agent-runtime

Adaptador DI de `AgentRuntimePort` (`agent.runtime` v1.0.0) para el proveedor `local-llm`. Mantiene preparado el contrato de configuración de instancia (`.SddIA/llm-runtime.local.json`) y emite fallo tipado `PROVIDER_UNAVAILABLE` cuando no existe endpoint o comando local configurado, sin auto-failover a otros proveedores.

## Estado

**Active.** Enlazado en `execute-process` (`AgentRuntimeDiFactory`, rol `local-llm`).

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `execute-process` |
| Directorio | `SddIA/infrastructure/adapters/local_llm_agent_runtime/` |
| Contrato | `agent.runtime` v1.0.0 |
