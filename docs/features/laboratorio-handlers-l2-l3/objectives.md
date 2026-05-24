---
feature_name: laboratorio-handlers-l2-l3
process: feature
created: "2026-05-24"
persist_ref: docs/features/laboratorio-handlers-l2-l3
branch_name: feat/laboratorio-handlers-l2-l3
related_todo: docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
tracks:
  - L.2
  - L.3
status: implementado
updated: "2026-05-24"
feature_ref: docs/features/laboratorio-handlers-l2-l3
---

# Objetivos — Laboratorio handlers L.2 + L.3

## Meta

Completar el perfil **laboratorio** de `delivery-close-cycle` (L.2) y `feature` (L.3): sustituir fases `simulated` restantes por **handlers mínimos físicos** o **gates documentados**, con `execution_report` honesto. Los agentes IDE (Mayeuta, Dedalo, Tekton, Argos) permanecen fuera de alcance lab.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| PR #11 entregó handlers L.2 fases 4–7 | Push, `gh`, sello Presented, higiene operativos |
| `capsule_eda_genomic_audit_gate` en fase 3 | Aduana EDA genómica ya física |
| `capsule_delivery_snapshot_final` en fase 1 | Snapshot final ya físico (skip vía `SDDIA_LAB_SKIP_SNAPSHOT`) |
| Fase 2 «Impacto SddIA condicional» | Solo `agent:argos` → **`simulated`** |
| `feature` fase 1 `workspace-init` | ✅ física desde PR #9 |
| `feature` fases 2–5 | Agentes IDE → **`simulated`** (deseable, no bug) |
| `feature` fases 6–7 | Sin handler → **`simulated`** / falso positivo potencial |
| PBI post-PR11 § P2 | L.2 y L.3 Prioridad 2 tras vanguardia P1 |

## Objetivos medibles

### Track L.2 — `delivery-close-cycle`

| ID | Objetivo | Criterio |
|----|----------|----------|
| **L2-O1** | **Gate Impacto SddIA** | Fase 2 con handler `delivery-impact-assessment`: detecta mutaciones bajo `SddIA/` vía diff; no-op documentado si `source_process != feature` |
| **L2-O2** | **Regresión fases físicas** | Fases 1, 3–7 mantienen `status: executed`; smokes PR #11 siguen pasando |
| **L2-O3** | **Genoma alineado** | `delivery-close-cycle.md` § Perfil laboratorio documenta fase 2 |

### Track L.3 — `feature`

| ID | Objetivo | Criterio |
|----|----------|----------|
| **L3-O1** | **Cierre documental físico** | Fase 6 invoca gate que valida `validacion.md` (`global: APTO`, `pbi_archived: true`) y mueve PBI a `docs/todos/done/` si aplica |
| **L3-O2** | **Cierre entrega físico** | Fase 7 invoca subproceso `delivery-close-cycle` vía `invoke_subprocess_process`; propaga `pr_url`, `event_id` |
| **L3-O3** | **Honestidad agentes** | Fases 2–5 permanecen `simulated` con nota canónica `agentes IDE; sin handler físico en laboratorio` |
| **L3-O4** | **Genoma alineado** | `feature.md` § Perfil laboratorio actualizado con matriz fase × handler |

## Orquestación

- **L.2** y **L.3** implementables en **paralelo** (tracks independientes, un PR unificado).
- **Precedencia:** L.2 cierra brecha en cierre de entrega; L.3 cablea el proceso `feature` completo hasta `delivery-close-cycle`.
- **No reabrir:** handlers entregados en `pr-presented-orchestration`, vanguardia, `accept-pr`.

## No objetivos (esta feature)

- Implementar agentes V5 (Mayeuta, Dedalo, Tekton, Argos) en `execute-process.py`.
- Perfil runtime IDE completo — solo laboratorio.
- Modificar hooks Hito 3 ni cadena `pull-request-review` → `accept-pr`.
- L1-O5 runbooks (residual vanguardia P1) — feature aparte o Kaizen posterior.

## Artefactos previstos

| Track | Rutas principales |
|-------|-------------------|
| L.2 | `execute_process_capsules.py`, `SddIA/process/delivery-close-cycle.md` |
| L.3 | `execute_process_capsules.py`, `SddIA/process/feature.md` |
| Feature | `clarify.md`, `spec.md`, `plan.md`, smoke JSON, `validacion.md` |

## Estado

| Fase feature | Estado |
|--------------|--------|
| Objetivos | ✅ Este documento |
| Clarificación | ✅ `clarify.md` |
| Especificación | ✅ `spec.md` |
| Plan | ✅ `plan.md` |
| Implementación | ✅ `implementation.md` |
| Validación | ✅ `validacion.md` + `execution.md` |
