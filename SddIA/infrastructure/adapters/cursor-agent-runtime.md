---
id: cursor-agent-runtime
uuid: "7a11c010-2026-4122-9101-000000000001"
type: infrastructure-adapter
version: "1.0.0"
status: active
crate_name: execute-process
impl_dir: cursor_agent_runtime
contract: "adapters-contract v1.0.0"
port: "AgentRuntimePort"
context: ecosystem-evolution
provider_id: "cursor"
transport_mode: "headless_cli"
supports_unattended: true
---

# Adaptador: cursor-agent-runtime

Adaptador DI de `AgentRuntimePort` (`agent.runtime` v1.0.0) para el proveedor `cursor`. Soporta ejecución desatendida (`headless_cli` vía comando configurado) y modo `interactive_relay`. No contiene slugs comerciales de modelos en el Genoma; la resolución `thinking_profile -> modelo` se delega a `.SddIA/llm-runtime.local.json` o variables de bóveda `.SddIA/.dev/.env`.

## Estado

**Active.** Enlazado en `execute-process` (`AgentRuntimeDiFactory`, rol `cursor`).

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `execute-process` |
| Directorio | `SddIA/infrastructure/adapters/cursor_agent_runtime/` |
| Contrato | `agent.runtime` v1.0.0 |
