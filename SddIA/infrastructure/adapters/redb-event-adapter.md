---
id: redb-event-adapter
uuid: "f4a8b1c2-3d4e-5f60-a7b8-c9d0e1f2a3b4"
type: infrastructure-adapter
version: "1.0.0"
status: active
crate_name: sddia-infrastructure-redb-event
impl_dir: redb_event_adapter
contract: "adapters-contract v1.0.0"
port: "EventManagerPort"
context: ecosystem-evolution
---

# Adaptador: redb-event-adapter

Implementación ACID de `EventManagerPort` sobre `redb` (eda.event_manager v1.0.0).

## Estado

**Active.** Dep `redb` acotada a este crate; sin dependencia en `sddia-core`.

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-redb-event` |
| Directorio | `SddIA/infrastructure/adapters/redb_event_adapter/` |
| Contrato | `eda.event_manager` v1.0.0 |
