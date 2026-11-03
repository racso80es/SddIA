---

feature_name: aut-74-workspace-server-target-path
created: "2026-11-03"
process: bug-fix
branch_name: fix/aut-74-workspace-server-target-path
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/aut-74-workspace-server-target-path
pbi_ref: AUT-74
execution_id: "dd4f1ae7-e72e-49fb-9943-f84cbee18120"
---
# Objetivos — aut-74-workspace-server-target-path

## Misión

Esquema MCP del workspace server declara target_path

En el refino `030d8a50-3dc1-43a2-aa20-16b97ad96770`, Mayeuta llamó `fs_read` y `fs_list` con argumentos `{}`. `tools/list` anuncia esas tools con `inputSchema.properties` vacío (`tool_def` en `SddIA/tools/sddia-workspace-server/src/server.rs`), así que el agente no ve `target_path`.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
