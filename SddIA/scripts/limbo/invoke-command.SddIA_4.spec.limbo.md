---
origin: "SddIA_4/skills/invoke-command/spec.md"
limbo_reason: "Purga de entropía: invoke-command reemplazado por shell-executor (system-operations) y git-manager (source-control)."
---

---
command_file_routing:
  via_bat: Usar ruta absoluta al archivo (p. ej. --command-file con path absoluto) para que el exe resuelva bien desde cualquier directorio.
  via_exe_from_repo_root: Ejecutar el exe desde la raíz del repo; entonces las rutas relativas al archivo se resuelven correctamente.
contract_ref: paths.skillsDefinitionPath/skills-contract.md (Cúmulo)
implementation_path_ref: paths.skillCapsules.invoke-command
parameters:
  command:
    description: Comando a ejecutar (obligatorio si no se usa command_file). Acepta -Command/--command.
    required: true
  command_file:
    description: Ruta a archivo cuyo contenido es el comando. Evita inyección en terminal. Acepta --command-file/-CommandFile.
    required: false
  contexto:
    default: GesFer
    required: false
  fase:
    default: Accion
    enum:
    - Triaje
    - Analisis
    - Evaluacion
    - Marcado
    - Accion
    required: false
rules:
- 'MANDATORY: Cualquier comando de sistema que el agente deba ejecutar ha de invocarse a través de esta skill.'
- 'Interface: Command (obligatorio) o --command-file; Contexto (default GesFer); Fase (Triaje|Analisis|Evaluacion|Marcado|Accion). El exe acepta -Command/-Fase y --command-file.'
- 'Compliance: AC-001 validación sintáctica; registro en docs/diagnostics/{branch}/execution_history.json; alineación Protocolo Racso-Tormentosa.'
scope_clarification: 'Aplica siempre que el agente ejecute cualquier comando de sistema: sin excepción para git, dotnet, npm, pwsh. No hay excepciones.'
skill_id: invoke-command
usage_instruction: Cada vez que el agente deba ejecutar un comando, invocar la cápsula invoke-command (paths.skillCapsules.invoke-command). Prohibido ejecutar comandos directamente en el shell.
---
# Skill: Invoke Command (Interceptor de ejecuciones de sistema)

