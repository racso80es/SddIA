---
document_id: PBI-ARQUITECTURA-LLM-TIERS
uuid: "8a3b5c7d-4e2f-41a9-9b6e-7c1d3e5f8a2b"
title: "[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica"
format: markdown
version: "1.3.0"
created: "2026-08-28"
updated: "2026-09-04"
status: done
refinement_status: refinado
priority: alta
type: arquitectura
process: feature
suggested_branch: feat/arquitectura-llm-tiers
persist_ref_suggested: docs/features/arquitectura-llm-tiers
related:
  - SddIA/agents/agents-contract.md
  - SddIA/agents/index.md
  - SddIA/core/cumulo.paths.json
  - SddIA/core/capability-bindings.md
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs
  - SddIA/engine/execute-process/src/core/env.rs
  - SddIA/engine/execute-process/src/core/parser.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/skills/mayeuta-llm.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/library/codexes/codex-software-engineering/process/feature.md
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - SddIA/scripts/starter-kit/.SddIA/.dev/.env.example
  - SddIA/scripts/starter-kit/.dev/.env.example
  - docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md
  - "docs/todos/done/[OPERATIVO] Activación live gemini-http-infer — catálogo de modelos y humo de instancia.md"
  - "docs/todos/done/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md"
refinement_notes: "Filtro A v1.3.0 (2026-09-04). Segunda pasada: v1.2.0 purgó 8 hallazgos pero dejó 9 inexactitudes. Superficies LLM desmezcladas (kalma2-bridge ≠ agent_runtime ≠ kalma2-agent-runtime-cursor.py ≠ mayeuta-llm ≠ gemini-http-infer). H3 corregido (no existe SddIA/actions/clarify.md). Ceguera Espacial no equivale a agnosticismo de proveedor. SDDIA_LLM_TIER_* no colisiona con SDDIA_AGENT_RUNTIME_MODEL / SDDIA_GEMINI_MODEL / SDDIA_LLM_CLI_COMMAND. tier none implica no spawn del CLI LLM. Argos medium acotado a síntesis. DA-2 para mutación de genoma. Payload se extiende, no se sustituye."
---

# [ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica

## 0. Hallazgos Filtro A

### 0.1 v1.1.0 → v1.2.0 (conservados; H2 y H3 enmendados en 0.2)

| # | Afirmación v1.1.0 original | Veredicto | Hecho contrastado |
|---|----------------------------|-----------|-------------------|
| **H1** | «manifiestos JSON individuales» / `llm_profile` obligatorio en JSON de agente | **Alucinación estructural** | Los agentes son `{name}.md` con YAML Frontmatter. SSOT: `SddIA/agents/agents-contract.md` + catálogo `SddIA/agents/index.md` (7 filas). **No** citar `entidades-dominio-ecosistema-sddia.md` como prueba de formato de agentes: ese documento describe el patrón `spec.md` de otras familias, no `SddIA/agents/{name}.md`. |
| **H2** | `.SddIA/client/sddia-client-bridge.py` intercepta y enruta; «refactorizar backend Python» | **Archivo fantasma** (hecho parcial) | El fichero **no existe**. Eliminado en `40ef941` (`feat(kalma2-bridge): puente HTTP nativo Rust`). Ese commit materializó el **WUI HTTP** `kalma2-bridge`, no el runtime de fases `agent:`. Enmienda en H9. |
| **H3** | «tier: high (… / Clarifier)» | **Entidad inventada** (hecho parcial) | No hay agente `clarifier` en `agents/index.md`. Enmienda en H10: tampoco existe la acción `SddIA/actions/clarify.md`. |
| **H4** | «tier: medium (Argos / Cerbero) → Peaje RBAC» | **Incoherencia crítica** | Cerbero no usa LLM. Gate nativo: `cerbero_di_rbac.rs`. Cúmulo y Radamanto son deterministas. |
| **H5** | Omisión de `radamanto` | **Incompletitud** | Catálogo vigente: `cerbero`, `cumulo`, `tekton`, `argos`, `dedalo`, `mayeuta`, `radamanto`. |
| **H6** | Marcadores `[cite: N]` | **Artefactos espurios** | Purgados. |
| **H7** | Slugs `.env` tipo `antigravity/gemini-3.1-pro` como verdad eterna | **Putrefacción de catálogo** | Misma clase que A12 de `PBI-GEMINI-HTTP-INFER-LIVE-ACTIVATION`. Examples: clave comentada, **sin** slug vendido como ley. |
| **H8** | Nombre de archivo con `:` y comillas | **Defecto de filesystem** | Disco actual: `docs/todos/pending/PBI-ARQUITECTURA-LLM-TIERS.md`. |

### 0.2 v1.2.0 → v1.3.0 (esta pasada; contrastado contra código)

| # | Afirmación v1.2.0 | Veredicto | Hecho contrastado en el repositorio |
|---|-------------------|-----------|-------------------------------------|
| **H9** | «eliminado en `40ef941` al migrarse a `kalma2-bridge`; la orquestación de agentes reside en `agent_runtime.rs`» (como si fueran la misma superficie) | **Conflación de tres órganos** | (1) `kalma2-bridge` = daemon/interfaz HTTP Kalma2 (`SddIA/interfaces/kalma2-bridge/`). (2) `agent_runtime.rs` = orquestador Rust: si `SDDIA_AGENT_RUNTIME_COMMAND` está definido y no hay `SDDIA_AGENT_RELAY_IDE`, spawnea ese CLI con JSON `operation: AGENT_PHASE` por stdin. **Hoy no lee** `SddIA/agents/<name>.md` ni emite `llm_profiles`. (3) Receptor vigente de forja: `SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh` → `.py`. Modelo físico actual del backend SDK: **una** variable `SDDIA_AGENT_RUNTIME_MODEL` (default `composer-2.5`). |
| **H10** | «`clarify` es una acción (`SddIA/actions/clarify.md`) bajo jurisdicción de `mayeuta`» | **Archivo fantasma residual** | `SddIA/actions/clarify.md` **no existe**. Evolution `51b4d573`: acciones de fase de negocio (`clarify`, `spec`, …) salieron del Core al limbo. `clarify.md` es **artefacto documental** bajo `persist_ref` (`features-documentation-pattern`). En `feature` v1.3.2 la fase «Estabilización de Requisitos» delega `agent:mayeuta` y persiste transcript como `clarify.md`. |
| **H11** | Usar **Ceguera Espacial** como «el genoma no conoce el nombre comercial del LLM» | **Sobrecarga semántica** | Ceguera Espacial / Consciencia Espacial (contratos de agente y `README.md`) = no hardcodear rutas del host; resolver vía `cumulo.paths.json` / `cumulo_topology`. El agnosticismo de proveedor LLM es el **agnosticismo del Core** (`README.md` § Core independiente de instancia), no el mismo principio. |
| **H12** | Bloque de ejemplo `SDDIA_LLM_TIER_HIGH="gemini-1.5-pro"` (y flash/8b) | **Reincidencia de H7** | Esos slugs no son SSOT. El starter-kit vigente documenta `SDDIA_GEMINI_MODEL=` vacío + comentario fechado. Este PBI no debe reintroducir marcas como valor-ley. |
| **H13** | Título «Resolución Dinámica» + plan que solo inyecta JSON en `agent_runtime` | **Alcance hueco** | Inyectar `llm_profiles` sin que el receptor mapee `tier` → modelo deja las vars `SDDIA_LLM_TIER_*` como ornamento. El mapeo físico vive hoy en el harness Python (y/o backend CLI/SDK), **no** en `kalma2-bridge` y **no** en `skill:mayeuta-llm`. |
| **H14** | Argos `tier: medium` sin acotar contra su propia doctrina | **Tensión no laudada** | `argos.md`: Principio de Evidencia Determinista; **prohibido** sustituir linters/tests por razonamiento LLM sobre el código. Un tier cognitivo solo es coherente para **síntesis** de `audit_report_md` / `correction_blueprint_md` **después** de evidencia de cápsulas. |
| **H15** | Fase A/CA cubren `cerbero`/`cumulo` pero omiten `radamanto`; «no asignar tier» basta | **Incompletitud + hueco de despacho** | `radamanto.md`: genoma determinista, sin interpretación de intenciones. En `pull-request-review` v2.3.0, «Certificación RBAC» delega `agent:cerbero` y «Cosecha Kaizen» `agent:cumulo`. `executor.rs` envía **cualquier** fase solo-`agent:` al CLI LLM si el runtime está configurado. Meter `tier: none` en el payload **no** impide el spawn. Hace falta veto de despacho. |
| **H16** | «Todo el pipeline de SddIA opera en Rust nativo» / CA «cero menciones a `sddia-client-bridge.py`» | **Sobreclaim + CA ajeno** | Orquestador y cápsulas nuevas: Rust. El harness de fases agente de forja **sigue siendo Python** (`kalma2-agent-runtime-cursor.py`). `llm:interact` → `skill:mayeuta-llm` (Rust) es **otra** superficie. El CA del bridge Python no aporta Done de tiers. |
| **H17** | Ejemplo JSON de payload como esquema nuevo (`process_name: feature`, fase «Arquitectura y diseño») | **Payload de ficción** | Payload real (`agent_runtime.rs` ~388–402): `operation`, `process_name`, `phase_name`, `agents`, `persist_ref`, `branch_name`, `execution_id`, `correlation_id`, `pbi_ref`, `inputs`, `workspace_path`, `repo_root`, más `runtime_evidence` / `di_binding`. En `feature` v1.3.2 no existe la fase «Arquitectura y diseño»; Dedalo corre en «Diseño de Blueprint». Este PBI **extiende** el objeto vigente con `llm_profiles`; no lo sustituye. |

---

## 1. Contexto y justificación

La selección fija de un único modelo para todas las fases `agent:` (`SDDIA_AGENT_RUNTIME_MODEL` hoy) aplasta coste y capacidad: Dedalo/Mayeuta necesitan más razonamiento; Tekton, iteración barata; Argos, síntesis acotada. El genoma debe declarar un **nivel cognitivo abstracto**; la instancia resuelve el modelo físico en bóveda.

Tres superficies LLM **ortogonales** (no fusionar):

| Superficie | Órgano | Qué enruta | Este PBI |
|------------|--------|------------|----------|
| Fases `agent:` de proceso (feature / bug-fix / refactorization / PPR) | `agent_runtime.rs` → `SDDIA_AGENT_RUNTIME_COMMAND` (harness `kalma2-agent-runtime-cursor`) | Quién (agente) y con qué tier abstracto | **Sí** |
| Capacidad `llm:interact` | `capability-bindings.md` → `skill:mayeuta-llm` (`SDDIA_LLM_CLI_COMMAND` / `SDDIA_LLM_CHAT_COMMAND`) | Chat/clasificación Kalma2, Telegram, email-triage | **No** (L-ORTHOGONAL-INTERACT) |
| Martillos HTTP/CLI | `tool:gemini-http-infer` (`SDDIA_GEMINI_MODEL`, `GEMINI_API_KEY`); `skill:antigravity-cli-executor` | Inferencia/CLI invocados por `delegates_to` o `--tool` | **No** (ya laudado en PBI Antigravity L5) |

Predecesor fósil: `docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md` (v1.0.0) aún cita `sddia-client-bridge.py`, agente «Tormentosa» y hardware de instancia. **No implementar** ese texto. Este PBI lo sustituye para el recorte *tiers de agentes*. Adaptador multi-proveedor amplio (Groq, Ollama como producto) permanece kitchen.

Agnosticismo del Core: el YAML de `SddIA/agents/*.md` no contiene slugs comerciales, IPs ni URLs de inferencia.

---

## 2. Especificación técnica canónica

### 2.1 Declaración del perfil cognitivo en el genoma (`directories.agents`)

Contrato `agents-contract.md`: campo **opcional** `llm_profile`. Subir `contract_version` (el YAML del contrato declara hoy `1.0.0`; `tekton.md` ya cita `v1.1.0` — drift preexistente, no lo inventa este PBI).

```yaml
llm_profile:
  tier: "high"   # high | medium | low | none
  description: "Diseño arquitectónico y especificación formal de procesos"
```

Mutación de `SddIA/agents/**` y del contrato: **DA-2** — vía `entity-manager` / `{entity}-creator` en el ciclo `feature`, no Write IDE.

#### Matriz de clasificación (laudo de este PBI)

| Agente | Naturaleza | Tier | Notas de doctrina |
|--------|------------|------|-------------------|
| `dedalo` | Arquitectura y procesos | `high` | Blueprint y especificación. Fase `feature`: «Diseño de Blueprint». |
| `mayeuta` | Clarificación | `high` | Transcript + requisito estable. Produce el artefacto `clarify.md`; no es una Action Core. |
| `argos` | Verificación y juicio | `medium` | **Solo síntesis** post-evidencia. Ver L-ARGOS-SYNTHESIS. |
| `tekton` | Forja | `low` | Ejecución táctica según spec. |
| `radamanto` | Actuario / umbrales | `none` | Telemetría y sellos; prohibido LLM. |
| `cerbero` | Gobernanza RBAC | `none` | Gate nativo `cerbero_di_rbac.rs` + skill DI `gov:rbac` → `rbac-governor`. Prohibido LLM. |
| `cumulo` | Topología / SSOT | `none` | Rutas e índices; prohibido LLM. |

Agentes `none` **declaran** `llm_profile.tier: none` (explícito, auditable). No omitir el bloque «por defecto».

### 2.2 Extensión del payload en `invoke_agent_phase`

Estado actual: `agent_runtime.rs` construye el JSON `AGENT_PHASE` **sin** leer definiciones de agente.

Cambio:

1. Tras `agent_names(delegates)` (prefijo `agent:`), resolver cada `{name}.md` bajo `directories.agents` (misma convención que `capability_di_gate::resolve_capsule_md`: `{repo}/SddIA/agents/{name}.md` hasta que Cúmulo inyecte la ruta; prohibido inventar un tercer parser).
2. Extraer frontmatter con `crate::core::parser::parse_frontmatter` (Path) o `parse_frontmatter_from_str` (misma regla `split("---")`). El mapa ya admite nodos YAML anidados (`serde_yaml::Value`).
3. Construir `llm_profiles: { "<name>": { "tier": "...", "description": "..." } }` **solo** para agentes cuyo `.md` se leyó. Agente ausente en disco: fail-soft (omitir clave, no panic); test obligatorio.
4. **Veto de despacho (L-NONE-NO-SPAWN):** si **todos** los agentes de la fase tienen `tier: none` (o son el conjunto `{cerbero, cumulo, radamanto}` aun sin YAML aún parchado), **no** spawnear `SDDIA_AGENT_RUNTIME_COMMAND`. Devolver `status` no-LLM (`executed` si el gate nativo ya cubre el peaje — Cerbero DI ya corre antes en `executor.rs` — o `simulated` con `note: deterministic-agent-no-llm` si no hay handler nativo de fase). Prohibido «interpretar a Cerbero» con Cursor.
5. Si la fase mezcla un agente cognitivo con uno `none`, spawnear el CLI (el cognitivo lo necesita) e incluir ambos en `llm_profiles` para que el harness no asigne modelo al `none`.
6. Insertar `llm_profiles` en el **objeto existente**; conservar `persist_ref`, `branch_name`, `execution_id`, `repo_root`, `runtime_evidence`, `di_binding`, etc.

Forma del campo nuevo (el resto del payload no se redibuja aquí):

```json
{
  "llm_profiles": {
    "dedalo": {
      "tier": "high",
      "description": "Diseño arquitectónico y especificación formal de procesos"
    }
  }
}
```

Relevo lab: `SDDIA_AGENT_RELAY_IDE` truthy sigue forzando `simulated` **antes** de leer el comando (norma `external-ai-constraints.md` DA-5). Este PBI no cambia ese contrato.

### 2.3 Matriz de resolución en bóveda (instancia, no genoma)

Jerarquía real (`load_hierarchical_env` / `_sddia_load_vault`): `.dev/.env` (global) luego `.SddIA/.dev/.env` (instancia pisa). El hijo de `Command` hereda `std::env` ya aplicado.

Familias de variables **ya existentes** (no redefinir ni aliasar):

| Variable | Superficie | Relación con este PBI |
|----------|------------|------------------------|
| `SDDIA_AGENT_RUNTIME_COMMAND` | Spawn del harness de fases agente | Intocado |
| `SDDIA_AGENT_RUNTIME_MODEL` | Un modelo para backend SDK del harness | **Fallback** si `SDDIA_LLM_TIER_*` del tier pedido está vacío |
| `SDDIA_LLM_CLI_COMMAND` / `SDDIA_LLM_CHAT_COMMAND` / `SDDIA_LLM_INFER_COMMAND` | `skill:mayeuta-llm` / chat Kalma2 | Fuera de alcance |
| `SDDIA_GEMINI_MODEL` / `GEMINI_API_KEY` | `tool:gemini-http-infer` | Fuera de alcance |

Nuevas (solo bóveda / starter-kit, comentadas, sin slug-ley):

```bash
# SddIA — Tiers cognitivos de fases agent: (instancia). Vacío = fallback SDDIA_AGENT_RUNTIME_MODEL.
# El slug lo elige el operador el día que el proveedor lo acepte. No es contrato del Core.
# SDDIA_LLM_TIER_HIGH=
# SDDIA_LLM_TIER_MEDIUM=
# SDDIA_LLM_TIER_LOW=
```

Plantillas: `SddIA/scripts/starter-kit/.dev/.env.example` y `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example` (existen; Write directo, no DA-2).

### 2.4 Despacho físico (receptor, no `kalma2-bridge`)

1. `execute-process` carga bóveda (`load_hierarchical_env`) y, si procede, spawnea `SDDIA_AGENT_RUNTIME_COMMAND`.
2. El receptor vigente (`kalma2-agent-runtime-cursor.py`) lee `llm_profiles` del JSON stdin.
3. Para cada agente cognitivo, `tier` → `SDDIA_LLM_TIER_{HIGH|MEDIUM|LOW}` → si vacío, `SDDIA_AGENT_RUNTIME_MODEL` → si vacío, el default **actual del harness** (hoy `composer-2.5` en el backend SDK; no sustituirlo por un slug Google en el Core).
4. Backend CLI (`cursor-agent --print`): aplicar el modelo solo si el CLI lo expone sin inventar flags. Si el CLI de forja no acepta modelo por agente, documentar degradación: un modelo por fase (el del agente titular de mayor tier en esa fase). Prohibido fingir multiplexado.
5. `SddIA/scripts/tools/` **no** está en la tabla DA-2 de genoma; el parche del harness es lícito en este ciclo. **No** reabrir `kalma2-bridge`.

---

## 3. Plan de implementación

### Fase A — Genoma (DA-2)

1. Update `agents-contract.md`: esquema opcional `llm_profile`.
2. Update `dedalo.md`, `mayeuta.md`, `argos.md`, `tekton.md`, `radamanto.md`, `cerbero.md`, `cumulo.md` con el bloque de la matriz (incluye `none` explícito).
3. Update `agents/index.md` si se añade columna/nota; sincronizar YAML ↔ fila.
4. Nota en `argos.md`: el tier no autoriza sustituir evidencia determinista.

### Fase B — Orquestador (`execute-process`, DA-2 crate)

1. Extender `agent_runtime.rs`: lectura de `{name}.md`, `llm_profiles`, veto L-NONE-NO-SPAWN.
2. Tests Cargo:
   - `tier: high` → payload `llm_profiles.<agente>.tier == "high"` y el mock stdin lo recibe.
   - Fase solo `agent:cerbero` (o `none`) → **no** spawn; status/note del laudo.
   - `.md` ausente → fail-soft, sin panic.
   - Payload conserva `repo_root` / `persist_ref` tras la extensión.

### Fase C — Bóvedas starter-kit

Documentar `SDDIA_LLM_TIER_*` comentadas, vacías, con puntero a fallback `SDDIA_AGENT_RUNTIME_MODEL`. Cero secretos. Cero slugs eternos.

### Fase D — Receptor de forja (`kalma2-agent-runtime-cursor.py`)

1. Consumir `llm_profiles`; resolver modelo (2.4).
2. Test del harness (mock, sin Cursor de red): JSON con `llm_profiles.dedalo.tier=high` y env `SDDIA_LLM_TIER_HIGH` → el backend SDK/CLI recibe ese id; sin env de tier → fallback `SDDIA_AGENT_RUNTIME_MODEL`.
3. No tocar `skill:mayeuta-llm` ni bindings `llm:interact`.

---

## 4. Criterios de aceptación

* [ ] `agents-contract.md` define `llm_profile.tier`: `"high" \| "medium" \| "low" \| "none"`.
* [ ] Los siete agentes del catálogo declaran `llm_profile` acorde a la matriz (§2.1).
* [ ] `argos.md` deja explícito L-ARGOS-SYNTHESIS (síntesis ≠ verificación física).
* [ ] Mutaciones de `SddIA/agents/` y del crate `execute-process` pasan por `entity-manager` (DA-2).
* [ ] `cd SddIA && cargo check -p execute-process` OK.
* [ ] `invoke_agent_phase` adjunta `llm_profiles` al payload **existente** de `SDDIA_AGENT_RUNTIME_COMMAND`.
* [ ] Fase solo determinista (`tier: none`) **no** spawnea el CLI LLM (test).
* [ ] Tests Cargo cubren extracción, fail-soft y veto de spawn.
* [ ] Harness `kalma2-agent-runtime-cursor.py` mapea tier → env → fallback `SDDIA_AGENT_RUNTIME_MODEL` (test mock).
* [ ] Starter-kit documenta `SDDIA_LLM_TIER_HIGH|MEDIUM|LOW` comentadas, vacías, sin marcas eternas.
* [ ] Cero slugs `gemini-1.5-*` / `gemini-3.1-*` como valor-ley en este PBI, examples o genoma.
* [ ] `capability-bindings.md` `llm:interact` → `skill:mayeuta-llm` intacto.

---

## 5. Fuera de alcance (explícito)

* Rebind de `llm:interact` o cambiar el provider `skill:mayeuta-llm`.
* Usar `tool:gemini-http-infer` / `skill:antigravity-cli-executor` como router de fases `agent:`.
* Reimplementar o acoplar `kalma2-bridge` (WUI HTTP) a los tiers.
* Resucitar `.SddIA/client/sddia-client-bridge.py`.
* Sustituir `cerbero_di_rbac.rs` (o `skill:rbac-governor`) por un LLM.
* Implementar el kitchen `PBI-MULTI-LLM-ROUTER` (multi-proveedor, Groq, Ollama de producto, hardware de instancia).
* Forzar marca o API en el crate Core.
* Cambiar el contrato de `SDDIA_AGENT_RELAY_IDE`.

---

## 6. Laudos arquitectónicos

| ID | Pregunta | Laudo vigente |
|----|----------|---------------|
| **L-TIER-LEVELS** | ¿Tiers normalizados? | `high` (Dedalo, Mayeuta), `medium` (Argos síntesis), `low` (Tekton), `none` (Cerbero, Cúmulo, Radamanto). |
| **L-DETERMINISM-RBAC** | ¿Cerbero con LLM medium? | **No.** Gate: `cerbero_di_rbac.rs`. Agente doctrinal: `cerbero.md`. DI: `gov:rbac` → `skill:rbac-governor`. Tres caras, cero estocástica. |
| **L-NONE-NO-SPAWN** | ¿Basta omitir el tier en el JSON? | **No.** Fase solo `none` no invoca `SDDIA_AGENT_RUNTIME_COMMAND`. |
| **L-ARGOS-SYNTHESIS** | ¿Argos puede tener tier? | `medium` solo para redactar informe/blueprint **tras** stdout/exit de cápsulas. Prohibido LLM-como-linter. |
| **L-DISPATCH-SURFACE** | ¿Quién extrae el perfil? | `execute-process::engine::agent_runtime` inyecta `llm_profiles` en el payload `AGENT_PHASE`. |
| **L-RESOLVE-SURFACE** | ¿Quién resuelve el modelo físico? | El receptor de `SDDIA_AGENT_RUNTIME_COMMAND` (hoy `kalma2-agent-runtime-cursor.py`), cruzando `SDDIA_LLM_TIER_*` con fallback `SDDIA_AGENT_RUNTIME_MODEL`. **No** `kalma2-bridge`. |
| **L-ORTHOGONAL-INTERACT** | ¿Tiers rebindan `llm:interact`? | **No.** Mayeuta-llm y las fases agente no comparten router. |
| **L-ENV-VARS** | ¿Dónde viven los slugs? | Solo bóveda de instancia. Starter-kit: comentarios vacíos. Genoma: cero marcas. |
| **L-BRIDGE-REMOVAL** | ¿Hace falta un puente Python nuevo? | **No.** El WUI ya es Rust (`40ef941`). El harness de forja ya existe en Python; este PBI lo **extiende**, no lo sustituye por un cuarto órgano. |
| **L-PAYLOAD-EXTEND** | ¿Nuevo esquema stdin? | **No.** Extender el `AGENT_PHASE` vigente. Fases reales: las de `feature.md` / `bug-fix.md` / `pull-request-review.md`, no nombres inventados. |
| **L-KITCHEN-ROUTER** | ¿Qué pasa con `PBI-MULTI-LLM-ROUTER`? | Fósil táctico. No es spec ejecutable. Recorte de tiers = este PBI. |
