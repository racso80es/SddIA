---
id: lancedb-evolution-repo
uuid: "ab9bef02-c2c1-426b-a2b2-ca1cc170f21c"
type: infrastructure-adapter
version: "1.0.0"
status: placeholder
crate_name: sddia-infrastructure-lancedb-evolution
impl_dir: lancedb_evolution_repo
contract: "adapters-contract v1.0.0"
port: "EvolutionStore"
context: ecosystem-evolution
---

# Adaptador: lancedb-evolution-repo

Implementación host del puerto `EvolutionStore` (`SddIA/core/memory/`) para eventos de evolución vectorial.

## Estado actual

**Placeholder.** El crate persiste JSON bajo `.SddIA/vector_store/evolution/` sin tabla LanceDB real. La integración física corresponde a `PBI-CORE-LANCEDB-REAL-001`.

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-lancedb-evolution` |
| Directorio | `SddIA/infrastructure/adapters/lancedb_evolution_repo/` |
| Substrato | Host nativo (no WASI) |
