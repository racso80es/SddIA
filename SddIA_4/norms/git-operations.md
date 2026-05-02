---
id: "norm-git-operations"
name: "Operaciones Git — estándar y leyes de hierro"
type: "norm"
status: "active"
---
# Norma: Operaciones Git (git-operations)

**Fuente:** SddIA/norms. Complementa **git-via-skills-or-process.md** y la skill de definición **paths.skillsDefinitionPath/git-operations/** (Cúmulo).

## Alcance

Establece reglas semánticas y de cumplimiento para el uso de Git en el ecosistema GesFer/SddIA, incluida la suite táctica en **paths.skillCapsules** (git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr).

## Ley de Hierro — Prohibición de comandos primitivos para Git

**Prohibición de comandos primitivos:** Queda estrictamente prohibido el uso de `invoke-command`, `powershell -Command` o cualquier ejecución de terminal nativa para operaciones de Git. Si una Skill falla, el agente debe reportar la incidencia y solicitar la refactorización de la cápsula, prohibiéndose explícitamente el bypass manual para garantizar la trazabilidad.

### Implicaciones

- Ningún atajo por shell interactiva o interceptor genérico sustituye a la cápsula Rust/script oficial de la skill Git afectada.
- Los fallos se documentan y se corrigen en el binario o envoltorio de la skill, no mediante re-ejecución manual no auditada.

## Coherencia

- Para el canal permitido de invocación (skills, tools, acciones, procesos), seguir **git-via-skills-or-process.md**.
- Esta ley de hierro **estrecha** el uso de `invoke-command` u otros ejecutores genéricos: no aplican como bypass para Git aunque estén permitidos para otros tipos de comando según **commands-via-skills-or-tools.md**.

---
*Norma canónica. Hotfix integridad: upstream safety en git-sync-remote y trazabilidad S+.*
