---
uuid: "1cd7bd40-b72f-4114-ac44-68b912774aa6"
name: "notify-humanized-pr-merged"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "notify-humanized-pr-merged"
  - "delegate-gemini-http-infer"
  - "delegate-send-telegram-notification"
inputs:
  - "event_type": "string; PullRequest_Merged"
  - "payload": "object; ECST"
  - "correlation_id": "string; envelope"
outputs:
  - "success": "boolean"
  - "synthesized": "boolean"
  - "skipped": "boolean"
hash_signature: "sha256:126a051b2b326109a93a20f2053692117445d06996637bcd2fa99b810e4fa902"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: notify-humanized-pr-merged

## 1. Propósito

Suscriptor Argos de `PullRequest_Merged`. Ensambla el bloque estático (compositor ola 1) con una síntesis de valor (≤2 líneas) obtenida vía `gemini-http-infer`. Fail-soft: si el LLM falla, timeout o HTTP error, envía solo el estático. Un merge → un `send-telegram-notification`.

El texto anti-verbosidad del prompt **no** es el prefijo creator de `external-ai-constraints`.

## 2. Orquestación

### Paso 1 — Estático

Construir el bloque con el **evento completo** (envelope `correlation_id` + payload ECST). No leer commits/diffs del payload (no existen en la Clase v1.0.0).

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
```

`temperature` ≤ 0.2. `model` desde `SDDIA_GEMINI_MODEL` (si ausente, omitir síntesis). Un intento. Sin retry.

### Paso 3 — Ensamblaje

Si hay texto no vacío: concatenar `\n\n🧠 Síntesis de Valor: {texto truncado a 2 líneas}`. Despachar `send-telegram-notification`.

### Paso 4 — Cierre

Envelope `{success: true, synthesized, skipped}`. Fallo Telegram → error del suscriptor. Fallo Gemini → `synthesized: false`, `success: true`.

## 3. Límites

* No segundo suscriptor `tool: send-telegram-notification` en el mismo evento (doble envío).
* No ampliar `git-manager` ni versionar ECST con diffs.
* No colapsar el fan-out IOTA.
* Handler nativo en `execute-process` (`notify_humanized_pr_merged`).
