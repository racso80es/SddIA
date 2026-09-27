---
document_id: PBI-ARQUITECTURA-INSTALLER-V3-REQUEST-STDIN-ENV
uuid: "f7b237b0-dd48-4b85-9834-d85067123115"
title: "[ARQUITECTURA] Installer v3 — entrada stdin y SDDIA_CAPSULE_REQUEST"
format: markdown
version: "1.1.0"
status: done
priority: media
type: arquitectura
process: refactorization
dispatch: false
feature_name: sddia-installer-v3-request-stdin-env
historia_ref: docs/todos/done/historias/[ARQUITECTURA] Installer v3 — contrato de entrada-salida por comando y UX de Deploy - Eliminar Cliente.md
historia_document_id: HU-INSTALLER-V3-IO-CONTRACT-UX
historia_uuid: "489f5b85-fa2f-4f0e-ae52-7278e039dff1"
created: "2026-09-27"
updated: "2026-09-27"
author: tekton
residual_de: audit_cierre HU-INSTALLER-V3-IO-CONTRACT-UX (2026-09-27)
pbi_antecesores:
  - document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
    uuid: "cb5483e7-4e39-4fb6-9c11-4a8285b957b7"
unblocks: []
related:
  - SddIA/scripts/sddia-installer.sh
  - sddia-installer.sh
  - SddIA/scripts/installer/installer_io.py
  - SddIA/library/norms/sddia-installer-contract.md
  - SddIA/norms/capsule-json-io.md
baseline:
  - "Motor: solo --request-file (main() ~L670). Parse fallido sale 7 por stderr, sin envelope."
  - "Fachada: exec al motor; no consume stdin. El presentador sí (ELIMINAR y hold)."
  - "Norma 1.3.0 y capsule-json-io ya nombran stdin y SDDIA_CAPSULE_REQUEST; SDDIA_SKIP_STDIN=1 existe en capsule-json-io."
changelog:
  - "1.1.0: precedencia cerrada, veto de stdin en TTY, envelope en REQUEST_INVALID, sin tocar el presentador."
---

# Installer v3 — entrada stdin y SDDIA_CAPSULE_REQUEST

Cierra el hueco de **AC-2** de la historia: `--request-file` y `argv` existen; stdin y `SDDIA_CAPSULE_REQUEST` no.

## 0. Decisiones (no reabrir)

| ID | Laudo |
|----|--------|
| D-IN-1 | Precedencia: `--request-file` > stdin JSON > `SDDIA_CAPSULE_REQUEST` > `argv`. |
| D-IN-2 | Stdin solo si **no es TTY**, `SDDIA_SKIP_STDIN` no es `1`, y el primer carácter no blanco es `{`. Si no, se ignora y sigue `argv`. Así el presentador (TTY) no pierde `ELIMINAR` ni la tecla de cierre. |
| D-IN-3 | Punto de lectura: **motor** (`SddIA/scripts/sddia-installer.sh`), antes de `_parse`. La fachada no reimplementa el parser; hereda stdin por `exec`. |
| D-IN-4 | JSON inválido o comando ausente → **un** envelope, `exitCode` 7, `error.code=REQUEST_INVALID`. Prohibido el `exit 7` mudo actual del `--request-file`. |
| D-IN-5 | Sin bump de norma salvo una frase que cite `SDDIA_SKIP_STDIN`. No 1.4.0 si el texto 1.3.0 ya cubre las vías. |

## 1. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-R1 | Mismo request dry-run por `--request-file` y por stdin (`printf … \|`) → `result.plan` igual salvo campos volátiles (`live` si aplica). | caso en `test-sddia-installer.sh` |
| AC-R2 | Sin `--request-file` y sin stdin: `SDDIA_CAPSULE_REQUEST='{…}'` produce el mismo plan. | smoke |
| AC-R3 | `--request-file` gana sobre stdin y sobre la variable de entorno. | smoke (un caso) |
| AC-R4 | JSON roto → exit 7, stdout = un JSON con `error.code=REQUEST_INVALID`. | smoke |
| AC-R5 | stdin es TTY (invocación normal `deploy --dry-run`) → no se lee como request; el comportamiento argv no cambia. | smoke existente 1–17 |
| AC-R6 | `SDDIA_SKIP_STDIN=1` con pipe JSON y sin `--request-file` ni env → no interpreta el pipe; cae en argv (o error de comando ausente si no hay argv). | smoke |

## 2. Fuera de alcance

- Presentador (`sddia-installer-ui.sh`): no parsea request.
- Envelope de éxito, catálogo de pasos, PTC.
- Entidad `tool` (D7).

## 3. Plan

1. Extraer en el motor una función `_load_request_json` con la precedencia D-IN-1..3.
2. Unificar el fallo de parse con `_die` / `installer_io.py` para emitir envelope (D-IN-4), también en `--request-file`.
3. Casos AC-R1..R6 en el smoke. Cierre documental en la misma rama.

## 4. Riesgo

Leer stdin siempre rompería el presentador. D-IN-2 es el cierre de ese riesgo; AC-R5 lo fija.
