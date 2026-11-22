---
feature_name: man-52-runtime-parity-live-organs
created: "2026-11-22"
process: feature
branch_name: feat/man-52-runtime-parity-live-organs
persist_ref: docs/features/man-52-runtime-parity-live-organs
execution_id: "07a7307e-caf6-4a8c-b263-ad64910f48ea"
tracker_ref: MAN-52
---

# Objetivos — man-52-runtime-parity-live-organs

## Misión

Ampliar `sddia-runtime-parity` a los 9 órganos vivos (`execute-process`, `kalma2-bridge`, `event-watcher`, `github-bridge-watcher`, `email-watcher`, `telegram-watcher`, `iota-publish-relay`, `sddia-workspace-server`, `sddia-qa`), verificar `MainPID`/`exe` en unidades `systemd --user` activas, activar el gate `validacion_runtime_parity` por diff de crates (`merge_touches_live_organs`) sin depender de tokens Markdown, validar frescura en `verify-compiled-capsules` y proveer un ritual canónico de sincronización pre-cierre (`SddIA/scripts/sync-live-organs.sh` + `runtime_recycle`).

## Criterios de aceptación

- **AC-1 (`K-ORGANS`):** `LIVE_ORGANS` cubre los 9 órganos listados; `resolve_organ_crate_dir` y `merge_touches_live_organs` cubren todos ellos.
- **AC-2 (`K-PROC`):** `assess_runtime_binary_parity` contrasta `source_sha256` / `elf_sha256` en disco y verifica `MainPID` → `/proc/<pid>/exe` en unidades `systemd --user` activas (`sddia-<organ>@*.service`); unidad ausente no genera falso positivo.
- **AC-3 (`K-TRIGGER`):** `validacion_runtime_parity` se activa por diff git de crates de órganos vivos (`branch_diff_touches_live_organs`), sin depender de menciones Markdown en `persist_ref`.
- **AC-4 (`K-SYNC`):** `SddIA/scripts/sync-live-organs.sh` recompila `--release` los 9 órganos, sella testigos `.sha256` y reinicia las unidades `systemd --user` activas del workspace; `runtime_recycle` alineado a los mismos órganos.
- **AC-5 (`K-CAP` / `H-NEW-07`):** `sddia-qa verify-compiled-capsules` falla (`success: false`, exit `1`) si un binario presente está stale frente a su testigo `.sha256` o sus fuentes.
- **AC-6 (`K-REGRESS`):** Tests de `sddia-runtime-parity`, `execute-process` (`validacion_runtime_parity`) y `sddia-qa` (`verify_compiled_capsules`) verdes; CI GitHub verde.

## Fuera de alcance

- Residuos `.events` / `gate-event-manager-port` (MAN-51).
- Endurecimiento `apto_coherence` (MAN-53).
- Blindaje `entity-manager` / `recalc-process-hash-signatures` (MAN-54).
- Estandarización de prompts de agentes (MAN-55).
