---
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
name: Reproducir skills en otros entornos SddIA
skill_id: reproducir-skills-en-otros-entornos-sddia
related_norm: SddIA/norms/reproducir-create-skill-en-otros-entornos-sddia.md
related_process: paths.processPath/create-skill/
---
# Skill: Reproducir skills en otros entornos SddIA

**skill_id:** `reproducir-skills-en-otros-entornos-sddia`

## Objetivo

Guía complementaria al proceso **`create-skill`** (`paths.processPath/create-skill/`). Describe cómo **portar una skill concreta** (definición + cápsula + índice) a otro repositorio SddIA.

## Procedimiento (resumen)

1. Copiar o recrear **`paths.skillsDefinitionPath/<skill-id>/`** (`spec.md` con YAML Frontmatter; si hay ejecutable, metadatos alineados con **`implementation_path_ref`** según contrato).
2. Copiar la **cápsula** bajo la ruta que resuelva **`paths.skillCapsules.<skill-id>`** en el Cúmulo del destino (o crear entrada nueva y ajustar `cumulo.paths.json`).
3. Fusionar **`paths.skillsIndexPath`** sin romper entradas existentes.
4. Validar contra **`SddIA/skills/skills-contract.md`** y **`SddIA/norms/capsule-json-io.md`** (en destino: rutas vía `paths.*`).
5. Registrar evolution en el destino si alteras `./SddIA/` (`SddIA/norms/sddia-evolution-sync.md`).

## Relación con otras piezas

- Portar solo el **proceso documental** de alta de skills (carpeta `paths.processPath/create-skill/` y visibilidad): norma **`SddIA/norms/reproducir-create-skill-en-otros-entornos-sddia.md`** y skill **`reproducir-create-skill-sddia`**.

## Alcance

Skill de **definición únicamente** (sin cápsula ejecutable). Consumida por agentes y procesos al orientar portabilidad de skills entre repos.

---
*Definición en paths.skillsDefinitionPath/reproducir-skills-en-otros-entornos-sddia/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
