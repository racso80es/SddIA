---
uuid: "93d23720-d79a-412f-a85d-ab9b2d9862bd"
name: "shell-executor"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "system-operations"
capabilities:
  - "execute-external-binary"
  - "orchestrator-bridge"
hash_signature: "sha256:26186ed7178123f51fa56f78157f07ebf0b2c4a78fc1ce02d0c2878804c1a8db"
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
Cápsula nativa **`shell-executor`** resuelta vía `cumulo.paths.json` → `compiled_capsules` → `SddIA/target/{release|debug}/shell-executor` (preferente WASM en `compiled_capsules.wasm_root`). Un único objeto JSON por **stdin**; respuesta JSON en **stdout** (`success`, `exitCode`, `data`, `error` si fallo).

Invocación orientativa: `./sddia-run.sh --tool shell-executor`

Invocación directa (tras `cargo build -p shell-executor`): `SddIA/target/debug/shell-executor`

Implementación: crate Rust `SddIA/skills/shell-executor/` — sanitización estricta por token; **bloqueo anti-git** por nombre y basename de ruta resuelta; allowlist local + whitelist Cerbero previa a invocación.

