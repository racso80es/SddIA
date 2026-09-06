---
uuid: "1cd7bd40-b72f-4114-ac44-68b912774aa6"
name: "notify-humanized-pr-merged"
version: "1.1.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "notify-humanized-pr-merged"
  - "delegate-gemini-http-infer"
  - "delegate-send-telegram-notification"
  - "delegate-git-manager"
inputs:
  - "event_type": "string; PullRequest_Merged"
  - "payload": "object; ECST"
  - "correlation_id": "string; envelope"
outputs:
  - "success": "boolean"
  - "synthesized": "boolean"
  - "skipped": "boolean"
hash_signature: "sha256:0a0490d95516cc9bd4af8a97d80a1bb9a6341d1346e4e7ced20f4518e03fe8f2"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: notify-humanized-pr-merged

## 1. Propósito

Suscriptor Argos de `PullRequest_Merged`. Ensambla el bloque estático (compositor ola 1) con una síntesis de valor (≤2 líneas) obtenida vía `gemini-http-infer`. Antes de Gemini, resuelve `git-manager commit_summary` (fail-soft). Fail-soft LLM: si falla, timeout o HTTP error, envía solo el estático. Un merge → un `send-telegram-notification`.

El texto anti-verbosidad del prompt **no** es el prefijo creator de `external-ai-constraints`.

## 2. Orquestación

### Paso 1 — Estático

Construir el bloque con el **evento completo** (envelope `correlation_id` + payload ECST). No leer commits/diffs del payload (no existen en la Clase v1.0.0).

### Paso 1.5 — Git (fail-soft)

Invocar `git-manager` `commit_summary` con `ref = payload.merge_commit_hash`, `max_files: 30`, `max_subject_chars: 200` vía `invoke_git_manager`. Si falla (ref ausente, shallow, root, parse): continuar sin SUBJECT/FILES.

### Paso 2 — Inferencia

Invocar `gemini-http-infer` con `request.prompt`:

```text
[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. MAX 2 LINES]
Return only business value of this merge. Do not restate hash, auditor, branch, or correlation.
Do not invent files, commits, or intent absent from CONTEXT.
CONTEXT:
source_branch=…
target_branch=…
merge_commit_hash=…
author=…
auditor=…
policy=…
SUBJECT: …          (omit if git fail-soft)
FILES: a, b, c      (omit if git fail-soft)
truncated=true totalFilesChanged=N   (omit if not truncated)
```

`temperature` ≤ 0.2. `model` desde `SDDIA_GEMINI_MODEL` (si ausente, omitir síntesis). Un intento. Sin retry.

### Paso 3 — Ensamblaje

Si hay texto no vacío: concatenar `\n\n🧠 Síntesis de Valor: {texto truncado a 2 líneas}`. Despachar `send-telegram-notification`.

### Paso 4 — Cierre

Envelope `{success: true, synthesized, skipped}`. Fallo Telegram → error del suscriptor. Fallo Gemini o Git → no colapsa el suscriptor (Git solo omite enriquecimiento; Gemini omite síntesis).

## 3. Límites

* No segundo suscriptor `tool: send-telegram-notification` en el mismo evento (doble envío).
* No versionar ECST con diffs.
* No colapsar el fan-out IOTA.
* Handler nativo en `execute-process` (`notify_humanized_pr_merged`).
* Git = enriquecimiento interno; no es input EDA.
