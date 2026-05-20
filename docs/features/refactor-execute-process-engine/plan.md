---
feature_name: refactor-execute-process-engine
created: "2026-05-20"
process: feature
branch_name: feat/refactor-execute-process-engine
---

# Plan — refactor execute-process engine

## Fases de trabajo

| Fase | Actividad | Resultado |
|------|-----------|-----------|
| 1 | Inicialización orgánica (`process: feature`) | Rama `feat/refactor-execute-process-engine` + `objectives.md` |
| 2 | Extracción núcleo YAML/validación (`execute_process_core.py`) | Sin ramas `if canonical == …` |
| 3 | REGISTRY cápsulas + bucle de fases (`execute_process_capsules.py`) | `workspace-init`, entity-manager vía delegación, forja por forma de inputs |
| 4 | Ampliación registry acciones EDA | `emit-pr-presented-event.md`, handlers en `execute-action.py` |
| 5 | Humo, PR #9, merge, bus EDA + IOTA real | `validacion.md` APTO |

## Secuencia de verificación

1. `execute-process.py --process feature --inputs-file …` → fase 1 `executed`, 2–6 `simulated`.
2. `execute-action.py --action emit-pr-*` / `emit-domain-mutation`.
3. `event-watcher.py --once` sin `SDDIA_LAB_SIMULATE_IOTA`.

## Referencias

- Planificación previa: sesión Agente Integración (Arquitecto), mayo 2026.
- Deuda post-entrega: `docs/todos/[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI execute-process y execute-action.md`.
