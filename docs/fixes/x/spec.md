---
feature_name: x
created: "2026-08-28"
updated: "2026-08-28T17:00:00Z"
process: bug-fix
phase: Diseño del fix
agent: dedalo
agents: dedalo
branch_name: fix/x
persist_ref: docs/fixes/x
pbi_ref: docs/todos/pending/[FIX] x.md
base: main
scope: lab-cascade-voids
execution_id: "75bda8b4-372d-475e-8a20-f3acb48fb78b"
correlation_id: "00de947d-9da4-4ba0-a595-0f930d95d2c1"
status: blocked
design_verdict: blocked
plan_emitted: false
mayeuta_escalation: required
uuid: 9f2c4e81-6a3b-4d5e-8c1f-0b7a2e9d4f63
---

# Spec — x (Dedalo · Diseño del fix)

## Veredicto de diseño

**blocked** — carga insuficiente. Prohibido improvisar fix de producto. Escalado a **Mayeuta** (ambigüedad severa / PBI fantasma).

`plan.md` **no** emitido: no hay blueprint de proceso nuevo; `bug-fix` ya es el proceso vivo. Forjar fases adicionales sin requisito refinable violaría el Principio de Carga Estricta.

## Ingesta

| Input | Estado | Nota |
|-------|--------|------|
| `objectives.md` | Presente | Semilla lab: `inicia fix docs/todos/pending/[FIX] x.md` |
| `bug_summary` | Alias de objectives | Sin defecto de producto descrito |
| `pbi_ref` | **AUSENTE en FS** | Ni `pending/` ni `done/` |
| `cumulo_topology` | Implícita sesión | `paths.fixPath` → `docs/fixes` |
| `active_norm_pack` | `features-documentation-pattern` | Cascada mínima obligatoria |
| `target_executor_rbac` | No inyectado explícito en prompt | Asumido Tekton: `ecosystem-evolution` + `filesystem-ops` + `source-control` vía skill |

## Vacíos / ambigüedad (falla controlada)

| ID | Vacío | Impacto |
|----|-------|---------|
| V1 | PBI `[FIX] x.md` inexistente | Sin `document_id`, sin CA, sin alcance de código |
| V2 | Semilla solo orquestación lab | No hay síntoma, repro ni superficie de fallo |
| V3 | Cascada previa Tekton/Argos `blocked` | `implementation.md` / `execution.md` / `validacion.md` ya registran aborto; no hay diff producto |
| V4 | Identidad de rama vs worktree | `branch_name: fix/x` declarado; evidencia `git-manager` no materializada en esta fase Dedalo |

## Lo que NO es este fix

- No es remediación de `capsula-binario-fosil-release-stale` ni de ningún PBI R1 concurrente.
- No autoriza mutación de genoma (`SddIA/tools|skills|actions|process|agents|…`).
- No autoriza escritura bajo `docs/todos/` (RBAC KM: solo Cúmulo / evento Kaizen).
- No inventa éxito ni criterios de producto ad hoc.

## Criterios de aceptación (desbloqueo de diseño)

Antes de re-inyectar Dedalo (o Tekton con mandato de código):

1. **CA-PBI:** Materializar PBI real en `docs/todos/pending/` (o corregir `pbi_ref` / `persist_ref` al fix correcto) con `document_id`, síntoma y alcance.
2. **CA-OBJ:** `objectives.md` o cuerpo PBI con defecto reproducible (no solo `inicia fix …`).
3. **CA-SCOPE:** Lista explícita de superficies tocables (paths bajo topología) o declaración de **NO-OP producto** si el ciclo es solo validación de orquestación lab.
4. **CA-GIT:** Evidencia `skill:git-manager` (`status` / rama) vía `./sddia-run.sh --tool git-manager` o Evidence Bridge nativo PPR — sin bypass raw destructivo.

Si el laudo humano declara **solo laboratorio de cascada** (sin código): Mayeuta debe sellar `clarify.md` con `product_fix: none` y Dedalo reemitirá spec NO-OP acotado a artefactos bajo `persist_ref`.

## Mandato a Tekton (mientras `status: blocked`)

| Acción | Orden |
|--------|-------|
| Fix físico de producto | **Prohibido** |
| Mutación genoma | **Prohibido** |
| `docs/todos/**` | **Prohibido** |
| Reescritura de este `spec.md` | Solo Dedalo en re-diseño |
| `delivery-close-cycle` | **Prohibido** hasta Argos APTO + PBI archivado |

Tekton puede registrar `implementation.md` / `execution.md` como `blocked` coherente con este spec; no inventar items aplicados.

## Mandato a Argos

- `cascade_spec` → este archivo existe (legible).
- `global` permanece **NO_APTO** mientras V1–V2 no se cierren y no haya fix/NO-OP laudo.
- `pbi_archived: false` hasta PBI real en `done/`.

## Escalado

Runtime → **Mayeuta** (`clarify.md` + refinamiento de `objectives.md`) o laudo biológico que sustituya `pbi_ref` por un PBI existente.

Mensaje canónico de handoff: *Diseño abortado por vacíos V1/V2. Sin blueprint. A la espera de PBI o clarificación Mayeuta.*
