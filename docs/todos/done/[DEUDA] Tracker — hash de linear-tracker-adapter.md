---
document_id: PBI-DEUDA-TRACKER-ADAPTER-HASH
uuid: "4504d70c-7cb7-42a4-95bf-0032d14f2300"
title: "[DEUDA] Tracker — hash_signature real de linear-tracker-adapter"
format: markdown
version: "1.1.0"
status: pending
priority: media
type: deuda
process: bug-fix
dispatch: true
feature_name: linear-tracker-adapter-hash
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-7
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
refined: "2026-10-02"
especificacion_cerrada: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/pending/
derived_from_pr: "https://github.com/racso80es/SddIA/pull/316"
blocked_by: []
unblocks: []
---

# Hash de la ficha `linear-tracker-adapter`

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001`.

## 0. Filtro A

| Afirmación de la semilla | Realidad | Corrección |
|--------------------------|----------|------------|
| El contrato de tools exige digest real | `SddIA/tools/tools-contract.md` no menciona `hash_signature`. El algoritmo de forja está en `canonical_artifact_hash` (`forges/common.rs`): SHA-256 del markdown tras quitar las líneas que empiezan por `hash_signature:`. | El digest se calcula con esa función, no hasheando el archivo entero. |
| El gate de procesos no bloquea el placeholder | `valid_sha256_str` acepta cualquier `sha256:` de longitud > 15. El placeholder de ceros pasa. | Sustituir el campo. No hace falta cambiar el predicado en este PBI. |
| «El contrato» es una frase suelta | `entity-manager.md` exige `hash_signature_new` válido y no placeholder para el publicador IOTA. La matriz tiene esta tool con `last_hash: sha256:deadbeef`. | El sello pasa por `entity-manager` (update de la ficha) para que `last_hash` deje de ser `deadbeef`. |

## 1. Intención

Dejar en `SddIA/tools/linear-tracker-adapter.md` el digest canónico de la ficha. El binario no se toca.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `hash_signature` = `canonical_artifact_hash` de la ficha (línea `hash_signature:` excluida). |
| R-2 | Update vía `entity-manager` / `tool-creator`, de modo que `eda-coverage.json` de `8f3c2a1b-9d4e-4f5a-b6c7-1234567890ab` quede con `last_hash` igual a ese digest y `last_emitted_event` de forja. |
| R-3 | Si `PBI-DEUDA-TRACKER-GENOMA-FORJA` corre después, no restaura `deadbeef` ni el placeholder de ceros en esta ficha. |

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | `hash_signature` es `sha256:` + 64 hex, distinto del placeholder de ceros y de `sha256:deadbeef`, e igual al canónico de la ficha. |
| AC-2 | `cargo test -p linear-tracker-adapter` sigue verde. |
| AC-3 | La entrada de cobertura de la tool repite ese mismo digest. |

## 4. Fuera de alcance

- Endurecer `valid_sha256_str` para rechazar ceros en todo el genoma.
- Recompilar la cápsula.
