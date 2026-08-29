---
feature_name: route-domain-event-fracture-b3a715381787
created: "2026-08-29"
process: bug-fix
branch_name: fix/route-domain-event-fracture-b3a715381787
persist_ref: docs/fixes/route-domain-event-fracture-b3a715381787
items_applied:
  - spec-plan-commit-36e318a
  - publisher-relay-error-prefixes
  - classify-F-DLT-PUBLISH-ERROR
  - seal-anchor-iota-immutable-publisher
---

# Execution — fractura `b3a715381787`

## Fases ejecutadas

| Fase | Estado | Evidencia |
|------|--------|-----------|
| Diseño (`spec.md`, `plan.md`) | done | commit `36e318a` |
| Publisher prefijos | done | `main.rs` |
| Clasificador fricción | done | `route_domain_core.rs` |
| Tests unitarios | done | ver abajo |
| Diagnóstico instancia (Fase 0) | diferido | relay off; bóveda presente |
| `validacion.md` / cierre documental | done | APTO taxonomía; PBI en `done/` |
| `delivery-close-cycle` | **no invocado** | barrera `simulated` (PBI `c51acf014c0f`) |

## Comandos de verificación

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p iota-immutable-publisher
CARGO_TARGET_DIR=target cargo test -p execute-process classify_publish_error
CARGO_TARGET_DIR=target cargo test -p execute-process emit_dlt_batch_fracture
CARGO_TARGET_DIR=target cargo build --release -p iota-immutable-publisher
./sddia-run.sh --process entity-manager --inputs '{"lifecycle_operation":"seal-anchor","entity_class":"tool","entity_name":"iota-immutable-publisher","semantic_seed":{"profile":"release"}}'
```

## Causa HTTP 500 (Fase 0 — diagnóstico instancia)

| Check | Resultado |
|-------|-----------|
| Bóveda `.SddIA/.dev/.env` | presente: `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`, `SDDIA_LAB_SIMULATE_IOTA` definidos (valores no volcados) |
| Cola `dlt/reanchor-queue` | vacía o sin entradas visibles al momento del diagnóstico |
| Log hijo Node relay | no disponible en workspace (sin daemon activo en sesión) |

**Hipótesis operativa (no confirmada en runtime):** el 500 original probablemente fue fallo SDK/red Testnet o transacción Move (candidatos 2–3 del PBI), no ausencia de wallet — las claves existen en bóveda. La taxonomía corregida permitirá ver el mensaje `error`/`feedback` real en la próxima reproducción sin mimetizar como `SIN-SUPERVISOR`.

## Pendiente

- Publish E2E con relay activo (CA5 operacional).
- PR vía `delivery-close-cycle` cuando exista barrera IDE o agentes ejecutados (PBI `c51acf014c0f`).
