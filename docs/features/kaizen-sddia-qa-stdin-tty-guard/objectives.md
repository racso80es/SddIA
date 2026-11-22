---

feature_name: kaizen-sddia-qa-stdin-tty-guard
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Objetivos estabilizados — guarda Fail-Fast TTY y paridad capsule-json-io v2.0 en sddia-qa evolution-register y sddia-io
branch_name: feat/kaizen-sddia-qa-stdin-tty-guard
persist_ref: docs/features/kaizen-sddia-qa-stdin-tty-guard
pbi_ref: AUT-99
document_id: PBI-KAIZEN-SDDIA-QA-EVOLUTION-REGISTER-TTY-HANG
document_uuid: "94c98f11-20d4-485b-af06-6d9e66b7e162"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/AUT-99.jsonl"
agents: mayeuta
execution_id: "c1910c23-3ebf-456b-a60b-9c3aa30884a5"
---
# Objetivos — kaizen-sddia-qa-stdin-tty-guard

## Misión

Eliminar el bloqueo indefinido sobre `stdin` interactivo (TTY) en `sddia-qa evolution-register` y en el resolver compartido `sddia-io::read_stdin_json()`, dotándolos de paridad completa con `capsule-json-io` v2.0 (`--request-file`, `SDDIA_CAPSULE_REQUEST`, `SDDIA_SKIP_STDIN=1`, y guarda `is_terminal()`).

Tracker **AUT-99** (`PBI-KAIZEN-SDDIA-QA-EVOLUTION-REGISTER-TTY-HANG`, `uuid`: `94c98f11-20d4-485b-af06-6d9e66b7e162`).

## Criterios de aceptación (Métricas de mejora)

| ID | Criterio |
|----|----------|
| **K-TTY** | `sddia-qa evolution-register --json` en PTY sin entrada devuelve JSON `success: false`, `exitCode: 1` y termina en **< 500 ms** (nunca `wait_woken`). |
| **K-FILE** | `--request-file <ruta>` lee JSON sin tocar `stdin` (soporte en `evolution-register`, `sddia-io` y cápsula nativa). |
| **K-ENV** | `SDDIA_CAPSULE_REQUEST` y `SDDIA_SKIP_STDIN=1` respetan la precedencia de `capsule-json-io` v2.0. |
| **K-PIPE** | Pipe/heredoc existentes y `invoke_capsule_subprocess` mantienen paridad total (incluyendo compilación `wasm32-wasip1`). |
| **K-SDDIA-IO** | `sddia-io::read_stdin_json()` incorpora guarda `is_terminal()` y vías alternativas; `cargo test -p sddia-io` en verde. |
| **K-HELP** | `sddia-qa --help` declara `--request-file` y el requisito de entrada JSON para `evolution-register`. |
| **K-DOC** | `SddIA/norms/capsule-json-io.md` refuerza Fail-Fast TTY y `SddIA/skills/sddia-evolution-register.md` incluye ejemplo mínimo; `gate-evolution --json --range --sync-base` en verde (`EVOL_OK`). |
