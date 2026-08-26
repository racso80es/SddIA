# Validación de PR: [FIX] accept-pr delete_branch payload vs git-manager

## Cambios realizados
1. **`SddIA/engine/execute-process/src/engine/accept_pr.rs`**:
   - Se reescribió `delete_branch_hygiene` para efectuar dos llamadas a la cápsula `git-manager` (`remote: false` para eliminación local, y `remote: true` para eliminación remota).
   - El payload ahora cumple estrictamente con el contrato `["branch_name", "remote", "force"]` con los valores `remote` y `force` como booleanos, corrigiendo la Fricción `F-GIT-DELETE-BRANCH-PAYLOAD`.
   - Se estructura el array `operations[]` en caso de fallos dentro de la variable `hygiene_failure`, respetando el fail-soft. `closed_branch` contiene el nombre de la rama **solo** si la operación de eliminación local (`remote: false`) resultó exitosa.
2. **`SddIA/norms/skill-io-git-manager-frozen.md`**:
   - Se actualizaron la lista enum de operaciones permitidas y los esquemas `operation_payload_json` para sincronizar con la cápsula implementada.
   - Operaciones añadidas al contrato estricto: `merge`, `get_last_commit`, `delete_branch`, `diff_name_only`.

## Pruebas y verificación
- **Compilación de la cápsula WASM**: `execute-process` se compila sin errores a binario nativo y `git-manager` se recompiló correctamente al objetivo `wasm32-wasip1`.
- **Smoke test**: Se ejecutó en el entorno de pruebas local (`./SddIA/target/debug/execute-process --process accept-pr ...`) y se verificó que la fase no colapsaba por fallos `payload_exact` (el test empírico anterior a la corrección demostraba que se emitía un payload ilegítimo, ahora corregido).
- **Test cases**: Se ejecutaron los tests del motor vía `cargo test -p execute-process`. No se introdujeron nuevas fallas.

## Tareas completadas del PBI:
- [x] `accept_pr.rs`: 0 `"remote": "origin"` en `delete_branch`. Dos `invoke_git_manager` con booleanos + `force`.
- [x] `skill-io-git-manager-frozen.md` (SemVer) incluye `delete_branch` o documenta exclusión consciente.
- [x] `validacion.md` APTO + PBI movido a `docs/todos/done/`.
