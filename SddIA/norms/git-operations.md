---
uuid: "e5f6a7b8-c9d0-4123-e456-7890abcdef01"
name: "git-operations"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "cerbero"
---

# Política de operaciones Git (norma, no ejecución)

## 1. Separación norma / ejecución

La skill **`git-manager`** es un peón determinista: ejecuta subcomandos del binario `git` nativo según el esquema congelado. **No** interpreta convenciones de negocio del proyecto.

Las reglas de negocio (p. ej. prefijos de rama `feat/`, `fix/`, mensajes de commit, ramas prohibidas) viven **aquí** y son aplicadas **antes** de invocar `git-manager`, por Cerbero y Argos (u otros agentes de auditoría autorizados).

## 2. Contenido normativo (extensible)

Esta versión inicial establece el marco; Dédalo y Cúmulo completarán los detalles según el repositorio activo.

- **Ramas de trabajo:** convención de nombres y troncal de integración (definir por proyecto).
- **Commits:** alineación con Conventional Commits u otra política acordada.
- **Force push:** cuándo está permitido y bajo qué token/contexto.

## 3. Referencias

- Entrada congelada de la skill: `SddIA/norms/skill-io-git-manager-frozen.md`
- Mapa de rutas Core: `SddIA/core/cumulo.paths.json` → `directories.norms`
