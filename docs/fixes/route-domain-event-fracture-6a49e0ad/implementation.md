---
feature_name: route-domain-event-fracture-6a49e0ad
created: "2026-08-28"
process: bug-fix
persist_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
document_id: PBI-FIX-FRACTURE-6a49e0ad310e
---

# Implementation — route-domain-event fractura 6a49e0ad310e

## Cambios

| Artefacto | Acción |
|-----------|--------|
| `.cursor/rules/kintsugi-fracture-protocol.mdc` | **Nuevo** — `alwaysApply: true`; paridad `obediencia-procesos.md` § Escalado ante fallo |
| `docs/todos/pending/[FIX] route-domain-event — …` | **Eliminar** — copia stale |
| `docs/todos/done/[FIX] route-domain-event — …` | **Actualizar** — remediación `prompt_adjustment` + referencia a este fix |

## Evidencia física preexistente (main)

- `SddIA/daemons/iota-publish-relay.md` + supervisor Rust
- `route_domain_core.rs`: `emit_dlt_batch_fracture`, cola `dlt_reanchor`, tests `emit_dlt_batch_fracture_writes_pending`
- Merge Kaizen: `ecd84387db7408e46de6a153de799b5505f32b06` (PR #208)
