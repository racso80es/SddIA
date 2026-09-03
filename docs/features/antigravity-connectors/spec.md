---
feature_name: antigravity-connectors
created: "2026-09-03"
process: feature
base: main
scope: antigravity-connectors
version_spec: "1.0.0"
status: executed
pbi_uuid: "7f966f32-5502-4bd7-b252-44849f29f5d3"
pbi_version: "1.2.0"
---

# Especificación — antigravity-connectors (post-auditoría)

SSOT de requisitos: PBI v1.2.0. Este spec no reabre H1–H12; concreta el estado **objetivo** tras refactor. El esqueleto Jules no es baseline aceptable.

## 1. Activos objetivo

### 1.1 Tool `gemini-http-infer`

| Campo | Valor |
|-------|--------|
| Familia | `tool` bajo `directories.tools` |
| Crate | `SddIA/tools/gemini-http-infer/` nativo |
| `context` | `system-operations` |
| Operación | POST `{base}/v1beta/models/{model}:generateContent` |
| Env | `GEMINI_API_KEY`; `SDDIA_GEMINI_API_BASE_URL` opcional; timeout `SDDIA_GEMINI_HTTP_TIMEOUT_SECS` |
| Lab | `SDDIA_LAB_MOCK_OUTBOUND` + `SDDIA_LAB_MOCK_GEMINI_URL` (extender `outbound_lab`, no reusar URL Telegram/IOTA) |
| HTTP | crate nativo (`ureq`/`reqwest` blocking). **Prohibido** `curl`. **Prohibido** poner el secreto en argv. |
| Request | `prompt` + `model` obligatorios; `temperature` opcional. Sin default de marca en código. |

### 1.2 Skill `antigravity-cli-executor`

| Campo | Valor |
|-------|--------|
| Familia | `skill` (re-forja; el `{name}.md` Jules no es sello válido) |
| Crate | `SddIA/skills/antigravity-cli-executor/` nativo (excepción §4 spawn) |
| `context` | `system-operations` |
| Argv v1 | `{bin} --print --output-format json` + prompt en `-p`. No stdin JSON al estilo mayeuta. |
| Binario | `SDDIA_AGY_PATH` o `agy` en `PATH` |
| Permisos | default `--sandbox`; `--dangerously-skip-permissions` iff `request.parameters.skip_permissions===true` **y** `SDDIA_AGY_ALLOW_SKIP_PERMISSIONS` truthy |
| Whitelist params | `--model`, `--effort`, `--add-dir` (solo paths inyectados), `--print-timeout` |
| Auth | no sustituir keyring de `agy` con API key. Auth required → `success: false` |
| Lab | stub en `SDDIA_AGY_PATH` o `SDDIA_LAB_MOCK_OUTBOUND` |

## 2. Contrato I/O

`capsule-json-io.md` schema 2.0. `entityKind` = `tool` | `skill`. stdout una línea con `meta` eco. `exitCode===0` iff `success`. Fallos sin `panic!`.

Si `sddia-io::SddiaResponse` no emite `meta`/`message`, serializar el sobre 2.0 a mano o extender el crate **en este mismo ciclo** (no un cuarto sobre).

## 3. DI

Cero `provides`. Invocación explícita. `llm:interact` permanece en `skill:mayeuta-llm`.

## 4. Fuera de alcance

SDK Python Antigravity; Vertex/ADC; rebind Códice; router de tiers; CI verde contra `agy` real autenticado.

## 5. Retirada del esqueleto

| Artefacto Jules | Destino |
|-----------------|--------|
| `SddIA/skills/antigravity-http-connector/` + `.md` | Delete vía cadena `entity-manager` `lifecycle_operation` aplicable + quitar exclusión WASI de ese package |
| `SddIA/skills/antigravity-cli-executor.md` (hash ceros) | Update/re-forja; crate se reescribe |
| Fila EDA `ae2927f5-fd78-4950-b9f6-497624159e95` | No es UUID de ninguna ED de este PBI. Corregir cobertura a UUIDs reales post-forja; no reusar `deadbeef` |
| `skills/index.md` | Filas reales post-creator; HTTP no va al índice de skills |
