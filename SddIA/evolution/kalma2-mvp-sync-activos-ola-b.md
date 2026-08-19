---
uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
hito: "kalma2-mvp-sync-activos · Ola B · T6-T9b"
fecha_forja: "2026-08-19"
feature_ref: docs/features/kalma2-mvp-sync-activos
pbi: PBI-KALMA2-MVP-01B
pr_ref: feat/kalma2-mvp-sync-activos
entidades_nuevas:
  - uuid: "66daf19f-217a-4874-b417-99e5be2571f3"
    name: github-raw-fetcher
    type: tool
    path: SddIA/tools/github-raw-fetcher.md
  - uuid: "6175f5cd-7844-4d0c-aa93-d2ce3a41d18e"
    name: download-remote-asset
    type: action
    path: SddIA/actions/download-remote-asset.md
  - uuid: "0f6bf2ff-a067-46fb-9175-ee97e6a5dcd8"
    name: sync-client-assets
    type: process
    path: SddIA/process/sync-client-assets.md
mutaciones:
  - path: SddIA/core/capability-bindings.md
    cambio: "+binding asset:fetch → tool:github-raw-fetcher (pivote DLT G7)"
  - path: SddIA/interfaces/kalma2-bridge/src/main.rs
    cambio: "+POST /api/sync-assets (fire-and-forget DA-5, 202 + correlation_id)"
  - path: interfaces/kalma2/index.html
    cambio: "+botón Sincronizar Genoma"
  - path: interfaces/kalma2/app.js
    cambio: "+función syncGenome() con SSE progress stream"
gates_verificados:
  - G5: "github-raw-fetcher forjado + asset:fetch en capability-bindings.md"
  - G6: "tubería sync-client-assets → download-remote-asset → asset:fetch trazable; exitCode:0 ⟺ success:true"
  - G7: "grep github-raw-fetcher en download-remote-asset.md y sync-client-assets.md = 0"
  - G8: "POST /api/sync-assets devuelve 202 + correlation_id; WUI botón presente; SSE observable"
  - G9: "aduana SHA-256 pre-escritura definida en Fase 3; cicatriz digital materializada"
---

# Cicatriz Digital — Kalma2 MVP 01B · Ola B · T6–T9b

Hito: primera sincronización autoiniciada de activos desde repositorio maestro.

## Resumen de forja

Tres entidades nuevas cierran el circuito H4: `github-raw-fetcher` (cápsula temporal simuladora DLT), `download-remote-asset` (abstracción de reclamación) y `sync-client-assets` (proceso de 4 fases con aduana de integridad SHA-256).

El pivote DLT (`capability-bindings.md` binding `asset:fetch`) permite migrar a IOTA Rebased modificando una sola línea, sin tocar acción ni proceso.

`POST /api/sync-assets` en `kalma2-bridge` acepta la operación en < 100 ms (fire-and-forget DA-5) y devuelve `correlation_id` para seguimiento SSE. El botón "Sincronizar Genoma" en la WUI cierra el circuito para el usuario final.
