---
id: "norm-git-via-skills-or-process"
name: "Git solo vía skill, herramienta, acción o proceso"
type: "norm"
status: "active"
---
# Norma: Git solo vía skill, herramienta, acción o proceso

**Fuente:** SddIA/norms. Aplicable a todo agente o IA que opere en el repositorio.

## Principio

**La IA nunca debe ejecutar comandos de git directamente** en la shell. Cualquier operación de control de versiones ha de realizarse **siempre** a través de al menos uno de los siguientes canales:

- **Skill:** suite Git táctica S+ Grade: paths.skillCapsules.git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr. Contrato: paths.skillsDefinitionPath/\<skill-id\>/spec.json. **Nota:** `invoke-command` y terminal nativa **no** son canal válido para operaciones Git; ver **git-operations.md** (Ley de Hierro).
- **Herramienta (tool):** definida en paths.toolsDefinitionPath, implementación en paths.toolCapsules[tool-id].
- **Acción:** definida en paths.actionsPath (spec, planning, implementation, execution, validate, finalize-process, etc.). Las acciones pueden orquestar skills o herramientas que ejecuten git.
- **Proceso:** definido en paths.processPath (feature, bug-fix, create-tool, create-principle, etc.). Los procesos invocan acciones y skills; la IA sigue el proceso en lugar de lanzar git por su cuenta.

## Justificación

- **Trazabilidad:** Las skills/herramientas/acciones registran y auditan las operaciones (p. ej. Karma2Token, invoke-command).
- **Consistencia:** Mismo flujo para humanos y agentes; no hay “atajos” que eviten la validación.
- **Cumplimiento:** Ramas feat/fix, commits convencionales y principio nomenclatura se aplican cuando el flujo pasa por los artefactos definidos en SddIA.

## Aplicación

- Si la IA necesita hacer una operación de control de versiones (crear rama, commit, publicar rama, crear PR, etc.), debe **proponer** o **ejecutar** la invocación mediante la skill, herramienta, acción o proceso correspondiente (p. ej. “Ejecutar git-workspace-recon y luego git-branch-manager” o “Invocar invoke-command con el comando indicado”), **nunca** escribiendo comandos `git ...` directamente en la terminal.
- Los agentes que ejecutan código (p. ej. Tekton) ya tienen esta restricción en su contrato (execution_contract, constraints); esta norma la extiende a **cualquier** contexto de IA en el proyecto.
- **Referencia en protocolo:** AGENTS.md, ley GIT, esta norma y **git-operations.md**. Skills Git: únicamente la suite táctica S+ Grade (git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr).
