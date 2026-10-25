---
id: interacciones-llm-audit
uuid: "a8f3c2e1-9b4d-4a7f-8c6d-0123456789ab"
type: audit
version: "1.0.0"
tracker_ref: AUT-5
document_id: PBI-5cb3fcb9
process: refactorization
created: "2026-10-25"
---

# Auditoría de interacciones LLM — procesamiento AUT-5

Registro de interacciones con el modelo de asistencia (Cursor / agente Tekton) durante el ciclo del PBI **AUT-5** (*Adecuación del frente documental al ciclo de vida*).

| # | Timestamp (UTC) | Rol | Tipo | Resumen |
|---|-----------------|-----|------|---------|
| 1 | 2026-10-25T10:53:00Z | Vértice Biológico | user_query | Orden: procesar PBI AUT-5 y registrar interacciones LLM en este fichero. |
| 2 | 2026-10-25T10:54:00Z | Tekton (Composer) | tool_batch | Búsqueda de AUT-5 en repo; lectura de normas `external-ai-constraints`; fetch Linear `AUT-5` vía `linear-tracker-adapter` (token bóveda). |
| 3 | 2026-10-25T10:55:00Z | Tekton | reasoning | Contexto desde transcripción previa: AUT-5 = kaizen `refactorization`, cuerpo en issue Linear, criterios columna `tracker_column` + `refinement`. |
| 4 | 2026-10-25T10:56:00Z | Tekton | code_edit | Alta módulo `item_lifecycle_frontmatter.rs` y presente fichero de auditoría. |
| 5 | 2026-10-25T10:57:00Z | Tekton | code_edit | Parches `forge_pbi`, `refine`, `tracker_stamp`; módulo `item_lifecycle_frontmatter`. |
| 6 | 2026-10-25T11:05:00Z | Tekton | tool_batch | `cargo test -p execute-process --lib` (item_lifecycle, file_provider); rama `refactor/frente-documental-ciclo-vida`. |
| 7 | 2026-10-25T11:06:00Z | Tekton | tool_call | Linear `update_issue_state` AUT-5 → In Progress (ya estaba en columna). |
| 8 | 2026-10-25T11:07:00Z | Tekton | code_edit | Cascada documental `docs/refactors/frente-documental-ciclo-vida/` + `validacion.md` APTO local. |
| 9 | 2026-10-25T11:08:00Z | Tekton | assistant_reply | Entrega resumen al Vértice Biológico (sin commit/push salvo orden). |
| 10 | 2026-10-25T11:10:00Z | Vértice Biológico | user_query | Orden: commit, push, `delivery-close-cycle`; seguir auditoría LLM e incluir fricción `./sddia-run.sh`. |
| 11 | 2026-10-25T11:11:00Z | Tekton | tool_call | `./sddia-run.sh --process refactorization` → **fallo** (ver § Fricción). |
| 12 | 2026-10-25T11:12:00Z | Tekton | pending | `gate-evolution`, commit acotado, `delivery-close-cycle`, actualizar filas 12+ en esta tabla. |

## Fricción — `./sddia-run.sh` (refactorization / workspace-init)

| Campo | Valor |
|-------|--------|
| Comando | `./sddia-run.sh --process refactorization --inputs-file /tmp/refactor-aut5-inputs.json` |
| `success` | `false` |
| `error_code` | `DOMAIN_AUTHORITY_DENIED` |
| Handler | `domain-authority` |
| `profile_source` | `input_execution_profile` |
| Mensaje | `PROJECT_GIT_INVALID: /home/racso/Proyectos/SddIA contiene otro proyecto /home/racso/Proyectos/SddIA/_refine_lab_refine-ac-ref1` |
| Mitigación aplicada | Rama y documentación vía git manual; init oficial pendiente de retirar árboles `_refine_lab_*` / `_forge_lab_*` y restos bajo `.SddIA/projects/*-ac-*`. |
| Relación PBI | No bloquea el corte AUT-5 en motor; encaja con **AUT-6** (autoridad Core-self / WASM). |

Entrada JSON usada: `project_slug: core`, `execution_profile.software_forge: true`, `codex_slug: codex-software-engineering`, `branch_name: refactor/frente-documental-ciclo-vida`, `tracker_ref: AUT-5`.

## Metadatos de sesión

- **issuer (invoked_by):** `agent:tekton` / sesión Cursor IDE
- **subject (entity_id):** `AUT-5` / `PBI-5cb3fcb9`
- **Contrato:** `karma2-token` § interaction_audit (campos alineados con tabla anterior)
