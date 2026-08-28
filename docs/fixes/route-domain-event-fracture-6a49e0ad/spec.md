---
feature_name: route-domain-event-fracture-6a49e0ad
created: "2026-08-28"
process: bug-fix
branch_name: fix/route-domain-event-fracture-6a49e0ad
persist_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md
document_id: PBI-FIX-FRACTURE-6a49e0ad310e
uuid: 6a49e0ad310e-0000-4000-8000-000000000001
incident_ref: "System_Fracture_Detected — 6a49e0ad310e"
---

# Spec — route-domain-event fractura 6a49e0ad310e

## Incidente

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Acción | `merkle-batch-preseal` |
| Traza | `F-DLT-RELAY-SIN-SUPERVISOR: merkle-batch-preseal failed: Campo obligatorio ausente o inválido: payload` |

## Causa raíz (dual)

| Capa | Causa | Remediación |
|------|-------|-------------|
| **Física** | Relay IOTA sin centinela supervisado → publisher sin payload válido | Kaizen DLT #208 (`iota-publish-relay`, `route_domain_core.rs`) — **ya en main** |
| **Operativa** | Operador IA continuó entrega sin escalado Kintsugi | Touchpoint `.cursor/rules/kintsugi-fracture-protocol.mdc` (`prompt_adjustment` Mayeuta) |

## Alcance de este fix

1. Materializar regla operador IA (Kintsugi) en `.cursor/rules/`.
2. Purgar copia stale en `docs/todos/pending/` (canónico ya en `done/`).
3. Cierre documental Argos en `{persist_ref}/validacion.md`.

## Fuera de alcance

- Reimplementar relay DLT (merge `ecd84387` en main).
- Mutación genómica adicional.
