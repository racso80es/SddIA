---
feature_name: pull-request-resealed
pbi_ref: AUT-43
branch_name: feat/pull-request-resealed
persist_ref: docs/features/pull-request-resealed
process: feature
---

# Objetivo (AUT-43)

Tras un re-sello commiteado en pre-push (PR OPEN), emitir **`PullRequest_Resealed`** en el bus con `correlation_id` del `PullRequest_Presented` de la misma rama, más `validated_head_sha` y `diff_digest`.

## Alcance

- Clase ECST `pull-request-resealed` y acción `emit-pr-resealed-event`.
- Registro en `event-domain-subscriptions.json` / `event-subscriptions.json` (sin suscriptores nuevos).
- Invocación desde `sddia-qa reseal-diff-validacion` solo si `outcome: committed`.
- El primer push (DCC) no emite este evento.

## Criterios de aceptación

1. Re-sello commiteado → un `PullRequest_Resealed` con los tres campos requeridos en payload.
2. `correlation_id` = `event_id` del `PullRequest_Presented` de la rama.
3. Sin `PullRequest_Presented` previo → fallo accionable en reseal (no emite resealed huérfano).
