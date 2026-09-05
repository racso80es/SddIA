---
document_id: PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS
uuid: "1540ab52-4354-49a6-9d4e-63135aaccde2"
title: "[KAIZEN] Diff vía git-manager y evaluación de síntesis LLM post-merge"
format: markdown
version: "1.0.0"
created: "2026-09-05"
updated: "2026-09-05"
status: abierto
refinement_status: refinado
priority: media
process: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-git-diff-llm-synthesis
persist_ref_suggested: docs/features/kaizen-git-diff-llm-synthesis
parent_pbi: docs/todos/done/PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED.md
parent_document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
parent_uuid: "5880d6fc-99f3-4ecf-8c9e-a4885d45f117"
friction_ids:
  - F-GIT-MANAGER-NO-DIFF-CONTENT
  - F-LLM-SYNTHESIS-NO-EVAL-HARNESS
tech_debt_ids:
  - DT-GIT-MANAGER-DIFF-SHOW
  - DT-LLM-SYNTHESIS-GOLDEN
derived_from:
  - PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
related:
  - SddIA/norms/skill-io-git-manager-frozen.md
  - SddIA/skills/git-manager.md
  - SddIA/tools/gemini-http-infer.md
  - SddIA/actions/notify-humanized-pr-merged.md
  - SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs
---

# [KAIZEN] Diff git-manager + evaluación de síntesis LLM

Deuda explícita de **ola 2** de `PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED`. No se implementa aquí. Prohibido descongelar `git-manager` ni «entrenar» un modelo desde este PBI.

## 0. Filtro A

| Afirmación | Verdad |
|------------|--------|
| El payload `PullRequest_Merged` trae diffs | Falso. ECST v1.0.0 no tiene commits/diffs. |
| `git-manager` puede hacer `show`/`log`/`diff` de contenido | Falso. Enum congelado v1.1.0: `diff_name_only` = nombres; `get_last_commit` = `rev-parse`. |
| Hay que fine-tunear Gemini en el Core | Falso. No hay pipeline de entrenamiento. Mitigación vigente = prompt anti-conjetura + fail-soft. |
| Ampliar el payload ECST 1.1.0 con el diff | Alcance distinto (Clase + emisores `accept-pr` / `emit-pr-merged-event`). No mezclar con un op git. |

## 1. F-GIT-MANAGER-NO-DIFF-CONTENT

**Hecho:** la síntesis post-merge usa solo campos ECST. El valor de negocio fino (qué archivos, qué intención) no está en el sello.

**Cambio canónico (futuro, proceso `feature` + `norm-creator`):**

1. Evolucionar `skill-io-git-manager-frozen` (descongelar con Cerbero) con un `operation_type` de **contenido acotado** (p. ej. `diff_name_only` ya existe; falta subject/`show` truncado, no un diff ilimitado).
2. `notify-humanized-pr-merged` consume esa salida **fail-soft** (si git-manager falla, prompt ECST-only).
3. Tope de bytes/timeout para no bloquear el fan-out IOTA.

**Fuera:** `git show` raw desde Tekton; `gh`; ampliar el enum sin norma.

## 2. F-LLM-SYNTHESIS-NO-EVAL-HARNESS

**Hecho:** no hay oracle de «síntesis correcta». El ejemplo canónico del PBI padre es plantilla de formato.

**Cambio canónico (futuro):**

- Golden set de eventos `PullRequest_Merged` + síntesis humana ≤2 líneas (lab).
- Test de contrato: truncado, ausencia de conjetura de files no listados, fail-soft HTTP.
- **No** fine-tune, no dataset de entrenamiento, no Vertex.

## 3. Criterios

| ID | Criterio |
|----|----------|
| K-DIFF-CA1 | Norma congelada versionada **antes** de que `git-manager` acepte el nuevo op. |
| K-DIFF-CA2 | Prompt LLM recibe nombres o subject truncado; fail-soft si la skill falla. |
| K-LLM-CA1 | Golden/eval en lab-mock; cero red obligatoria en `cargo test`. |
| K-LLM-CA2 | Ningún artefacto de «entrenamiento» bajo `SddIA/`. |

## 4. Fuera

Implementar en el PR #262. Mutar `send-telegram-notification`. Entrenar o hostear un modelo.
