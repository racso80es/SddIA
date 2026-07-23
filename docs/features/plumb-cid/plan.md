---
feature_name: plumb-cid
created: "2026-07-23"
updated: "2026-07-23"
process: feature
phases: [T-GATE, T0, T1, T2, T3, T4]
document_id: LAB-PLUMB-CID
branch_name: feat/plumb-cid
persist_ref: docs/features/plumb-cid
pbi_ref: docs/todos/pending/[FEATURE] plumb-cid.md
correlation_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
phase: Diseño de Blueprint
agents: dedalo
version_plan: "1.0.0"
---

# Plan — plumb-cid

Blueprint de **evidencia CID + gates no-fake**. Sin producto de dominio; sin forja genoma; sin escritura en `docs/todos/` desde agentes de ejecución.

## Fases

### T-GATE — Unlock git (preflight Tekton)
- name: Unlock source-control
- intent: Verificar que el ejecutor invoca `skill:git-manager` sin Rejected antes de narrar evidencia git.
- delegates_to:
  - skill:git-manager
- checklist:
  - [ ] `target_executor_rbac.allowed_policies` incluye `source-control` y `filesystem-ops`
  - [ ] Smoke: `./sddia-run.sh --tool git-manager` stdin JSON `operation_type=status` + `repository_path` (Cúmulo) + `operation_payload_json={}` → stdout físico
  - [ ] Si Rejected → `execution.md` `verdict: blocked` + `block_reason`; **no** inventar AC-L-GIT verde

### T0 — Documentación Dedalo
- name: Diseño blueprint
- intent: Materializar `spec.md` + `plan.md` bajo `persist_ref` desde `objectives.md` como `refined_requirements`.
- delegates_to:
  - skill:filesystem-manager
- checklist:
  - [x] Consumir `objectives.md` / `clarify.md` (D0–D8, Q1–Q4)
  - [x] `spec.md` v1.0.0 laudos L1–L9
  - [x] este `plan.md` v1.0.0 (T-GATE…T4)
  - [x] Declarar git Dedalo `not_materialized` (sin source-control en RBAC agente)

### T1 — Auditoría plumb CID (baseline Mayeuta)
- name: Verificar identidad CID
- intent: Confirmar AC-L-CID sobre artefactos ya estabilizados; no reescribir requisitos.
- delegates_to:
  - skill:filesystem-manager
- checklist:
  - [ ] Leer frontmatter `clarify.md` y `objectives.md`
  - [ ] Assert `correlation_id` idéntico = `a1b2c3d4-e5f6-4789-a012-3456789abcde`
  - [ ] Confirmar `persist_ref` = `docs/features/plumb-cid` y patrón FM mínimo
  - [ ] Documentar gap PBI ausente (AC-L-PBI) sin crear el archivo

### T2 — Cascada Tekton documental
- name: Persistencia implementation + execution
- intent: Cerrar artefactos de ejecución con evidencia CID y honestidad git; forja código = 0.
- delegates_to:
  - skill:filesystem-manager
- checklist:
  - [ ] `implementation.md` — `items: []` / baseline documental; declarar no mutación genoma
  - [ ] `execution.md` — tabla AC-L-CID/DOC/PBI/GIT + cid + resultado T-GATE
  - [ ] Propagar `correlation_id` en frontmatter de ambos
  - [ ] **No** escribir bajo `docs/todos/`

### T3 — Evidencia git
- name: Captura git-manager
- intent: Materializar stdout status (y diff si aplica) vía cápsula; sin bypass Shell.
- delegates_to:
  - skill:git-manager
- checklist:
  - [ ] `operation_type: status` → capturar JSON stdout en `execution.md`
  - [ ] Opcional: confirmar rama `feat/plumb-cid` vía salida parseada (sin inventar)
  - [ ] Si falla → `git_evidence: not_materialized` / blocked (AC-L-GIT honesto)

### T4 — Handoff Argos
- name: Preparar verificación no-fake
- intent: Dejar checklist Argos acotado a evidencia física; no pre-sellar APTO.
- delegates_to:
  - skill:filesystem-manager
- checklist:
  - [ ] Checklist Argos = AC-L-CID, AC-L-DOC, AC-L-PBI, AC-L-GIT, AC-DONE-LAB
  - [ ] Recordar L7: Done documental bloqueado mientras PBI ausente (Cumulo)
  - [ ] Prohibido `global: APTO` narrativo

### T-PBI (condicional, fuera de Tekton)
- name: Materializar PBI (Cumulo/operador)
- intent: Solo Cumulo / evento `Kaizen_Alert_Required` / operador crea `docs/todos/pending/[FEATURE] plumb-cid.md` si se quiere Done de proceso.
- delegates_to: []
- checklist:
  - [ ] PBI físico con `document_id: LAB-PLUMB-CID` coherente
  - [ ] Tras APTO lab + archivo → move a `docs/todos/done/` en rama (cierre documental)

## Orden de ejecución

```text
T0 (Dedalo) [hecho]
  → T-GATE (git-manager unlock)
       → ok → T1 (audit CID) → T2 (docs Tekton) → T3 (git capture) → T4 (handoff Argos)
       → fail → execution.md blocked; T1 puede aún auditar CID local; AC-L-GIT = no verde
T-PBI || Cumulo (paralelo / previo a Done documental; no bloquea AC-L-CID)
```

## Delegación / RBAC (ejecutor Tekton)

| Fase | Cápsulas | Políticas mínimas | Notas |
|------|----------|-------------------|-------|
| T-GATE / T3 | `skill:git-manager` | `source-control` | Preflight + captura |
| T0 / T1 / T2 / T4 | `skill:filesystem-manager` | `filesystem-ops` | Docs bajo `persist_ref` |
| `skill:shell-executor` | — | — | **No** en este blueprint |
| KM / `docs/todos/` | Cumulo / `Kaizen_Alert_Required` | — | Tekton/Argos/Dedalo no siembran |

Cruce mecánico Dedalo:

| Cápsula | `context` YAML | Requiere en ejecutor |
|---------|----------------|----------------------|
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:git-manager` | `source-control` | sí |

Si falta alguna política → abortar con causa; **prohibido** bypass raw destructivo.

## Criterios de salida por fase

| Fase | Done local |
|------|------------|
| T-GATE | stdout git-manager o blocked honesto |
| T0 | `spec.md` + `plan.md` en `persist_ref` |
| T1 | Assert CID documentado en `execution.md` |
| T2 | `implementation.md` + `execution.md` con FM + cid |
| T3 | Evidencia git física o `not_materialized` explícito |
| T4 | Handoff Argos sin pre-APTO |

## Riesgos operativos

| Riesgo | Mitigación |
|--------|------------|
| IDE Rejected git-manager | T-GATE blocked; AC-L-GIT honesto |
| PBI ausente | AC-L-PBI documentado; Done proceso bloqueado (L7); no forjar desde Tekton |
| Tentación dominio / F3 #136 | Veto spec L8 |
| APTO narrativo | AC-DONE-LAB / L9 |

## Explicitamente no planificado

Forja PBI desde Tekton · residual F3 PPR #136 · pasarela async PBI-044 · DI · GesFer · mutación genoma · `shell-executor` · inventar stdout git · bajar AC-L-* por Rejected.
