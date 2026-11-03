---
feature_name: aut-68-equipo-desde-issue
created: "2026-11-03"
purpose: decision
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-68-equipo-desde-issue
pbi_ref: AUT-68
execution_id: "plan-aut-68-20261103"
---

# Clarificaciones — AUT-68

## Decisiones

1. `tracker.team_key` del manifiesto sigue siendo el equipo por defecto de altas sin padre. No se cambia a MAN.
2. Si la operación lleva `issue_ref`, equipo, estados y etiquetas se resuelven en el equipo de ese issue (`issue.team`).
3. Si la operación es `create_issue` con padre, el equipo es el del padre. Sin padre, `team_key` de entrada.
4. Match de `state_name` y de nombres de label: insensible a mayúsculas, sin alterar el `state_map` ni las claves `tracker.labels`.
5. Label ausente en el equipo del issue: `LINEAR_LABEL_UNKNOWN` con clave de equipo y nombre buscado. No se crea la label ni se omite en silencio.
6. Rama de trabajo: `refactor/man-26-ciclo-vida-software`. Sin rama nueva.
