---
feature_name: aut-53-accept-pr-hygiene
created: "2026-11-02"
process: bug-fix
base: main
scope: accept-pr-seal-scope-hygiene-threshold
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-53-accept-pr-hygiene
pbi_ref: AUT-53
tracker_ref: AUT-53
execution_id: "c2dd1bc0-3556-4149-90ed-52cf13d44b8c"
---

# Spec — aut-53-accept-pr-hygiene (AUT-53)

## Hecho

Incidente `accept-pr` sobre `fix/bridge-presented-tracker-ref` (AUT-25, PR 424, execution `dc1ca12a-24dd-41f8-9eb2-74fceb262eed`). Fusión Soberana salió 1 (`diff_seal_mismatch` por fixture hermano) y la fase Sync borró la rama remota; GitHub cerró el PR el 2026-10-30T10:37:03Z con `mergedAt` null. `main` quedó en `84c2ee24`; tip `1b170489` sobrevivió solo por worktree local.

Misma familia que el PBI fractura `c291912fe9a5` (AUT-54). Evolution canónica ya registrada: `SddIA/evolution/254e4344-dbf5-4493-b5c1-d37d4174905e.md`.

| ID | Defecto | Evidencia / ancla |
|----|---------|-------------------|
| F1 | Barrido de sello de todo el árbol: `validate_for_accept_pr_merge` unía `persist_ref` explícito con `list_persist_refs_with_validacion` (todos los `validacion.md` bajo `docs/features\|fixes\|refactors`). Un hermano APTO sin `validated_*` (p. ej. `docs/features/_fixture-done-gate-lab`) abortaba la fusión del ciclo con sello válido. | `diff_seal.rs` `validate_for_accept_pr_merge`; traza PR 424 / tip `1b170489` |
| F2 | Higiene remota desacoplada del éxito de fusión: `residual_runner` continúa fases tras fail; Sync hacía push (`Everything up-to-date` = OK) y `delete_branch` remoto sin `merge_commit_hash`. `L-HYGIENE-SOFT` no cortaba el delete por ausencia de hash. | `accept_pr.rs` Sync; `execution_report` `dc1ca12a` (`branch_deleted_remote: true`) |
| F3 | Colapso mudo: fail-path de Fusión no emite `System_Fracture_Detected` desde `accept_pr.rs` (emisión retroactiva `ea8ac1fc-…`; route con `data: null` / pending). | `accept_pr.rs` (sin `write_fractal_event`); handoff/fracture-payload del workspace accept-pr |
| F4 | Dedupe por proceso de `materialize-fracture-pbi` contra `c291912fe9a5` ocultó traza propia PR 424. | materialize-fracture-pbi; AUT-54 |

**Baseline ya en árbol (no reinventar):** con `persist_ref` informado, `to_check` es solo ese path (comentario AUT-53 en `diff_seal.rs`); Sync early-return `skipped` / `reason: merge_commit_hash ausente` vía `accept_pr_physical_threshold_crossed`; test `sync_skips_hygiene_without_merge_commit`.

## Corte

### C1 — Scope de sello acotado al ciclo

1. Si `inputs.persist_ref` (o equivalente) viene no vacío tras trim: `validate_for_accept_pr_merge` verifica **solo** ese `persist_ref`. Prohibido concatenar / barrer hermanos vía `list_persist_refs_with_validacion`.
2. Si `persist_ref` ausente/vacío: conservar fallback a listado del árbol (comportamiento legacy para invocaciones sin ciclo documental).
3. Fixture hermano APTO sin sello **no** debe provocar `diff_seal_mismatch` cuando el `persist_ref` del ciclo tiene sello válido.
4. Tests unitarios: (a) persist_ref válido + hermano fixture APTO sin sello → Ok; (b) persist_ref del ciclo con sello stale → Err `diff_seal_mismatch` acotado a ese path.

### C2 — Higiene remota solo tras umbral físico

1. `accept_pr_physical_threshold_crossed(state)` ⇔ `merge_commit_hash` no vacío en state tras Fusión Soberana.
2. Fase `Sincronización y Limpieza`: si el umbral no se cruza → `skipped: true`, `reason: merge_commit_hash ausente`, **sin** push causal de higiene de rama y **sin** `delete_branch` local/remoto.
3. Con umbral cruzado: conservar contrato actual (push `main` causal; si push falla, no delete; `L-HYGIENE-SOFT` en fallos de delete post-merge).
4. Invariante DoD: fallo de fusión (incl. `diff_seal_mismatch`) no cierra el PR por borrado de rama remota.
5. Mantener/asegurar test `sync_skips_hygiene_without_merge_commit`.

### C3 — Observabilidad del fail-path (mínimo DoD)

El DoD exige caso en corpus **o** `unclassified` con justificación. No exige rediseñar el fan-out EDA (eso es familia AUT-52/54).

1. Anclar el incidente PR 424 / `dc1ca12a` / tip `1b170489` en corpus de fractura **o** marcar `unclassified` en evolution/validacion con justificación explícita (familia `c291912fe9a5`, daño materializado, baseline C1+C2).
2. Opcional no bloqueante si C1+C2+corpus cerrados: emitir `System_Fracture_Detected` desde el fail-path de Fusión (`diff_seal_mismatch` u otros Err pre-merge) sin acoplar higiene. Si se hace, `friction_id`/`error_code` estables y sin depender de dedupe muda contra `c291912fe9a5` para ocultar la traza del ciclo.
3. Fuera del mínimo: arreglar dedupe por proceso / route `data: null` (AUT-52/54).

### C4 — Contrato documental del proceso

1. Actualizar `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md`:
   - Fase 2: con `persist_ref` informado, el sello se limita a ese path (no barrido total del árbol).
   - Fase 4: higiene (delete local/remoto) **solo** si existe `merge_commit_hash`; si no, skip explícito.
2. Refrescar `hash_signature` del proceso tras el cambio de genoma.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AUT53-CA1 | Con `persist_ref` informado, el barrido de sello no exige APTO/`validated_*` de hermanos del tip |
| AUT53-CA2 | Sin `merge_commit_hash`, Sync no ejecuta delete remoto (ni local de higiene); salida `skipped` + reason estable |
| AUT53-CA3 | Fallo de fusión (p. ej. `diff_seal_mismatch` por fixture) deja la rama remota intacta — el PR no se cierra por higiene |
| AUT53-CA4 | Con fusión exitosa y `merge_commit_hash` presente, la higiene remota sigue permitida (regresión L-HYGIENE-SOFT / push causal) |
| AUT53-CA5 | Caso PR 424 / fixture+`persist_ref` en corpus **o** `unclassified` justificado en evolution/validacion |
| AUT53-CA6 | `accept-pr.md` documenta scope de sello + umbral físico; integridad de hash del proceso OK |
| AUT53-CA7 | Verificación: lab/repro fixture+`persist_ref` aborta fusión y no borra remoto; path feliz con hash sí higieniza |
| AUT53-CA8 | AUT-53 → Done en Linear; sin sembrar `docs/todos/` desde Tekton/Argos; nota evolution ligada (reusar/ampliar `254e4344-…` si aplica) |

## Fuera de alcance

- Gate DCC previo `53eedb6e` (`tree_sha mismatch`) que aun así abrió el PR 424.
- Borrador Argos local `NO_APTO` no commitado.
- Rediseño de `residual_runner` para cortar el bucle de fases (basta el skip de Sync).
- Remedio dedupe `materialize-fracture-pbi` / fan-out route (`data: null`) — AUT-52/54.
- Rehabilitar tip `1b170489` / reabrir PR 424 como entrega funcional de AUT-25.
- Excluir fixtures del barrido legacy cuando **no** hay `persist_ref` (solo se exige acotar cuando el ciclo informa `persist_ref`).

## Touchpoints (Tekton)

| Módulo | Acción |
|--------|--------|
| `SddIA/engine/execute-process/src/engine/diff_seal.rs` | C1: confirmar/tests scope `persist_ref` |
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | C2: umbral + tests; C3 opcional emit fractura |
| `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` | C4 + refresh hash |
| Corpus / `SddIA/evolution/` (+ validacion de este fix) | C3/CA5/CA8 |

Sin `plan.md`: no hay blueprint de proceso nuevo (cortes sobre handlers/contrato `accept-pr` existentes; baseline C1+C2 ya aterrizada — Tekton cierra gaps de test/doc/corpus y verifica CA1–CA7).
