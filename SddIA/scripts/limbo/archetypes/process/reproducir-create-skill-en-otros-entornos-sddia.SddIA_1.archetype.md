---
document_id: reproducir-create-skill-en-otros-entornos-sddia
document_type: guia_portabilidad
norm_ref: SddIA/norms/paths-via-cumulo.md
process_ref: paths.processPath/create-skill/
analog_process_ref: paths.processPath/create-tool/
contract_ref: paths.processPath/process-contract.md
skills_contract_ref: SddIA/skills/skills-contract.md
evolution_ref: SddIA/norms/sddia-evolution-sync.md
cumulo_ref: SddIA/agents/cumulo.json
paths_contract_ref: SddIA/agents/cumulo.paths.json
idioma: es-ES
version: "1.0.0"
descripcion: >-
  Gu├¡a para reproducir el proceso de tarea create-skill (definici├│n en paths.processPath,
  listados y difusi├│n) en otro repositorio o entorno SddIA.
---

# Reproducir el proceso `create-skill` en otros entornos SddIA

Esta gu├¡a describe **qu├® artefactos copiar, qu├® ├¡ndices actualizar y c├│mo validar** para que el proceso **`create-skill`** exista y sea descubrible en **otro repo** que siga el mismo ecosistema SddIA. Las rutas l├│gicas son siempre las del **C├║mulo** (`SddIA/agents/cumulo.json` ÔåÆ `pathsContract` ÔåÆ `SddIA/agents/cumulo.paths.json`); no documentar rutas f├¡sicas como SSOT sin anclarlas a claves `paths.*`.

**Alcance de este documento:** portar la **definici├│n del proceso** (`paths.processPath/create-skill/`) y su **visibilidad** (README de procesos, disparador `#Process`, difusi├│n Cursor si aplica). **No** incluye portar skills concretas; para eso usar `SddIA/skills/reproducir-skills-en-otros-entornos-sddia.md`.

## 1. Prerrequisitos del entorno destino

| Requisito | Uso |
|-----------|-----|
| **C├║mulo** presente | `cumulo.json` + `cumulo.paths.json` con `paths.processPath`, `paths.featurePath`, `paths.skillsDefinitionPath`, `paths.skillCapsules`, `paths.skillsIndexPath`, `paths.skillsRustPath`. |
| **Contrato de procesos** | `paths.processPath/process-contract.md` (y `spec.json` por proceso si el destino lo exige). |
| **Windows 11 + PowerShell 7+** | Convenci├│n del proyecto GesFer; alinear si el destino es distinto. |
| **Evolution SddIA** (si aplica) | Cualquier alta/modificaci├│n bajo `./SddIA/` seg├║n `SddIA/norms/sddia-evolution-sync.md`. |

## 2. Analog├¡a con `create-tool`

El proceso **`create-skill`** est├í modelado como **espejo** de **`create-tool`**, sustituyendo:

| Concepto | create-tool | create-skill |
|----------|-------------|--------------|
| Rama de tarea | `feat/create-tool-<tool-id>` | `feat/create-skill-<skill-id>` |
| Persistencia tarea | `paths.featurePath/create-tool-<tool-id>` | `paths.featurePath/create-skill-<skill-id>` |
| Definici├│n entidad | `paths.toolsDefinitionPath` | `paths.skillsDefinitionPath` |
| C├ípsula | `paths.toolCapsules` | `paths.skillCapsules` |
| ├ìndice | `paths.toolsIndexPath` | `paths.skillsIndexPath` |
| C├│digo Rust | `paths.toolsRustPath` | `paths.skillsRustPath` |

Referencia de plantilla: `paths.processPath/create-tool/spec.md`.

## 3. Checklist: definici├│n del proceso en `paths.processPath`

1. **Carpeta del proceso**  
   Crear `paths.processPath/create-skill/` con al menos:
   - **`spec.md`** ÔÇö frontmatter YAML + cuerpo Markdown (`process_id: create-skill`, entradas, rutas v├¡a refs `paths.*`, fases, restricciones).
   - **`spec.json`** ÔÇö metadatos machine-readable (`process_id`, `persist_ref`, `phases`, `outputs`, etc.) alineado con el cuerpo de `spec.md`.

2. **Coherencia con contrato de skills**  
   En `spec.md`, la c├ípsula ejecutable debe describirse como **`<nombre>.exe` en la ra├¡z de la c├ípsula** (sin `bin/`), seg├║n `SddIA/skills/skills-contract.md` y envelope `SddIA/norms/capsule-json-io.md`.

## 4. Checklist: visibilidad y consumo por agentes / IDE

| Artefacto | Acci├│n |
|-----------|--------|
| `paths.processPath/README.md` | A├▒adir fila **create-skill** en la tabla de procesos y entrada numerada en la secci├│n **Uso**. |
| `SddIA/norms/interaction-triggers.md` | En el listado del disparador **`#Process`**, a├▒adir fila `create-skill` con descripci├│n y `paths.processPath/create-skill/`. |
| `.cursor/rules/process-suggestions.mdc` | Si el destino usa Cursor, a├▒adir la misma fila en la tabla del disparador `#Process` (difusi├│n; no sustituye la norma). |
| `AGENTS.md` (opcional) | Si el destino lista procesos en el protocolo maestro, a├▒adir **`create-skill`** en la tabla de procesos para no contradecir `paths.processPath`. |

## 5. Evolution SddIA (obligatorio si alteras `./SddIA/`)

En la misma intervenci├│n o PR:

1. Fichero de detalle `paths.sddiaEvolutionPath/{uuid}.md` con frontmatter seg├║n `paths.sddiaEvolutionContractFile`.
2. Entrada en `paths.sddiaEvolutionLogFile` (├¡ndice maestro).

**Tipolog├¡a:** normalmente `alta` al introducir el proceso por primera vez; `modificacion` si solo se ajusta texto o checklist.

## 6. Documentaci├│n de la feature de portado (recomendado)

Si el portado se hace como tarea trazable, usar el proceso **`feature`** y persistir en `paths.featurePath/<nombre_feature>/` con el patr├│n **un `.md` por acci├│n** y frontmatter (`objectives.md`, `clarify.md`, `plan.md`, ÔÇª) seg├║n `SddIA/norms/features-documentation-pattern.md` y `docs/features/features-contract.md`.

Nombre sugerido de feature: p. ej. `create-process-create-skill` o `port-process-create-skill-<repo-slug>`.

## 7. Verificaci├│n m├¡nima en el destino

1. Existe `paths.processPath/create-skill/spec.md` y `spec.json` (si el destino exige JSON por proceso).  
2. `paths.processPath/README.md` menciona **create-skill** y el enlace relativo `./create-skill/` resuelve.  
3. `SddIA/norms/interaction-triggers.md` lista **create-skill** bajo `#Process`.  
4. B├║squeda global: no quedan referencias rotas a `create-skill` sin carpeta destino.  
5. Si aplica evolution: ├¡ndice + detalle UUID presentes y coherentes.

## 8. Referencias cruzadas

- Proceso can├│nico: `paths.processPath/create-skill/spec.md`  
- Proceso an├ílogo: `paths.processPath/create-tool/spec.md`  
- Portar skills ejecutables: `SddIA/skills/reproducir-skills-en-otros-entornos-sddia.md`  
- Rutas: `SddIA/norms/paths-via-cumulo.md`  
- E/S JSON agentes: `SddIA/norms/capsule-json-io.md`

---

*Gu├¡a de portabilidad del proceso. Mantener alineada con C├║mulo, process-contract y skills-contract al evolucionar.*
