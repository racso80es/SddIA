---
feature_name: plumb-cid
created: "2026-07-23"
updated: "2026-10-02"
process: feature
base: main
scope: lab-plumb-correlation-id-cascada-documental
version_spec: "1.1.2"
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
git_evidence_dedalo: not_materialized_shell_rejected
refined_requirements_source: docs/features/plumb-cid/objectives.md
execution_id: "8d69c53d-42dc-4462-a95a-8a869f9d0726"
---
# Especificación — plumb-cid

## 1. Naturaleza del ciclo

**Lab de tubería / humo documental**, no feature de dominio. Demuestra trazabilidad auditable del `correlation_id` inyectado por `kalma2-agent-runtime-cursor` a través de la cascada `feature` (Mayeuta → Dedalo → Tekton → Argos) bajo `persist_ref` resuelto vía topología (`paths.featurePath` → `docs/features` + `feature_name` → `docs/features/plumb-cid`).

```text
Runtime (cid inyectado; persist_ref → workspace)
  → Mayeuta: clarify.md + objectives.md (cid en frontmatter)  [hecho 2026-10-02 / exec 8d69c53d]
  → Dedalo: spec.md + plan.md (este ciclo)                    [en curso]
  → Tekton: implementation.md + execution.md + evidencia git
  → Argos: validacion.md solo con evidencia física (no-fake)
```

**Invariantes:** paths vía `SddIA/core/cumulo.paths.json`; git solo `skill:git-manager`; KM/`docs/todos/` solo Cumulo / `Kaizen_Alert_Required`; ausencia física = **blocked/NO_APTO** (prohibido inventar éxito); sin mutación de genoma Core.

## 2. Entrada estabilizada (`refined_requirements`)

Fuente: `objectives.md` (O-PLUMB-CID, O1–O5, L-CID-FM…L-NO-FAKE) + laudos Mayeuta D0–D9 / Q1–Q4 en `clarify.md` (transcript 2026-10-02, `execution_id` 8d69c53d-…). Semilla cruda: «inicia feature docs/todos/pending/[FEATURE] plumb-cid.md».

| Hecho | Estado al diseño (2026-10-02 / Dedalo 8d69c53d) |
|-------|------------------------------------------------|
| `clarify.md` + `objectives.md` con mismo `correlation_id` | Presente (Mayeuta ok / exec 8d69c53d) |
| PBI `docs/todos/pending/[FEATURE] plumb-cid.md` | **Ausente** (0 hits `*plumb*` en `docs/todos/`; hueco KM) |
| Evidencia git Dedalo vía `./sddia-run.sh --tool git-manager` | **No materializada** (Rejected; sin stdout) |
| Cascada previa Tekton/Argos | Existe baseline; este Dedalo **re-bloquea** blueprint frente a Mayeuta estabilizado; no inventa nuevo producto |

## 3. Laudos Dedalo (cierran Q1–Q4 + handoff Tekton/Argos)

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | ¿Forjar PBI desde Tekton/Argos/Dedalo? | **No.** Solo Cumulo / `Kaizen_Alert_Required` / operador | Q1; veto KM agentes ejecución |
| **L2** | ¿Producto de dominio? | **No.** Alcance = plumb CID + gates no-fake | Q2; O-PLUMB-CID |
| **L3** | ¿Blueprint? | **Sí, mínimo** (`plan.md` T-GATE…T4) | Q4; process-contract con `delegates_to` canónicos |
| **L4** | ¿Git en diseño Dedalo? | Dedalo **no** exige `source-control` en RBAC agente; evidencia git = fase Tekton (T-GATE/T3). Esta sesión: **declarar no materializado** | Q3; intento Rejected |
| **L5** | CID canónico | `a1b2c3d4-e5f6-4789-a012-3456789abcde` — idéntico machine-readable en frontmatter de toda la cascada tocada | AC-L-CID / O1 |
| **L6** | Forja código / genoma | **Forja=0** salvo fallo demonstrable fuera de alcance; **prohibido** mutar `SddIA/{tools,skills,actions,process,agents,events,norms,library}` | Lab tubería |
| **L7** | Cierre documental Done | Exige PBI físico archivado + `validacion.md` APTO + `pbi_archived: true`. Con PBI ausente → **Done documental bloqueado**; lab CID puede verificar AC-L-* sin fingir Done | features-documentation-pattern v1.2.1 |
| **L8** | Soft-dep F3 / Tracker residual | **Fuera** de alcance | D2 / D7 |
| **L9** | APTO narrativo | Prohibido. Sin stdout/artefacto → `blocked` / `NO_APTO` | AC-DONE-LAB / L-NO-FAKE |

## 4. Contrato de artefactos (qué materializar)

| Artefacto | Owner | Obligatorio lab | Nota |
|-----------|-------|-----------------|------|
| `clarify.md` | Mayeuta | Sí (baseline) | CID en FM |
| `objectives.md` | Mayeuta | Sí (baseline) | CID en FM; fuente `refined_requirements` |
| `spec.md` | Dedalo | Sí | Este documento (v1.1.2) |
| `plan.md` | Dedalo | Sí | Blueprint T-GATE…T4 |
| `implementation.md` | Tekton | Sí | `items: []` / baseline documental si forja=0 |
| `execution.md` | Tekton | Sí | Tabla evidencia CID + resultado git-manager o blocked |
| `validacion.md` | Argos | Sí | APTO solo con checks físicos |
| PBI pending→done | Cumulo/operador + cierre documental | Gate Done, no gate AC-L-CID | Path referenciado ausente hoy |

**Topología (lógica, SSOT `cumulo.paths.json`):**

| Clave | Valor |
|-------|-------|
| `paths.featurePath` | `docs/features` |
| `persist_ref` | `docs/features/plumb-cid` |
| `directories.documentation` | `docs` |
| Soft-dep (fuera) | residuales Tracker / F3 git-manager KM |

## 5. Criterios de aceptación (Argos)

| ID | Criterio | Evidencia física |
|----|----------|------------------|
| **AC-L-CID** | Mismo `correlation_id` en frontmatter de `clarify.md` y `objectives.md`; cascada Dedalo/Tekton propaga el mismo valor en FM tocados | Lectura FM bajo `persist_ref` |
| **AC-L-DOC** | Artefactos con frontmatter `features-documentation-pattern`; `spec.md`+`plan.md`+`implementation.md`+`execution.md` presentes | Paths físicos |
| **AC-L-PBI** | Gap PBI documentado en cascada; **ningún** agente ejecución escribe bajo `docs/todos/` | Ausencia de writes KM desde Tekton/Argos/Dedalo |
| **AC-L-GIT** | Stdout `skill:git-manager` (`./sddia-run.sh --tool git-manager`) **o** declaración explícita `git_evidence: not_materialized` / blocked | JSON stdout o entrada honesta en `execution.md` |
| **AC-DONE-LAB** | `validacion.md` `global: APTO` solo si AC-L-* verdes con evidencia; sin inventar | Argos |

**Nota Done vs lab:** AC-DONE-LAB verifica honestidad del lab. El **Done de proceso feature** (PBI en `done/` + `pbi_archived: true`) queda **bloqueado** hasta materialización Cumulo del PBI (L7).

## 6. RBAC ejecutor (Tekton) — cruce mecánico

`target_executor_rbac` esperado (homólogo a `agent:tekton` / process `feature` context):

```json
{
  "allowed_policies": ["ecosystem-evolution", "filesystem-ops", "source-control"]
}
```

| Cápsula | `context` YAML | ¿Permitida? |
|---------|----------------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | Sí |
| `skill:git-manager` | `source-control` | Sí |
| `skill:shell-executor` | `system-operations` | **No requerida** (forja=0; sin build) |
| `action:execute-process` | (cierre) | Solo fase cierre entrega; fuera del mínimo lab evidencia |

Si el runtime **no** otorga `source-control`: T-GATE → blocked honesto; **prohibido** bypass Shell destructivo.

## 7. Límites duros

- No inventar PBI ni semillas Kaizen desde Tekton/Argos/Dedalo.
- No absorber F3 git-manager residual / Tracker / pasarela async / DI / GesFer.
- No declarar APTO sin evidencia física.
- No mutar genoma Core como alcance de este lab.
- Dedalo no finge stdout git de esta fase (`git_evidence_dedalo: not_materialized_shell_rejected`).

## 8. Veredicto Dedalo

**ok** — requisitos lab estables (Mayeuta D9 / exec 8d69c53d); blueprint mínimo viable frente a RBAC Tekton (`filesystem-ops` + `source-control`). Hueco PBI = bloqueo de Done documental, no de diseño. Evidencia git Dedalo: **not_materialized** (Rejected IDE/cápsula, sin stdout).
