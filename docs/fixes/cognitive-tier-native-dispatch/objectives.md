---
feature_name: cognitive-tier-native-dispatch
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/cognitive-tier-native-dispatch
pbi_ref: AUT-26
tracker_ref: AUT-26
execution_id: "73d65b19-25cb-489c-94e5-abffe60b1e46"
document_id: PBI-b2f3f6fb
---

# Objetivos — cognitive-tier-native-dispatch

## Misión

Cerrar AUT-26: en modo redb, un suscriptor que declara `process`, `action` o `tool` ejecuta esa cápsula aunque el ECST solo esté en el store. El tier cognitivo no la sustituye ni convierte la salida 0 del runtime en `success` de entrega.

## Alcance

`subscriber_declares_native` en el despacho de dominio. Hoy cuenta `action` y `tool`; no cuenta `process`. El corte vivo es Tekton (`low`) + `tracker-stamp` y Argos (`high`) + `pull-request-review`. `AGENT_PHASE` permanece cuando el suscriptor es solo el agente. Test de regresión en el módulo `tier_dispatch`.

## Ley aplicada

- Git solo vía `skill:git-manager`. Este ciclo no movió HEAD: `SDDIA_LAB_SKIP_GIT` para permanecer en `Fix_Events_Multiple`. El init etiquetó `fix/cognitive-tier-native-dispatch`; esa rama no existe y no se abre.
- `SddIA/engine/` no es genoma de las claves `tools` / `skills` / `actions` / `process` / `events` / `norms`.
- Provider `linear`: cierre en el issue, sin fichero nuevo en `docs/todos/pending/` ni `docs/todos/done/`.
- Fuera: AUT-21, AUT-16, AUT-25 y el reenvío manual del Telegram de AUT-8.
