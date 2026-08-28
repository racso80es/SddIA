---
feature_name: x
created: "2026-08-28"
updated: "2026-08-28"
process: bug-fix
branch_name: fix/x
persist_ref: docs/fixes/x
pbi_ref: docs/todos/pending/[FIX] x.md
base: main
scope: lab-bug-fix-cascade-smoke
uuid: "f3a8c2e1-4b7d-4f9a-9e2c-1d5b6a8c0e3f"
correlation_id: cc6d6e2c-b84b-40f9-ac01-acff25ed252e
execution_id: 92716387-568c-42c9-895d-2bf2aa186659
dedalo_verdict: ok
nature: lab-smoke
---

# Spec — bug-fix lab smoke (slug `x`)

## Problema

Semilla Kalma2/TQM: `inicia fix docs/todos/pending/[FIX] x.md`. El orquestador nativo `execute-process` encola `bug-fix` con `persist_ref: docs/fixes/x` y `branch_name: fix/x`, pero la cascada documental quedó **incompleta** en ciclos previos:

| Artefacto / gate | Estado previo |
|------------------|---------------|
| `objectives.md` | Presente (bootstrap lab) |
| `spec.md` / `plan.md` | **Ausentes** → Tekton abortó |
| PBI `pbi_ref` | **Inexistente** en `docs/todos/pending/` |
| `implementation.md` / `execution.md` | `status: blocked` |
| `validacion.md` | `global: NO_APTO` |

Hallazgo operativo: el laboratorio usa el slug placeholder `x` (tests `kalma2_classifies_fix_heuristic`, `build_bug_fix_inputs`) para validar clasificación y encolado; **no** implica un defecto de producto identificado en genoma.

## Causa raíz

| Pieza | Hecho |
|-------|--------|
| PBI | Nunca materializado; autoría restringida a `agent:cumulo` / `Kaizen_Alert_Required` |
| Diseño | Fase Dedalo no emitió `spec.md` → precondición Tekton fallida |
| Git | Evidencia `skill:git-manager` no acusada en sesiones IDE (Shell Rejected) |
| Alcance lab | Objetivo = validar cadena Inicialización → Diseño → Ejecución → Verificación, no parche funcional |

## Solución

### L1 — Completar cascada documental (Dedalo → Tekton)

1. **Dedalo (esta fase):** emitir `spec.md` + `plan.md` bajo `docs/fixes/x/`.
2. **Tekton:** consumir spec/plan; **sin mutación de genoma** (`SddIA/tools/`, `skills/`, `actions/`, `process/`, `agents/`, `events/`, `norms/`, `library/`).
3. **Touchpoints Tekton permitidos:** solo bajo `persist_ref` — actualizar `implementation.md`, `execution.md`; no inventar diff de producto si el PBI no define defecto físico.

### L2 — PBI semilla (upstream cumulo)

Materializar `docs/todos/pending/[FIX] x.md` vía **`agent:cumulo`** (Cosecha Kaizen) con cuerpo mínimo:

- Título: lab smoke bug-fix cascade
- Cuerpo: semilla `inicia fix docs/todos/pending/[FIX] x.md`; alcance documental; prohibido fix de producto sin hallazgo real
- Frontmatter: `document_id`, `type: fix`, coherencia con `persist_ref` / `branch_name`

**Dedalo/Tekton/Argos no escriben** bajo `docs/todos/**`.

### L3 — Evidencia git

Toda operación git vía `skill:git-manager` (`./sddia-run.sh --tool git-manager`, JSON stdin). Verificar rama `fix/x` antes de cierre. Sin stdout inventado si Shell Rejected — declarar check `GIT_EVIDENCE_SESSION_SHELL: NO_APTO` y usar Evidence Bridge solo si upstream lo acreditó.

### L4 — Naturaleza del fix físico

| Escenario | Acción Tekton |
|-----------|---------------|
| PBI ausente tras L2 | Mantener `implementation.md` / `execution.md` en `blocked`; no marcar APTO |
| PBI presente, lab smoke | Fix **documental** únicamente; `items: []` o lista de paths bajo `docs/fixes/x/` |
| PBI define defecto real | Tekton aplica touchpoints explícitos del PBI; fuera de alcance de este spec hasta que cumulo materialice cuerpo |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `spec.md` y `plan.md` presentes bajo `persist_ref` con frontmatter válido |
| CA2 | PBI `pbi_ref` existe en `docs/todos/pending/` (cumulo) antes de certificar `pbi_seed_exists` |
| CA3 | Tekton no muta genoma; diff acotado a `docs/fixes/x/` salvo laudo |
| CA4 | `validacion.md` refleja checks reales; `global: APTO` solo si cascada completa + PBI archivable |
| CA5 | Cierre documental: PBI en `docs/todos/done/` + `pbi_archived: true` en la rama del PR (norma `task-closure-documental`) |
| CA6 | Evidencia git vía git-manager acusada o declarada NO_APTO sin inventar OID |
| CA7 | Regresión lab: prompt `inicia fix docs/todos/pending/[FIX] x.md` sigue encolando vía Kalma2 (test existente `kalma2_classifies_fix_heuristic`) |

## Fuera de alcance

- Parche funcional en `execute-process`, centinelas o UI Kalma2 salvo que el PBI materializado lo exija.
- Escritura directa de TODOs por Tekton/Dedalo/Argos.
- Forjar entidad `process` nueva o mutar `bug-fix.md`.
- Declarar `pbi_archived: true` o `global: APTO` con PBI ausente.

## Touchpoints previstos (Tekton)

- `docs/fixes/x/implementation.md`
- `docs/fixes/x/execution.md`
- `docs/fixes/x/_agent_handoff.md` (bloque machine handoff)
- `docs/fixes/x/validacion.md` (fase Argos)

## Bloqueo residual conocido

**PBI `pbi_ref` ausente** al emitir este spec → Tekton permanece en precondición fallida hasta intervención `agent:cumulo`. Veredicto Dedalo fase Diseño: **ok** (artefactos de diseño materializados); veredicto global cascada: **blocked** hasta L2.
