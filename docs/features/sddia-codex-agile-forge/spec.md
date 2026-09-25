---
feature_name: sddia-codex-agile-forge
created: "2026-09-25"
process: feature
base: docs/features/sddia-codex-agile-forge/objectives.md
scope: motor-y-codice
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-04
agents: dedalo
---

# Spec — sddia-codex-agile-forge

## 1. Contrato de configuración (`project-config-contract` v1.0.0)

Archivo: `SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md`.

Índice Core `.SddIA/projects/{slug}.md` — campos obligatorios: `id`, `uuid`, `project_root` (absoluto), `manifest_ref`, `codex_slug`, `status` (`active`|`inactive`).

Manifiesto `{project_root}/.SddIA/project.md` — obligatorios: `id`, `uuid`, `git_remote`, `default_branch`, `delivery_mode` (`branch_pr`|`trunk_direct`), `contract_version` (`1.0.0`), `codex_slug`, `docs_layout` con `features`, `fixes`, `todos_pending`, `todos_done` (relativos al `project_root`). Opcional: `qa_gates` (lista de strings).

Validación (forma, no negocio): tipos string, enum, SemVer de contrato **igual** a `1.0.0`, `uuid` índice == `uuid` manifiesto, `project_root` absoluto sin `..` que escape, `manifest_ref` apunta al manifiesto. Cualquier fallo → error tipado `PROJECT_CONFIG_INVALID`.

## 2. Resolución de proyecto

`inputs.project_slug` → índice `instance.projects` = `.SddIA/projects/`. Sin slug: no hay proyecto cliente; rutas y git siguen el repo actual (AC-CORE-SELF).

Slug ausente del índice → `PROJECT_NOT_REGISTERED`.

`project_root/.git` debe ser directorio o fichero de gitdir de **raíz** (no anidado en otro `project_root` registrado). Fallo → `PROJECT_GIT_INVALID`.

Escritura de `persist_ref`: el path resuelto debe tener prefijo `project_root`. Si no → `PROJECT_SCOPE_ESCAPE` y el caller emite `System_Fracture_Detected` (sin panic).

## 3. `delivery_mode`

```text
inputs.delivery_mode  >  manifiesto.delivery_mode  >  branch_pr
source ∈ {inputs, project, default}
```

`trunk_direct`: `workspace_init` no crea rama de trabajo ni reescribe prefijo; `branch_name` = `default_branch`. `omits_pr_cycle` = true → `delivery-close-cycle` no invoca presentación PR ni `pull-request-review`/`accept-pr`; emite `Delivery_Committed`.

`branch_pr`: comportamiento actual intacto.

## 4. Rutas documentales

`resolve_project_doc(project_root, docs_layout, key) -> PathBuf` une `project_root` + layout y rechaza escape. Sin `project_root`, `resolve_documentation_features_path` / fixes / todos siguen leyendo Cúmulo relativo al repo.

Cúmulo `paths.featurePath|fixPath|todos` ganan clave hermana `resolution: "project_root_or_repo"` (documental). Los defaults del layout cliente viven en el contrato §1.

## 5. Eventos y suscripciones

- `directories.events_domain_roots`: `["SddIA/library/codexes/codex-software-engineering/events"]`.
- `resolve_event_contract(repo, name)`: domain roots primero, luego `directories.events`.
- Suscripciones: `composed_subscriptions(repo, profile)` = JSON Core ∪ `…/codex-software-engineering/subscriptions.json` **solo si** `has_software_authority(profile)`.
- Claves nuevas solo en el códice: `PBI_Forged` → `{agent: tekton, process: feature}`, `Delivery_Committed` → lista vacía explícita no; un suscriptor de telemetría no es obligatorio. `Delivery_Committed` puede tener array vacío: el router ya barre sin suscriptores. `PBI_Forged` **sí** tiene suscriptor cuando hay autoridad.
- Sin autoridad: las claves del códice no se fusionan. Si el evento es de clave exclusiva del códice y queda sin suscriptor por ese motivo, disposición `dead-letter` con `reason: no_subscriber`, `exitCode: 0`, sin `System_Fracture_Detected`. El barrido genérico de eventos Core con array vacío **no** cambia.

## 6. `forge-pbi`

Process en el packing del códice. Inputs: `raw_idea` (string), `project_slug`, `process` (`feature`|`bug-fix`|`refactorization`). Fases declaradas: recepción → expansión (delegado `agent:mayeuta`, tier reflexivo vía contrato de agente, sin modelo literal) → sellado (delegado `agent:argos`, tier balístico) → emisión.

Handler nativo de sellado (lab y relevo sin LLM): escribe `{project_root}/{docs_layout.todos_pending}/{slug}.md` con frontmatter mínimo (`document_id`, `uuid`, `status: pending`, `process`, `created`) y cuerpo = `raw_idea`. Emite `PBI_Forged` al bus. No inventa criterios de aceptación: el cuerpo es el caos recibido; el sello es formato.

## 7. Hooks

Función pura `main_push_allowed(repo_is_core: bool, mode: DeliveryMode) -> bool`: `true` solo si `!repo_is_core && mode == TrunkDirect`.

`pre_push_gate.sh` lee `delivery_mode` del manifiesto si existe `.SddIA/project.md` y el repo no contiene `SddIA/core/cumulo.paths.json` como Core. Core → veto actual. Proyecto `trunk_direct` → no veto de main. `trunk_direct` exige que el hook de pre-commit no esté saltado; si `SDDIA_SKIP_HOOKS=1` en un commit de cierre `trunk_direct` iniciado por proceso, el handler rechaza (`TRUNK_HOOKS_REQUIRED`).

## 8. Fractura

`PROJECT_CONFIG_INVALID`, `PROJECT_NOT_REGISTERED`, `PROJECT_GIT_INVALID`, `PROJECT_SCOPE_ESCAPE` en `workspace_init` devuelven `Err` (el orquestador ya convierte error de init en fallo, exitCode 1). Emisión de `System_Fracture_Detected` queda en el camino de error de `workspace_init` **solo** para `PROJECT_SCOPE_ESCAPE` (mutación intentada fuera de root). Los otros tres son deny de configuración, exitCode 1, sin fractura de bus (no son colapso de proceso en marcha, son rechazo de entrada). Ajuste respecto al PBI: fractura solo en escape de scope; el resto es deny. Queda registrado aquí para que Argos no exija fractura en un manifiesto malformado.
