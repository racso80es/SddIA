---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
execution_id: "13161205-2a2a-4320-9953-554e18a1f7c5"
---

# Objetivos — capsula-binario-fosil-release-stale

## Misión

Sellar el diseño del PBI R1 v1.3.0: el motor ejecuta solo artefactos que acreditan `source_sha256` del genoma más testigo local `elf_sha256`. El gate `mtime` queda revocado. Primer código: desescape porcelain para que el ciclo oficial pueda continuar.

## Alcance de este sello

`spec.md` + `plan.md` (+ este manifiesto). Sin mutación de genoma ni de `capsule_paths.rs` en el commit de Diseño.
