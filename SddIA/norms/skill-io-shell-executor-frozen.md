---
uuid: "c3d4e5f6-a7b8-4901-c234-567890abcdef"
name: "skill-io-shell-executor-frozen"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "cerbero"
freeze_status: "congelado"
applies_to_skill: "shell-executor"
schema_version: "2026-05-07"
---

# Esquema de entrada congelado — `shell-executor`

**Estado:** Congelado antes de la forja física de la cápsula. Cerbero y Argos deben rechazar solicitudes que no cumplan esta forma o que violen la lista blanca de ejecutables.

**Propósito:** Ejecutar herramientas del ecosistema que **no** sean el binario `git` nativo (p. ej. `gh`, `npm`, `python`, scripts del proyecto). El binario `git` debe enrutarse exclusivamente a `git-manager`.

## 1. Raíz del mensaje (stdin JSON)

| Campo | Tipo | Obligatorio | Descripción |
| :--- | :--- | :---: | :--- |
| `executable` | `string` | Sí | Nombre o ruta del ejecutable permitido (p. ej. `gh`, `npm`). Debe **superar la lista blanca** definida por Cerbero para el contexto de ejecución; no se aceptan metacaracteres de shell. |
| `arguments` | `string[]` | Sí | Lista estricta de argumentos **ya separados**; prohibido pasar una sola cadena con `&&`, `;`, `\|` u otras construcciones de shell que impliquen encadenamiento o sustitución. Array vacío permitido si el ejecutable no requiere args. |
| `working_directory` | `string` | Sí | Directorio de trabajo absoluto, **resuelto previamente por Cúmulo**. |
| `environment_vars` | `object` | No | Mapa `string` → `string` de variables **efímeras** para la invocación. **Prohibido** incluir secretos hardcodeados o credenciales persistentes; el suministro de secretos es responsabilidad del orquestador externo autorizado (fuera de este esquema). |

## 2. Ejemplo (`gh pr create`)

```json
{
  "executable": "gh",
  "arguments": ["pr", "create", "--fill"],
  "working_directory": "C:\\Proyectos\\Ejemplo",
  "environment_vars": {
    "CI": "true"
  }
}
```

## 3. Contramedidas antinyección

- `arguments` es la **única** vía de parámetros: no concatenar `executable` + string único para formar una línea de shell.
- Cualquier token que Cerbero no reconozca en la lista blanca → rechazo con `exitCode` de fallo de política (definición de salida en la skill definitiva).

## 4. Referencias

- Resolución de rutas: `SddIA/core/cumulo.paths.json`.
- Orquestación de PRs: `SddIA/norms/pull-request-orchestration.md`.
- Contextos de ejecución (RBAC): `SddIA/norms/execution-contexts.md`.
