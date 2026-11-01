---
feature_name: dcc-snapshot-hash-stuck
created: "2026-10-31"
process: bug-fix
base: main
scope: snapshot-final-hash-no-avanza
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-snapshot-hash-stuck
tracker_ref: AUT-22
document_id: PBI-FIX-FRACTURE-51dc7bdfd2fe
fracture_hash: 51dc7bdfd2fe
---

# Spec — AUT-22 (`51dc7bdfd2fe`)

## Hecho

`capsule_delivery_snapshot_final_with_repo` (`phase_capsules.rs`):

1. Lee `refs/heads/{branch_name}`. Solo hace checkout si ese `get_last_commit` falla.
2. Si hay paths tras el filtro, llama a `git-manager commit`.
3. Si el porcelain posterior está vacío y el hash del mismo ref no cambió, falla con `SNAPSHOT_DIRTY_SKIPPED` y el texto `hash_after == hash_before; snapshot no consolidó WIP`.

Esa traza es AUT-22. El commit se dio por éxito (`invoke_git_manager` solo sigue si `success`). El mensaje no adjunta `gitStdout`. No comprueba que HEAD sea `branch_name`.

Dos condiciones llegan al mismo fallo:

| ID | Condición | Efecto |
|----|-----------|--------|
| H1 | `branch_list` marca otra rama (o detached) y `refs/heads/{branch_name}` ya existe | El commit avanza HEAD; el ref comparado no |
| H2 | El commit de la cápsula responde éxito y ni `HEAD` ni el ref avanzan | Misma traza, sin evidencia de stdout |

`F-DCC-SNAPSHOT-GITIGNORE` y `F-DCC-SNAPSHOT-BRANCH-EXISTS` no matchean esta cadena. El catálogo la deja sin firma.

## Corte

### C1 — Rama alineada antes del commit

1. Tras resolver `hash_before`, operar `branch_list` de `git-manager`.
2. Rama actual = línea que empieza por `*`, sin el prefijo ni espacios. Detached (sin `*`, o `HEAD detached`) no cuenta como alineada.
3. Si no es exactamente `branch_name`, devolver `failed` con `error_code` `SNAPSHOT_BRANCH_MISMATCH`. No checkout. No commit.
4. Predicado puro testeable: entrada (nombre actual, `branch_name`) → alineado o no.

### C2 — El hash tiene que avanzar

1. Conservar `gitStdout` / `gitStderr` del commit.
2. Tras árbol limpio, leer `HEAD` y `refs/heads/{branch_name}` con `get_last_commit`.
3. Si alguno es igual a `hash_before`, `SNAPSHOT_DIRTY_SKIPPED` y el mensaje incluye la salida del commit. No `executed`.
4. Predicado puro: `(before, head_after, branch_after)` → avanzó solo si ambos difieren de `before` y no están vacíos.

### C3 — Firma

Nueva entrada en `fractureSignatures`:

| Campo | Valor |
|-------|--------|
| id | `F-DCC-SNAPSHOT-HASH-STUCK` |
| consumers | `mayeuta` |
| match.all | `hash_after == hash_before`, `snapshot no consolidó` |
| verdict | `process_fix` |

Fila `51dc7bdfd2fe` en `fracture_corpus_regression`: proceso `delivery-close-cycle`, acción `Snapshot final`, emisor `execute-process`, traza literal del incidente, firma esperada `F-DCC-SNAPSHOT-HASH-STUCK`.

Hito en `SddIA/evolution/` con el uuid del ítem de evolución nuevo y referencia a `51dc7bdfd2fe`.

## Fuera de alcance

- Re-parchear `checkout -b` (`187079f2d82d`).
- Checkout automático para “corregir” una rama distinta.
- `F-DCC-SNAPSHOT-GITIGNORE`.
- Cierre Linear Done y `validacion.md` APTO: fase Argos, no este corte.
