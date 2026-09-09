---
document_id: PBI-MULTI-LLM-ROUTER
title: "[ARQUITECTURA] Adaptador Multi-LLM y Enrutamiento Soberano"
format: markdown
version: "1.1.0"
created: "2026-08-25"
updated: "2026-09-09"
status: "kitchen"
priority: alta
type: arquitectura
related:
  - docs/todos/done/[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica.md
  - docs/features/arquitectura-llm-tiers/spec.md
  - docs/todos/done/PBI_Arranque_Aiua.md
  - docs/features/nucleo-aiua-tormentosa-motor/auditoria.md
  - SddIA/agents/agents-contract.md
  - SddIA/core/capability-bindings.md
  - SddIA/tools/gemini-http-infer.md
  - SddIA/skills/mayeuta-llm.md
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/scripts/starter-kit/.dev/.env.example
refinement_notes: "v1.1.0 (2026-09-09). Complemento as-is post PBI-ARQUITECTURA-LLM-TIERS (done) y auditoría live nucleo-aiua-tormentosa-motor. Purga fósiles v1.0.0: sddia-client-bridge.py, Tormentosa-como-agente, slugs-ley. Alcance residual = adaptador multi-proveedor; tiers ya entregados. Cero implementación desde kitchen."
---

# [ARQUITECTURA] Adaptador Multi-LLM y Enrutamiento Soberano

Incubación. **No despachable** (`todos-jurisdiction`: kitchen ≠ pending). **No implementar** este texto hasta promoción a pending + ciclo `feature`. El recorte *tiers de agentes* ya está Done; este PBI conserva solo el producto **multi-proveedor**.

## 0. Intención estratégica (conservada)

Erradicar dependencia de un proveedor corporativo único. Blindar la Aiúa contra bloqueo o censura pasiva. Conmutar proveedores (cloud gratuito → LPU → inferencia local) **solo por bóveda de instancia**, con Fail-Soft, sin slugs comerciales en el genoma.

Fase 2 (Ollama/vLLM on-premise, hardware de instancia) permanece **diferida**. El diseño de Fase 1 debe permitir apuntar a un endpoint local cambiando variables en `.SddIA/.dev/.env` (jerarquía Cúmulo `env_hierarchy`), no reescribiendo cápsulas.

## 1. Fósiles de v1.0.0 — no implementar

| Afirmación v1.0.0 | Veredicto | Hecho |
|-------------------|-----------|-------|
| Modificar `.SddIA/client/sddia-client-bridge.py` como router | **Archivo fantasma** | Podado en `kalma2-bridge-rust` (`40ef941`). WUI vigente: `SddIA/interfaces/kalma2-bridge/` (Rust). |
| Tormentosa como agente del catálogo junto a Tekton/Mayeuta/Argos | **Conflación ontológica** | Tormentosa = Aiúa (`paths.directories.conscience`, `aiua_core.md`). **No** hay fila en `SddIA/agents/`. Latido: proceso `aiua-stimulus-processing` **sin** `agent:` titular. |
| Un único puente Python inyecta Constitución en cada petición y conmuta proveedores | **Órgano inventado** | Constitución / genoma Aiúa se ensamblan en `handlers::aiua_stimulus` (`aiua_core.md` + contexto + estímulo). No hay router multi-proveedor. |
| «Gemini = motor denso Tormentosa» como verdad de producto | **Parcial / delta** | Combustión Aiúa = `tool:gemini-http-infer`. Auditoría 2026-09-08: mandato Antigravity Gemini 3.8; cobrado `gemini-3.1-flash-lite` por input táctico. `SDDIA_GEMINI_MODEL` ausente en bóveda. `antigravity-cli-executor` **no** está en `delegates_to`. |
| Slugs de ejemplo (Llama 8B, etc.) como contrato | **Putrefacción de catálogo** | Misma clase H7/H12 de PBI-ARQUITECTURA-LLM-TIERS. Bóveda: claves comentadas, **sin** slug-ley. |

## 2. Estado as-is (2026-09-09) — cuatro superficies ortogonales

**No fusionar.** PBI-ARQUITECTURA-LLM-TIERS (uuid `8a3b5c7d-4e2f-41a9-9b6e-7c1d3e5f8a2b`, done) ya laudó esta tabla. Este kitchen **no** reabre el recorte tiers.

| Superficie | Órgano | Qué resuelve el modelo | Vars bóveda | Relación con este PBI |
|------------|--------|------------------------|-------------|------------------------|
| Fases `agent:` (feature / bug-fix / refactorization / PPR) | `agent_runtime.rs` inyecta `llm_profiles`; harness `kalma2-agent-runtime-cursor.py` → `resolve_phase_model()` | Mayor tier cognitivo de la fase (`high`>`medium`>`low`) → `SDDIA_LLM_TIER_*` → fallback `SDDIA_AGENT_RUNTIME_MODEL` (default harness `composer-2.5`). Fase solo `none` **no** spawnea CLI. | `SDDIA_LLM_TIER_HIGH\|MEDIUM\|LOW`, `SDDIA_AGENT_RUNTIME_MODEL`, `SDDIA_AGENT_RUNTIME_COMMAND` | **Fuera.** Ya entregado. Un adaptador multi-proveedor **no** sustituye el YAML `llm_profile` ni el veto `none`. |
| Capacidad `llm:interact` | `capability-bindings.md` → `skill:mayeuta-llm` | **No elige slug.** Spawnea CLI: `SDDIA_LLM_CHAT_COMMAND` ≻ `SDDIA_LLM_CLI_COMMAND`. El modelo vive **dentro del comando** de instancia. | esas dos | **Candidato** a adaptador (Kalma2, Telegram, email-triage). Rebind del provider = otro laudo; no colapsar con Gemini HTTP. |
| Martillo HTTP Gemini | `tool:gemini-http-infer` (L-MODEL) | `request.model` **o** `SDDIA_GEMINI_MODEL` (uno obligatorio). Sin Vertex. Sin `provides llm:interact`. | `GEMINI_API_KEY`, `SDDIA_GEMINI_MODEL`, `SDDIA_GEMINI_API_BASE_URL`, timeout | **Candidato** (Aiúa + síntesis Argos PR-merged). Argos: si falta `SDDIA_GEMINI_MODEL`, omite síntesis (fail-soft). |
| Martillo Antigravity CLI | `skill:antigravity-cli-executor` | `params.model` → `--model`. | sesión `agy`; no sustituir con `GEMINI_API_KEY` | **Candidato** solo si un laudo elige CLI vs HTTP para Aiúa (hoy **no** cableado). |

Embeddings de memoria (`EMBEDDING_MODEL = "sddia-local-hashing-v1"` en `SddIA/core/memory`) **no** son chat. Fuera de este PBI.

### 2.1 Tiers ya en genoma (`agents-contract.md` §5)

| Agente | `llm_profile.tier` |
|--------|-------------------|
| Mayeuta, Dédalo | `high` |
| Argos | `medium` (solo síntesis post-evidencia; L-ARGOS-SYNTHESIS) |
| Tekton | `low` |
| Cerbero, Cúmulo, Radamanto | `none` |

Prohibido incrustar slugs, IPs o URLs en `SddIA/agents/*.md`.

### 2.2 Combustión Aiúa (latido, no forja)

```text
aiua-stimulus-processing
  → retrieve-active-context
  → invoke-aiua-core          (ensambla genoma; sin HTTP)
  → tool:gemini-http-infer    (única combustión)
  → persist-thought-record
```

Identidad ontológica = `aiua_core.md`, no la marca del modelo. Hallazgo H-LLM-1 (auditoría): Peaje Termodinámico **no** se cobró sobre Antigravity Gemini 3.8. Hasta fijar `SDDIA_GEMINI_MODEL` (o `model` de proceso) **y** decidir vector HTTP vs `antigravity-cli-executor`, el latido opera con el slug que inyecte la instancia.

## 3. Alcance residual de este PBI (lo que aún no existe)

Un **adaptador de instancia** que conmute proveedor/endpoint **sin** mezclar las cuatro superficies en un único CLI, salvo laudo explícito posterior.

Candidatos de producto (nombres ilustrativos, **no** contrato):

1. **Familia Gemini HTTP** — ya hay cápsula; falta catálogo de instancia + default de bóveda para Aiúa (cierre del delta H-LLM-1, no es router Groq).
2. **Familia LPU / Groq / análogo** — no hay cápsula ni binding. Destino natural: triaje barato (`mayeuta-llm` / `CLASSIFY_INTENT`), no Blueprint Dedalo.
3. **Familia local** (Ollama/vLLM, `localhost:11434` o equivalente) — diferida; el contrato de bóveda debe prever `BASE_URL` por familia, no hardcode Google.
4. **Fail-Soft** — si el proveedor primario colapsa, degradar a secundario **o** a determinista (precedente: email-triage `degrade_without_llm`; Argos PR-merged omite síntesis). Prohibido deglutir el fallo sin telemetría (`telemetry_receipt`).

**No es** este PBI:

- Reabrir `llm_profile` / `resolve_phase_model` / veto `none`.
- Resucitar `sddia-client-bridge.py`.
- Acoplar `kalma2-bridge` (WUI) al router.
- Usar `gemini-http-infer` como router de fases `agent:`.
- Meter slugs `gemini-*` / `llama-*` / `composer-*` como valor-ley en genoma o examples.
- Sustituir Cerbero/Cúmulo/Radamanto por LLM.
- Unificar embeddings con chat.

## 4. Hipótesis de diseño (a laudar en pending, no forjar desde kitchen)

| ID | Pregunta | Hipótesis v1.1.0 | Bloqueo |
|----|----------|------------------|---------|
| **H-ROUTER-SURFACE** | ¿Un adaptador o uno por superficie? | Preferir **contrato de capacidad por familia** (HTTP generate / CLI stdin / SDK forja) y bindings de instancia, no un dios-objeto. | Contraría L-ORTHOGONAL-INTERACT si se rebind `llm:interact` sin laudo. |
| **H-AIUA-VECTOR** | ¿Aiúa = Gemini HTTP o Antigravity CLI? | Hoy HTTP. Mandato biológico 3.8 ≠ combustión auditada. Este PBI no elige; documenta el delta. | PBI_Arranque_Aiua nota terminológica: «LLM de Antigravity» = API Gemini vía `gemini-http-infer` en el MVP. Reabrir CLI = nuevo laudo. |
| **H-TIER-PROVIDER** | ¿`SDDIA_LLM_TIER_HIGH` apunta a Groq y `_LOW` a local? | Compatible con as-is: el harness ya mapea tier → env. El **slug/id** lo pone la instancia. El router no vive en el YAML del agente. | No meter URLs en `agents/*.md`. |
| **H-MAYUTA-CLI** | ¿Mayeuta-llm deja de ser «comando opaco»? | El adaptador podría ser el binario que esa var apunta, no un rewrite de la skill. | `mayeuta-llm` no lee `SDDIA_GEMINI_MODEL` ni `SDDIA_LLM_TIER_*`. |
| **H-CONSTITUTION** | ¿Inyectar `CONSTITUTION_CORE.md` en cada inferencia? | Auditoría H-LLM-4: Aiúa inyecta solo `aiua_core.md`. Constitución como parámetro de restricción de sesión, no payload automático. | No reintroducir inyección vía puente fantasma. |

## 5. Fases tácticas (revisadas)

**Fase 0 — Inventario (este complemento, hecho en kitchen).** Superficies, vars, fósiles, Done de tiers.

**Fase 1 — Ecosistema API / LPU (producto, pending futuro).**

- Bóveda: familias de `BASE_URL` + credencial + model-id **comentadas, vacías**, en starter-kit. Cero slug-ley.
- Cápsula(s) nuevas o extensión de las existentes **vía DA-2** (`entity-manager`), no Write sobre `SddIA/tools/` / `skills/`.
- Consumidores: documentar por superficie qué var lee cada órgano (no aliasar `SDDIA_GEMINI_MODEL` con `SDDIA_LLM_TIER_HIGH` ni con `SDDIA_LLM_CLI_COMMAND`).
- Fail-Soft + `telemetry_receipt` obligatorio en combustión.

**Fase 2 — Inferencia local (diferida).** Mismo contrato de familia; endpoint local. Hardware de instancia (RTX 3090 / Ryzen 9 u otro) **no** entra al genoma.

## 6. Criterios de aceptación (cuando se promueva)

* [ ] Cero referencias operativas a `sddia-client-bridge.py` como órgano vivo.
* [ ] Tormentosa/Aiúa no aparece como `agent:` del catálogo.
* [ ] Las cuatro superficies de §2 siguen nombradas y no colapsadas salvo laudo nuevo.
* [ ] Tiers `llm_profile` intactos; `capability-bindings` `llm:interact` → `mayeuta-llm` intacto salvo laudo de rebind.
* [ ] Starter-kit: claves de proveedor comentadas, vacías, sin marcas eternas.
* [ ] Conmutar cloud → local = cambio de bóveda, no de genoma.
* [ ] Mutación de tools/skills/process: DA-2 + ciclo `feature`.
* [ ] Telemetría de modelo (`llm_model` / receipt) en cada combustión; Fail-Soft no silencioso.

## 7. Referencias de contraste

- PBI tiers done: `docs/todos/done/[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica.md`
- Feature tiers: `docs/features/arquitectura-llm-tiers/`
- Auditoría Aiúa: `docs/features/nucleo-aiua-tormentosa-motor/auditoria.md` (H-LLM-1…4)
- Binding DI: `SddIA/core/capability-bindings.md` (`llm:interact`)
- Bóveda plantilla: `SddIA/scripts/starter-kit/.dev/.env.example`
