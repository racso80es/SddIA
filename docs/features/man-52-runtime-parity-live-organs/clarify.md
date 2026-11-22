---
feature_name: man-52-runtime-parity-live-organs
created: "2026-11-22"
process: feature
purpose: Estabilización Mayeuta de MAN-52 tras init lab
version_clarify: "1.0.0"
execution_id: "07a7307e-caf6-4a8c-b263-ad64910f48ea"
tracker_ref: MAN-52
document_id: PBI-0037f862
pbi_uuid: "f80e5a57-9bd7-4289-871f-1aa4cf15b513"
---

# Clarificación — man-52-runtime-parity-live-organs

Init: `SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process feature --inputs-file .tmp/feature-man-52.json`.
`execution_id`: `07a7307e-caf6-4a8c-b263-ad64910f48ea`. Rama `feat/man-52-runtime-parity-live-organs`.

Semilla: `MAN-52` refinado por Mayeuta (`32f382f2-54d4-4656-b206-1376f2e3dc13`), sin preguntas abiertas.

## Decisiones estabilizadas

| ID | Decisión |
|----|----------|
| **D-R1** | Catálogo `LIVE_ORGANS` cerrado y exhaustivo de 9 órganos: `execute-process`, `kalma2-bridge`, `event-watcher`, `github-bridge-watcher`, `email-watcher`, `telegram-watcher`, `iota-publish-relay`, `sddia-workspace-server`, `sddia-qa`. |
| **D-R2** | Inspección `MainPID`/`/proc/<pid>/exe` aplica solo cuando la unidad `systemd --user` `sddia-<organ>@<escaped_repo>.service` está activa (`ActiveState=active`); unidad ausente o inactiva no es fallo de paridad de proceso. |
| **D-R3** | Disparo del gate `validacion_runtime_parity` por diff de código (`merge_touches_live_organs`) contra `origin/main` / `main`, sin depender de tokens en Markdown de `persist_ref`. |
| **D-R4** | `sddia-qa` entra en el catálogo de órganos/frescura; `verify-compiled-capsules` falla si un binario presente está stale respecto a su testigo `.sha256` o sus fuentes. |
| **D-R5** | Ritual canónico pre-cierre `SddIA/scripts/sync-live-organs.sh` recompila los 9 órganos en `--release`, sella testigos `.sha256` y reinicia las unidades `systemd --user` activas del workspace. |
