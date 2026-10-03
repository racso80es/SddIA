---
document_id: PBI-DEUDA-PREPUSH-ARGOS-QA
uuid: "e5432926-abbd-4160-b978-e7190faa8af0"
title: "[DEUDA] Tracker — witness Argos fallido en Local_QA del pre-push"
format: markdown
version: "1.1.0"
status: done
priority: baja
type: deuda
process: bug-fix
dispatch: true
feature_name: prepush-argos-qa-witness
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-12
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
refined: "2026-10-02"
especificacion_cerrada: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/done/
derived_from_pr: "https://github.com/racso80es/SddIA/pull/316"
blocked_by:
  - PBI-DEUDA-FEATURES-F2-DOC
unblocks: []
closed: "2026-10-02"
execution_branch: fix/linear-tracker-adapter-hash
---

# Witness `argos.pull-request-review` en el pre-push

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001`.

Dictamen cerrado en `docs/features/prepush-argos-qa-witness/` (witness `a55f1d12-003b-40d3-ae21-6e69d575c257`, `delegation.exit_code: 1`, sync; hook coherente).

## 0. Filtro A

| Afirmación de la semilla | Realidad | Corrección |
|--------------------------|----------|------------|
| DCC empujó aunque Argos falló, así que el bus trata el fallo como ruido | `pre_push_gate.sh` solo llama a `delivery-close-cycle` si `route-domain-event` sale 0. Si sale distinto de 0, imprime `BLOCKED — Local_QA_Requested failed` y no lanza DCC en esa rama. | El gancho no ignora un exit de route. El dictamen tiene que explicar por qué ese exit fue 0 con un witness en dead-letter. |
| `blocking: true` del payload hace el dispatch síncrono | El modo sync es `SDDIA_LAB_ROUTE_SYNC` (`sync_dispatch_mode`). El gancho no exporta esa variable. El payload `blocking` no activa `SyncRouteGuard`. | No citar `blocking: true` como prueba de espera. |
| El camino async devuelve antes de que Argos termine | El camino async hace `join` de los hilos y, si el suscriptor no es terminal-ok, el JSON lleva `exitCode: 1` (`route_domain_core.rs`). Terminal-ok = `success` o prefijo `skipped`. | El hueco posible es otro: el proceso `pull-request-review` puede salir 0 aunque el veredicto escrito sea `blocked` / `delivery_state: failed`, y entonces el join ve `success`. |
| El contrato del evento ya exige sync | `local-qa-requested.md` dice que el dispatch es `SDDIA_LAB_ROUTE_SYNC=1` y que el exit code propaga a Git. El gancho no pone esa variable. | Si el dictamen es «el fallo debe cerrar el push», el arreglo es alinear gancho y contrato, no inventar un segundo gate. |

`invoke_process` exporta `SDDIA_HOOK_DELIVERY_CLOSE=1`. Un push anidado (el que dispare el propio DCC) entra al guard de la cabecera del hook y no repite el QA. Eso no explica un DCC de la misma pasada tras un route fallido.

## 1. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Citar el envelope de `a55f1d12-003b-40d3-ae21-6e69d575c257` (o el witness `kaizen-finalized`): exit del proceso `pull-request-review`, status que guardó el route, y exit del binario que vio el hook. Sin ese envelope no hay parche. |
| R-2 | Dictamen único. O el route sale ≠ 0 cuando el veredicto es `blocked` y el hook ya bloquea (entonces el comportamiento observado fue un exit 0 indebido y se corrige el mapeo). O el veredicto documental F2 no debe abortar el push y se escribe ese párrafo en la norma del hook, coherente con `local-qa-requested.md`. |
| R-3 | Prohibido dejar el witness como único registro del dictamen. |

## 2. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | El entregable cita exit y status del envelope, no una hipótesis. |
| AC-2 | O hay un test del mapeo veredicto→exit, o hay un párrafo de norma que coincida con el contrato del evento. No los dos en contradicción. |

## 3. Dependencia

`PBI-DEUDA-FEATURES-F2-DOC` primero. Si el único `blocked` era F2, este PBI no reabre ese veredicto: solo la semántica del hook frente a un `pull-request-review` que falla de verdad.

## 4. Fuera de alcance

- Rellenar `spec.md` / `plan.md` / `implementation.md` (PBI F2).
- Cambiar el join async del route si el envelope demuestra que el exit ya era 1 y el push salió por otro camino (`SDDIA_SKIP_HOOKS`, push fuera del hook).
