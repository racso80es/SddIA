---
document_id: PBI-FIX-FRACTURE-187079f2d82d
title: "[FIX] delivery-close-cycle — fractura sistémica"
format: markdown
version: "1.1.0"
created: "2026-10-03"
updated: "2026-10-12"
refined: "2026-10-12"
status: "cerrado"
priority: alta
process: bug-fix
fracture_hash: 187079f2d82d
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — 187079f2d82d"
suggested_branch: fix/dcc-snapshot-branch-exists
persist_ref_suggested: docs/fixes/dcc-snapshot-branch-exists
source_audit: "2026-10-12. Traza literal SNAPSHOT_BRANCH_CHECKOUT + «una rama llamada 'fix/dcc-snapshot-missing-branch' ya existe». Ese texto es el fatal de git checkout -b. 6d15648d (23:08) mete ese checkout en todo Snapshot; git-manager aún hacía checkout -b incondicional. La rama es la del fix hermano 969f05933a46. 80961b29 y ad968d80 (misma noche) cierran la conducta en HEAD. Sin test y sin firma: el enrich sigue en el fallback."
review_notes: "v1.0.0: plantilla unclassified («no clasificada» / «Auditar proceso»). v1.1.0: eso es el texto fijo de analyze_fracture_kaizen cuando ningún cubo casa. La acción «Snapshot final» del stub sale del interpolado attempted_action, no de un diagnóstico. La causa es checkout -b sobre refs/heads ya presente."
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/skills/git-manager/src/main.rs
  - SddIA/core/fracture-signatures.json
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (969f05933a46).md
related_pbis:
  - id: PBI-FIX-FRACTURE-969f05933a46
    rol: "Hermano de la misma noche. Traza distinta: «argumento ambiguo» / revisión desconocida (rama que rev-parse no resolvía). Su rama local fix/dcc-snapshot-missing-branch es el nombre que este sello intenta crear otra vez. No reabrir."
gates_this_wave:
  - DCC-BRANCH-EXISTS-CA1
  - DCC-BRANCH-EXISTS-CA2
---

# [FIX] delivery-close-cycle — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Snapshot final` |

## Traza de error

```
[SNAPSHOT_BRANCH_CHECKOUT] fatal: una rama llamada 'fix/dcc-snapshot-missing-branch' ya existe
```

Ese `fatal` es el de `git checkout -b` cuando `refs/heads/{branch}` ya está. El prefijo es el `error_code` de `delivery_phase_failed` en Snapshot final.

## Filtro A — inexactitudes del stub v1.0.0

| Afirmación v1.0.0 | Veredicto | Hecho |
|-------------------|-----------|--------|
| «Causa raíz no clasificada… requiere laudo humano» | **Plantilla, no diagnóstico.** | Texto fijo de `analyze_fracture_kaizen` con `matched` vacío. Ninguna firma casa `SNAPSHOT_BRANCH_CHECKOUT` + «ya existe». |
| «Auditar proceso `delivery-close-cycle`, acción `Snapshot final`» | **Interpolado.** | `proposal` del fallback rellena `process_name`, `attempted_action` y `agent_emitter`. Nombra la fase correcta por accidente. |
| Misma causa que `969f05933a46` | **No.** | Aquel sello es `rev-parse` ambiguo / revisión desconocida (la rama no resolvía). Este es la rama local ya creada. |
| El colapso sigue abierto en el algoritmo de Snapshot | **No en HEAD.** | `80961b29` hace `rev-parse refs/heads/{branch}` antes de crear. `ad968d80` hace `show-ref --verify` y, si el ref existe, `checkout` sin `-b`. Ambos son ancestros de HEAD. |

## Causa

`6d15648d` (2026-10-03 23:08) obliga a Snapshot final a llamar `git-manager checkout` con `create_if_not_exists: true` en cada cierre. En ese instante la operación era `git checkout -b` sin mirar el ref. `fix/dcc-snapshot-missing-branch` ya existía: es la rama del fix hermano `969f05933a46`. Git rechaza el alta y la fase emite `SNAPSHOT_BRANCH_CHECKOUT`.

La misma noche el árbol corrige la conducta (`49a08b2d` prueba el checkout existente; `80961b29` antepone `rev-parse refs/heads/`; `ad968d80` hace idempotente `create_if_not_exists`). El parche no deja test ni entrada en `fracture-signatures.json`. El siguiente sello idéntico vuelve a salir `unclassified`.

## Mandato

Candado de lo ya presente en HEAD. Prohibido reescribir el checkout de Snapshot como si siguiera siendo `-b` incondicional.

- **DCC-BRANCH-EXISTS-CA1.** `git-manager` `checkout` con `create_if_not_exists: true` y `refs/heads/{branch}` ya presente: exit 0. stderr sin «ya existe» y sin «already exists».
- **DCC-BRANCH-EXISTS-CA2.** Firma `F-DCC-SNAPSHOT-BRANCH-EXISTS`: traza con `SNAPSHOT_BRANCH_CHECKOUT` y («ya existe» o «already exists») → `process_fix` que nombra el checkout idempotente. `unclassified` queda en falso. La traza «argumento ambiguo» / revisión desconocida de `969f05933a46` no casa.
- Caso de esa traza en `fracture-corpus`.

## Criterio de cierre

- [x] **DCC-BRANCH-EXISTS-CA1** checkout idempotente con rama local previa
- [x] **DCC-BRANCH-EXISTS-CA2** firma distinta del hermano `969f05933a46`; la traza de este sello no cae en el fallback
- [x] Caso añadido a `fracture-corpus`
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`

## Fuera de alcance

- Reabrir `969f05933a46` (rama que `rev-parse` no resolvía).
- Volver a parchear `capsule_delivery_snapshot_final_with_repo` por esta traza: el corto-circuito ya está en HEAD.
- Rama ocupada por otro worktree (`already checked out` / `already used by worktree`): otra traza, otro sello.
