---
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
name: Reproducir proceso create-skill en otros entornos SddIA
skill_id: reproducir-create-skill-sddia
norm_ref: SddIA/norms/reproducir-create-skill-en-otros-entornos-sddia.md
process_ref: paths.processPath/create-skill/
analog_process_ref: paths.processPath/create-tool/
---
# Skill: Reproducir proceso create-skill en otros entornos SddIA

**skill_id:** `reproducir-create-skill-sddia`

## Objetivo

Orientar la **portabilidad del proceso de tarea `create-skill`** (definición bajo `paths.processPath/create-skill/`, índices, disparador `#Process`, difusión IDE) hacia **otro repositorio** que use el mismo ecosistema SddIA. El contenido normativo detallado (checklists, evolution, verificación) está en **`SddIA/norms/reproducir-create-skill-en-otros-entornos-sddia.md`**.

## Cuándo usarla

- Alta del proceso **`create-skill`** en un clon o producto hermano.
- Auditoría de que el destino expone el proceso en `paths.processPath/README.md`, `interaction-triggers` y reglas Cursor si aplica.

## No cubre

Portar **skills ejecutables concretas**; para eso usar **`reproducir-skills-en-otros-entornos-sddia`**.

## Adaptación GesFer

En este repo, la **cápsula** de skills con binario sigue **`skills-contract`**: ejecutable en **`<cápsula>/bin/`**, launcher `.bat`, fallback `.ps1`. No usar como referencia de este proyecto la variante “`.exe` solo en raíz de cápsula” si contradice ese contrato.

## Alcance

Skill de **definición únicamente** (sin cápsula ejecutable).

---
*Definición en paths.skillsDefinitionPath/reproducir-create-skill-sddia/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
