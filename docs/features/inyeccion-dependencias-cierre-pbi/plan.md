---
feature_name: inyeccion-dependencias-cierre-pbi
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-cierre-pbi
persist_ref: docs/features/inyeccion-dependencias-cierre-pbi
document_id: PBI-042-CIERRE-PBI
execution_id: d4e8f1a3-6c7b-4d9e-a2f0-3b4c5d6e7f8a
phases: 6
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Done global — Archivo PBI-042 + cascada documental (R15 / AC-DONE)"
blast_radius_genome: 0
---

# Plan / Blueprint — Cierre documental Done global PBI-042 (R15)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `knowledge-management`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula / acción | ¿Permitida? | Uso en este ciclo |
|------------------|-------------|-------------------|
| `skill:filesystem-manager` | sí | Cascada `persist_ref`; move PBI; evolution write |
| `skill:git-manager` | sí | Evidencia git / commits (sin raw) |
| `action:crypto-broker` | sí | UUID evolution |
| `action:execute-process` | sí | Solo si runtime encadena crypto-broker; **no** entity-manager DI |
| `skill:shell-executor` | sí | Opcional asserts lectura; **no** `git` nativo |

Ninguna fase exige mutación genoma ni `entity-manager` sobre ED DI. Git solo vía `skill:git-manager`. **Sin** ola H7.

---

## Fases declarativas

```yaml
phases:
  - name: "Baseline documental post-H6"
    intent: "Verificar PBI-042 en pending/ status abierto; cascada clarify+objectives+spec+plan presentes; traza MVP→H6 en objectives/clarify; abort si se pide mutar genoma DI."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Implementation touchpoints"
    intent: "Materializar implementation.md con touchpoints docs/evolution/PBI y blast_radius_genome=0."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Evolution cierre multi-hito"
    intent: "GENERATE_UUID + escribir un registro evolution vinculando execution_id y tabla MVP→H6→R15 (spec §4.4)."
    delegates_to: ["action:crypto-broker", "skill:filesystem-manager"]
  - name: "R15 Archivo PBI-042"
    intent: "move-file pending→done; status cerrado; frontmatter §4.2; sección cierre Q6; version 1.2.1; sin semilla Kaizen nueva."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Execution + Validacion APTO"
    intent: "execution.md con evidencias; validacion.md global APTO, pbi_archived true, checks AC-DONE/AC-REG-*, branch coherente (solo si PBI ya en done/)."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Handoff Argos / git evidencia"
    intent: "Handoff Argos AC-DONE; evidencia git vía git-manager si el runtime lo exige antes de delivery-close-cycle."
    delegates_to: ["skill:filesystem-manager", "skill:git-manager"]
```

---

## Detalle por fase

### 0 — Baseline documental post-H6

| # | Entregable | Detalle |
|---|------------|---------|
| B.1 | PBI pending | Existe path origen; `document_id` = `PBI-042-INYECCION-DEPENDENCIAS-CAPACIDADES`; `status: abierto` |
| B.2 | Cascada diseño | `clarify.md`, `objectives.md`, `spec.md`, `plan.md` presentes |
| B.3 | Traza | Smoke lectura MVP→H2→H3→H4→H5→H6 en objectives/clarify (**AC-REG-TRACE**) |
| B.4 | Abort | Si el operador exige tocar engine/bindings/taxonomía/creators → `blocked` (**L-NO-GENOME**) |

### 1 — Implementation

| # | Entregable | Detalle |
|---|------------|---------|
| I.1 | `implementation.md` | Items: cascada restante, evolution, move PBI, validacion; lista paths permitidos; exclusiones H7/genoma |

### 2 — Evolution (Q3-A)

| # | Entregable | Detalle |
|---|------------|---------|
| E.1 | UUID | `action:crypto-broker` `GENERATE_UUID` |
| E.2 | `{uuid}.md` | Frontmatter + tabla multi-hito spec §4.4; `execution_id` del ciclo |

### 3 — R15 Archivo PBI

| # | Entregable | Detalle |
|---|------------|---------|
| A.1 | Move | `pending/…PBI-042…` → `done/…PBI-042…` (mismo nombre de fichero) |
| A.2 | Frontmatter | `status: cerrado`; `close_*` + `closed_at`; bump `version` → `1.2.1` |
| A.3 | Cuerpo | Sección «Cierre Done global (R15)» + residual H7 diferido (Q6-A) |
| A.4 | KM | Solo archivo; **no** crear TODOs nuevos |

### 4 — Execution + Validacion

| # | Entregable | Detalle |
|---|------------|---------|
| X.1 | `execution.md` | Move confirmado; uuid evolution; paths tocados |
| X.2 | `validacion.md` | `global: APTO`; `branch: feat/inyeccion-dependencias-cierre-pbi`; `pbi_archived: true`; checks AC-DONE, AC-REG-R1-R14, AC-REG-TRACE; `git_changes` |

**Gate:** si PBI no está en `done/` → **prohibido** `pbi_archived: true` → fase `blocked`.

### 5 — Handoff Argos / git

| # | Entregable | Detalle |
|---|------------|---------|
| H.1 | Argos | Criterios `spec.md` §5; asserts Q5-A |
| H.2 | Git | Solo `skill:git-manager`; un PR (**L-SINGLE-PR**) |

---

## Orden (Q4-A) — resumen

1. Baseline → 2. `implementation.md` → 3. Evolution → 4. Move PBI → 5. `execution.md` + `validacion.md` → 6. Handoff Argos / git.

---

## Handoff Argos

Criterios en `spec.md` §5. `validacion.md` `global: APTO` solo si **AC-DONE** + **AC-REG-R1-R14** + **AC-REG-TRACE**. Diff sin genoma DI. H7 explícitamente diferido en PBI archivado (no blocker).
