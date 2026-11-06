---
id: external-capsule-event-adapter
uuid: "d2c6a9b8-1f0e-4d30-85a4-c3b2a1098765"
type: infrastructure-adapter
version: "1.0.0"
status: active
crate_name: sddia-infrastructure-external-capsule-event
impl_dir: external_capsule_event_adapter
contract: "adapters-contract v1.0.0"
port: "EventManagerPort"
context: ecosystem-evolution
---

# Adaptador: external-capsule-event-adapter

Puente NFT-ready: proceso hijo JSON por stdin/stdout; variable `SDDIA_BYPASS_THERMO_TOLL=1` en el hijo. Binario de laboratorio: `eda-event-manager-capsule-dummy`.

## Estado

**Active.** Suite E2E en `tests/e2e_dummy.rs` (CARGO_BIN_EXE).

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-external-capsule-event` |
| Directorio | `SddIA/infrastructure/adapters/external_capsule_event_adapter/` |
| Contrato | `eda.event_manager` v1.0.0 |
