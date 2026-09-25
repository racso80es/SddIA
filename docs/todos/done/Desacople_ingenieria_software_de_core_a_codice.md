---
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-04
uuid: "ff8a0c37-a03d-4945-933c-8b54c03b9707"
title: "[ARQUITECTURA] Desacople total de ingeniería de software: Core → Códice, proyectos aislados y modo de entrega configurable"
format: markdown
version: "1.1.0"
created: "2026-09-25"
refined: "2026-09-25"
laudos_resolved: "2026-09-25"
status: done
archived: "2026-09-25"
pr_url: https://github.com/racso80es/SddIA/pull/296
ci_run: https://github.com/racso80es/SddIA/actions/runs/36105527347
priority: Alta
process: feature
feature_name: sddia-codex-agile-forge
branch_name: feat/sddia-codex-agile-forge
persist_ref: docs/features/sddia-codex-agile-forge
parent_pbi: PBI-SDDIA-DOMAIN-ABSTRACT-03
depends_on:
  - PBI-SDDIA-DOMAIN-ABSTRACT-01
  - PBI-SDDIA-DOMAIN-ABSTRACT-02
  - PBI-SDDIA-DOMAIN-ABSTRACT-03
codex_target: codex-software-engineering
codex_alias: codex-agile-forge
project_registry_core: .SddIA/projects/{slug}.md
project_manifest_client: "{project_root}/.SddIA/project.md"
project_config_contract: SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
delivery_mode_default: branch_pr
delivery_mode_precedence: [inputs, project_manifest, default]
annexes:
  - A1-project-isolation
  - A2-delivery-mode
---

# [ARQUITECTURA] Desacople total de ingeniería de software: Core → Códice

**Ciclo:** `feature` · `feat/sddia-codex-agile-forge` · `docs/features/sddia-codex-agile-forge/`  
**Padres:** ABSTRACT-01 (autoridad de dominio), ABSTRACT-02 (códice `codex-software-engineering`), ABSTRACT-03 (relocalización física de process).

## 1. Objetivo

Que el Core SddIA sea **100 % agnóstico a la ingeniería de software**: sin rutas `docs/features|fixes|todos` cableadas, sin procesos de ciclo de vida de código, sin supuesto implícito «siempre desarrollamos software». Todo el conocimiento de fabricación de software (procesos, acciones, contratos de evento, normas documentales, cápsulas `git`/`gh`) viaja en un **Códice inyectable** y opera sobre **proyectos cliente físicamente aislados**, cada uno con su propio repositorio git y su propia política de entrega.

## 2. Estado actual (verdad objetiva, no repetir trabajo)

| Pieza | Estado | Evidencia |
|-------|--------|-----------|
| Autoridad de dominio por perfil (`codex_slug`, `git_required`) | Done | `engine/domain_profile.rs`, `.SddIA/active-domain-profile.json` |
| Códice `codex-software-engineering` indexado | Done | `SddIA/library/codexes/codex-software-engineering.md` (uuid `a69d04b0-…`) |
| Process `feature`/`bug-fix`/`refactorization`/`pull-request-review`/`accept-pr`/`delivery-close-cycle` relocalizados | Done | `SddIA/library/codexes/codex-software-engineering/process/` vía `directories.process_domain_roots` |
| Rutas documentales en Cúmulo | **Acopladas** | `cumulo.paths.json` → `paths.featurePath`, `paths.fixPath`, `paths.todos.*` relativas al árbol Core |
| Normas documentales software en Core | **Acopladas** | `features-documentation-pattern`, `pr-acceptance-protocol`, `pull-request-orchestration`, `git-operations` viven en `SddIA/norms` / `library/norms` |
| Contratos de evento de dominio software (`PBI_Forged`, `PullRequest_Merged`…) | **En Core** | `SddIA/events/` |
| Suscripciones EDA inyectables por códice | **No existe** | `event-domain-subscriptions.json` es monolítico |
| Proceso `forge-pbi` (línea de montaje LLM) | **No existe** | — |
| Aislamiento por proyecto (repo propio) | **Parcial** | `workspace_init.rs` asume `repo` = raíz Core |
| Modo de entrega configurable (PR vs commit directo) | **No existe** | `pull-request-orchestration.md` impone PR + `accept-pr` sin excepción |

**Decisión de nombre (laudo Q1):** se **extiende** `codex-software-engineering` (Filtro C: no duplicar entidad). `codex-agile-forge` se registra como `alias` en el frontmatter del códice; prohibido forjar un segundo códice.

## 3. Alcance

### 3.1 Poda ontológica del Core
- Migrar al códice: `features-documentation-pattern`, `pr-acceptance-protocol`, `pull-request-orchestration`, `git-operations`, `git-via-skills-or-process`, `skill-io-git-manager-frozen` (evaluar cada una; las que sean pura «física» git sin semántica de negocio pueden quedar en Core).
- Migrar al códice los contratos ECST de dominio software (`SddIA/events/` → `codexes/codex-software-engineering/events/`) y declarar `directories.events_domain_roots` en Cúmulo, análogo a `process_domain_roots`.
- El Core conserva únicamente: bus físico (`.events/`, DLQ, `route-domain-event`, `event-watcher`), creators, `entity-manager`, gobernanza, telemetría, vitalidad.

### 3.2 Sinapsis: suscripciones EDA inyectables
- Cada códice aporta `subscriptions.json` propio; el enrutador compone la tabla efectiva = Core ∪ códices activos del perfil.
- El router sigue en Ceguera Espacial: no conoce `PBI`; solo resuelve `event_type → process` en la tabla compuesta.

### 3.3 Resolución dinámica de rutas documentales
- `paths.featurePath`, `paths.fixPath`, `paths.todos.*` dejan de resolverse contra la raíz Core. Se resuelven contra `project_root` (ver A1) inyectado por el CLI.
- Los valores por defecto (`docs/features`, `docs/fixes`, `docs/todos/{pending,done}`) pasan a ser **relativos al proyecto** y se declaran en el códice (contrato documental), no en Cúmulo Core.

### 3.4 Proceso `forge-pbi` (línea de montaje)
| Fase | Agente | Perfil LLM (tier) | Misión |
|------|--------|-------------------|--------|
| 1 Inyección de caos | Vértice Biológico vía Kalma2 | — | Idea en bruto (voz/texto) |
| 2 Expansión (Yunque) | Mayeuta o Dédalo | reflexivo (p. ej. Gemini Flash High / Claude Thinking) | Estructura negocio, AC (Protocolo de Acero), borrador `spec` |
| 3 Refinamiento (Martillo) | Argos | balístico (p. ej. Grok High) | Ceguera Espacial: no inventa; audita, formatea según patrón documental, sella en `{project_root}/docs/todos/pending/` |
- Emite `PBI_Forged` al bus; la suscripción del códice lo enruta a `feature`/`bug-fix` según `process` del PBI.
- Los tiers se resuelven vía contratos de agente (PBI Inyección de perfiles LLM, Done); prohibido cablear nombres de modelo en el proceso.

### 3.5 Anexo A1 — Aislamiento físico por proyecto
**Cada proyecto de desarrollo reside en su propia carpeta física con su propio repositorio git.**

- Un proyecto = `{project_root}/` con `.git/` propio, `docs/{features,fixes,todos}` propios y `.SddIA/` de instancia propio. El Core **nunca** es destino de documentación de cliente (excepto cuando el proyecto activo es el propio repositorio SddIA, caso auto-hospedado).
- **Registro dual (laudo Q2):**
  - **Índice Core (básico, linkado):** `.SddIA/projects/{slug}.md` en la instancia Core. Frontmatter mínimo: `id`, `uuid`, `project_root` absoluto, `manifest_ref` (puntero al manifiesto cliente), `codex_slug`, `status`. Sin detalle operativo. Resuelto vía Cúmulo (`instance.projects`), no inferido.
  - **Manifiesto cliente (detalle):** `{project_root}/.SddIA/project.md`. Frontmatter completo: `id`, `uuid` (mismo que el índice), `git_remote`, `default_branch`, `delivery_mode`, `docs_layout` (`features`, `fixes`, `todos.pending`, `todos.done`), `qa_gates`, `codex_slug`, `contract_version`.
  - **Contrato de configuración:** `project-config-contract.md` (vive en el códice; schema de campos, tipos, obligatoriedad, valores permitidos, versión SemVer). El Core es **responsable de validar** el manifiesto cliente contra este contrato en `workspace_init` antes de cualquier fase: campo ausente/inválido, `uuid` no coincidente con el índice o `contract_version` incompatible → `exitCode: 1` + `System_Fracture_Detected`, sin panic. El Core valida forma; no interpreta semántica de negocio del proyecto.
- El CLI inyecta `project_slug` en `inputs`; `workspace_init` resuelve índice → `manifest_ref` → manifiesto y valida: (a) contrato de configuración conforme, (b) `project_root/.git` existe y es raíz de repo (no submódulo ni anidado en otro proyecto registrado), (c) toda escritura de `persist_ref` cae bajo `project_root`. Fallo → `exitCode: 1` + `System_Fracture_Detected`, sin panic.
- Prohibido un `.git` compartido entre proyectos y prohibido operar sobre un proyecto con `project_root` dentro de otro proyecto registrado.
- `workspacesRoot` (`.SddIA/workspaces/`) se resuelve bajo la instancia del proyecto, no bajo el Core.

### 3.6 Anexo A2 — Modo de entrega configurable por proyecto
**Cada proyecto elige entre trabajar con ramas + PR o commitear directamente sobre la rama principal.**

- Clave `delivery_mode` en el manifiesto cliente (A1) con valores:
  - `branch_pr` (**default**, seguro): flujo actual — rama `feat/|fix/|refactor/`, `delivery-close-cycle` abre PR, `pull-request-review` audita, `accept-pr` fusiona.
  - `trunk_direct`: sin rama de trabajo; el ciclo commitea sobre `default_branch` y hace `push` directo; `delivery-close-cycle` omite fases PR y emite `Delivery_Committed` en lugar de `PullRequest_Presented`. **Laudo Q3:** `pull-request-review` y `accept-pr` quedan **totalmente omitidos**; no hay auditoría post-commit asíncrona. La única aduana es el gate pre-commit.
- **Precedencia (laudo Q4, ambos niveles):** `inputs.delivery_mode` (por PBI/ejecución) > `delivery_mode` del manifiesto cliente (por proyecto) > default `branch_pr`. El override por `inputs` debe quedar registrado en `objectives.md` del ciclo (`delivery_mode_source: inputs|project|default`). Nunca se hereda del Core ni de otro proyecto.
- Efectos normativos obligatorios:
  - `pull-request-orchestration` y `pr-acceptance-protocol` pasan a condicionar su vigencia a `delivery_mode == branch_pr`.
  - Definición de Done por modo: `branch_pr` = un PR mergeado + `validacion.md` APTO + PBI en `done/` en la misma rama; `trunk_direct` = commit(s) en `default_branch` con `validacion.md` APTO y PBI en `done/` en el **mismo commit** de cierre.
  - Hooks `pre-push`/aduana leen `delivery_mode` del proyecto activo: en `trunk_direct` el veto de push a `main` **no aplica** al proyecto (sigue aplicando al Core SddIA, que permanece `branch_pr`).
  - En `trunk_direct` la aduana QA (`sddia-qa`, gate evolution) se ejecuta **pre-commit** en lugar de en CI del PR; su fallo bloquea el commit de cierre.
- `workspace_init` no canonicaliza prefijo de rama ni exige rama de trabajo cuando `delivery_mode == trunk_direct`.

## 4. Fuera de alcance
- Comercialización/tokenización del códice como NFT (solo se garantiza empaquetado `codex-contract` autocontenido).
- Cambios en Kalma2 más allá de aceptar prompt en bruto y `project_slug`.
- Migración de proyectos cliente existentes (GesFer/Paciente 0) — PBI de despliegue separado.
- Soporte multi-remoto (GitLab, Gitea): solo `gh` en este ciclo; el diseño de A2 no debe impedirlo.

## 5. Criterios de aceptación
- [ ] **AC-PRUNE:** `SddIA/norms` y `SddIA/events` sin entidades de semántica software; inventario de lo migrado en `clarify.md`; `sddia-qa` índices OK.
- [ ] **AC-EVENTS-ROOT:** `directories.events_domain_roots` en Cúmulo; `route-domain-event` resuelve contratos ECST del códice.
- [ ] **AC-SUBS:** tabla de suscripciones compuesta Core ∪ códice; con perfil sin códice software, `PBI_Forged` → dead-letter con motivo `no_subscriber`, sin panic.
- [ ] **AC-PATHS:** `paths.featurePath|fixPath|todos` resueltos contra `project_root`; test con `project_root` ≠ Core genera artefactos fuera del árbol SddIA y ninguno dentro.
- [ ] **AC-FORGE:** `forge-pbi` indexado en el códice; smoke desde Kalma2 con prompt en bruto sobre proyecto vacío → PBI sellado en `{project_root}/docs/todos/pending/` conforme `features-documentation-pattern` + `PBI_Forged` enrutado.
- [ ] **AC-TIERS:** `forge-pbi` referencia tiers de agente, no modelos literales.
- [ ] **AC-A1-REG:** índice Core `.SddIA/projects/{slug}.md` con `uuid` + `manifest_ref`; Cúmulo expone `instance.projects`; proyecto no registrado → deny.
- [ ] **AC-A1-MANIFEST:** manifiesto cliente `{project_root}/.SddIA/project.md` con `uuid` coincidente con el índice; `uuid` divergente → deny + fractura.
- [ ] **AC-A1-CONTRACT:** `project-config-contract.md` indexado en el códice con `uuid` y SemVer; `workspace_init` valida el manifiesto contra el contrato (campos obligatorios, tipos, enum `delivery_mode`, `contract_version` compatible); tests Rust con manifiesto válido, campo ausente, enum inválido y versión incompatible.
- [ ] **AC-A1-GIT:** `workspace_init` rechaza `project_root` sin `.git` propio o anidado en otro proyecto; test unitario Rust.
- [ ] **AC-A1-SCOPE:** escritura fuera de `project_root` → `exitCode: 1` + fractura; sin mutación del Core.
- [ ] **AC-A2-DEFAULT:** sin `delivery_mode` declarado → `branch_pr`; comportamiento idéntico al actual (no regresión en suites PR).
- [ ] **AC-A2-TRUNK:** proyecto `trunk_direct`: ciclo `feature` completo sin rama de trabajo, commit + push sobre `default_branch`, evento `Delivery_Committed`, **cero** invocaciones a `pull-request-review`/`accept-pr` (verificado en bus: ningún `PullRequest_Presented`).
- [ ] **AC-A2-PRECEDENCE:** con manifiesto `branch_pr` e `inputs.delivery_mode: trunk_direct` el ciclo opera en `trunk_direct` y registra `delivery_mode_source: inputs`; sin `inputs` opera según manifiesto (`source: project`); sin ambos, `branch_pr` (`source: default`). Test triple.
- [ ] **AC-A2-HOOKS:** `pre-push` permite push a `main` del proyecto `trunk_direct` y sigue vetándolo en el Core SddIA (test doble).
- [ ] **AC-A2-DONE:** `validacion.md` acepta `pbi_archived: true` en `trunk_direct` sin `pr_url`; `task-closure-documental` actualizado por modo.
- [ ] **AC-A2-NORM:** `pull-request-orchestration` y `pr-acceptance-protocol` versionadas con cláusula de vigencia por `delivery_mode`.
- [ ] **AC-CORE-SELF:** el repositorio SddIA sigue operando como proyecto `branch_pr` auto-hospedado sin cambios de flujo.
- [ ] **AC-BUILD:** `cargo build -p execute-process --release` y `cargo test -p execute-process` OK.
- [ ] **AC-EVO:** entrada en `SddIA/evolution/` vinculando `uuid` de este PBI y de las entidades mutadas; `gate-evolution --range` `exitCode: 0`.
- [ ] **AC-DOC:** PBI en `done/` + `validacion.md` `pbi_archived: true` en el mismo PR.

## 6. Plan por fases
1. **F0 Clarify:** inventario exacto de normas/eventos a migrar; schema de `project-config-contract.md`, índice Core y manifiesto cliente; enum y precedencia `delivery_mode` en `clarify.md`.
2. **F1 Poda Core:** migración de normas y eventos al códice vía `entity-manager`; `events_domain_roots`; suscripciones compuestas.
3. **F2 Cúmulo + A1:** `project-config-contract` en el códice, `instance.projects`, índice Core + manifiesto cliente, validación de contrato y resolución de `paths.*` contra `project_root` en `workspace_init`.
4. **F3 A2:** `delivery_mode` en perfil/registro, ramas condicionales en `feature`/`bug-fix`/`refactorization`/`delivery-close-cycle`, hooks y normas versionadas, evento `Delivery_Committed`.
5. **F4 forge-pbi:** proceso + acciones + tiers; smoke Kalma2.
6. **F5 Validación empírica:** proyecto cliente vacío en carpeta ajena al Core, dos variantes (`branch_pr` y `trunk_direct`); Core intacto (`git status` limpio bajo `SddIA/` y `docs/` del Core).

## 7. Riesgos
- **Ruptura de hooks:** `pre_push_gate.sh` y aduana asumen repo único; mitigación: resolver proyecto activo por `git rev-parse --show-toplevel` + registro, fallback `branch_pr`.
- **`trunk_direct` sin aduana:** sin PR ni review post-commit (laudo Q3), el gate pre-commit es la **única** barrera; mitigación: gate pre-commit obligatorio y bloqueante (AC-A2-TRUNK/HOOKS), prohibido `SDDIA_SKIP_HOOKS` para IAs, override `inputs.delivery_mode: trunk_direct` trazado en `objectives.md` (AC-A2-PRECEDENCE).
- **Deriva índice ↔ manifiesto:** mitigación: `uuid` cruzado obligatorio (AC-A1-MANIFEST) y validación de contrato en cada `workspace_init` (AC-A1-CONTRACT); el índice Core no duplica campos operativos.
- **Alcance excesivo:** F1–F4 son divisibles; si el ciclo excede, dividir en ABSTRACT-04a (F1–F2), 04b (F3), 04c (F4) manteniendo este PBI como padre.
- **Doble fuente de rutas documentales** durante transición: Cúmulo Core debe marcar `paths.featurePath|fixPath|todos` como `deprecated` con puntero al contrato del códice hasta AC-PATHS.

## 8. Referencias
- `docs/features/sddia-domain-abstraction/` (ABSTRACT-01), `docs/features/sddia-codex-software-engineering/` (ABSTRACT-02), `docs/features/sddia-domain-abstract-03-relocalizacion/` (ABSTRACT-03)
- `SddIA/library/codexes/codex-software-engineering.md`, `codex-contract.md`
- `SddIA/engine/execute-process/src/engine/{domain_profile,domain_authority,workspace_init}.rs`
- `SddIA/norms/{pull-request-orchestration,git-operations,execution-contexts,external-ai-constraints}.md`
- `SddIA/core/cumulo.paths.json` §`directories.process_domain_roots`, §`paths`
- `.cursor/rules/task-closure-documental.mdc`
- PBI Done: Inyección de perfiles LLM (Tiers) en contratos de agentes

## 9. Laudos (Vértice Biológico, 2026-09-25) — cerrados
| # | Cuestión | Laudo | Aplicado en |
|---|----------|-------|-------------|
| Q1 | ¿Extender `codex-software-engineering` o forjar `codex-agile-forge`? | Extender; `codex-agile-forge` = alias | § 2, frontmatter `codex_alias` |
| Q2 | Ubicación del registro de proyectos | Ambos: índice básico linkado en Core + manifiesto detallado en cliente. El Core valida el manifiesto contra contrato de configuración | § 3.5, AC-A1-REG/MANIFEST/CONTRACT, F2 |
| Q3 | `pull-request-review` en `trunk_direct` | Totalmente omitido | § 3.6, AC-A2-TRUNK, § 7 |
| Q4 | `delivery_mode` por `inputs` o por proyecto | Ambos; precedencia inputs > proyecto > default | § 3.6, AC-A2-PRECEDENCE |

Sin preguntas abiertas. PBI listo para `./sddia-run.sh --process feature`.
