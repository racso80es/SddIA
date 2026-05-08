# Proceso: Creaci├│n de Principio (create-principle)

Este documento define el **proceso** para a├▒adir nuevos principios t├®cnicos a `paths.principlesPath` (SddIA/principles/).

## Objetivo

Estandarizar la incorporaci├│n de principios (gu├¡as t├®cnicas, normas de nomenclatura, etc.), asegurando que cada entrada cumpla el contrato de principios y sea utilizable por Arquitecto, Tekton, C├║mulo y, si aplica, por la validaci├│n pre-PR.

## Alcance

- **Ubicaci├│n:** paths.principlesPath (C├║mulo) = SddIA/principles/
- **Estructura:** Carpeta `<principle-id>` en kebab-case.
- **Contenido:** spec.md (legible) y spec.json (metadatos). Ambos obligatorios seg├║n principles-contract.

## Fases del Proceso

### 1. Definici├│n

1. **Elegir principle_id:** Identificador en kebab-case (ej. `nomenclatura`, `norma-nomenclatura`).
2. **Crear carpeta:** paths.principlesPath/\<principle-id\>/.
3. **Redactar spec.md:**
   - T├¡tulo, categor├¡a, resumen y objetivo.
   - Contenido del principio (reglas, criterios).
   - Aplicaci├│n para Arquitecto y Tekton (y, si aplica, para C├║mulo o QA Judge).
   - Referencias.
   - Idioma: espa├▒ol (es-ES).
   - Pie: *Definici├│n en paths.principlesPath/\<principle-id\>/ (contrato paths.principlesPath/principles-contract.md).*
4. **Crear spec.json:**
   - **id:** UUID v4 del principio.
   - **principle_id:** mismo que el nombre de la carpeta (kebab-case).
   - **title:** T├¡tulo legible.
   - **category:** Categor├¡a (Clean Code, Principios SOLID, Normas SddIA, etc.).
   - **tags:** Lista de etiquetas.
   - **metadata:** difficulty (Beginner | Intermediate | Advanced), status (Draft | Published | Deprecated).
   - **contract_ref:** paths.principlesPath/principles-contract.json.
   - **blocking_for_pr:** (opcional) true si el PR no debe aprobarse si no se cumple este principio. La acci├│n validate debe incluir un check para este principio.
   - **defined_by_agent:** (opcional) Agente que define/custodia el principio (ej. cumulo).

### 2. Validaci├│n

1. Verificar que spec.json cumple SddIA/principles/principles-contract.json.
2. Asegurar que spec.md tiene formato claro y contenido ├║til.
3. Si **blocking_for_pr** es true: confirmar que la acci├│n validate (paths.actionsPath/validate/) incluye la comprobaci├│n de este principio y que el resultado es bloqueante (global fail / blocking).

## Principio bloqueante para PR

Si el principio debe impedir que un PR pase cuando no se cumple (ej. norma de nomenclatura):

- En spec.json: `"blocking_for_pr": true`.
- En spec.md: indicar expl├¡citamente que *Un PR no debe aprobarse si no se cumple esta norma*.
- La acci├│n **validate** debe tener un check (ej. `nomenclatura` o `principle_<principle-id>`) que falle cuando se incumpla y que figure como obligatorio/blocking en validacion.json.

## Artefactos

- paths.principlesPath/\<principle-id\>/spec.md
- paths.principlesPath/\<principle-id\>/spec.json

## Referencia

- Contrato: SddIA/principles/principles-contract.json y principles-contract.md.
- C├║mulo: paths.principlesPath.
