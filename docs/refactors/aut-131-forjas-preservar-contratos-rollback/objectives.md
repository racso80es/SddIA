---
feature_name: aut-131-forjas-preservar-contratos-rollback
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-131-forjas-preservar-contratos-rollback
persist_ref: docs/refactors/aut-131-forjas-preservar-contratos-rollback
tracker_ref: AUT-131
---
# Objetivos — AUT-131

Restablecer la promesa de forja determinista en updates parciales de acciones, índice de acciones, suites y agentes, con rollback atómico y tests ejecutables (KA-FORGE-1..6).

| ID | Objetivo |
| --- | --- |
| O-1 | Preservar contrato de acción en update sin campos de contrato en payload |
| O-2 | Sync de `actions/index.md` sin truncar secciones posteriores a la tabla |
| O-3 | Rollback byte-a-byte de acción + índice intacto ante post-validación fallida |
| O-4 | Suite: YAML + cuerpo aplicados juntos o error explícito con rollback |
| O-5 | Agente: hash coherente en YAML-only; restauración ante error |
| O-6 | Paquete `execute-process` verde bajo filtro `forges` |