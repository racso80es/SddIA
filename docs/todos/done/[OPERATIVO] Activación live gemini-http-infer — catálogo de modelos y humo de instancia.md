---
document_id: PBI-GEMINI-HTTP-INFER-LIVE-ACTIVATION
uuid: "d7f1f8aa-0d51-4b2d-ac27-ed17c8a23d09"
title: "[OPERATIVO] Activación live gemini-http-infer — catálogo de modelos y humo de instancia"
format: markdown
version: "1.2.0"
created: "2026-09-04"
updated: "2026-09-04"
status: done
refinement_status: refinado
priority: alta
process: feature
executor_vehicle: feature
type: operativo
dispatch: false
suggested_branch: feat/gemini-http-infer-live-activation
persist_ref: docs/features/gemini-http-infer-live-activation
branch_name: feat/gemini-http-infer-live-activation
persist_ref_suggested: docs/features/gemini-http-infer-live-activation
parent_pbi: docs/todos/done/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md
parent_document_id: PBI-CAPSULES-ANTIGRAVITY-NATIVE
parent_uuid: "7f966f32-5502-4bd7-b252-44849f29f5d3"
depends_on:
  - PBI-CAPSULES-ANTIGRAVITY-NATIVE
related:
  - docs/todos/done/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md
  - docs/features/antigravity-connectors/spec.md
  - docs/features/antigravity-connectors/validacion.md
  - SddIA/tools/gemini-http-infer.md
  - SddIA/tools/gemini-http-infer/src/main.rs
  - SddIA/skills/antigravity-cli-executor.md
  - SddIA/skills/antigravity-cli-executor/src/main.rs
  - SddIA/engine/execute-process/src/main.rs
  - SddIA/core/cumulo.paths.json
  - SddIA/norms/capsule-json-io.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/scripts/starter-kit/.SddIA/.dev/.env.example
  - SddIA/scripts/starter-kit/.dev/.env.example
  - docs/todos/pending/PBI-ARQUITECTURA-LLM-TIERS.md
refinement_notes: "Filtro A v1.2.0 (2026-09-04). Corrige v1.1: latencias 1.8s/21.3s, gemini-3.8-flash 503 y gemini-3.5-flash ~8s no son SSOT (no reproducidos en esta pasada; no CA). MAX_TOKENS+text vacío se observó en sonda urllib con maxOutputTokens:8, no en el crate (el tool no envía ese campo). thought:true no observado. finishReason!=STOP como fallo universal es excesivo. Display ureq 2.12 es `{url}: status code {N}` (cuerpo 4xx descartado; `if status>=400` código muerto — eso sí es hecho de crate). agy: `--print` consume el siguiente argv (error literal confirmado); host no logueado (`You are not logged into Antigravity`) vs crate que solo busca `authentication required`. Cargo: `cd SddIA && cargo build -p …` (DA-3), no solo --manifest-path. Mutación crates = entity-manager (DA-2). Humo skill live SUCCESS no es gate."
---

# [OPERATIVO] Activación live gemini-http-infer — catálogo de modelos y humo de instancia

Residual de `PBI-CAPSULES-ANTIGRAVITY-NATIVE` (`7f966f32-5502-4bd7-b252-44849f29f5d3`). El padre forjó las EDs y cerró **laboratorio**. Este PBI cierra instancia live + defectos de integración contrastados contra crate, `ureq` 2.12.1 y `agy` host.

## Mandato

1. Plantillas starter-kit: documentar `GEMINI_API_KEY`, `SDDIA_GEMINI_API_BASE_URL`, `SDDIA_GEMINI_HTTP_TIMEOUT_SECS` y (si L-MODEL) `SDDIA_GEMINI_MODEL` **como comentario fechado**, no como marca eterna. Cero secretos.
2. Build: `cd SddIA && cargo build -p gemini-http-infer -p antigravity-cli-executor` (forma DA-3). ELF en `SddIA/target/debug/`.
3. Humo HTTP live: `./sddia-run.sh --tool gemini-http-infer` (carga `env_hierarchy`). Slug = uno que POST acepte **ese día**. No gate de latencia.
4. 4xx Google: capturar `ureq::Error::Status` y devolver el JSON de error en el sobre 2.0 (`success: false`). El `if status >= 400` actual es inalcanzable.
5. HTTP 200 + `result.text` vacío: `success: false`. No exigir `finishReason == STOP` como único éxito si hay texto usable.
6. Skill argv: no emitir `--print` bare antes de `--output-format`. Usar `-p "<prompt>"` (o `--print='<prompt>'`) **más** `--output-format json` y `--sandbox` por defecto.
7. Skill auth: detectar también `not logged into antigravity` (texto real del CLI). Humo SUCCESS print-mode = extra, no gate. Mutación de crates vía `entity-manager` (DA-2), no Write IDE.

## 0. Hallazgos Filtro A (v1.1.0 → v1.2.0)

| # | Afirmación v1.1.0 | Veredicto | Hecho |
|---|-------------------|-----------|-------|
| A1 | `gemini-3.1-flash-lite` ~1.8s; `gemini-3.6-flash` 21.3s / 181 thinking; `gemini-3.8-flash` 503; `gemini-3.5-flash` ~8s | **No SSOT** | No hay traza en repo ni reproducción en esta pasada. Sonda previa (sesión 2026-09-04): `gemini-2.5-flash` / `2.0-flash` / `2.5-flash-lite` → 404 *no longer available to new users*; `gemini-3.6-flash` → 200; `GET /v1beta/models` 200 (catálogo incluye `gemini-3.1-flash-lite`). Latencias y 503 **no** son CA. |
| A2 | CA `latencia < 5s` | **Incoherente** | Red, cuota y thinking varían. Quitar del Done. |
| A3 | 200 + text vacío «comprobado en el crate» con `MAX_TOKENS` / `thoughtsTokenCount` | **Mezcla de superficies** | El crate **no** envía `generationConfig.maxOutputTokens`. La sonda con techo 8 tokens fue urllib, no el ELF. Hecho de **código**: `extract_text(…).unwrap_or_default()` + `emit_v2(true, …)` → falso `success: true` si `parts` vacíos. Eso sí se parchea. |
| A4 | Omitir parts `"thought": true` como hallazgo empírico | **No observado** | Defensivo opcional en spec; no CA ni «comprobado». |
| A5 | Fallo si `finishReason != "STOP"` | **Excesivo** | `MAX_TOKENS` con texto parcial puede ser éxito truncado. Gate: **text vacío** → `success: false`; incluir `finishReason` en feedback si existe. |
| A6 | Display ureq = `http-post-failed: status code 404` | **Inexacto en la cadena** | `ureq` 2.12.1 `Error::Status`: `"{url}: status code {N}"`. `map_err` envuelve en `http-post-failed: …`. El body 4xx **no** se lee. `if status >= 400` tras `send_string()?` es código muerto. |
| A7 | `--print` bare es bug; pin `agy` 1.1.25 | **Bug confirmado; pin falso** | Host: `--print --output-format json` → `Error: --print took "--output-format" as its prompt…` (también con `-p` *después*). `--output-format json --sandbox -p ping` **sí** entra en print-mode. Language server reportó 1.1.26. Tests del padre usan stub que ignora argv. No pin de versión en CA. |
| A8 | Crate mapea auth con `authentication required` | **Desalineado del CLI** | `agy` (host no logueado): `You are not logged into Antigravity.` `map_agy_result` no lo caza. Tras arreglar argv, el fail-soft de auth debe incluir esa frase (case-insensitive). |
| A9 | Build **exige** `--manifest-path SddIA/Cargo.toml` | **Incompleto** | Workspace = `SddIA/Cargo.toml`. DA-3: `cd SddIA && cargo build -p …`. Ambas formas válidas. Desde raíz del repo sin `-C`/`--manifest-path` falla. |
| A10 | Humo skill = ELF directo; CA SUCCESS contra `agy` | **Parcial / bloqueante** | Confirmado: `execute-process` CLI = `--process` \| `--tool` \| `--action`. No `--skill`. `--tool` llama `load_hierarchical_env` + `invoke_tool_capsule_json` (label `tool`). Skills: `invoke_capsule_json` interno (`delegates_to`) o ELF. ELF **no** carga bóveda. Host actual **no** está logueado → exigir `status: SUCCESS` / «pong» bloquea el PBI. |
| A11 | «Parchear `main.rs`» | **DA-2** | `SddIA/tools/` y `SddIA/skills/` son genoma. Update vía `entity-manager` en ciclo `feature`. Starter-kit (`SddIA/scripts/starter-kit/`) **no** está en la tabla DA-2. |
| A12 | `SDDIA_GEMINI_MODEL=gemini-3.1-flash-lite` en example como valor | **Putrefacción de catálogo** | Misma clase de error que hardcodear `gemini-2.5-flash`. Example: clave vacía + comentario fechado. Fallback env sí (L-MODEL); el slug lo elige la instancia el día del humo. |
| A13 | YAML `related` con `PBI-ARQUITECTURA-LLM-TIERStitle:` | **Sintaxis** | Dos puntos sin comillas rompen YAML. Cadena entrecomillada (ya en frontmatter). |

## 1. Superficie

| Capa | Estado | Mutación |
|------|--------|----------|
| Tool `gemini-http-infer` | Lab APTO; 4xx sin body; 200+text vacío = success | `entity-manager` update: `Error::Status`; L-MODEL; text vacío → fallo. Tests lab sin red. |
| Skill `antigravity-cli-executor` | `build_argv` emite `--print` + `--output-format` + `-p`; auth substring padre | `entity-manager` update: argv print-mode válido; matcher de login. Tests de argv (sin spawn `agy` real). |
| CLI orquestador | `--tool` sí; `--skill` no | **No** inventar `--skill` en este PBI. Humo skill: ELF o proceso que `delegates_to`. Documentar. |
| Starter-kit `.env.example` | Sin `GEMINI_*` | Write directo (no DA-2). Comentarios, cero secretos. |
| CI GHA | Mock lab | Prohibido POST a `generativelanguage.googleapis.com` y `agy` autenticado. |
| Bindings | `llm:interact` → mayeuta-llm | Intactos. |

## 2. Contrato de instancia

Orquestador inyecta `env_hierarchy` **antes** del spawn del **tool**. Cápsula ciega: no abre `.env`.

| Variable | Live tool | Rol |
|----------|-----------|-----|
| `GEMINI_API_KEY` | Obligatorio | Header `x-goog-api-key`. Solo bóveda instancia. |
| `SDDIA_GEMINI_API_BASE_URL` | No | Default crate ya: `https://generativelanguage.googleapis.com`. Path `/v1beta/models/{model}:generateContent`. |
| `SDDIA_GEMINI_HTTP_TIMEOUT_SECS` | No | Default 30, techo 300. |
| `SDDIA_GEMINI_MODEL` | L-MODEL | Si `request.model` ausente/vacío. Crate **sin** literal de marca. |
| `SDDIA_AGY_PATH` | Skill | Default `agy` en `PATH`. |
| `SDDIA_LAB_MOCK_OUTBOUND` | Lab | Humo live: apagado. |

**Prohibido:** `ANTIGRAVITY_API_KEY`, `ANTIGRAVITY_API_ENDPOINT`, `GEMINI_API_ENDPOINT`, key en JSON stdin, `.env` bajo `tools/` o `skills/`.

Humo tool:

```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "gemini-http-infer" },
  "request": { "prompt": "ping", "model": "<slug vigente>" }
}
```

`./sddia-run.sh --tool gemini-http-infer` + stdin. ELF: `SddIA/target/debug/gemini-http-infer`.

Humo skill (fail-soft): stdin al ELF `SddIA/target/debug/antigravity-cli-executor` **o** fase de proceso que ya invoque skill. No asumir carga de bóveda en ELF directo.

## 3. Línea de montaje

| Fase | Qué | Cómo |
|------|-----|------|
| A | Plantillas | Starter-kit global + instancia: bloque Gemini comentado; skill `agy` = sesión cacheada ≠ API key. |
| B | Crates | `./sddia-run.sh --process entity-manager` update tool + skill. Ver L-*. Tests unitarios: argv sin `--print` bare precediendo `--output-format`; ureq Status con body mock; text vacío → Err. |
| C | Build | `cd SddIA && cargo build -p gemini-http-infer -p antigravity-cli-executor`. |
| D | Humo HTTP | `--tool` + key instancia. Modelo = POST 200 ese día (no `gemini-2.5-flash` si 404). Control: mismo payload con slug 404 → feedback con mensaje Google. |
| E | Humo skill | (1) argv: `agy` con el vector corregido no emite el error `--print took`. (2) Si no hay login: sobre `success: false` accionable. (3) Print SUCCESS = extra. |
| F | Cierre | PBI → `docs/todos/done/` en la rama; `validacion.md` `pbi_archived: true`. Un PR. |

## 4. Criterios de aceptación

* [ ] Starter-kit documenta las vars Gemini y el fallback de modelo **sin** secretos y **sin** slug vendido como eterno.
* [ ] `cd SddIA && cargo build -p gemini-http-infer -p antigravity-cli-executor` produce ambos ELF en `SddIA/target/debug/`.
* [ ] Humo live `--tool gemini-http-infer` con key de bóveda y slug vigente → `success: true`, `meta.schemaVersion=2.0`, `result.text` no vacío.
* [ ] Mismo humo con slug 404 de catálogo (*no longer available to new users*) → `success: false`; `feedback`/`error` contiene el mensaje de Google (no solo `status code 404`).
* [ ] Sin `request.model` y con `SDDIA_GEMINI_MODEL` no vacío → infiere (L-MODEL). Sin ambos → `success: false`, no panic.
* [ ] 200 con candidato sin texto → `success: false` (`gemini-empty-candidate` o equivalente), no `success: true` + `text: ""`.
* [ ] `build_argv` no genera `--print` inmediatamente seguido de `--output-format`. Test unitario rojo→verde. Lab `cargo test -p gemini-http-infer -p antigravity-cli-executor` sin red.
* [ ] Matcher de auth cubre `not logged into antigravity` además de `authentication required`.
* [ ] Cero `provides`. Cero rebind `llm:interact`. Cero nombres vetados H2 del padre.
* [ ] CI GHA no llama Gemini real ni `agy` autenticado.
* [ ] Crates tocados vía `entity-manager`, no bisturí IDE.

**No CA:** latencia numérica; SUCCESS live de `agy` si el host no tiene sesión; pin de versión `agy`; `thought: true`; `finishReason == STOP` exclusivo.

## 5. Fuera de alcance

* `PBI-ARQUITECTURA-LLM-TIERS` / kitchen `PBI-MULTI-LLM-ROUTER`.
* Vertex / ADC / SDK Python Antigravity.
* Flag CLI `--skill` en `execute-process`.
* Reabrir `docs/features/antigravity-connectors` salvo cita.
* CI verde contra Google o `agy` logueado.
* Literal de modelo en Rust.

## 6. Laudos

| ID | Pregunta | Laudo v1.2.0 |
|----|----------|----------------|
| L-MODEL | ¿Fallback env si `request.model` vacío? | **Sí.** Request no vacío gana. Sin ambos → error. Crate sin slug. |
| L-404 | ¿Body 4xx? | Match `ureq::Error::Status(code, resp)`; leer JSON; si `no longer available` / modelo `NOT_FOUND` → prefijo `gemini-model-unavailable:`. |
| L-EMPTY | ¿200 sin texto? | `success: false`. Adjuntar `finishReason` si viene. No fallar solo por `!= STOP` si hay texto. |
| L-THOUGHT | ¿Saltar `thought: true`? | Fuera de CA. Permitido en spec si el JSON lo trae; no afirmar evidencia. |
| L-SKILL-ARGV | ¿Argv `agy`? | Sin `--print` bare. `--output-format json`, `--sandbox` default, prompt en `-p` / `--print=`. |
| L-SKILL-AUTH | ¿Substring login? | Incluir `not logged into antigravity` (y el legado `authentication required`). |
| L-SKILL-INVOKE | ¿Cómo humo? | ELF o `delegates_to`. No `--skill`. SUCCESS live no gate. |
| L-FORGE | ¿Cómo mutar crates? | `entity-manager` update en ciclo `feature`. |
| L-PERSIST | ¿Topología? | `docs/features/gemini-http-infer-live-activation`. |
| L-SECRET | ¿Dónde la key? | Solo `.SddIA/.dev/.env`. Global: no duplicar secreto. |
| L-SMOKE-MODEL | ¿Slug de humo? | El que POST acepte el día D. No `gemini-2.5-flash` si 404. Example comentado, no valor-ley. |

## 7. Relación con el padre

No mueve el PBI padre de `done/`. No reabre H1–H12 salvo el **residuo de implementación de H4**: el padre mandó `{bin} --print --output-format json` + `-p`; en `agy` actual `--print` **es** el flag del prompt y no puede ir bare. Este PBI corrige el transductor; no reescribe el resto del contrato padre (lab mock, ceguera, DI, no `llm:interact`).
