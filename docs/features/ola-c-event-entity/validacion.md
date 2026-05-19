---
feature_name: ola-c-event-entity
validated_at: "2026-05-19"
validator: "Argos (laboratorio automatizado)"
---

# Validación — Ola C: Evento como entidad de dominio

## Checklist Fase 6

| Check | Resultado | Evidencia |
|-------|:---------:|-----------|
| `cumulo.paths.json` JSON válido | ✅ | Parse OK; `eda_bus` → `docs/events/*`; `directories.events` → `SddIA/events` |
| Sin literales obsoletos `.SddIA/events/pending` en consumidores activos | ✅ | Solo docs `evolution/*-temp.md` (histórico) |
| Genoma: contrato + índice + ≥5 clases | ✅ | `events-contract.md`, `index.md` (5 filas), 5 `{name}.md` |
| Constitución §3.1 Evento de Dominio | ✅ | `CONSTITUTION_CORE.md` Hito 1 |
| `event-creator` + `entity-manager` piloto `event` | ✅ | Commits Fase 2–3 |
| Forense DLT: `merge_commit_hash` REQUIRED; `hash_signature` FORBIDDEN | ✅ | `pull-request-merged.md` + contrato §5.1 |
| Forense genoma: `hash_signature_new` REQUIRED; `payload_schema_hash` OPTIONAL | ✅ | `domain-entity-created.md` + contrato §5.2 |
| Validación instancia ↔ Clase (Paso 2b) | ✅ | `event-watcher.py`; smoke: válido → `processed/`, inválido → `dead-letter/` |
| Plantilla Vía C | ✅ | `SddIA/templates/eda-instance-events/README.md` |
| Bus E2E emit → watcher → terminal | ✅ | `PullRequest_Presented` no-op → `processed/` |

## Smoke tests ejecutados

1. **ECST válido:** `emit-pr-presented-event` + `route-domain-event` → `docs/events/processed/`.
2. **ECST inválido:** `event_type: Unknown_Event` → `docs/events/dead-letter/` con `ecst_errors`.

## Deuda documentada (no bloqueante)

| Ítem | Notas |
|------|-------|
| `payload_schema_hash` REQUIRED | Pendiente cuando emisores Ola A calculen huella; contrato marca OPTIONAL |
| Fusión `event-subscriptions.local.json` | Plantilla Vía C documentada; cableado runtime Fase 6+ |
| `emit-pr-presented-event.md` | Acción no indexada; emisor referenciado en Clase |
| PR final | Vía `delivery-close-cycle` (operador) |

## Veredicto

**APTO** para handoff a `delivery-close-cycle` y apertura de PR desde `feat/ola-c-event-entity`.
