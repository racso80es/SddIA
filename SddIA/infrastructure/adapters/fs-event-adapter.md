---
id: fs-event-adapter
uuid: "e3a7b0c1-2d3e-4f50-96a5-b4c3d2e1f0a9"
type: infrastructure-adapter
version: "1.0.0"
status: active
crate_name: sddia-infrastructure-fs-event
impl_dir: fs_event_adapter
contract: "adapters-contract v1.0.0"
port: "EventManagerPort"
context: ecosystem-evolution
---

# Adaptador: fs-event-adapter

Implementación de `EventManagerPort` sobre árbol de ficheros (write-temp + rename; pending / leases / deliveries / dlq / traces).

## Estado

**Active.** Sin dependencia `redb`.

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-fs-event` |
| Directorio | `SddIA/infrastructure/adapters/fs_event_adapter/` |
| Contrato | `eda.event_manager` v1.0.0 |
