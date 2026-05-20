---
feature_name: pbi-005-hito2-action-engine
created: "2026-05-20"
process: feature
purpose: Estabilización Hito 2 — motor de acciones y capas Skills/Tools
---

# Clarificación — PBI-005 Hito 2 (Motor de Acciones)

Transcript de decisiones, 2026-05-20.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso? | **`feature`** v1.2.0 |
| ¿Puerta física? | `execute-process.py` con handler **fase 1 viva** (`skill:git-manager`); fases 2–6 simuladas en laboratorio |
| Rama | `feat/pbi-005-action-engine` desde `main` |
| `persist_ref` | `docs/features/pbi-005-action-engine` |
| Manifiesto | `docs/todos/PBI-005-Hito2-TODO.md` |

---

## D2 — Jerarquía ontológica

| Capa | Artefacto |
|------|-----------|
| Acción | `sync-entity-index` |
| Agente | `cumulo` |
| Skill agrupadora | `bus-operator` |
| Tools | `markdown-table-editor`, `read-event-subscriptions`, `manage-event-receipt`, `transit-event-payload` |

El watcher **no** importa scripts ad-hoc: despacha `execute-action.py`.

---

## D3 — Estado heredado en main

`execute-action.py`, `markdown-table-editor` y desacoplamiento del watcher ya existían en `main`. Este hito **completa** el handler `feature`, la skill `bus-operator`, las micro-tools del bus y la documentación forense.

---

## D4 — Git y commits

Commits atómicos por fase en la rama de feature; merge vía `delivery-close-cycle` cuando Argos emita **APTO**.
