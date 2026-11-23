---
id: antigravity-agent-runtime
uuid: "7a11c010-2026-4122-9101-000000000002"
type: infrastructure-adapter
version: "1.0.0"
status: active
crate_name: execute-process
impl_dir: antigravity_agent_runtime
contract: "adapters-contract v1.0.0"
port: "AgentRuntimePort"
context: ecosystem-evolution
provider_id: "antigravity"
transport_mode: "interactive_relay"
supports_unattended: false
---

# Adaptador: antigravity-agent-runtime

Adaptador DI de `AgentRuntimePort` (`agent.runtime` v1.0.0) para el proveedor `antigravity`. Opera por defecto en modo `interactive_relay` con pausa controlada (`awaiting_agents`) o fallo explícito (`PROVIDER_UNATTENDED_UNSUPPORTED`) según `unattended_policy`, y admite comando CLI opcional. Sin slugs comerciales en el Genoma.

## Estado

**Active.** Enlazado en `execute-process` (`AgentRuntimeDiFactory`, rol `antigravity`).

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `execute-process` |
| Directorio | `SddIA/infrastructure/adapters/antigravity_agent_runtime/` |
| Contrato | `agent.runtime` v1.0.0 |
