---
feature_name: cognitive-tier-native-dispatch
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/cognitive-tier-native-dispatch
tracker_ref: AUT-26
execution_id: "73d65b19-25cb-489c-94e5-abffe60b1e46"
phases:
  - id: T1
    title: El predicado nativo incluye process
  - id: T2
    title: Regresión de despacho store en tier_dispatch
---

# Plan — AUT-26

Rama `Fix_Events_Multiple`. Un commit por tarea cuando se ejecute. Sin mutación de genoma indexado. Sin abrir `fix/cognitive-tier-native-dispatch`.

## T1 — Predicado

En `subscriber_declares_native`, añadir `process` a la lista que ya tiene `action` y `tool`. Cadena vacía o solo espacios sigue en falso.

El `if` de `store_backed && agent_tier_is_cognitive && !declares_native` no se reordena. Con `process` declarado, el flujo cae en la rama de proceso que ya existe (`tracker-stamp`, `pull-request-review` y el resto).

## T2 — Regresión

Ampliar `tier_dispatch` en `route_domain_core`. El test vigente del agente sin cápsula permanece.

Casos nuevos, path de pending que no es fichero, script de runtime que deja una marca:

- `process` + tier `low` o `high`: la marca no existe. El estado no es el `success` del script. En un repo temporal el orquestador falta y el estado es `failed`.
- `action: notify-humanized-pr-merged` + tier cognitivo: la marca no existe. El estado sale de la acción.
- `process` + tier `none`: la marca no existe.

`cargo test -p execute-process tier_dispatch --lib`.

## Hecho que este plan no repite

`action` y `tool` ya están en el predicado. `NotifyOutcome::Sent` y `SkippedEmpty` ya distinguen la entrega de `notify-humanized-pr-merged`. T2 solo impide que el runtime las sustituya.
