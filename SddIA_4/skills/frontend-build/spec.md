---
commands:
  build_product: Set-Location src; npm run build
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
name: Frontend Build (Next.js)
owner: tekton-developer
related_agents:
  - tekton-developer
  - frontend-architect
rules:
  - El script 'build' en package.json debe apuntar a node scripts/next-build-fallback.cjs.
  - 'Ejecutar desde la raíz del repositorio: Set-Location src; luego npm run build (o node scripts/next-build-fallback.cjs).'
  - 'Si MODULE_NOT_FOUND next/dist/bin/next persiste, reinstalar: Remove-Item -Recurse node_modules; npm install en src/.'
scripts:
  - cwd: src
    id: next-build-fallback-product
    name_short: Build Product (cliente)
    npm_script: npm run build
skill_id: frontend-build
spec_version: 1.1.0
status: Active
---
# Skill: Frontend Build (Next.js)

**skill_id:** `frontend-build`

## Objetivo

Scripts y comandos para compilar el frontend **GesFer.Product.Front** (`src/`, paquete `gesfer-cliente`) sin depender del binario `next` empaquetado de forma frágil (`dist/bin/next`). Usar cuando el build falle con `MODULE_NOT_FOUND`.

**Ámbito:** Este repositorio es solo el front de producto; no existe Admin ni `src/Product/Front` en el árbol.

## Reglas

- El script `build` en `src/package.json` debe apuntar a `node scripts/next-build-fallback.cjs`.
- Ejecutar desde la raíz del repositorio: `Set-Location src` (PowerShell); luego `npm run build`.
- Si `MODULE_NOT_FOUND` respecto a `next/dist/bin/next` persiste: en `src/`, `Remove-Item -Recurse node_modules`; `npm install`.

## Scripts

- **next-build-fallback-product:** Build del cliente — directorio de trabajo `src/`.

## Comandos

- **build_product:** `Set-Location src; npm run build`

---
*Definición en paths.skillsDefinitionPath/frontend-build/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
