---
document_id: PBI-ARQUITECTURA-FS-MANAGER-PHYSICAL
uuid: "d225d3ef-8e72-4fbc-9ee1-d25447ec6611"
title: "[ARQUITECTURA] Workspace 1×N — filesystem-manager físico 2.0.0"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: filesystem-manager-physical
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
unblocks:
  - PBI-ARQUITECTURA-WS-SERVER
skill_uuid: "f4a5b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c"
skill_version_actual: "1.1.0"
skill_version_objetivo: "2.0.0"
baseline_decisiones:
  - "D6: mismo uuid y provides (doc:closure, fs:persist); LLM-Native extinguida; PATCH_FILE atómico"
  - "No existe skill-io-filesystem-manager-frozen.md; se forja en este PBI"
---

# filesystem-manager 2.0.0 — cápsula Rust

Historia madre: §4.C, fase F2 (solo la cápsula), D6, AC-13. Cierra G1 en el genoma. El adaptador MCP que la invoca es `PBI-ARQUITECTURA-WS-SERVER`.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Hay contrato congelado que ampliar | No hay `skill-io-filesystem-manager-frozen.md`. Solo existen congelados de `git-manager` y `shell-executor`. Este PBI **crea** la norma y la registra en `cumulo.paths.json → normative_documents`. |
| `PATCH_FILE` ya está en el enum | El enum 1.1.0 es `READ_FILE, WRITE_FILE, LIST_DIR, DELETE_FILE, CREATE_DIR, MOVE_FILE`. `PATCH_FILE` es ampliación. |
| La cápsula emite `System_Fracture_Detected` | El emisor vigente está en `workspace_init` cuando el error empieza por `PROJECT_SCOPE_ESCAPE`. La cápsula **devuelve** ese código. No se añade un segundo emisor dentro del binario. Quien traduzca el fallo de tool a fractura es el orquestador, por el camino ya existente. |
| Bump menor 1.2.0 basta | La operación nueva sería menor; la extinción de LLM-Native (cuerpo §2 del `{name}.md`) es el mayor. Entregable: **2.0.0**. |

## 1. Intención

Sustituir la modalidad LLM-Native por una cápsula Rust con el mismo contrato de capacidades, más `PATCH_FILE` atómico, confinada a la raíz inyectada. Sin IDE, `fs:persist` resuelve a binario.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-FS-1 | Skill `filesystem-manager` **2.0.0**, uuid `f4a5b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c` intacto. `provides`: `doc:closure` 1.0.0 y `fs:persist` 1.0.0 intactos. Forja por `skill-creator` / `entity-manager`. |
| R-FS-2 | Enum = el de 1.1.0 más `PATCH_FILE`. Cuerpo sin la sección «Modalidad LLM-Native». |
| R-FS-3 | `PATCH_FILE`: diff unificado aplicado en bloque. Hunk que no casa → `exitCode` 1, fichero byte-idéntico (sin escritura parcial). |
| R-FS-4 | Raíz = `project_root` inyectado, o `workspace_path` en modo Core-self. `..`, absoluto fuera de raíz y symlink que escape → error `PROJECT_SCOPE_ESCAPE`, sin panic. Reusar `assert_workspace_bound`. |
| R-FS-5 | Norma `skill-io-filesystem-manager-frozen` creada con `norm-creator` y registrada en `normative_documents`. |
| R-FS-6 | `requires_capability fs:persist` de `task-queue-manager` y `feature` resuelve al binario (DI), no a «la IA del IDE». |

## 3. Plan

1. Init `feature` `filesystem-manager-physical`.
2. Forjar cápsula Rust + bump del `{name}.md` + norma congelada + clave Cúmulo.
3. Tests de hunk inválido, escape y resolución DI. `capsule-invoke-smoke`.
4. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-13 | `PATCH_FILE` con hunk que no casa → exit 1, fichero intacto; sin sección LLM-Native; `provides` idénticos; `fs:persist` resuelve al binario. | Tests + smoke + DI. |
| AC-2 (escritura) | Escritura con `../` o symlink fuera de la raíz → `PROJECT_SCOPE_ESCAPE`, sin panic, sin mutación. | Test unitario. La mitad `resources/read` es del PBI del servidor. |

## 5. Fuera de alcance

- Servidor MCP, tools `git_*` / `run_check`, bóveda `env_ref`.
- Cambiar el esquema congelado de `git-manager` o `shell-executor`.
- Emitir eventos de dominio desde la cápsula.

## 6. Dependencias

- Ningún antecesor. El Workspace Server no se fusiona en este PR: esta cápsula la consumen procesos Core aunque MCP no exista.
