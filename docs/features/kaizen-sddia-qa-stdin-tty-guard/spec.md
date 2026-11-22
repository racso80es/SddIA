---

feature_name: kaizen-sddia-qa-stdin-tty-guard
created: "2026-11-22"
updated: "2026-11-22"
process: feature
base: main
scope: "Guarda Fail-Fast TTY y paridad capsule-json-io v2.0 en sddia-io, sddia-qa y sddia-evolution-register"
branch_name: feat/kaizen-sddia-qa-stdin-tty-guard
persist_ref: docs/features/kaizen-sddia-qa-stdin-tty-guard
pbi_ref: AUT-99
document_id: PBI-KAIZEN-SDDIA-QA-EVOLUTION-REGISTER-TTY-HANG
document_uuid: "94c98f11-20d4-485b-af06-6d9e66b7e162"
agent: dedalo
phase: diseno-blueprint
version_spec: "1.0.0"
status: blueprint-ready
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/AUT-99.jsonl"
execution_id: "c1910c23-3ebf-456b-a60b-9c3aa30884a5"
---
# Especificación técnica — kaizen-sddia-qa-stdin-tty-guard

## 1. Resolver compartido en `sddia-io` (`K-FILE`, `K-ENV`, `K-TTY`, `K-SDDIA-IO`)
- En `SddIA/sddia-io/src/lib.rs`:
  - Implementar `resolve_capsule_json_input(...)` con precedencia:
    1. `--request-file <ruta>` o `--request-file=<ruta>` en `args`
    2. `SDDIA_CAPSULE_REQUEST` (si definida y no vacía tras `trim()`)
    3. `SDDIA_SKIP_STDIN` (`1|true|yes|on`) → `Ok(json!({}))` (si `empty_stdin_as_empty_obj`, o `Err` si se exige payload en modo estricto? Nota: según D-R1 y propuesta 2.3, `SDDIA_SKIP_STDIN=1` → no leer `stdin` y devolver `{}`).
    4. Si `stdin_is_tty` (`std::io::stdin().is_terminal()`) → `Err(...)` inmediato con opciones válidas.
    5. Lectura de `stdin` por pipe/heredoc.
  - Exponer `read_capsule_json_from_args(args: &[String], empty_stdin_as_empty_obj: bool) -> Result<Value, String>`.
  - Actualizar `read_stdin_json()` para delegar en `read_capsule_json_from_args(&env::args().collect::<Vec<_>>(), true)` llamando a `emit_error(&e, 1)` si falla.
  - Tests unitarios en `sddia-io` cubriendo las 5 ramas de precedencia y error en TTY.

## 2. `sddia-qa` (`gate_evolution.rs` y `main.rs`) (`K-TTY`, `K-HELP`)
- `SddIA/tools/sddia-qa/src/gate_evolution.rs`: en `run_mutate`, sustituir `io::stdin().read_to_string` por `sddia_io::read_capsule_json_from_args(args, false)`, devolviendo `{"success": false, "exitCode": 1, "message": e}` con exit code `1` en caso de error.
- `SddIA/tools/sddia-qa/src/main.rs`: actualizar `usage()` para documentar `evolution-register [--json] [--dry-run] [--request-file <path>] (lee JSON de --request-file, SDDIA_CAPSULE_REQUEST o stdin no-TTY)`.

## 3. `sddia-evolution-register` (`src/main.rs`) (`K-PIPE`)
- `SddIA/skills/sddia-evolution-register/src/main.rs`: usar `sddia_io::read_capsule_json_from_args(&env::args().collect::<Vec<_>>(), false)`, emitiendo envelope v2 con `success: false, exitCode: 1` si falla.
- Verificar compilación para `wasm32-wasip1`.

## 4. Documentación y norma (`K-DOC`)
- `SddIA/norms/capsule-json-io.md`: reforzar la regla **Fail-Fast TTY** y precedencia determinista.
- `SddIA/skills/sddia-evolution-register.md`: añadir ejemplo mínimo de invocación (`--request-file` y pipe) vía `entity-manager`.
