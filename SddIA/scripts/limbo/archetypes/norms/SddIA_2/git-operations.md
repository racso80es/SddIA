# Norma: Operaciones Git (leyes de ejecución)

**Ámbito:** cualquier agente o IA que ejecute o orqueste operaciones Git en el repositorio. Complementa `SddIA/norms/git-via-skills-or-process.md`; en caso de conflicto sobre **cómo** resolver un fallo de skill o un bypass, **prevalece esta norma** para operaciones Git.

## Ley de Hierro — Prohibición de comandos primitivos

«Prohibición de Comandos Primitivos: Queda estrictamente prohibido el uso de invoke-command, powershell -Command o cualquier ejecución de terminal nativa para operaciones de Git. Si una Skill falla, el agente debe reportar la incidencia y solicitar la refactorización de la cápsula, prohibiéndose explícitamente el bypass manual para garantizar la trazabilidad.»

## Aplicación

- Las operaciones Git deben realizarse **únicamente** mediante skills, herramientas, acciones o procesos definidos en SddIA (sin `git` directo ni interceptores genéricos de shell para Git).
- Un fallo en la cápsula o binario de una skill Git es un **defecto a corregir en la skill** (p. ej. hotfix en `scripts/skills-rs`), no un motivo para operar Git fuera del canal trazado.
