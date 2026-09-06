---
feature_name: kaizen-git-diff-llm-synthesis
created: "2026-09-06"
process: feature
base: main
scope: core
document_id: PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS
execution_id: "00214f28-2a66-4597-8222-6fdc31250d16"
---

# Spec — kaizen-git-diff-llm-synthesis

PBI v1.2.0. Enriquecimiento Git del prompt de `notify-humanized-pr-merged` sin tocar ECST v1.0.0.

## `commit_summary` (frozen v1.2.0)

Payload exacto:

```json
{"ref": "string", "max_files": 30, "max_subject_chars": 200}
```

`max_files` ∈ [1, 30]. `max_subject_chars` ∈ [1, 200]. `ref` vía `assert_safe_token`.

`data`: `commitHash` (40 hex), `subject` (recortado), `files` (≤ max_files), `totalFilesChanged`, `truncated`, más `gitStdout`/`gitStderr`/`errorSummary`.

Git (args lista, sin shell):

1. `rev-parse --verify <ref>`
2. `show -s --format=%s <ref>`
3. Tras `rev-parse --verify <ref>^`: `diff --name-only -M <ref>^ <ref>`

`<ref>^` ausente → exit ≠ 0. Handler fail-soft.

## Handler

`invoke_git_manager(repo, "commit_summary", {ref: merge_commit_hash, max_files: 30, max_subject_chars: 200})` **antes** de Gemini.

Éxito → `build_synthesis_prompt(event, Some(summary))` inyecta:

```text
SUBJECT: …
FILES: a, b, c
truncated=true totalFilesChanged=N   # solo si truncated
```

Fallo → prompt ola 2 (ECST-only). Telegram/Gemini invariantes.

## Action

EM update: `delegate-git-manager`; Paso 1.5 Git; retirar «No ampliar git-manager». Inputs EDA intactos.

## Tests

Crate `git-manager`: payload_exact, token inseguro, rangos, subject recorte, truncado files. Repo fixture mínimo.

Lib `notify_humanized_*`: SUBJECT/FILES solo del fixture; sin ellos si Git Err; truncado 2 líneas/400; fail-soft Gemini; no-regresión topología.

## Fuera de spec

`diff_name_only` como atajo; timeout `capsules.rs`; `norm-creator`; EM skill; `cumulo.paths.json`; parche unificado; juez LLM.
