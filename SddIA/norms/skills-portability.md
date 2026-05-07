---
uuid: "f6a7b8c9-d0e1-4234-f567-890abcdef012"
name: "skills-portability"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "cumulo"
---

# Portabilidad de skills y del Core SddIA (metaconocimiento)

Este documento **no** es una skill ni un proceso ejecutable. Consolida el metaconocimiento para reproducir definiciones de skills y su gobierno en otro repositorio SddIA.

## 1. Principios

- **SSOT de topología:** `SddIA/core/cumulo.paths.json` define directorios (`directories.*`) y cápsulas; no duplicar rutas literales en documentación operativa.
- **Formato canónico de skill:** un único `{name}.md` en `directories.skills` con cabecera YAML S+ (uuid, name, version, contract, context, capabilities, inputs/outputs).
- **Esquemas de entrada congelados:** las skills `git-manager` y `shell-executor` obedecen las normas `skill-io-git-manager-frozen.md` y `skill-io-shell-executor-frozen.md` hasta nueva versión explícita.

## 2. Checklist al portar el Core

1. Copiar o fusionar `cumulo.paths.json` y validar claves: `directories.norms`, `directories.skills`, `execution_capsules.skills`.
2. Registrar skills en `SddIA/skills/index.md` con filas alineadas al YAML de cada `{name}.md`.
3. Asegurar que agentes consumen rutas vía Cúmulo, no paths absolutos del desarrollador.
4. Para binarios en transición, usar `SddIA/scripts/limbo/` y la convención de extensión `.limbo.*` hasta que un Proceso los sustituya.

## 3. Referencias

- Contrato de familia: `SddIA/skills/skills-contract.md`
- Orquestación PR: `SddIA/norms/pull-request-orchestration.md`
- Limbo: `SddIA/scripts/limbo/README.md`
