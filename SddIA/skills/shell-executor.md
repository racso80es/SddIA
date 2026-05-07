---
uuid: "93d23720-d79a-412f-a85d-ab9b2d9862bd"
name: "shell-executor"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "system-operations"
capabilities:
  - "execute-external-binary"
  - "orchestrator-bridge"
hash_signature: "sha256:f1e1478de696f477f7c66024d876cdb69d9eba270a3ed6a74e58e1ebab23a0ed"
inputs:
  - "executable": "string; ejecutable permitido; validación por whitelist de Cerbero. Invariante anti-git: prohibido `git` / `git.exe` (por nombre o basename de ruta resuelta)."
  - "arguments": "string[]; tokens ya separados. Anti-inyección: prohibidos por token `\\n`, `\\r`, `;`, `&&`, `|`, `>`, `<`, backticks, `$(`, `)` y `&`."
  - "working_directory": "string; ruta absoluta resuelta previamente por Cúmulo."
  - "environment_vars": "(Opcional) object string→string; variables efímeras. Prohibido inyectar secretos hardcodeados."
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "data": "objeto con `stdout` y `stderr` (strings)."
  - "error": "string visible si exitCode != 0 o falla validación/whitelist."
---

# Skill: shell-executor (definición + cápsula Fase B)

## 1. Propósito y naturaleza
Puente seguro y aislado hacia la terminal del sistema para ejecutar **binarios de terceros** e integraciones del SO (p. ej. `gh`, `npm`, `docker`, `python`), **excluyendo explícitamente** el binario `git` nativo (que debe enrutarse por `git-manager`).

## 2. SSOT del esquema de entrada (congelado)
El esquema de entrada y sus contramedidas antinyección se rigen por la norma congelada:

- `SddIA/norms/skill-io-shell-executor-frozen.md`

## 3. Invariante anti-git (regla dura)
La skill debe abortar inmediatamente con `exitCode != 0` si `executable` es exactamente `git` o `git.exe`, o si el basename de su ruta resuelta evalúa a `git` / `git.exe`. Su uso debe enrutarse obligatoriamente por `git-manager`.

## 4. Motor de ejecución (cápsula física)
Cápsula **`shell-executor.py`** resuelta vía `cumulo.paths.json` → `execution_capsules.skills` → `shell-executor.py`. Un único objeto JSON por **stdin**; respuesta JSON en **stdout** (`success`, `exitCode`, `data`, `error` si fallo). Invocación orientativa desde la raíz del workspace:

`python {paths.execution_capsules.skills}/shell-executor.py`

Implementación: `subprocess` **sin** `shell=True`, con `arguments` como array; sanitización estricta por token; **bloqueo anti-git** por nombre y por basename de ruta resuelta; allowlist local + capa de whitelist por Cerbero previa a invocación.

