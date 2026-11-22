---

feature_name: kaizen-sddia-qa-stdin-tty-guard
created: "2026-11-22"
updated: "2026-11-22"
process: feature
branch_name: feat/kaizen-sddia-qa-stdin-tty-guard
persist_ref: docs/features/kaizen-sddia-qa-stdin-tty-guard
pbi_ref: AUT-99
document_id: PBI-KAIZEN-SDDIA-QA-EVOLUTION-REGISTER-TTY-HANG
phase: ejecucion
status: implemented
norm_ref: features-documentation-pattern@1.3.1
agent: tekton
execution_id: "c1910c23-3ebf-456b-a60b-9c3aa30884a5"
---
# Implementación — kaizen-sddia-qa-stdin-tty-guard

## 1. Resolver compartido en `sddia-io` (`K-FILE`, `K-ENV`, `K-TTY`, `K-SDDIA-IO`)
- `SddIA/sddia-io/src/lib.rs`: añadidos `resolve_capsule_json_with` y `read_capsule_json_from_args` con precedencia `capsule-json-io` v2.0 (`--request-file <ruta>` / `--request-file=<ruta>` > `SDDIA_CAPSULE_REQUEST` > `SDDIA_SKIP_STDIN=1` > guarda `std::io::stdin().is_terminal()` > `stdin` pipe/heredoc).
- `read_stdin_json()` delega en `read_capsule_json_from_args` (con `empty_stdin_as_empty_obj = true`), cubriendo a todas las tools/skills consumidoras.
- Añadidos 5 tests unitarios en `sddia-io` (`tty_without_alternative_fails_fast`, `request_file_takes_precedence_over_env_and_tty`, `env_request_takes_precedence_over_skip_and_tty`, `skip_stdin_returns_empty_object_when_allowed_or_err_in_strict`, `pipe_stdin_reads_json_when_not_tty`).

## 2. `sddia-qa` y `sddia-evolution-register` (`K-TTY`, `K-PIPE`, `K-HELP`)
- `SddIA/tools/sddia-qa/Cargo.toml`: añadida dependencia `sddia-io`.
- `SddIA/tools/sddia-qa/src/gate_evolution.rs`: `run_mutate` usa `sddia_io::read_capsule_json_from_args(args, false)` emitiendo JSON `success: false, exitCode: 1` sin bloquear en TTY.
- `SddIA/tools/sddia-qa/src/main.rs`: `usage()` actualizado con `--request-file <path>` y requisito de entrada JSON.
- `SddIA/skills/sddia-evolution-register/src/main.rs`: usa `sddia_io::read_capsule_json_from_args(&args, false)`. Verificada compilación `wasm32-wasip1`.

## 3. Norma y Skill (`K-DOC`)
- `SddIA/norms/capsule-json-io.md`: reforzada la regla **Fail-Fast TTY** y precedencia determinista.
- `SddIA/skills/sddia-evolution-register.md`: añadido ejemplo mínimo CLI con `--request-file`.
