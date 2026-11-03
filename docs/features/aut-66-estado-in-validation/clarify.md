---
feature_name: aut-66-estado-in-validation
created: "2026-11-03"
purpose: decision
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-66-estado-in-validation
pbi_ref: AUT-66
execution_id: "plan-aut-66-20261103"
---

# Clarificaciones — AUT-66

## Decisiones

1. El nombre canónico es `in_validation`. El nombre Linear propuesto es `In Validation`.
2. Ordinal: backlog 0, todo 1, in_progress 2, in_validation 3, in_review 4, done 5, cancelled 100.
3. El disparador es el éxito de la fase `agent:tekton`, no el inicio de Argos ni el de `delivery-close-cycle`.
4. Evento nuevo `Validation_Started`. `PullRequest_Presented` sigue yendo a `in_review`.
5. Modo file: solo cambia `tracker_column`. El ítem permanece en `docs/todos/pending/`.
6. Entrega en `refactor/man-26-ciclo-vida-software`. Padre MAN-26, desfase D3.
7. `cancelled` sigue siendo sumidero (100) y bloquea cualquier salida.
