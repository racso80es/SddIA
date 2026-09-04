---
feature_name: gemini-http-infer-live-activation
created: "2026-09-04"
process: feature
base: main
scope: gemini-http-infer-live-activation
version_spec: "1.0.0"
pbi_uuid: "d7f1f8aa-0d51-4b2d-ac27-ed17c8a23d09"
pbi_version: "1.2.0"
execution_id: "0926e45d-db83-42ea-8a5b-3bafcdb00b57"
---

# Especificación — gemini-http-infer-live-activation

SSOT: PBI v1.2.0. No reabre H1–H12 del padre salvo residuo argv H4.

## 1. Tool `gemini-http-infer`

| Cambio | Contrato |
|--------|----------|
| 4xx | `send_string` → `Err(Status(code, resp))`. Leer JSON. `no longer available` / modelo NOT_FOUND → `gemini-model-unavailable: {message}`. Resto `http-status-{code}: {body}`. |
| Modelo | `request.model` no vacío ≻ `SDDIA_GEMINI_MODEL` ≻ error. |
| Vacío | POST 200 con `extract_text` None/vacío → Err `gemini-empty-candidate: finishReason=…`. Mock lab intacto. |
| Lab | `SDDIA_LAB_MOCK_OUTBOUND` sin URL mock → `lab-mock:`. Tests sin red. |

## 2. Skill `antigravity-cli-executor`

| Cambio | Contrato |
|--------|----------|
| argv | No `--print` como flag posicional antes de `--output-format`. Orden mínimo: `--output-format json`, `--sandbox` xor skip dual-opt-in, whitelist, `-p` + prompt. |
| Auth | `stdout`/`stderr` case-insensitive: `authentication required` **o** `not logged into antigravity`. |
| Humo | ELF stdin JSON. Login ausente → `success: false` accionable. |

## 3. Instancia

Starter-kit `.env.example`: `GEMINI_API_KEY`, base URL, timeout, `SDDIA_GEMINI_MODEL` **comentados**. Cero secretos. Cero slug como valor-ley.

Humo HTTP: `./sddia-run.sh --tool gemini-http-infer` (carga bóveda).

## 4. DI

Cero `provides`. `llm:interact` intacto.
