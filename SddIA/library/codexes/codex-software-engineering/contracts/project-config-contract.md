---
uuid: "e7b1c4a2-6f08-4d3e-9a55-1c8e2b7d0f44"
name: "project-config-contract"
version: "1.3.0"
contract_version: "1.3.0"
nature: "configuration-contract"
codex: "codex-software-engineering"
---

# Contrato de configuración de proyecto v1.3.0

El Core valida **forma**. No interpreta semántica de negocio.

## Índice Core `.SddIA/projects/{slug}.md`

Obligatorios: `id`, `uuid`, `project_root` (absoluto), `manifest_ref`, `codex_slug`, `status` (`active`|`inactive`).

## Manifiesto `{project_root}/.SddIA/project.md`

Obligatorios: `id`, `uuid` (igual al índice), `git_remote`, `default_branch`, `delivery_mode` (`branch_pr`|`trunk_direct`), `contract_version` (`1.0.0`, `1.1.0`, `1.2.0` o `1.3.0`), `codex_slug`, `docs_layout.features`, `docs_layout.fixes`, `docs_layout.todos_pending`, `docs_layout.todos_done`.

Los valores de `docs_layout` son relativos al `project_root`, sin `..`.

### Campos opcionales (1.1.0)

| Campo | Tipo | Reglas |
|-------|------|--------|
| `env_ref` | string | Ruta relativa a `project_root` hacia la bóveda de proyecto (p. ej. `.SddIA/.dev/.env`). Sin `..`. |
| `qa_gates` | array | Comandos o ids de comprobación declarados por el proyecto. |
| `mcp.allowed_executables` | array | Whitelist de ejecutables para `run_check` vía Workspace Server. |

### Campos opcionales (1.2.0) — `tracker`

Objeto `tracker` opcional. Ausente = proyecto sin tracker (Kalma2 responde `tracker_configured: false`).

| Campo | Tipo | Reglas |
|-------|------|--------|
| `tracker.provider` | string | Solo `linear`. |
| `tracker.team_key` | string | Obligatorio si hay `provider`. |
| `tracker.project_id` | string | Opcional; acota a un Linear Project. |
| `tracker.state_map` | object | Claves ∈ `{backlog, todo, in_progress, in_review, done, cancelled}`; valores = nombre de WorkflowState del equipo. `todo` opcional. |
| `tracker.labels.hu` | string | Default lógico `hu`. |
| `tracker.labels.pbi` | string | Default lógico `pbi`. |
| `tracker.labels.fix` | string | Default lógico `fix`. |
| `tracker.labels.kaizen` | string | Default lógico `kaizen`. |
| `tracker.labels.deuda` | string | Default lógico `deuda`. |
| `tracker.labels.spike` | string | Default lógico `spike`. |
| `tracker.labels.editable` | string | Default lógico `sddia-editable`; opt-in edición de cuerpo desde Linear (HU-B). |
| `tracker.done_gate` | string | `git` \| `linear` \| `both`; default `git`. En entregas HU-A el validador solo acepta `git` hasta laudo HU-B. |

Prohibido incluir secretos (`LINEAR_*`) en el manifiesto.

### Compatibilidad

Manifiestos `1.0.0`, `1.1.0` y `1.2.0` sin los campos nuevos siguen siendo válidos.

## delivery_mode

Precedencia: `inputs.delivery_mode` > manifiesto > `branch_pr`.
`trunk_direct` omite `pull-request-review` y `accept-pr`.
