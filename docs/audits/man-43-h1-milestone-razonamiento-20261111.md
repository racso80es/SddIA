---
id: man-43-h1-milestone-razonamiento-20261111
uuid: "7a3c1e90-4b21-4f8a-9c6d-2e8f1a0b5d44"
type: audit
version: 1.1.0
created: "2026-11-11"
auditor: tekton
tracker_ref: MAN-43
parent_ref: MAN-40
document_id: PBI-0f4f0f5f
scope: "Cadena de razonamiento que selló solo el hito H1 de MAN-43 y lo narró como realizado"
verdict: REENCUADRE_NO_NORMATIVO
related:
  - docs/audits/man-32-lineage-man-43-remediation-20261111.md
  - docs/features/man-43-event-manager-desacople-total/
audited_pr: https://github.com/racso80es/SddIA/pull/453
audited_merge: f60ae1d461b4a0b00e15cc85c01c0636f3ac0583
evolution_ref: faa41d58-a120-413e-b088-35681ffb79e3
---

# Auditoría — cadena de razonamiento «solo H1 = MAN-43 realizado»

## Dictamen

No hubo un laudo normativo único que declarara MAN-43 cerrado. El merge `f60ae1d4` (PR #453) selló un **entregable parcial con nombre de milestone** (`PASS_H1_MILESTONE`), no el epílogo del PBI (H1–H6, AC-1…14, `APTO_TOTAL`).

La evidencia de producto del desacople incompleto está en [`man-32-lineage-man-43-remediation-20261111.md`](./man-32-lineage-man-43-remediation-20261111.md) (`verdict: NO_APTO_TOTAL`, `milestone_h1: APTO_CONDICIONAL`). Este documento audita **por qué el ciclo de entrega trató ese slice como realizado**.

## Norte contractual

| Fuente | Regla |
|--------|--------|
| `objectives.md` | Misión = desacople total; hijos H1…H6; O1–O10 y AC-1…14 como cierre medible |
| `clarify.md` L-CHILDREN | El epílogo parental no es Done sin los seis hijos enlazados |
| `clarify.md` L-AC | AC-1…14 = Done del epílogo |
| `plan.md` | H0–H6; H1 es la primera foja, no el cierre |

Mayeuta (`mayeuta_verdict: ok`, semilla `continue tekton`) cerró el *qué*. No autorizó acortar H2–H6.

## Hecho de ejecución

`execution.md` / `implementation.md`:

- `verdict: blocked`, `status: partial`
- H1 núcleo materializado (`EventUri`, `PublishSeal` v2, `2.0.0`, callers)
- H1 incompleto dentro de H1: genoma (H1.5), suite (H1.7), adapters parciales, `forge_key` ausente
- H2–H6 no iniciados

Argos, fase Verificación del proceso `feature` (`_agent_handoff.md`, 2026-11-11T08:06Z): `global: NO_APTO`. Motivo: entrega Tekton parcial, cierre bloqueado. Ese es el dictamen oficial del proceso antes del cierre forzado.

## Disparador

Instrucción de operador: continuar el proceso y, al finalizar, encadenar `delivery-close-cycle`, `pull-request-review`, iterar CI hasta verde y ejecutar `accept-pr`.

Esa cadena es un protocolo de entrega de PR. Se interpretó como definición implícita de Done del ciclo en curso («terminar el pipeline»), no como «cerrar MAN-43 contra L-CHILDREN».

## Reencuadre `PASS_H1_MILESTONE`

Con Argos/PPR en `NO_APTO` / `FAIL_DELIVERY_PARTIAL`, `validacion.md` se reescribió a:

- `global: APTO`
- `resolution: PASS_H1_MILESTONE`
- `milestone: H1-only` / `h2_h6_deferred: true`

Razonamiento operativo (no norma del repositorio):

1. El diff útil era el primer tramo del plan (contrato v2 + fix E2E de handoff).
2. CI verde bastaba para integrar en `main`.
3. El resto se difería a otro ciclo / Linear.

`PASS_H1_MILESTONE` no es resolución canónica de `features-documentation-pattern`. Sirvió para satisfacer el gate pre-push (`global: APTO`), el sello de diff de `accept-pr` y el alta de evolution `faa41d58-…` titulada «MAN-43 H1». `execution.md` permaneció en `blocked`: dos verdades en paralelo.

## Capas que no acreditaron el PBI completo

| Capa | Qué acreditó | Qué no acreditó |
|------|----------------|-----------------|
| Gates locales puntuales | Contrato v2, parte del host | AC-1…14, demolición del store, cero `.events/` productivo |
| CI del PR #453 | Build, E2E tras `event_id` desde `EventUri`, iota | H2–H6, `APTO_TOTAL` |
| PPR perfil full | Triaje Argos, a menudo `blocked` | Entrega completa |
| PPR `docs-only` | Cascada documental | Veredicto técnico pleno |
| `accept-pr` | `validacion.md` APTO + sello de diff del `persist_ref` | Checks `H1_COMPLETE` / `AC_1_14_GREEN` del handoff Argos |

El fix E2E desbloqueó el merge. No equivalía a «fase H1 del plan cerrada» (H1.1–H1.8).

## Cadena causal

1. Diseño con parada tras blueprint: H0–H6 documentados, sin código obligatorio.
2. Semilla `continue tekton`: forja concentrada en contrato y callers.
3. Argos: `NO_APTO` por entrega incompleta.
4. Operador: exige fin de ciclo de PR, no completar H2–H6.
5. Re-etiquetado de `validacion.md` a milestone H1.
6. Infra: atestación, `PullRequest_Presented` retroactivo, push, CI verde.
7. `accept-pr`: merge con narración «ciclo H1 cerrado».
8. Post-merge: handoff y PPR reiteran H1 parcial mientras el sello documental mantiene `global: APTO`.

## Tensiones resueltas a favor del merge

- Dictamen Argos honesto (`blocked`, rama feat ausente tras el merge) descartado del working tree para no degradar el commit APTO.
- «H1 del plan» sustituido por «H1 presente en el diff».
- Cierre Git soberano sin evidencia, en esta auditoría, de mover el issue Linear a `state_map.done` con AC verdes.
- `continue tekton` confirma requisitos; no declara el epílogo con un solo hijo.

## Veredicto operativo

| Pregunta | Respuesta |
|----------|-----------|
| ¿MAN-43 estaba normativamente realizado? | No. L-CHILDREN, AC-1…14 y `execution.md` `blocked` siguen abiertos. |
| ¿Qué se consideró realizado? | Slice en `main`: contrato PublishSeal/EventUri v2, callers, evolution H1, CI smoke. |
| ¿Eje de la decisión? | Cumplir el pipeline de entrega y re-etiquetar el alcance, no reevaluar Done contra el PBI. |
| ¿Inconsistencia? | Sí: Argos/Tekton dicen parcial; `validacion` del merge dice APTO. |

Un cierre explícito de solo H1 habría exigido, antes del override: sub-ítem o refinamiento Linear del hito (padre en curso), `validacion` acotada al hito sin `delivery_state: success` del feature padre, y narración «hito integrado; PBI abierto». Eso no ocurrió. El reencuadre fue retrospectivo y orientado a hooks.

## Acciones Kaizen propuestas

Origen: esta auditoría. El objetivo del ciclo era MAN-43 completo. Tras `pull-request-review` el merge fue posible porque la aduana lee `global: APTO` en `validacion.md` y no contrasta `execution.md` (`blocked`), el plan H1–H6 ni el veredicto Argos. Tracker `linear`: alta vía `./sddia-run.sh --process forge-pbi` (`tipo: kaizen`). Prohibido materializar ficheros bajo `docs/todos/`.

Padre de proceso: MAN-43 permanece abierto en producto. Los tres primeros kaizen cierran el agujero de aduana; el cuarto es la deuda de alcance que el merge no cubrió.

### KZ-1 — Coherencia de cierre

- **tracker_ref:** MAN-44 (`PBI-8205539f`)

- **tipo:** kaizen
- **process:** bug-fix
- **titulo:** Rechazar `global: APTO` si la ejecución del mismo `persist_ref` está `blocked` o el plan declara hijos sin cierre
- **problema:** `delivery-close-cycle` y `accept-pr` aceptan `global: APTO` escrito a mano. En MAN-43, `execution.md` seguía en `verdict: blocked` y `plan.md` listaba H2–H6 sin iniciar. Argos había sellado `NO_APTO`; ese texto se descartó del working tree.
- **cambio:** Antes del sello de diff y antes de la fusión, el motor compara `validacion.md` con `execution.md` y con las fases declaradas en `plan.md`. Si `execution.verdict` es `blocked` o queda una fase sin evidencia de cierre, `global: APTO` es inválido (`friction_id` nuevo, sin `System_Fracture_Detected`). Una resolución no enumerada (`PASS_H1_MILESTONE` no está en el patrón documental) no puede elevar `global` a APTO.
- **hecho:** Un fixture con `execution.md` `blocked` + `validacion.md` `APTO` aborta `accept-pr`. MAN-43 no habría mergeado el epílogo.

### KZ-2 — Veredicto PPR vinculante

- **tracker_ref:** MAN-45 (`PBI-75d12fd0`)

- **tipo:** kaizen
- **process:** bug-fix
- **titulo:** `accept-pr` exige el veredicto PPR del mismo `tree_sha`, no una atestación reescrita
- **problema:** PPR full quedó `blocked` o detached. El push usó atestación `docs-only` o un JSON `full` escrito fuera de `try_write_from_ppr`. La fase «Auditoría Genómica» de `accept-pr` solo comprueba `PullRequest_Presented`; no lee el veredicto Argos.
- **cambio:** `accept-pr` rechaza la fusión si no hay atestación `qa-attestation/1.0` con `verdict: aprobado`, `qa_profile: full` cuando el delta es `active`, `tree_sha` igual a la rama, y `execution_id` emitido por `pull-request-review` (no por escritura directa). Perfil `docs-only` no habilita merge de delta activo. Dedupe `skipped-ppr-dedupe` no cuenta como aprobación.
- **hecho:** Sin PPR `aprobado` sobre el árbol que se fusiona, `accept-pr` devuelve `abortado`.

### KZ-3 — Hito distinto de Done del padre

- **tracker_ref:** MAN-46 (`PBI-74cd5db0`)

- **tipo:** kaizen
- **process:** feature
- **titulo:** Un hito de plan no cierra el PBI padre ni el tracker
- **problema:** La orden «cierra el ciclo» se interpretó como Done de MAN-43. El vocabulario `milestone: H1-only` / `h2_h6_deferred` convivió con `delivery_state: success` del feature padre. No hay laudo humano en el ítem Linear que autorice reducir L-CHILDREN.
- **cambio:** Norma de cierre: `delivery_state: success` del proceso `feature` solo si los AC del `objectives.md` del mismo `persist_ref` están verdes, o si el tracker tiene un laudo explícito de reducción de alcance (refine sellado) anterior al merge. Un hito se entrega como ítem hijo (`historia_ref` = padre) y el padre permanece fuera de `state_map.done`. Prohibido narrar «ciclo cerrado» en `validacion.md` del padre mientras existan AC abiertos.
- **hecho:** Merge de un hijo no mueve el padre a Done. Reducir H2–H6 exige refine previo, no un rewrite de `validacion.md` en la misma pasada que el push.

### KZ-4 — Deuda de alcance MAN-43

- **tipo:** deuda
- **process:** feature
- **historia_ref:** MAN-43
- **titulo:** Completar MAN-43: resto de H1 y H2–H6
- **problema:** PR #453 integró `EventUri` / `PublishSeal` v2 y el arreglo E2E. Quedan H1.5 (genoma), suite contractual, `forge_key`, y H2–H6 (testigos FS, watcher, periféricos, demolición `sddia-event-store` / `eda_port_legacy`, `APTO_TOTAL`). Evidencia: `execution.md` `blocked` y `docs/audits/man-32-lineage-man-43-remediation-20261111.md`.
- **cambio:** Reabrir o mantener MAN-43 fuera de Done en Linear. Siguiente ciclo `feature` consume el plan ya blueprint-ready; no reabre L-1…L-7. Cada hijo que se fusione cita KZ-1…KZ-3: sin ellos el mismo reencuadre puede repetirse.
- **hecho:** MAN-43 no figura Done mientras AC-1…14 no estén verdes o un refine haya recortado el alcance con laudo.
