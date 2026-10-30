---
feature_name: core-git-isolation-lab-absolute
created: "2026-10-30"
process: bug-fix
branch_name: fix/core-git-isolation-lab-absolute
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/core-git-isolation-lab-absolute
pbi_ref: AUT-6
execution_id: "652511ad-2740-49f1-8ef5-ef9397fadefe"
---

# Objetivos — core-git-isolation-lab-absolute

## Misión

bind(core) falla con PROJECT_GIT_INVALID cuando .SddIA/projects/{slug}.md tiene project_root absoluto anidado (_refine_lab_* / _forge_lab_*). L-ISOLATION ya omite relativas. Criterio: exempt labs/fixtures en assert_git_isolation_for_slug; test bind(core) con índice lab absoluto; barcelonaxplorer y nested real siguen aislados.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
