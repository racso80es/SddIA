---
feature_name: dcc-snapshot-hash-stuck
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-snapshot-hash-stuck
tracker_ref: AUT-22
document_id: PBI-FIX-FRACTURE-51dc7bdfd2fe
fracture_hash: 51dc7bdfd2fe
---

# Objetivos — dcc-snapshot-hash-stuck

## Misión

Cerrar AUT-22: Snapshot final de `delivery-close-cycle` no debe declarar consolidado un WIP cuando `refs/heads/{branch_name}` no se mueve, y no debe commitear si la rama actual no es `branch_name`.

## Alcance

La traza `[SNAPSHOT_DIRTY_SKIPPED] hash_after == hash_before; snapshot no consolidó WIP` (`51dc7bdfd2fe`). Guardia de rama antes del commit. Evidencia de la salida del commit si el hash no avanza. Firma de fractura y fila de corpus.

## Ley aplicada

- Git solo vía `skill:git-manager`.
- Sin bypass raw (`gh`, `git`, `curl`) para cerrar la fractura.
- `SddIA/engine/` no es genoma de las claves `tools` / `skills` / `actions` / `process` / `events` / `norms`.
- `fractureSignatures` se resuelve por Cúmulo (`SddIA/core/fracture-signatures.json`).
- Rama de este ancla: `Fix_Events_Multiple`.
- Provider `linear`: cierre en el issue, sin fichero nuevo en `docs/todos/pending/` ni `docs/todos/done/`.
