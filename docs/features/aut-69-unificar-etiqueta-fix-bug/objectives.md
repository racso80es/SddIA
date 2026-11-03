---

feature_name: aut-69-unificar-etiqueta-fix-bug
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-69-unificar-etiqueta-fix-bug
pbi_ref: AUT-69
execution_id: "plan-aut-69-20261103"
---
# Objetivos — aut-69-unificar-etiqueta-fix-bug

## Misión

Dejar un solo nombre de defecto en Linear para el equipo AUT: `Bug`. El manifiesto SddIA debe enviar ese nombre en `tracker.labels.fix`.

## Alcance

- Cambiar `.SddIA/project.md`: `fix: Bug`.
- Migrar los 25 issues AUT que llevan `Fix` (AUT-4, AUT-6, AUT-8, AUT-9, AUT-11, AUT-12, AUT-13, AUT-22, AUT-23, AUT-25, AUT-26, AUT-27, AUT-29, AUT-30, AUT-31, AUT-34, AUT-37, AUT-45, AUT-52, AUT-53, AUT-54, AUT-57, AUT-58, AUT-60, AUT-63): añadir `Bug`, quitar `Fix`.
- Eliminar la label `Fix` del equipo AUT cuando el conjunto quede vacío.
- Comprobar que `forge-pbi` (`bug-fix`) y `materialize-fracture-pbi` siguen leyendo el valor del manifiesto y no una cadena fija.

## Fuera de alcance

- Alta de la label `sddia-editable`.
- Renombrar las claves `hu`, `pbi`, `fix`, `kaizen`, `deuda`, `spike`.
- AUT-70 (tipo GraphQL `projectId` y cierre mudo del router).

## Ley aplicada

- Labels por igualdad exacta de nombre (`linear-tracker-adapter`).
- Alta y refino de backlog con `provider: linear` vía proceso, no fichero en `docs/todos/`.
- Genoma (`SddIA/tools/`, procesos) solo si un contrato cambia; este corte no lo cambia.
