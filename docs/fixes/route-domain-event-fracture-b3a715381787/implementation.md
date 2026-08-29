---
feature_name: route-domain-event-fracture-b3a715381787
created: "2026-08-29"
process: bug-fix
branch_name: fix/route-domain-event-fracture-b3a715381787
persist_ref: docs/fixes/route-domain-event-fracture-b3a715381787
items:
  - iota-immutable-publisher/publish_via_relay
  - route_domain_core/classify_batch_anchor_friction
  - entity-manager/seal-anchor
---

# Implementation — fractura `b3a715381787`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/tools/iota-immutable-publisher/src/main.rs` | `ureq::Error::Status` → `iota-relay-publish-error: status={n} {detail}`; transporte → `iota-relay-unreachable` |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | Rama `F-DLT-PUBLISH-ERROR` antes de `SIN-SUPERVISOR` |
| `SddIA/tools/iota-immutable-publisher.md` | `source_sha256` actualizado vía `entity-manager` `seal-anchor` |

## Contrato de prefijos (implementado)

- HTTP no-2xx con cuerpo relay: `iota-relay-publish-error: status=500 …`
- Transporte: `iota-relay-unreachable: …`
- JSON 2xx `success != true`: `iota-relay-publish-error: status=200 …`

## Tests añadidos

- `format_relay_publish_error_includes_status_and_body` (publisher)
- `classify_publish_error_vs_unreachable` (execute-process)
- `emit_dlt_batch_fracture_publish_error_friction` (execute-process)
