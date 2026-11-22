---

feature_name: kaizen-sddia-qa-stdin-tty-guard
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Clarificaciones estabilizadas — precedencia capsule-json-io v2.0 y compatibilidad wasm32-wasip1
branch_name: feat/kaizen-sddia-qa-stdin-tty-guard
persist_ref: docs/features/kaizen-sddia-qa-stdin-tty-guard
pbi_ref: AUT-99
document_id: PBI-KAIZEN-SDDIA-QA-EVOLUTION-REGISTER-TTY-HANG
document_uuid: "94c98f11-20d4-485b-af06-6d9e66b7e162"
phase: mayeuta-stabilization
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/AUT-99.jsonl"
agents: mayeuta
execution_id: "c1910c23-3ebf-456b-a60b-9c3aa30884a5"
---
# Clarificación — kaizen-sddia-qa-stdin-tty-guard

## Decisiones estabilizadas

- **D-R1 (Fail-Fast TTY):** En TTY sin `--request-file`, `SDDIA_CAPSULE_REQUEST` ni `SDDIA_SKIP_STDIN=1`, se emite error explícito (`success: false`, `exitCode: 1`, mensaje accionable). `{}` solo se devuelve con `SDDIA_SKIP_STDIN=1` (en `read_stdin_json()`) o pipe vacío explícito.
- **D-R2 (`--request-file`):** El flag `--request-file <ruta>` (y `--request-file=<ruta>`) forma parte del contrato de entrada canónico sin conflicto con `env::args()` de las tools.
- **D-R3 (Alcance documental `sddia-qa`):** Fuera de alcance dar de alta `SddIA/tools/sddia-qa.md` / fila en `tools/index.md`. Basta actualizar `usage()` en `sddia-qa/src/main.rs` y las normas/skills afectadas.
- **D-WASI (`wasm32-wasip1`):** Validar explícitamente la compilación WASI de `sddia-io` y `sddia-evolution-register` con `std::io::IsTerminal`.
