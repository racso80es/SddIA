---
document_id: PBI-MERGE-THERMO-04-ATTESTATION
uuid: "8484cd0c-dee1-434b-a507-7278a16c61e3"
title: "[OPERATIVO] Aduana Git 04 — Atestación de QA"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 4
hu_order_total: 6
historia_ref: "docs/todos/historias/[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia.md"
historia_document_id: HU-MERGE-THERMODYNAMICS
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-MERGE-THERMO-03-DELTA-PROFILE
unblocks:
  - PBI-MERGE-THERMO-06-E2E-MEASURE
baseline_decisiones:
  - "D-2: la invalidación es un paso de accept-pr Fase 4, no un suscriptor nuevo de PullRequest_Merged."
  - "Ancla tree_sha. head_sha se conserva como dato, no como condición de validez."
  - "HMAC opcional en v1. Obligatorio solo si instances.json declara la instancia multiusuario."
  - "No es ZKP. Es un testigo local anclado a hash."
---

# Atestación de QA

HU `HU-MERGE-THERMODYNAMICS`, orden **04/06**. HU §2.2. AC-2, AC-3, AC-4.

## 0. Filtro A

| Afirmación a evitar | Corrección |
|---|---|
| Directorio `.SddIA/proofs/audits/` | No existe. Namespace `qa-attestations/` bajo `resolve_eda_proofs_dir` (`eda_instance.proofs`). |
| Validez por `head_sha` | Un amend sin cambio de árbol invalidaría una revisión ya hecha. La condición es `tree_sha`. |
| Suscriptor nuevo para borrar la prueba | `accept-pr` ya sella `PullRequest_Merged` y limpia la rama. El borrado va en su Fase 4 (D-2). |

## 1. Intención

Una revisión aprobada se reutiliza mientras el árbol no cambie. Si la prueba falta, miente o caduca, la aduana corre entera.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `pull-request-review` con `verdict: aprobado` escribe `{proofs}/qa-attestations/{branch_slug}.json`: `schema` `qa-attestation/1.0`, `branch`, `head_sha`, `tree_sha`, `verdict`, `qa_profile`, `issued_at`, `ttl_secs` 86400, `correlation_id`, `execution_id`. `hmac` solo si la instancia es multiusuario. |
| R-2 | `pre-push` omite `Local_QA_Requested` y sigue a `delivery-close-cycle` solo si: el fichero existe, `tree_sha` igual a `git rev-parse <local_sha>^{tree}`, `verdict == aprobado`, TTL vigente, y (`qa_profile == full` o delta pasivo). Cualquier otra condición, incluida lectura fallida, ejecuta la cadena completa. |
| R-3 | `post_merge_gate.sh` pasa `attestation_ref`. `accept-pr` con `merge_already_done` y atestación válida registra *Auditoría Genómica* como `executed` con `note: attested-by:<execution_id>` y sin LLM. |
| R-4 | Fase 4 de `accept-pr` borra la atestación de `source_branch` al sellar `PullRequest_Merged`. Un push posterior con el mismo nombre de rama no la reutiliza. |
| R-5 | Perfil `docs-only` no autoriza saltar la cadena si el delta es activo. |
| R-6 | Bumps vía `entity-manager`: `pull-request-review` v2.4.0 (`qa_profile`, `attestation_path`); `accept-pr` v1.1.0 (`attestation_ref` opcional). Registro en `SddIA/evolution/`. |
| R-7 | `hook-timings` rellena `attestation_hit` (bool). |

## 3. Plan

1. Escritura en el cierre aprobado de `pull-request-review`.
2. Lectura fail-closed en `pre_push_gate.sh`.
3. Salto de *Auditoría Genómica* y borrado en `accept-pr`.
4. Red team: cinco mutaciones del JSON, cinco cadenas completas.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-2 | Atestación válida de perfil `full`: `pre-push` no emite `Local_QA_Requested`; `post-merge` no invoca LLM en *Auditoría Genómica*. | Proof sembrada + `execution_report` `note: attested-by` |
| AC-3 | `tree_sha` distinto, `verdict` distinto de `aprobado`, caducada, HMAC inválido (si aplica) o ausente → cadena completa. Error de lectura → cadena completa. | 5 mutaciones, 5 bloqueos del atajo |
| AC-4 | Tras `PullRequest_Merged` el fichero no existe. Reutilizar el nombre de rama no recupera el atajo. | Test de bus + filesystem |
| AC-ATT-1 | Atestación `docs-only` con delta activo no omite `Local_QA_Requested`. | Test shell |
