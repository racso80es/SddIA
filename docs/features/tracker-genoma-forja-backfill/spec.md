---
feature_name: tracker-genoma-forja-backfill
created: "2026-10-02"
process: refactorization
---

# Especificación

- Invocaciones `entity-manager` con `semantic_seed.hash_refresh_only: true` (vía `--inputs-file`).
- Eventos domain: `event_family: domain` obligatorio.
- Norm `execution-contexts`: sello `emit-domain-mutation` con uuid `d8e9f0a1-b2c3-4d5e-6f7a-8b9c0d1e2f3a` (artefacto en `SddIA/norms/`, sin duplicar en `library/norms/`).
- Forja: `hash_refresh_only` en eventos (`factory.rs`); newline al insertar `hash_signature` (`common.rs`).
