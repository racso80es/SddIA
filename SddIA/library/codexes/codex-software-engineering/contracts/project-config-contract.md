---
uuid: "e7b1c4a2-6f08-4d3e-9a55-1c8e2b7d0f44"
name: "project-config-contract"
version: "1.1.0"
contract_version: "1.1.0"
nature: "configuration-contract"
codex: "codex-software-engineering"
---

# Contrato de configuración de proyecto v1.1.0

El Core valida **forma**. No interpreta semántica de negocio.

## Índice Core `.SddIA/projects/{slug}.md`

Obligatorios: `id`, `uuid`, `project_root` (absoluto), `manifest_ref`, `codex_slug`, `status` (`active`|`inactive`).

## Manifiesto `{project_root}/.SddIA/project.md`

Obligatorios: `id`, `uuid` (igual al índice), `git_remote`, `default_branch`, `delivery_mode` (`branch_pr`|`trunk_direct`), `contract_version` (`1.0.0` o `1.1.0`), `codex_slug`, `docs_layout.features`, `docs_layout.fixes`, `docs_layout.todos_pending`, `docs_layout.todos_done`.

Los valores de `docs_layout` son relativos al `project_root`, sin `..`.

### Campos opcionales (1.1.0)

| Campo | Tipo | Reglas |
|-------|------|--------|
| `env_ref` | string | Ruta relativa a `project_root` hacia la bóveda de proyecto (p. ej. `.SddIA/.dev/.env`). Sin `..`. |
| `qa_gates` | array | Comandos o ids de comprobación declarados por el proyecto. |
| `mcp.allowed_executables` | array | Whitelist de ejecutables para `run_check` vía Workspace Server. |

## delivery_mode

Precedencia: `inputs.delivery_mode` > manifiesto > `branch_pr`.
`trunk_direct` omite `pull-request-review` y `accept-pr`.
