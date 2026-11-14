---
uuid: "a8f1c2d3-4e5f-6789-a0b1-c2d3e4f56789"
name: man-43-o10-contract-schema
version: "1.0.0"
audit_date: "2026-11-14"
scope: "O10 contract_schema + C-4 taxonomía 2.0.0"
verdict: "NO_APTO_TOTAL"
---

# MAN-43 O10 — contrato `contract_schema` y taxonomía

## Veredicto

**NO_APTO_TOTAL** (AC-13 sigue abierto por C-3 / C-5).

## Evidencia C-2

- `load_binding_rows` lee `contract_schema`; `contract_schema_file` con fallback y veto de path.
- Validador y gate usan nombre declarado en binding (`eda.event_manager.v1.schema.json`).
- Test `core_bindings_eda_event_manager_resolves_v1_schema`.

## Evidencia C-4

- `capability-taxonomy.md` v1.0.12; catálogo y cuerpo `eda:event-manager` **2.0.0**.
- **Honestidad O10:** cuerpo aplicado con edit IDE; O11 selló `hash_refresh` vía `entity-manager` sin re-parche bifásico (ver `man-43-o11-remediation-20261114.md`).

## Residual

- C-3: contenido del schema JSON aún v1.2.0.
- C-5: `route-domain-event.outputs` cabeceras.