---
document_id: PBI-CAPSULES-ANTIGRAVITY-NATIVE
uuid: "7f966f32-5502-4bd7-b252-44849f29f5d3"
title: "[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI)"
format: markdown
version: "1.2.0"
created: "2026-09-02"
updated: "2026-09-03"
status: done
refinement_status: refinado
priority: alta
process: feature
executor_vehicle: feature
type: operativo
dispatch: false
suggested_branch: feat/capsules-antigravity-native
persist_ref_suggested: docs/features/capsules-antigravity-native
related:
  - SddIA/norms/capsule-json-io.md
  - SddIA/skills/skills-contract.md
  - SddIA/tools/tools-contract.md
  - SddIA/core/cumulo.paths.json
  - SddIA/core/capability-bindings.md
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/library/norms/capability-contracts/llm.interact.schema.json
  - SddIA/skills/mayeuta-llm.md
  - SddIA/sddia-io/src/lib.rs
  - SddIA/sddia-io/src/outbound_lab.rs
  - SddIA/norms/execution-contexts.md
  - SddIA/process/skill-creator.md
  - SddIA/norms/external-ai-constraints.md
  - docs/todos/pending/PBI-ARQUITECTURA-LLM-TIERStitle: "[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica.md
refinement_notes: "Filtro A 2026-09-03. El v1.1.0 fusionaba tres superficies (Gemini REST, SDK Antigravity, CLI agy), robaba llm:interact a mayeuta-llm, inventaba ANTIGRAVITY_API_KEY/GEMINI_API_ENDPOINT, malcitaba outbound_lab y mezclaba sobres JSON. Flags headless de agy sí son reales (docs oficiales). Auth de agy ≠ API key."
---

# [OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI)

## 0. Hallazgos Filtro A (v1.1.0 → v1.2.0)

Correcciones de alucinación / incoherencia. No son matices de estilo.

| # | Afirmación v1.1.0 | Veredicto | Hecho contrastado |
|---|-------------------|-----------|-------------------|
| H1 | «API REST de Google Gemini / Vertex AI (backend cognitivo de Antigravity)» como un único HTTP | **Fusión indebida** | Tres superficies distintas: (A) Gemini REST `generateContent` (AI Studio, `GEMINI_API_KEY`); (B) Vertex AI (ADC / proyecto GCP, otro endpoint); (C) Antigravity CLI `agy` (harness agentic). No hay REST público documentado equivalente a `agy` para el agente. El SDK Python `google-antigravity` queda **fuera de alcance** (skills-contract §4: prohibido intérprete). |
| H2 | Credenciales `GEMINI_API_KEY` **o** `ANTIGRAVITY_API_KEY` + `GEMINI_API_ENDPOINT` | **Invención de nombres** | Oficial AI Studio: `GEMINI_API_KEY`. `ANTIGRAVITY_API_KEY` no es SSOT de Google ni del Core. `GEMINI_API_ENDPOINT` no es variable canónica; el path AI Studio es `https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent`. Vertex no se autentica con esa misma clave. |
| H3 | `agy` se autentica inyectando API key de entorno | **Falso** | Docs headless: credenciales **cacheadas** (sesión interactiva previa). Sin auth → `authentication required`, no hang. `GEMINI_API_KEY` en `agy` es issue abierto / no contrato actual. CI no se resuelve «exportando la key». |
| H4 | Flags `agy --print --output-format json --dangerously-skip-permissions` | **Cierto**, incompleto | Aliases `-p` / `--print` / `--prompt`. `--output-format`: `text` \| `json` \| `stream-json`. `--print-timeout` default `5m`. `--model`, `--effort`, `--add-dir`, `--sandbox` existen. `--dangerously-skip-permissions` auto-aprueba **writes y shell**; no debe ir hardcodeado como único modo. |
| H5 | HTTP connector «podrá mapearse como proveedor de `llm:interact`» | **Violación DI** | Códice + bindings v1.6.0: `llm:interact` → **única fila** `skill:mayeuta-llm`. Una capacidad = un provider canónico. Rebind o alta de término = laudo + mutación gobernada + evolution. Schema `llm.interact` exige `success`+`data`, no el sobre `result` del ejemplo v1.1.0. |
| H6 | `sddia_io::outbound_lab` como mock HTTP genérico | **Inexacto** | Módulo real: flags `SDDIA_LAB_MOCK_OUTBOUND` / `SDDIA_LAB_SIMULATE_IOTA` y URLs mock **IOTA/Telegram**. No hay helper Gemini. Reusar el flag de lab; la URL mock se **añade** (patrón `lab_mock_telegram_url`) o se inyecta. Prohibido `find_repo_root_from_cwd` (viola ceguera). |
| H7 | Sobre de salida = `SddiaResponse` con `error` y sin `meta` | **Mezcla de SSOT** | `capsule-json-io.md` 2.0: `meta`, `success`, `exitCode`, `message`, `feedback?`, `result`, `durationMs?`. `sddia-io::SddiaResponse`: `success`, `exitCode`, `feedback?`, `result?`, `error?` (sin `meta`). `mayeuta-llm` emite `{success,data,error}`. Este PBI **no** inventa un cuarto sobre. |
| H8 | «Ninguna cápsula accede a rutas del host» vs spawn de `agy` | **Autocontradicción** | Ceguera = la cápsula no **descubre** repo ni lee `env_hierarchy` (`./.dev/.env`, `.SddIA/.dev/.env`). `agy` **sí** opera sobre workspace (herramientas, `--add-dir`). El hijo no es ciego; el martillo no rastrea secretos en disco. |
| H9 | Frontmatter skill = `uuid, name, version, contract, capabilities, inputs, outputs` | **Incompleto** | `skills-contract.md` §1 también exige `hash_signature` y `context` ∈ `execution-contexts.md`. Index: `SddIA/skills/index.md`. Prohibido `spec.json`; `spec.md` en capsule-json-io es fósil. |
| H10 | Dos skills HTTP+CLI sin mencionar forja ni solape | **Hueco normativo** | DA-2/DA-3: `./sddia-run.sh --process entity-manager` → `skill-creator` / `tool-creator`. Ciclo `feature` (DA-4) antes de mutar genoma. `mayeuta-llm` ya spawnea CLI inyectado (`SDDIA_LLM_CLI_COMMAND`) y escribe el prompt por **stdin**; `agy -p` espera el prompt en **argv**. Ese mismatch es la justificación real del transductor CLI — no «falta un martillo LLM». |
| H11 | Modelo ejemplo `gemini-2.5-flash` | **No hardcodear** | Slug inyectado en request/env. Catálogo `agy models` y IDs AI Studio cambian; el genoma no fija marca. |
| H12 | «wasm32-wasip1 carece de sockets TCP» como base de la excepción HTTP | **Sobreafirmación** | Excepción canónica §4: spawn (`git`, etc.) → binario nativo `compiled_capsules.native_root`. Outbound HTTP en este repo vive en **tools nativos** (`github-raw-fetcher`, `send-telegram-notification`). No afirmar física WASI no citada en el contrato. |

## 1. Propósito y alcance (corregido)

Dotar a la **instancia** de dos actuadores ciegos, compilados a nativo Rust, para hablar con el ecosistema Google **sin** meter SDK ni orquestación en los agentes.

| Activo | Superficie real | Familia ED | Rol |
|--------|-----------------|------------|-----|
| A — HTTP infer | Gemini REST `generateContent` (AI Studio). Vertex **fuera** salvo laudo explícito. | **Tool** (mismo patrón que otros HTTP salientes del Core) | Inferencia stateless. No es el harness Antigravity. |
| B — CLI executor | Binario host `agy` en modo print/headless | **Skill** (transductor de binario, analogía `mayeuta-llm` / `git-manager`) | Tarea agentic no interactiva. El hijo puede mutar workspace. |

Ambos son martillos: no deciden cuándo correr. Configuración = env del proceso (bóveda `env_hierarchy` cargada por el orquestador) + JSON stdin. Prohibido `dotenv` / abrir ficheros de bóveda.

**Agnosticismo Core:** nombres comerciales y URLs no se hardcodean en lógica. Endpoint, modelo y path de binario se inyectan. El `{name}` de la ED puede mencionar el dominio táctico; el crate no embebe secretos ni IDs de modelo.

**Fuera de alcance (explícito):** SDK Python Antigravity; rebind de `llm:interact`; router de tiers (`PBI-ARQUITECTURA-LLM-TIERS`); Vertex/ADC; CI verde contra `agy` real (auth de keyring no reproducible en GitHub Actions).

## 2. Relación con el genoma vigente (no duplicar)

`skill:mayeuta-llm` (provider canónico de `llm:interact`) es un transductor CLI **genérico**: comando desde `SDDIA_LLM_CHAT_COMMAND` ≻ `SDDIA_LLM_CLI_COMMAND`, prompt por stdin del hijo, operaciones `SYNTHESIZE` / `CLASSIFY_INTENT` / `STREAM`. C3 histórico: sin red ni SDK en el genoma.

Consecuencia:

1. Apuntar `SDDIA_LLM_CLI_COMMAND` a `agy` **no** sustituye el Activo B: el protocolo de argv/flags/sobre JSON de `agy` no es el de mayeuta.
2. El Activo A (HTTP) **rompe C3** si se mete red Gemini en una skill del mismo nicho que mayeuta. Por eso A nace como **tool**, no como segundo provider de `llm:interact`.
3. Consumidores process siguen exigiendo `llm:interact` → mayeuta. Estas cápsulas se invocan por `delegates_to` / tool explícito, o por mayeuta *después* de un laudo de composición. **Prohibido** declarar `provides: llm:interact` en este ciclo.

## 3. Especificación de activos

Nombres de ED = candidatos de forja (kebab-case). Laudo L1 puede retitular.

### 3.1 Tool `gemini-http-infer` (Activo A)

* **Sustrato:** crate nativo bajo `directories.tools` (`SddIA/tools/{name}/`). Workspace Cargo ya incluye `tools/*`. Artefacto: `compiled_capsules.native_root` → `SddIA/target/{debug|release}/{name}`.
* **RBAC `context`:** `system-operations` (binario/red saliente; no inventar contexto).
* **Propósito:** POST JSON al endpoint **inyectado**; devolver texto/candidato en `result` del sobre 2.0.
* **Env (proceso, no disco):**
  * `GEMINI_API_KEY` — única credencial AI Studio reconocida aquí.
  * `SDDIA_GEMINI_API_BASE_URL` — opcional; default documentado en `.env.example` de instancia, no en el crate como constante de negocio. Path de método: `/v1beta/models/{model}:generateContent` salvo override.
  * Lab: si `SDDIA_LAB_MOCK_OUTBOUND` (truthy, `outbound_lab::lab_mock_outbound_enabled`), no tocar red real; URL mock `SDDIA_LAB_MOCK_GEMINI_URL` (alta simétrica a Telegram/IOTA; **extiende** `outbound_lab`, no reusa las URLs ajenas).
* **Request (cuerpo `request`, no mezclar con meta):** `prompt` (string, obligatorio); `model` (string, obligatorio — sin default de marca en código); `temperature` opcional. Prohibido Vertex (`GOOGLE_GENAI_USE_VERTEXAI`, ADC) en v1.
* **Fricción:** red saliente al host inyectado. Timeout acotado (constante de crate o env `SDDIA_GEMINI_HTTP_TIMEOUT_SECS`, default numérico documentado).

### 3.2 Skill `antigravity-cli-executor` (Activo B)

* **Sustrato:** crate nativo bajo `directories.skills`. Excepción `skills-contract` §4 (spawn). Miembro automático `skills/*`.
* **RBAC `context`:** `system-operations` (terceros SO; no `ecosystem-evolution` salvo laudo L2).
* **Propósito:** ensamblar invocación no interactiva de `agy` y traducir su sobre a capsule-json-io.
* **Resolución del binario:** `SDDIA_AGY_PATH` si no vacío; si no, `agy` vía `PATH`. Prohibido inventar `ANTIGRAVITY_CLI_PATH` como nombre «oficial». Prohibido leer `~/.gemini/` o keyring desde la cápsula.
* **Argv mínimo (print mode):** `{bin} --print --output-format json` + prompt como argumento de `-p` (no stdin al estilo mayeuta, salvo que `request.input_format=stream-json` se declare en un hito posterior — **fuera de v1**).
* **Flags opcionales desde `request.parameters` (whitelist, no passthrough libre):** `--model`, `--effort` (`low|medium|high`), `--add-dir` (rutas **solo** las inyectadas en el request; típico `workspace_path` del orquestador), `--print-timeout`, `--sandbox`.
* **Permisos del hijo:** default **`--sandbox`**. `--dangerously-skip-permissions` **solo** si `request.parameters.skip_permissions === true` **y** env `SDDIA_AGY_ALLOW_SKIP_PERMISSIONS` truthy. Ausencia de cualquiera → no emitir el flag. Documentar riesgo: writes + shell del host.
* **Auth:** no inyectar API keys al hijo como sustituto de sesión. Si `agy` sale con auth required → `success: false` (sobre válido), no panic.
* **Parseo stdout:** un JSON de `agy` (`status`, `response`, `usage`, `conversation_id`, `error?`). Mapear `status != SUCCESS` a fallo de negocio. `raw` puede ir en `result.raw_response`. Tokens → `telemetry_receipt` opcional (`telemetry_provided` en frontmatter si se promete).
* **Timeout de wait:** respetar `--print-timeout`; además kill del subproceso si el wait del crate supera el mismo techo.
* **Fricción de entorno:** `agy` instalado **y** sesión autenticada en el host. Lab/CI: mock de binario (`SDDIA_LAB_MOCK_OUTBOUND` o stub inyectado en `SDDIA_AGY_PATH` apuntando a un fixture) — no llamar red Google.

## 4. Contrato I/O (sin cuarto sobre)

SSOT de invocación: `capsule-json-io.md` schema_version `2.0`.

**stdin** — un objeto:

```json
{
  "meta": {
    "schemaVersion": "2.0",
    "entityKind": "skill",
    "entityId": "<name kebab>"
  },
  "request": {}
}
```

`entityKind` = `"tool"` en Activo A. `request` lo define el `{name}.md` de la ED (no `spec.md`). Alternativas ya normativas: `SDDIA_CAPSULE_REQUEST`, `SDDIA_SKIP_STDIN=1`.

**stdout** — una línea. Campos de `capsule-json-io`: `meta` (eco), `success`, `exitCode` (0 iff success), `message`, `feedback?`, `result`, `durationMs?`.

Si la implementación usa `sddia-io::SddiaResponse` (tiene `error`, no tiene `meta`): o se extiende el crate en el mismo ciclo para emitir meta+message, o se serializa a mano el sobre 2.0. **Prohibido** emitir solo `{success,error}` al estilo mayeuta en estas ED nuevas.

Anti-panic: fallos (red, timeout, spawn, JSON `agy`, key ausente) → JSON `success: false`, `exitCode != 0`, diagnóstico en `message`/`feedback`. Nada a stdout que no sea esa línea.

## 5. Gobernanza DI y capacidades

* `capabilities[]` = operaciones atómicas de la cápsula (`gemini-generate-content`, `agy-print-exec`, …). **No** sustituyen `provides`.
* **Este ciclo no declara `provides`.** Ningún `capability_id` nuevo. Ningún rebind de `llm:interact`.
* Alta futura de término (ej. harness agentic) = RFC + Códice + binding + evolution + sello `Domain_Entity_Updated`. Hasta entonces, invocación explícita.

## 6. Forja y ciclo (DA-2–DA-4)

1. Instanciar proceso `feature` (topología `docs/features/capsules-antigravity-native/` o el `persist_ref` laudoado).
2. Genoma ED vía `./sddia-run.sh --process entity-manager` → `tool-creator` (A) y `skill-creator` (B). Prohibida escritura IDE en `SddIA/tools/`, `SddIA/skills/`, `SddIA/norms/`, taxonomía.
3. Crates Rust + tests unitarios en el mismo PR que `{name}.md` + fila de índice.
4. Frontmatter completo: `uuid`, `name`, `version`, `contract`, `context`, `hash_signature`, `capabilities`, `inputs`, `outputs`.
5. Cierre documental en rama (`task-closure-documental`): PBI → `docs/todos/done/` + `validacion.md` APTO en el **mismo** PR.

## 7. Criterios de aceptación

* [x] Ciclo `feature` activo; forja A/B por `entity-manager` (no mutación manual de genoma).
* [x] Tool `gemini-http-infer` y skill `antigravity-cli-executor` (o nombres laudoados) compilan nativos en el workspace Cargo (`tools/*` / `skills/*`).
* [x] Ninguna de las dos abre ficheros `env_hierarchy` ni usa `find_repo_root_from_cwd`. Secretos y paths solo `std::env::var` + JSON.
* [x] Activo A: con `SDDIA_LAB_MOCK_OUTBOUND` y URL mock, CI verde **sin** red Google ni `GEMINI_API_KEY` real. Sin mock y sin key → sobre `success: false`, no panic.
* [x] Activo B: invocación print + `--output-format json`; `--dangerously-skip-permissions` no sale en argv salvo doble opt-in (§3.2). Timeout y exit ≠0 de `agy` → sobre válido `success: false`.
* [x] stdout conforme `capsule-json-io` 2.0 (`meta` presente; `exitCode===0` iff `success`).
* [x] Cero `provides` / cero toque a `capability-taxonomy.md` / `capability-bindings.md` en este PR salvo laudo L3 por escrito en `clarify.md`.
* [x] `{name}.md` + índice; `hash_signature` y `context` ∈ matriz Cerbero. Prohibido `spec.json` / `manifest.json`.

## 8. Laudos abiertos (bloquean ejecución, no el PBI)

| ID | Pregunta | Default si Racso no lauda |
|----|----------|---------------------------|
| L1 | ¿HTTP es **tool** (recomendado) o skill? ¿Nombres kebab definitivos? | tool `gemini-http-infer` + skill `antigravity-cli-executor` |
| L2 | `context` de B: `system-operations` vs otro existente. Prohibido inventar fila en `execution-contexts.md` en este ciclo. | `system-operations` |
| L3 | ¿Rebind `llm:interact` o alta Códice? | **No.** Fuera de alcance. |
| L4 | ¿Vertex AI en el mismo crate HTTP? | **No** en v1. |
| L5 | Relación con `PBI-ARQUITECTURA-LLM-TIERS` (bridge instancia vs cápsulas Core). | Ortogonal: tiers = routing de agentes; este PBI = martillos. No implementar el bridge aquí. |
