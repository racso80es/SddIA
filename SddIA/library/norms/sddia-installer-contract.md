---
uuid: "b1327ef3-5f07-4fba-9073-a72c5fdf97e2"
name: "sddia-installer-contract"
version: "1.2.0"
nature: "tactical-norm"
author: "tekton"
scope: "infrastructure"
category: "architecture"
dependencies: []
hash_signature: "sha256:780962b169cca143418c855eaa3604132d2d1720b338c0243b286ba86ea7b5b6"
---

## Directriz Core

Installer v3 I/O: un envelope `capsule-json-io` 2.0 por stdout (`meta.entityId=sddia-installer`); entrada por argv, `--request-file`, stdin JSON o `SDDIA_CAPSULE_REQUEST`; progreso JSONL (`@sddia-progress` en stderr o fd 3); logs en `instance.installer_logs` (Cúmulo). Hereda v2: bóveda compuesta, puerto derivado, unidades condicionales, verify en fachada, registro host.

## Códigos de salida

| exitCode | error.code (en envelope) |
|----------|--------------------------|
| 0 | — |
| 1 | INVALID_ARGS, UNSAFE_ROOT, HOST_TOOL_MISSING, VAULT_SOURCE_MISSING |
| 2 | ROOT_LIVE_REQUIRES_FORCE |
| 3 | TEARDOWN_REQUIRES_FORCE |
| 4 | MAILBOX_SHARED |
| 5 | VERIFY_NOT_APTO (fachada) |
| 6 | STEP_FAILED |
| 7 | REQUEST_INVALID |

## Invariantes

- **I-DEP-SINGLE-STDOUT:** un solo JSON en stdout por invocación (motor o fachada).
- **I-UX-NOPROMPT-MOTOR:** motor y fachada sin `read`/`select`/`zenity`/`whiptail`.

Schemas: `sddia-installer-request.schema.json`, `sddia-installer-result.schema.json` (junto a esta norma).

## Restricciones Duras (Aduana de Fricción)

Ninguna.
