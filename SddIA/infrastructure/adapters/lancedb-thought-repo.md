---
id: lancedb-thought-repo
uuid: "0a22c260-2c5a-4aaa-a632-2c9a78e983e4"
type: infrastructure-adapter
version: "1.0.0"
status: placeholder
crate_name: sddia-infrastructure-lancedb-thought
impl_dir: lancedb_thought_repo
contract: "adapters-contract v1.0.0"
port: "ThoughtGraphRepository"
context: ecosystem-evolution
---

# Adaptador: lancedb-thought-repo

Implementación host del puerto `ThoughtGraphRepository` (`SddIA/core/memory/`) sobre LanceDB.

## Estado actual

**Placeholder.** El crate `lancedb_thought_repo` devuelve éxito vacío (`Ok(None)`, `Ok(vec![])`) sin dependencia del crate `lancedb`. La integración física corresponde a `PBI-CORE-LANCEDB-REAL-001`.

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-lancedb-thought` |
| Directorio | `SddIA/infrastructure/adapters/lancedb_thought_repo/` |
| Substrato | Host nativo (no WASI) |
