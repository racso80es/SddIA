---
document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
title: "[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N"
format: markdown
version: "1.1.0"
status: "historia"
type: historia
priority: alta
context: "Cliente SddIA (Kalma2) / Proyectos cliente aislados (ABSTRACT-04)"
created: "2026-09-26"
updated: "2026-09-26"
source: "Conversación técnica Racso ↔ Tormentosa 2026-09-26, refinada contra el repo por Tekton"
process_candidate: feature
project_pilot:
  slug: barcelonaxplorer
  project_root: /home/racso/Proyectos/BarcelonaXplorer
client_instances_observed:
  forge_candidate: /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA
  consumer_paciente0: /home/racso/Proyectos/SddIA_AP
audit_ref: docs/audits/installer-deploy-aplicaciones-20260926T113533Z.md
blocked_by:
  - PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO (fase F5)
decisions_status: dictaminadas (D1–D6, 2026-09-26); residuales en §7 para Dédalo
changelog:
  - "1.1.0: §7 pasa de abiertas a dictaminadas (D1 MCP stdio directo; D2 SddIA/tools; D3 autoridad por project.codex_slug con candado software_forge en active-domain-profile.json; D4 env_ref con precedencia real SO > proyecto > instancia > global; D5 active_norm_pack; D6 filesystem-manager 2.0.0 Rust). Correcciones Filtro A: candado no va en bóveda; precedencia SO no invertible; no existe contrato congelado de filesystem-manager; PATCH_FILE es ampliación; soporte MCP de backends no verificado en repo; tools-contract exige io_mode. AC-11..AC-15; §4 y §5 alineados."
  - "1.0.1: errata G3/§2.3 — la instancia de Aplicaciones existe (full-node, autoridad software legado, runtime agentes heredado) pero no está operativa (WUI en crash loop por puerto 8765, bóveda de forja copiada). Ver audit_ref."
derived_from:
  - PBI-SDDIA-DOMAIN-ABSTRACT-04
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS
related_features:
  - docs/features/sddia-codex-agile-forge
  - docs/features/kalma2-bridge-rust
  - docs/features/kalma2-full-cycle
  - docs/features/kalma2-llm-live
  - docs/features/antigravity-connectors
ssot_refs:
  - SddIA/core/cumulo.paths.json (instance.projects, paths.resolution, paths.workspacesRoot, env_hierarchy)
  - SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/norms/execution-contexts.md
  - SddIA/skills/filesystem-manager.md
  - SddIA/skills/git-manager.md
  - SddIA/skills/shell-executor.md
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/domain_authority.rs
---

# Gobernar BarcelonaXplorer (y N proyectos) desde Kalma2 sin IDE de ejecución asistida

## 1. Historia de usuario

**Como** Vértice Biológico (Racso), operador de un cliente SddIA desplegado,
**quiero** seleccionar en Kalma2 un proyecto registrado (p. ej. BarcelonaXplorer) y lanzar sobre él el ciclo `feature` / `bug-fix` / `refactorization` completo,
**para** que Mayeuta, Dédalo, Tekton y Argos lean, muten, prueben y consoliden el código de ese proyecto **sin abrir el IDE de Antigravity ni Cursor**, con Ceguera Espacial, aduana RBAC y trazabilidad de extremo a extremo; y que dar de alta un proyecto nuevo sea registrar un manifiesto, no tocar el Core.

## 2. Contexto real verificado (estado a 2026-09-26)

La conversación origen contiene afirmaciones correctas mezcladas con inexactitudes. Esta sección fija la **verdad del repositorio**; la historia se construye sobre ella.

### 2.1 Lo que ya existe y se reutiliza

| Pieza | Estado real | Evidencia |
|-------|-------------|-----------|
| Cliente web Kalma2 | `interfaces/kalma2/` (HTML/Vanilla JS). Botones: `Chat`, `Hablar con Tormentosa`, `Forjar Proceso`, `Sincronizar Genoma`. Sin selector de proyecto. | `interfaces/kalma2/index.html` |
| Puente HTTP | **Rust** `kalma2-bridge` (`SddIA/interfaces/kalma2-bridge/`, `tiny_http`). Rutas: `/api/chat`, `/api/execute`, `/api/interact`, `/api/status`, `/api/progress/stream`. | `kalma2-bridge/src/main.rs` |
| Proceso de entrada | `kalma2-interact` v1.1.1 → allowlist `bug-fix`, `feature`, `refactorization`, `task-queue-manager` → evento `Kalma2_Process_Requested` → suscriptor `tekton/task-queue-manager` + anclaje DLT Cúmulo. | `SddIA/process/kalma2-interact.md`, `event-domain-subscriptions.json` |
| Modelo de proyecto aislado (ABSTRACT-04) | Índice Core `.SddIA/projects/{slug}.md` (SSOT `instance.projects`) + manifiesto `{project_root}/.SddIA/project.md` (`project-config-contract` v1.0.0: `git_remote`, `default_branch`, `delivery_mode`, `codex_slug`, `docs_layout`). `inputs.project_slug` resuelve `project_root`; `persist_ref` anclado con `PROJECT_SCOPE_ESCAPE` → `System_Fracture_Detected`. `forge-pbi` ya siembra PBIs en el `todos_pending` del proyecto. | `docs/features/sddia-codex-agile-forge/spec.md`, `project_binding.rs`, `workspace_init.rs` |
| Runtime de agentes headless | `SDDIA_AGENT_RUNTIME_COMMAND` → fases solo-`agent:` reciben payload `AGENT_PHASE` (JSON stdin → última línea JSON stdout). Backend actual: `kalma2-agent-runtime-cursor.{sh,py}` (`cursor-agent --print` o SDK). | `agent_runtime.rs`, `.dev/.env.example` |
| Cápsulas de acción físicas | `git-manager` (Rust, `repository_path` absoluto inyectado), `shell-executor` (Rust, whitelist Cerbero, `working_directory` inyectado, anti-git). | `SddIA/skills/*.md` |
| Conectores Antigravity | `antigravity-cli-executor` (skill Rust sobre `agy` CLI, `--add-dir` solo paths inyectados) y `gemini-http-infer` vía `tool:llm-router` (`llm:infer`). | `docs/features/antigravity-connectors` |
| Códices de dominio | `codex-software-engineering` (autoridad sobre `feature`/`bug-fix`/…), `codex-frontend-product-splus`, `codex-backend-admin-splus`, `codex-kalma2-assistant`. | `SddIA/library/codexes/` |
| Piloto | `/home/racso/Proyectos/BarcelonaXplorer` existe, con `.SddIA/library/norms/` (normas locales) pero **sin** `.SddIA/project.md`. | `ls` verificado |

### 2.2 Brechas reales (lo que impide hoy el objetivo)

| ID | Brecha | Detalle |
|----|--------|---------|
| G1 | **Sin runtime físico de filesystem headless** | `filesystem-manager` es *LLM-Native*: no tiene cápsula; "la propia IA en el entorno de desarrollo actúa como runtime". Sin IDE, ninguna cápsula gobernada escribe código en el proyecto. Es la brecha que Antigravity/Cursor tapan hoy. |
| G2 | **El runtime de agentes ignora `project_root`** | `AGENT_PHASE` inyecta `repo_root` (Core), `workspace_path`, `persist_ref`, `inputs`; `build_agent_command` fija `current_dir(repo)` y el runtime Cursor hace `cwd=repo_root`. `project_root` sale de `workspace_init` al estado pero **no** llega al payload del agente. Tekton opera siempre sobre el Core. |
| G3 | **La instancia con autoridad software no está operativa ni saneada** | Dos instancias desplegadas: (a) `SddIA_AP` (Paciente 0, consumer 13 ELF, `codex-kalma2-assistant`, `git_required:false`) → `DOMAIN_AUTHORITY_DENIED` para `feature`/`bug-fix`; el starter-kit prohíbe `SDDIA_AGENT_RUNTIME_COMMAND` en ella (Filtro C). **No** es forjador por diseño. (b) `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA` (installer PR #300, full-node 34 ELF, perfil default legado → autoridad software **sí**, `SDDIA_AGENT_RUNTIME_COMMAND` heredado de la forja) → es la **candidata a forjadora**, pero su `kalma2-bridge` lleva ~15 900 reinicios por `bind 8765` ocupado por el bridge-lab de la forja, `telegram-watcher` e `iota-publish-relay` en crash loop, y su bóveda es copia de la global de la forja (secretos IMAP/Gemini duplicados, sin `SDDIA_CLIENT_PORT`). Sin `active-domain-profile.json` explícito. Auditoría: `docs/audits/installer-deploy-aplicaciones-20260926T113533Z.md`. |
| G4 | **Kalma2 no transporta `project_slug`** | Ni `kalma2-interact` (input único `prompt`) ni la UI ni `Kalma2_Process_Requested` → `task-queue-manager` llevan el proyecto destino. |
| G5 | **Proyecto piloto no registrado** | No existe `.SddIA/projects/barcelonaxplorer.md` en ninguna instancia ni `BarcelonaXplorer/.SddIA/project.md`. |
| G6 | **No hay servidor ni cliente MCP en SddIA** | Cero implementación; en features previas el catálogo MCP visto desde el IDE fue `[]`. |

### 2.3 Correcciones a la conversación origen (Filtro A — alucinaciones e imprecisiones)

| Afirmación en la conversación | Corrección |
|-------------------------------|-----------|
| Puente `.SddIA/client/sddia-client-bridge.py` "o daemon en Rust" | El puente Python **fue podado** (feature `kalma2-bridge-rust`, O6). Solo existe `kalma2-bridge` en Rust. `.SddIA/client/` únicamente contiene `__pycache__` residual. |
| Botón «Forjar» | El botón es `Forjar Proceso` (`#forge`); coexiste con `Chat`, `Hablar con Tormentosa` y `Sincronizar Genoma`. |
| Proponer un *Workspace Registry* JSON nuevo (`"workspaces": {...}`) | **Rechazado**: duplica el SSOT ya vigente `instance.projects` (`.SddIA/projects/{slug}.md`) + `project.md`. La historia extiende ese contrato; no crea un registro paralelo. |
| `env_file` del proyecto en el registro | La jerarquía de bóvedas es SSOT (`env_hierarchy`: `.dev/.env` global, `.SddIA/.dev/.env` instancia). Una bóveda por proyecto es una **extensión** del contrato (`env_ref`, dictaminada en §7 D4), no un hecho actual. |
| Tools MCP `apply_patch`, `run_test_suite`, `git_checkpoint` como primitivas nuevas | Colisionan con `filesystem-manager`, `shell-executor` y `git-manager`. El servidor debe **exponer/adaptar** esas cápsulas y sus esquemas congelados (`skill-io-git-manager-frozen`, `skill-io-shell-executor-frozen`), no reimplementarlos (Filtro C). |
| "Tekton ejecuta de forma encapsulada a través de cápsulas WASI / Rust nativo" | Tekton es un **agente LLM** cuyo runtime es `SDDIA_AGENT_RUNTIME_COMMAND` (hoy `cursor-agent`). Las cápsulas son las skills/tools que invoca. Prescindir del IDE ≠ prescindir del modelo: Tekton seguirá necesitando un backend LLM (`cursor-agent`, `agy`, `gemini-http-infer`). |
| "Cliente SddIA desplegado (carpeta Aplicaciones)" | **Correcto con matiz** (errata propia de v1.0.0 corregida): `/home/racso/Aplicaciones/SddIA/` son solo atajos (`SddIA_Deploy.sh`, `SddIA_Eliminar_Cliente.sh` → forja); la instancia es `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA/` (installer PR #300, full-node). Está **materializada pero no operativa** como asistente: WUI sin puerto, Telegram sin token (G3, auditoría). Paciente 0 (`SddIA_AP`) es una instancia distinta, consumer, sin autoridad software. |
| "Auditoría de Argos y Radamanto sobre artefactos antes de consolidar" | Argos audita artefactos/código. Radamanto **no** evalúa diffs: consume telemetría agregada y gobierna estatus macroscópico (README § Argos vs Radamanto). |
| "DLT / IOTA Rebased" | El repo referencia IOTA (`iota-immutable-publisher`); "Rebased" no aparece en ningún SSOT. Se cita solo como *anclaje DLT IOTA*. |
| "Kalma2 negocia capacidades con el MCP Server" | El **frontend es inerte** (Dogma O3). La negociación MCP la hace el puente o el orquestador, nunca el navegador. |
| "Fase 2: dotar al puente de sesiones MCP" | La UI y el puente no ejecutan procesos: inyectan estímulos. El **cliente MCP** natural es el runtime de agentes (el LLM que ejecuta a Tekton), no el puente HTTP. |

## 3. Tesis arquitectónica refinada

El hueco que dejan Antigravity/Cursor no es "la inteligencia" (sigue habiendo un backend LLM), sino el **runtime físico gobernado sobre el proyecto destino**: leer árbol, aplicar diffs, correr tests, operar git, todo acotado a `project_root` y auditado.

Ese rol se materializa como un **Workspace Server por proyecto**, servidor **MCP** (JSON-RPC 2.0 sobre stdio) en Rust, que:

1. Se levanta bajo demanda por el orquestador para un `project_slug` registrado, con `--root {project_root}` resuelto vía Cúmulo (ceguera espacial: el agente nunca ve el path).
2. Expone **Resources** de lectura sandboxeada y **Tools** que son *adaptadores finos* sobre las cápsulas existentes (`git-manager`, `shell-executor`) más la implementación física que hoy falta de `filesystem-manager` (G1).
3. Aplica **Cerbero** antes de cada tool (contexto RBAC `filesystem-ops` / `source-control` / `system-operations`; whitelist de ejecutables y tests declarados en `project.md`).
4. Emite `Raw_Execution_Finished` por invocación al bus fractal para telemetría Radamanto.

Y el **cliente MCP** es el runtime de agentes (`SDDIA_AGENT_RUNTIME_COMMAND`): el backend LLM (cursor-agent, `agy`, u otro) recibe el Workspace Server como su **único** servidor MCP y `cwd` neutro, de modo que la única mano que toca BarcelonaXplorer es la aduana SddIA.

**Por qué MCP y no solo `capsule-json-io`:** las cápsulas siguen hablando JSON stdin/stdout; MCP es la envoltura estándar que los runtimes LLM consumen de forma nativa. Permite cambiar de backend (Cursor → `agy` → local) sin reescribir la aduana. **Decidido (§7 D1):** MCP JSON-RPC 2.0 sobre stdio, directo, sin adaptador intermedio; soporte MCP de cada backend se verifica por smoke en F3.

```text
[ Kalma2 UI ] --POST /api/execute {process, project_slug, prompt}--> [ kalma2-bridge (Rust) ]
                                                                         │
                                                                         ▼
                                            [ execute-process ] resuelve project_slug vía instance.projects
                                                                         │ spawn AGENT_PHASE (+ project_root, mcp_servers)
                                                                         ▼
                                     [ agent-runtime (backend LLM: cursor-agent | agy | …) ]
                                                                         │ MCP stdio
                                                                         ▼
                              [ sddia-workspace-server --root {project_root} ]  ← Cerbero + telemetría
                                       resources: tree, file, git-status, docs_layout
                                       tools:     fs_write/apply_patch → filesystem-manager físico
                                                  git_*               → git-manager (esquema congelado)
                                                  run_check           → shell-executor (whitelist project.md)
                                                                         │
                                                                         ▼
                                              /home/racso/Proyectos/BarcelonaXplorer  (sandbox estricto)
```

## 4. Alcance

### Incluido

- **A. Registro del piloto** bajo el contrato existente: `.SddIA/projects/barcelonaxplorer.md` en la instancia forjadora + `BarcelonaXplorer/.SddIA/project.md` con `codex_slug: codex-software-engineering` (D3), `delivery_mode`, `docs_layout`; `project-config-contract` 1.0.0 → **1.1.0** con `env_ref` (D4) y `qa_gates` / `mcp.allowed_executables` opcionales.
- **B. Crate `SddIA/tools/sddia-workspace-server`** (D2): tool MCP stdio (D1), spawn por fase, `--root {project_root}`. Entidad `{name}.md` con `uuid`, `type: tool`, `version`, `context` RBAC (`filesystem-ops`, `source-control`, `system-operations`), `io_mode: mcp-stdio`; alta vía `entity-manager` (`tool-creator`). Bump de `tools-contract` para reconocer `io_mode`.
- **C. `filesystem-manager` 2.0.0 físico en Rust** (D6, G1): mismo `uuid` y `provides`; enum ampliado con `PATCH_FILE`; norma congelada `skill-io-filesystem-manager-frozen` nueva; modalidad LLM-Native extinguida. Contención en `project_root` / `workspace_path`.
- **D. Propagación de `project_root` al runtime de agentes** (G2): `AGENT_PHASE` incluye `project_root` y descriptor `mcp_servers`; el runtime Cursor/`agy` recibe el Workspace Server como **único** MCP server y `cwd` neutro (**no** Core). Smoke `initialize` por backend.
- **E. Transporte de `project_slug` extremo a extremo** (G4): UI (selector inerte poblado desde `/api/status` o ruta nueva de lectura del índice), `kalma2-bridge`, `kalma2-interact`, `Kalma2_Process_Requested`, `task-queue-manager` → `feature`/`bug-fix` con `inputs.project_slug`.
- **F. Autoridad e instancia forjadora** (G3, D3): `domain_authority.rs` resuelve por `project.codex_slug` con candado `software_forge: true` en `active-domain-profile.json`; instancia candidata `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA` saneada según `PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO` (puerto, bóveda compuesta, unidades condicionales, perfil explícito). **Paciente 0 / SddIA_AP no se muta** y sigue denegada.
- **H. Bóveda de proyecto** (D4): el Workspace Server carga `env_ref` y la inyecta como entorno efímero solo a sus cápsulas hijas; precedencia real SO > proyecto > instancia > global.
- **G. Ciclo real de verificación**: un `bug-fix` sobre BarcelonaXplorer desde Kalma2 que muta un fichero, pasa `qa_gates` y cierra según `delivery_mode`, sin Antigravity ni Cursor IDE abiertos.

### Fuera de alcance

- Autenticación multiusuario / Karma2Token en el puente.
- Servidor MCP remoto (red); solo stdio local.
- Sustituir el backend LLM: `cursor-agent`/`agy` siguen siendo backends válidos vía `SDDIA_AGENT_RUNTIME_COMMAND`.
- Migrar Paciente 0 al modelo de proyecto aislado (ya excluido en `PBI-DT-PACIENTE0-DEPLOY-PROCESS`).
- Tokenización / anclaje DLT del Workspace Server más allá de la telemetría ya existente.

## 5. Fases de implementación (blueprint para Dédalo)

| Fase | Entregable | Cierra |
|------|-----------|--------|
| **F0 — Registro y contratos** | `project-config-contract` 1.1.0 (`env_ref`); `project.md` (`codex_slug: codex-software-engineering`) + índice Core del piloto; `execute-process --process forge-pbi --inputs '{"project_slug":"barcelonaxplorer",…}'` siembra un PBI en el `docs_layout.todos_pending` de BX. | G5 |
| **F1 — Workspace Server (lectura)** | `tools-contract` con `io_mode`; crate `SddIA/tools/sddia-workspace-server` + `{name}.md`; `initialize`, `resources/list`, `resources/read` (`project://tree`, `project://file/{rel}`, `project://git-status`, `project://docs/{key}`); carga de `env_ref` (solo hijos); sandbox anti-traversal; tests unitarios de escape. | G6 (parcial) |
| **F2 — Tools adaptadoras + filesystem físico** | `filesystem-manager` 2.0.0 Rust + norma congelada; `tools/list`, `tools/call`: `fs_*` (incl. `PATCH_FILE`) → `filesystem-manager`, `git_*` → `git-manager`, `run_check` → `shell-executor` con whitelist de `project.md`. Cerbero antes de cada `tools/call`. `Raw_Execution_Finished` por invocación. | G1, G6 |
| **F3 — Runtime con proyecto** | `AGENT_PHASE` + `project_root` + `mcp_servers`; `kalma2-agent-runtime-cursor.py` monta el servidor MCP y `cwd` neutro; `antigravity-cli-executor` igual (sin `--add-dir` al proyecto). Smoke `initialize` MCP por backend; backend sin MCP = excluido para proyectos. | G2 |
| **F4 — Kalma2 1×N** | Selector de proyecto inerte; `project_slug` en `/api/execute`, `kalma2-interact`, evento y `task-queue-manager`. | G4 |
| **F5 — Autoridad e instancia forjadora** | `domain_authority.rs`: regla D3 (`project.codex_slug` ∧ `software_forge` ∧ perfil no-consumer); `active-domain-profile.json` con `software_forge`. Instancia de Aplicaciones saneada (depende de `PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO`); WUI operativa; separada de SddIA_AP. | G3 |
| **F6 — Ciclo real** | `bug-fix` de BarcelonaXplorer end-to-end desde Kalma2; `validacion.md` APTO; evolución registrada. | Objetivo |

## 6. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | Ningún agente ni el frontend contiene la ruta de BarcelonaXplorer; solo `.SddIA/projects/{slug}.md` la conoce. | `rg` sobre payloads/UI/agents = 0 coincidencias de `project_root` literal. |
| AC-2 | `resources/read` con `../` o symlink fuera de `project_root` → error tipado, sin panic; en tool de escritura → `PROJECT_SCOPE_ESCAPE` + `System_Fracture_Detected`. | Tests unitarios + smoke. |
| AC-3 | `tools/call` con ejecutable fuera de la whitelist de `project.md` → deny Cerbero, `exitCode≠0`, sin ejecución. | Test. |
| AC-4 | Cada `tools/call` emite `Raw_Execution_Finished` con `entity` = Workspace Server y `correlation_id` de la ejecución. | Inspección `./.events/telemetry`. |
| AC-5 | `AGENT_PHASE` de una fase Tekton sobre `project_slug` registrado contiene `project_root` y `mcp_servers`; el proceso hijo del runtime **no** tiene `cwd` = Core. | Trazas del runtime. |
| AC-6 | Desde Kalma2, con el selector en `BarcelonaXplorer`, «Forjar Proceso» → `Kalma2_Process_Requested.payload.project_slug = barcelonaxplorer` y `task-queue-manager` lo propaga a `bug-fix`. | Evento en bus. |
| AC-7 | Sin `project_slug`, todo el comportamiento actual (Core sobre sí mismo) es idéntico (AC-CORE-SELF de ABSTRACT-04). | Suite existente verde. |
| AC-8 | En SddIA_AP (perfil `codex-kalma2-assistant`) el ciclo sigue denegado (`DOMAIN_AUTHORITY_DENIED`); no se relaja el Filtro C del consumidor. | Test de perfil. |
| AC-9 | Ciclo `bug-fix` real: fichero de BarcelonaXplorer modificado, `qa_gates` verdes, commit/PR según `delivery_mode`, con Antigravity y Cursor IDE cerrados. | Evidencia en `validacion.md` + PR/commit. |
| AC-10 | Alta de un segundo proyecto = crear dos `.md` (índice + manifiesto), sin cambios en Core. | Registro de proyecto ficticio en lab. |
| AC-11 | (D3) Instancia con `software_forge: true` + `project.codex_slug: codex-software-engineering` → `allow`; misma instancia con `software_forge: false` o ausente → `DOMAIN_AUTHORITY_DENIED`; `project.codex_slug` ≠ software → deny aunque la instancia sea forjadora. | Tests unitarios `domain_authority.rs` (matriz 2×2×2). |
| AC-12 | (D4) Variable definida en SO **y** en `env_ref` → prevalece la del SO (salvo `VAULT_PRECEDENCE_KEYS`); variable de `env_ref` **no** aparece en el entorno del orquestador ni del proceso LLM, solo en los hijos del Workspace Server. | Test de entorno con `shell-executor` (`env` whitelisted) vs. inspección `/proc/{pid}/environ` del runtime. |
| AC-13 | (D6) `filesystem-manager` 2.0.0: `PATCH_FILE` con hunk que no casa → `exitCode 1`, fichero intacto (sin escritura parcial); cuerpo del `{name}.md` sin la sección "Modalidad LLM-Native"; `provides` idénticos a 1.1.0; `requires_capability fs:persist` de `task-queue-manager`/`feature` resuelve al binario. | Tests unitarios + `capsule-invoke-smoke` + resolución DI. |
| AC-14 | (D2) `{name}.md` del servidor declara `io_mode: mcp-stdio`; cada `tools/call` deja un envelope `capsule-json-io` interno en el `workspace_path` de la fase. | Inspección de artefactos de fase. |
| AC-15 | (D1) Smoke `initialize` MCP contra el backend configurado en `SDDIA_AGENT_RUNTIME_CLI` devuelve `capabilities` con `tools` y `resources`; fallo → la fase Tekton sobre proyecto aborta con error tipado `MCP_BACKEND_UNSUPPORTED`, sin fallback a `--add-dir`. | Smoke F3. |

## 7. Decisiones dictaminadas (Vértice Biológico, 2026-09-26)

Dictamen recibido y contrastado con el repositorio. Cada decisión lleva su **veredicto**, las **inexactitudes corregidas** (Filtro A) y la **consecuencia contractual**.

### D1 — Protocolo: MCP JSON-RPC 2.0 sobre stdio, directo

**Veredicto:** aprobado. El servidor habla MCP nativo; no se envuelve `capsule-json-io` en un adaptador externo.

| Precisión | Detalle |
|-----------|---------|
| "cursor-agent y `agy` ya implementan cliente MCP estándar" | **No verificado en el repo**: ni `kalma2-agent-runtime-cursor.py` ni `antigravity-cli-executor` contienen referencia MCP hoy. Es capacidad del binario externo, no de SddIA. F3 incluye un **smoke de negociación `initialize`** por backend antes de dar D1 por cumplido; si un backend no soporta MCP, ese backend queda excluido como runtime de Tekton sobre proyectos (no se crea adaptador). |
| Capa interna | Las **tools MCP** del servidor siguen invocando `git-manager` / `shell-executor` / `filesystem-manager` como cápsulas `capsule-json-io` (stdin/stdout, un envelope). MCP es la cara hacia el LLM; `capsule-json-io` sigue siendo la cara hacia el genoma. No hay doble motor. |

### D2 — Ubicación: `SddIA/tools/sddia-workspace-server`

**Veredicto:** aprobado. Es artefacto de infraestructura invocado y parametrizado (`--root {project_root}`) por el runtime durante la vida de la fase; no es superficie de usuario (`interfaces/*`).

| Precisión | Detalle |
|-----------|---------|
| `tools-contract` §5 exige *"un único envelope JSON por stdout"* | Un servidor MCP emite **N mensajes JSON-RPC** por sesión; viola la letra del contrato. Requisito: bump de `tools-contract` (o campo en el `{name}.md`) declarando `io_mode: mcp-stdio` como modalidad reconocida, con la obligación de que **cada `tools/call`** produzca internamente un envelope `capsule-json-io` auditable y un `Raw_Execution_Finished`. Precedente de excepción declarada: `antigravity-cli-executor` (spawn). |
| Ciclo de vida | Spawn por fase (`AGENT_PHASE`), muerte al cerrar la fase o por timeout de la barrera. Prohibido como servicio systemd persistente por proyecto. |

### D3 — Autoridad software: por `project.codex_slug` con candado en el perfil de instancia

**Veredicto:** aprobado con dos correcciones.

| Precisión | Detalle |
|-----------|---------|
| "flag de forja de software **en su bóveda**" | **Corregido**: el candado va en `.SddIA/active-domain-profile.json` (SSOT de autoridad ya leído por `resolve_execution_profile`), no en la bóveda (`.env` = secretos/config). Campo nuevo: `software_forge: true|false` (default `false`). Evita que una copia accidental de bóveda (fricción DT-INST-VAULT-STAGE, auditoría) conceda autoridad. |
| Slug exigido en `project.md` | `has_software_authority` compara con `codex-software-engineering` **exactamente**. `BarcelonaXplorer/.SddIA/project.md` debe declarar `codex_slug: codex-software-engineering`; los códices de dominio (`codex-frontend-product-splus`, …) **no** son slugs de autoridad y viajan por D5. |
| Regla resultante (`domain_authority.rs`) | Con `inputs.project_slug`: `allow` ⇔ `project.codex_slug == codex-software-engineering` **∧** perfil de instancia `software_forge == true` **∧** perfil de instancia `codex_slug ∉ {codex-kalma2-assistant, …cualquier slug ≠ software}`. Sin `project_slug`: regla D4 legado intacta (AC-7). Instancia `codex-kalma2-assistant` → `DOMAIN_AUTHORITY_DENIED` **siempre** (AC-8). |
| Impacto en F5 | La instancia de Aplicaciones necesita `active-domain-profile.json` explícito con `software_forge: true`; el installer v2 (R-PROF-1 del PBI `PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO`) lo materializa. Hoy la autoridad ahí es por regla legado, no por declaración. |

### D4 — Bóveda por proyecto: `{project_root}/.SddIA/.dev/.env` vía `env_ref`

**Veredicto:** aprobado con corrección de precedencia.

| Precisión | Detalle |
|-----------|---------|
| "SO → global → instancia → proyecto" como cadena de precedencia creciente | **Inexacto**. `apply_env` (`execute-process/src/core/env.rs`) fusiona bóvedas y **no pisa** variables ya presentes en el entorno del SO, salvo `VAULT_PRECEDENCE_KEYS` (hoy solo `SDDIA_LAB_SIMULATE_IOTA`, `SDDIA_IOTA_TIMEOUT_SECONDS`). Regla real: **SO > proyecto > instancia > global** (SO gana; entre bóvedas, la más específica gana). La historia adopta esta regla; no se invierte. |
| Quién carga | El **Workspace Server** carga `env_ref` al arrancar (es el padre que spawnea cápsulas) y la inyecta como **entorno efímero** a `shell-executor` / `git-manager` (`tools-contract` §5, "inyección vía entorno efímero"). Las cápsulas no hacen `dotenv` propio (README § Jerarquía de Bóvedas). |
| Aislamiento | La bóveda de proyecto **no** se fusiona en el entorno del orquestador Core ni del LLM: solo en los hijos del servidor. Secretos de BX (BD, tokens de despliegue) no cruzan a la forja. |
| Contrato | `project-config-contract` **1.0.0 → 1.1.0**: campo opcional `env_ref` (relativo a `project_root`, sin `..`). La validación actual exige `contract_version == 1.0.0` exacto; el bump implica aceptar `1.0.0 | 1.1.0` en `project_binding.rs` (compatibilidad hacia atrás para manifiestos sin `env_ref`). |

### D5 — Códice aplicable a BX: vía `active_norm_pack`

**Veredicto:** aprobado sin correcciones. `Library_Codex`/`Library_Norm` de BX (`codex-frontend-product-splus`, `codex-backend-admin-splus`, normas locales ya presentes en `BarcelonaXplorer/.SddIA/library/norms/`) se resuelven e inyectan por el proceso (`active_norm_pack`, Principio de Enrutamiento Semántico de la Constitución §). El servidor **no** expone `prompts` MCP ni interpreta normas: solo `resources/read` físico del fichero si el agente lo pide. Un único motor de reglas.

### D6 — `filesystem-manager`: cápsula física en Rust, mismo contrato, LLM-Native extinguida

**Veredicto:** aprobado con dos correcciones.

| Precisión | Detalle |
|-----------|---------|
| "bajo el mismo **contrato congelado** `filesystem-manager`" | **No existe** `skill-io-filesystem-manager-frozen.md` (solo hay congelados para `git-manager` y `shell-executor`). El `inputs` actual (`operation` ∈ `READ_FILE, WRITE_FILE, LIST_DIR, DELETE_FILE, CREATE_DIR, MOVE_FILE`; `target_path` relativo a raíz; `content`; `destination_path`) es el contrato vigente en el `{name}.md`. Requisito: forjar la norma congelada como parte de F2 (`norm-creator`) y registrarla en `cumulo.paths.json → normative_documents`. |
| "operaciones (read, write, **patch**, list)" | `PATCH_FILE` **no** está en el enum actual → es **ampliación** de contrato (bump **menor** 1.1.0 → 1.2.0 por operación nueva; el bump **mayor** lo justifica la extinción de la modalidad LLM-Native, cuerpo §2 del `{name}.md`). Resultado: `filesystem-manager` **2.0.0**, mismo `uuid` `f4a5b6c7-…`, `provides` intactos (`doc:closure`, `fs:persist`) para no romper `task-queue-manager`, `feature` y demás consumidores por `requires_capability`. Semántica de `PATCH_FILE`: diff unificado aplicado atómicamente; rechazo si el hunk no casa (sin escritura parcial). |
| Contención | Raíz = `project_root` inyectado (o `workspace_path` en modo Core-self); canonicalización + rechazo de `..`/symlinks que escapen → `PROJECT_SCOPE_ESCAPE` (AC-2). Misma primitiva `assert_workspace_bound` ya usada por cápsulas de caos. |

### Decisiones residuales (sin dictamen; Dédalo en F1/F2)

1. Nombre y forma exacta del campo `io_mode` en `tools-contract` (D2).
2. Formato del descriptor `mcp_servers` en `AGENT_PHASE` (D1/F3): mínimo `{name, command, args, env_keys}` sin valores de secretos.
3. Si `--add-dir` en `antigravity-cli-executor` se elimina por completo para proyectos (dejar solo MCP) o se mantiene como lectura redundante. Preferente: eliminar (riesgo §8, "backend salta el MCP").

## 8. Riesgos

| Riesgo | Mitigación |
|--------|-----------|
| Backend LLM salta el MCP y escribe directamente (cursor-agent con acceso a disco). | `cwd` neutro/vacío, `--add-dir` ausente, permisos de fichero; el Workspace Server es la única superficie con `project_root`. Argos verifica que el diff provenga de tools auditadas (`Raw_Execution_Finished`). |
| Duplicar registros (JSON nuevo vs `instance.projects`). | Prohibido por esta historia (§2.3). |
| Ampliar autoridad software a SddIA_AP por comodidad. | AC-8 lo bloquea. |
| Reimplementar git/shell en el servidor. | Adaptadores finos; esquemas congelados vigentes. |
| Timeouts de fases largas (`SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION`). | Reusar barreras `kalma2-phase-barrier-timeout-persist`. |

## 9. Mandato de ejecución

- Toda entidad nueva (`sddia-workspace-server`, cápsula filesystem, versiones de `kalma2-interact`, `feature`, evento) se forja vía `execute-process` → `entity-manager` / `*-creator`; prohibida la mutación manual del genoma.
- Cierre documental en rama única (`task-closure-documental`): PBI a `done/`, `validacion.md` APTO.
- Registro en `SddIA/evolution/` vinculando `uuid` de esta historia y de las entidades creadas.
