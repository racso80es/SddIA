---
feature_name: plumb-cid
created: "2026-07-23"
updated: "2026-07-23"
process: feature
base: main
scope: lab-plumb-correlation-id-cascada-documental
version_spec: "1.0.0"
uuid: c3d4e5f6-a7b8-4c9d-0e1f-23456789abcd
status: dedalo_locked
document_id: LAB-PLUMB-CID
branch_name: feat/plumb-cid
persist_ref: docs/features/plumb-cid
pbi_ref: docs/todos/pending/[FEATURE] plumb-cid.md
pbi_status: absent_pending_path
correlation_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
phase: Diseño de Blueprint
agents: dedalo
laudo: lab-plumb-cid-evidence-only-no-domain-product
---

# Especificación — plumb-cid

## 1. Naturaleza del ciclo

**Lab de tubería / humo documental**, no feature de dominio. Demuestra trazabilidad auditable del `correlation_id` inyectado por `kalma2-agent-runtime-cursor` a través de la cascada `feature` (Mayeuta → Dedalo → Tekton → Argos) bajo `persist_ref` resuelto vía topología (`paths.featurePath` → `docs/features` + `feature_name` → `docs/features/plumb-cid`).

```text
Runtime (cid inyectado; persist_ref vacío → workspace-init)
  → Mayeuta: clarify.md + objectives.md (cid en frontmatter)  [hecho]
  → Dedalo: spec.md + plan.md (este ciclo)                    [en curso]
  → Tekton: implementation.md + execution.md + evidencia git
  → Argos: validacion.md solo con evidencia física (no-fake)
```

**Invariantes:** paths vía `SddIA/core/cumulo.paths.json`; git solo `skill:git-manager`; KM/`docs/todos/` solo Cumulo / `Kaizen_Alert_Required`; ausencia física = **blocked/NO_APTO** (prohibido inventar éxito); sin mutación de genoma Core.

## 2. Entrada estabilizada (`refined_requirements`)

Cuerpo de `objectives.md` (O-PLUMB-CID, O1–O5, L-CID-FM…L-NO-FAKE) + laudos Mayeuta D0–D8 / Q1–Q4 en `clarify.md`. Semilla cruda: «inicia feature docs/todos/pending/[FEATURE] plumb-cid.md».

| Hecho | Estado al diseño |
|-------|------------------|
| `clarify.md` + `objectives.md` con mismo `correlation_id` | Presente |
| PBI `docs/todos/pending/[FEATURE] plumb-cid.md` | **Ausente** (hueco KM documentado) |
| Evidencia git Dedalo vía `./sddia-run.sh --tool git-manager` | **No materializada** (Shell/cápsula Rejected en esta sesión IDE) |

## 3. Laudos Dedalo (cierran Q1–Q4 + handoff Tekton/Argos)

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | ¿Forjar PBI desde Tekton/Argos? | **No.** Solo Cumulo / `Kaizen_Alert_Required` / operador | Q1; veto KM agentes ejecución |
| **L2** | ¿Producto de dominio? | **No.** Alcance = plumb CID + gates no-fake | Q2; O-PLUMB-CID |
| **L3** | ¿Blueprint? | **Sí, mínimo** (`plan.md` T-GATE…T4) | Q4; process-contract con `delegates_to` canónicos |
| **L4** | ¿Git en diseño Dedalo? | Dedalo **no** tiene `source-control` en RBAC agente; evidencia git = fase Tekton (T-GATE/T3). Esta sesión: **declarar no materializado** | Q3; allowed_policies Dedalo |
| **L5** | CID canónico | `a1b2c3d4-e5f6-4789-a012-3456789abcde` — idéntico machine-readable en frontmatter de toda la cascada tocada | AC-L-CID |
| **L6** | Forja código / genoma | **Forja=0** salvo fallo demonstrable fuera de alcance; **prohibido** mutar `SddIA/{tools,skills,actions,process,agents,events,norms,library}` | Lab tubería |
| **L7** | Cierre documental Done | Exige PBI físico archivado + `validacion.md` APTO + `pbi_archived: true`. Con PBI ausente → **Done documental bloqueado**; lab CID puede verificar AC-L-* sin fingir Done | features-documentation-pattern v1.2.1 |
| **L8** | Soft-dep F3 PPR #136 | **Fuera** de alcance | D2 / D7 |
| **L9** | APTO narrativo | Prohibido. Sin stdout/artefacto → `blocked` / `NO_APTO` | AC-DONE-LAB / L-NO-FAKE |

## 4. Contrato de artefactos (qué materializar)

| Artefacto | Owner | Obligatorio lab | Nota |
|-----------|-------|-----------------|------|
| `clarify.md` | Mayeuta | Sí (baseline) | CID en FM |
| `objectives.md` | Mayeuta | Sí (baseline) | CID en FM; fuente `refined_requirements` |
| `spec.md` | Dedalo | Sí | Este documento |
| `plan.md` | Dedalo | Sí | Blueprint T-GATE…T4 |
| `implementation.md` | Tekton | Sí | `items: []` / baseline documental si forja=0 |
| `execution.md` | Tekton | Sí | Tabla evidencia CID + resultado git-manager o blocked |
| `validacion.md` | Argos | Sí | APTO solo con checks físicos |
| PBI pending→done | Cumulo/operador + cierre documental | Gate Done, no gate AC-L-CID | Path referenciado ausente hoy |

**Topología (lógica, no host abs inventado):**

| Clave | Valor |
|-------|-------|
| `paths.featurePath` | `docs/features` |
| `persist_ref` | `docs/features/plumb-cid` |
| `directories.documentation` | `docs` |
| Soft-dep (fuera) | `docs/todos/pending/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md` |

## 5. Criterios de aceptación (Argos)

| ID | Criterio | Evidencia física |
|----|----------|------------------|
| **AC-L-CID** | Mismo `correlation_id` en frontmatter de `clarify.md` y `objectives.md`; cascada Dedalo/Tekton propaga el mismo valor en FM tocados | Grep/lectura de FM bajo `persist_ref` |
| **AC-L-DOC** | Artefactos con frontmatter `features-documentation-pattern`; `spec.md`+`plan.md`+`implementation.md`+`execution.md` presentes | Paths físicos |
| **AC-L-PBI** | Gap PBI documentado en cascada; **ningún** agente ejecución escribe bajo `docs/todos/` | Ausencia de writes KM desde Tekton/Argos |
| **AC-L-GIT** | Stdout `skill:git-manager` (`./sddia-run.sh --tool git-manager`) **o** declaración explícita `git_evidence: not_materialized` / blocked | JSON stdout o entrada honesta en `execution.md` |
| **AC-DONE-LAB** | `validacion.md` `global: APTO` solo si AC-L-* verdes con evidencia; sin inventar | Argos |

**Nota Done vs lab:** AC-DONE-LAB verifica honestidad del lab. El **Done de proceso feature** (PBI en `done/` + `pbi_archived: true`) queda **bloqueado** hasta materialización Cumulo del PBI (L7).

## 6. RBAC ejecutor (Tekton) — cruce mecánico

`target_executor_rbac` esperado (homólogo a `process:feature` `context`):

```json
{
  "allowed_policies": ["ecosystem-evolution", "filesystem-ops", "source-control"]
}
```

| Cápsula | `context` YAML | ¿Permitida? |
|---------|----------------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | Sí |
| `skill:git-manager` | `source-control` | Sí |
| `skill:shell-executor` | `system-operations` | **No requerida** en este lab (forja=0; sin build) |
| `action:execute-process` | (cierre) | Solo fase cierre entrega; fuera del mínimo lab evidencia |

Si el runtime **no** otorga `source-control`: T-GATE → blocked honesto; **prohibido** bypass Shell destructivo.

## 7. Límites duros

- No inventar PBI ni semillas Kaizen desde Tekton/Argos/Dedalo.
- No absorber F3 git-manager residual / pasarela async / DI / GesFer.
- No declarar APTO sin evidencia física.
- No mutar genoma Core como alcance de este lab.
- Dedalo no ejecuta git; no fingir stdout de esta fase.

## 8. Veredicto Dedalo

**ok** — requisitos lab estables; blueprint mínimo viable frente a RBAC del proceso `feature` (filesystem-ops + source-control). Hueco PBI = bloqueo de Done documental, no de diseño. Evidencia git Dedalo: **not_materialized** (Rejected IDE/cápsula).
