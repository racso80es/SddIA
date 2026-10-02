---
feature_name: linear-issue-markdown-body
process: feature
created: "2026-10-02"
---

# Especificación — linear-issue-markdown-body

## Tool `linear-tracker-adapter`

Nueva operación JSON `update_issue_description`:

| Campo | Tipo | Obligatorio |
|-------|------|-------------|
| `operation` | `"update_issue_description"` | sí |
| `issue_ref` | string (`OSC-13` o UUID) | sí |
| `description` | string (Markdown completo) | sí |

GraphQL: `issueUpdate` con `description`. Errores tipados como el resto de ops. Lab mock en `SDDIA_LAB_MOCK_LINEAR_URL`.

## Proceso `tracker-linear-markdown-sync`

| Input | Default | Uso |
|-------|---------|-----|
| `sync_all` | `true` | Escanea `docs/todos/pending`, `docs/todos/done`, `Documentacion/PBI/Realizado` |
| `markdown_path` + `issue_ref` | — | Volcado puntual |

Por cada `.md` con `tracker_ref` en frontmatter: leer archivo, `scrub_secrets`, invocar tool.

## scrub_secrets

Redacta líneas `KEY=value` para claves conocidas (`LINEAR_API_TOKEN`, `GITHUB_TOKEN`, …) y tokens `lin_api_*`.

## Criterios de aceptación (PBI)

- AC-1: OSC-5 = HU Realizado.
- AC-2: OSC-6…OSC-13 = Markdown de su archivo local.
- AC-3: re-volcado idempotente tras editar sección local.
