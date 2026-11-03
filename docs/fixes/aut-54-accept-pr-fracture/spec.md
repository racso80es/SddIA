---
feature_name: aut-54-accept-pr-fracture
created: "2026-11-02"
process: bug-fix
base: main
scope: accept-pr-fixture-sweep-hygiene-audit-corpus
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-54-accept-pr-fracture
pbi_ref: AUT-54
tracker_ref: AUT-54
execution_id: "199ebb34-9a6a-400a-8731-b0c683c9dd11"
fracture_family: c291912fe9a5
---

# Spec — aut-54-accept-pr-fracture (AUT-54)

## Hecho

Incidente `accept-pr` sobre `docs/runtime-seal-tracker-outbound-20261030` (persist documental `docs/audits/runtime-seal-tracker-outbound-20261030`, emisor `tekton`, execution `d8310429-7fd7-4b0d-861a-c39e18e1eae5`). Misma familia que el PBI fractura `c291912fe9a5` y que AUT-53 / PR 424 (`dc1ca12a`).

| ID | Defecto | Evidencia / ancla |
|----|---------|-------------------|
| F1 | Barrido de sello del tip: Fusión Soberana aborta en `docs/features/_fixture-done-gate-lab/validacion.md` (`global: APTO` sin `validated_base_sha`) antes de evaluar el sello del ciclo. El `persist_ref` de auditoría bajo `docs/audits/…` **no** entra en `list_persist_refs_with_validacion` (solo `docs/{features,fixes,refactors}`). | `execution_report` `d8310429` fase «Fusión Soberana» `error: diff_seal_mismatch: …/_fixture-done-gate-lab/… validated_base_sha`; genoma `diff_seal.rs` |
| F2 | Higiene remota post-aborto **confirmada** (no narrativa). Tras fail de fusión y `merge_commit_hash` ausente, Sync empujó `main` (`Everything up-to-date`) y borró el remoto; local falló por worktree ocupado. | `execution_report` `d8310429` fase «Sincronización y Limpieza»: `branch_deleted_remote: true`, `branch_deleted_local: false`, `closed_branch: null`, `status_code: 1`. Simétrico a `dc1ca12a` |
| F3 | Fixture lab con `APTO` incompleto permanece en el tip y sigue siendo veneno del fallback legacy (barrido total cuando `persist_ref` vacío) y de cualquier camino que aún concatene hermanos. | `docs/features/_fixture-done-gate-lab/validacion.md` (+ hermanos `-review` / `-both`) |
| F4 | Observabilidad: fail-path de Fusión no emite `System_Fracture_Detected` propio; `materialize-fracture-pbi` puede `deduped_by_process` contra `c291912fe9a5` y ocultar traza del ciclo (familia PR 424 / audit). | AUT-53 F3/F4; `fix-fricciones.md` de `dc1ca12a` |
| F5 | Satélite: Argos selló el audit `NO_APTO` / `integridad_bloqueada` (bloquea reintento limpio; **no** explica el path fixture). Tip remoto audit ausente según sello Argos (`AUDIT_BRANCH_REF: AUSENTE`). | `docs/audits/runtime-seal-tracker-outbound-20261030/validacion.md` |

**Baseline ya en árbol (AUT-53 — no reinventar):**

- Con `persist_ref` informado: `validate_for_accept_pr_merge` verifica **solo** ese path (comentario + tests `aut53_scoped_persist_ref_*`).
- Sync early-return `skipped` / `reason: merge_commit_hash ausente` vía `accept_pr_physical_threshold_crossed`; test `sync_skips_hygiene_without_merge_commit`.
- Contrato `accept-pr.md` v1.0.3 documenta scope + umbral físico.
- Evolution `SddIA/evolution/254e4344-dbf5-4493-b5c1-d37d4174905e.md` (AUT-53/54).

AUT-54 **no** reabre el diseño C1/C2 de AUT-53: cierra residuales de fixture/legacy, ancla el laudo `d8310429`, corpus de familia y observabilidad diferida.

## Corte

### C1 — Defensa en profundidad del barrido (fixtures / lab)

Complemento al scope por `persist_ref` (AUT-53):

1. El fallback legacy de `validate_for_accept_pr_merge` (cuando `persist_ref` ausente/vacío) **debe excluir** persistencias lab cuyo slug empiece por `_fixture-` (p. ej. `docs/features/_fixture-done-gate-lab`). Prohibido que un fixture APTO sin sello aborte `accept-pr` en ese camino.
2. Alternativa aceptable (mínima, no ambas obligatorias si C1.1 cierra CA): sanear fixtures para que **no** declaren `global: APTO` sin campos `validated_*` completos — solo si no rompe `done_gate` / tests lab; preferir exclusión en el gate de accept-pr.
3. Con `persist_ref` informado: conservar invariante AUT-53 (solo ese path); un hermano fixture **no** provoca `diff_seal_mismatch`.
4. Tests: (a) sin `persist_ref`, tip con fixture APTO sin sello + otro persist real sellado → Ok o fallo acotado al persist real, **nunca** error cuyo path sea `_fixture-*`; (b) regresión AUT-53 con `persist_ref` + hermano fixture → Ok.

### C2 — Higiene solo tras umbral físico (regresión + laudo)

1. Conservar `accept_pr_physical_threshold_crossed` ⇔ `merge_commit_hash` no vacío.
2. Invariante DoD: un `diff_seal_mismatch` / aborto de Fusión Soberana **no** ejecuta `delete_branch` remoto ni local de higiene.
3. **Laudo `d8310429` (cerrado en diseño):** la traza **sí se sostiene** — Sync corrió higiene remota tras aborto (`branch_deleted_remote: true`). No es mezcla de otro camino; es la misma clase que `dc1ca12a` / familia `c291912fe9a5`.
4. No debilitar `L-HYGIENE-SOFT` post-merge (fallos de delete tras umbral cruzado siguen fail-soft).
5. Mantener test `sync_skips_hygiene_without_merge_commit`; si el genoma ya cumple, Tekton solo re-verifica (no reescritura cosméticas).

### C3 — Corpus / firma de familia

1. Anclar en `fracture-corpus` (fixture bajo `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/` + fila `fracture_corpus_regression`) el incidente `d8310429` / firma `diff_seal_mismatch` + higiene post-aborto, **o** `unclassified` justificado en `validacion.md`/evolution citando familia `c291912fe9a5` y peaje `execution_report`.
2. Si se añade firma en `fracture-signatures.json`, id estable (p. ej. `F-ACCEPT-PR-SEAL-HYGIENE`) con match sobre `diff_seal_mismatch` / fixture / delete remoto sin merge — solo si Mayeuta/catálogo lo consumen; no inventar firma huérfana.
3. Ampliar o referenciar evolution `254e4344-…` (o alta hermana) ligando AUT-54 + `d8310429`.

### C4 — Observabilidad del fail-path (secundario, no bloquea C1–C3)

1. Opcional: emitir `System_Fracture_Detected` desde fail-path de Fusión (`diff_seal_mismatch` u Err pre-merge) con `friction_id`/`error_code` estables, **sin** acoplar higiene.
2. Opcional: corregir `deduped_by_process` de `materialize-fracture-pbi` para no ocultar trazas distintas de la misma familia bajo un solo PBI abierto — sin reabrir fan-out `data: null` (AUT-52).
3. Si C1–C3 + Linear Done cierran DoD, C4 puede quedar documentado como deuda explícita en `validacion.md` (no APTO falso).

### C5 — Contrato documental

1. Si C1 muta comportamiento del fallback: actualizar `accept-pr.md` (Fase 2: exclusión `_fixture-*` en barrido legacy) + `refresh_process_hash`.
2. Si solo regresión AUT-53: citar contrato v1.0.3 sin bump innecesario.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AUT54-CA1 | Fallback sin `persist_ref`: el barrido de sello **no** exige APTO/`validated_*` de persistencias `_fixture-*` (exclusión o saneado equivalente) |
| AUT54-CA2 | Con `persist_ref` informado: hermano fixture APTO sin sello **no** provoca `diff_seal_mismatch` (regresión AUT-53) |
| AUT54-CA3 | Sin `merge_commit_hash`, Sync no ejecuta delete remoto/local; `skipped` + reason estable (regresión AUT-53) |
| AUT54-CA4 | Laudo documentado: `d8310429` **confirma** `delete_branch` remoto post-`diff_seal_mismatch` (cita `execution_report`; sin inventar peaje adicional) |
| AUT54-CA5 | Caso en `fracture-corpus` con firma esperada **o** `unclassified` justificado (familia `c291912fe9a5` / `d8310429`) |
| AUT54-CA6 | Path feliz con `merge_commit_hash`: higiene remota sigue permitida (no regresionar L-HYGIENE-SOFT / push causal) |
| AUT54-CA7 | Argos `validacion.md` APTO en este `persist_ref`; AUT-54 → Done en sumidero Linear; sin sembrar `docs/todos/` desde Tekton/Argos |
| AUT54-CA8 | Operación post-laudo (no bloquea diseño ni merge del fix): si tip remoto `docs/runtime-seal-tracker-outbound-20261030` sigue ausente, restaurar vía `skill:git-manager`; no reinyectar `accept-pr` hasta limpiar fixture o re-sellar validaciones en alcance |

## Fuera de alcance

- Bypass raw (`gh` / `git` / `curl`) para restaurar o fusionar.
- Remedio del contenido funcional de la auditoría `runtime-seal-tracker-outbound-20261030` más allá de desbloquear el gate de `accept-pr`.
- Rediseño amplio de Argos / cascada de sellos fuera del fail-closed de higiene y del alcance del barrido de `validacion.md`.
- Reinventar C1/C2 de AUT-53 si el tip ya los cumple (solo verificar + residual fixture/corpus).
- Rehabilitar contenido de AUT-25 / PR 424 como entrega funcional.
- Fan-out `route-domain-event` `data: null` (AUT-52).

## Touchpoints (Tekton)

| Módulo | Acción |
|--------|--------|
| `SddIA/engine/execute-process/src/engine/diff_seal.rs` | C1: excluir `_fixture-*` del fallback / tests AUT54 |
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | C2: re-verificar umbral; C4 opcional emit fractura |
| `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` | C5 si C1 cambia contrato |
| `docs/features/_fixture-done-gate-lab*/validacion.md` | Solo si se elige saneado APTO (alternativa C1.2) |
| `fracture-corpus` / `fracture-signatures.json` / evolution | C3 |
| `materialize_fracture_pbi.rs` | C4 opcional dedupe |
| Este `persist_ref` | `implementation.md` + `execution.md` + `validacion.md` |

## Nota Dedalo — blueprint

Sin `plan.md`: no hay blueprint de proceso nuevo. Cortes sobre handlers/contrato `accept-pr` existentes + corpus; baseline higiene/scope ya aterrizada en AUT-53.

## Evidencia de diseño (peajes leídos)

- `.SddIA/workspaces/accept-pr/d8310429-7fd7-4b0d-861a-c39e18e1eae5/execution_report.json`
- `.SddIA/workspaces/accept-pr/dc1ca12a-24dd-41f8-9eb2-74fceb262eed/execution_report.json` (+ `fix-fricciones.md`)
- `docs/fixes/aut-53-accept-pr-hygiene/spec.md` + `validacion.md`
- `docs/audits/runtime-seal-tracker-outbound-20261030/validacion.md`
- Genoma: `diff_seal.rs` `validate_for_accept_pr_merge`; `accept_pr.rs` Sync + `accept_pr_physical_threshold_crossed`
