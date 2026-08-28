---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
updated: "2026-08-28"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
document_id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
uuid: a91f2d40-6e3b-4c8a-b7f1-2d9e0c5a84f6
execution_id: "13161205-2a2a-4320-9953-554e18a1f7c5"
phases:
  - l0-porcelain-unescape
  - l1-phase-fracture-emit
  - l2-source-digest-core
  - l3-witness-write
  - l4-resolve-anchor
  - l5-genome-em
  - l6-call-sites-trace-purge
  - tests-unit
  - l7-rebuild-then-hard-gate
  - l8-drain-dlt-start
  - doc-closure
---

# Plan — Anclaje de ejecución (sella Diseño)

Orden: L0 → L1 → L2 → L3 → L4 → L5 → L6 → tests → L7 → L8 → cierre. **Este commit sella Diseño** (`spec.md` + `plan.md` + PBI v1.3.0). Código = fase Ejecución (parada siguiente).

Prohibido en este sello: mutar `SddIA/tools|skills|actions|process|agents|events|norms` a mano; genoma `source_sha256` solo en L5 vía `entity-manager`.

## Fase L0 — Porcelain (CA1/CA2)

Archivo: `SddIA/engine/execute-process/src/engine/workspace_init.rs`.

1. Desescapar quoting git **o** status `-z` vía `git-manager` si el I/O frozen lo permite sin ampliar genoma; si hace falta clave nueva en skill → `entity-manager`, no Write.
2. Test unitario con la línea octal del PBI §8.1 y `pbi_ref` UTF-8 → `path_in_scope == true`.
3. Comando: `cd SddIA && cargo test -p execute-process workspace_init`.

Sin L0 el resto del ciclo no abre por proceso oficial.

## Fase L1 — Fractura de precondición (CA3)

Archivo: `workspace_init.rs` (+ `route_domain_core` / `materialize_pending_domain_event` si el emit reutiliza el helper existente).

1. Tras `dirty-worktree`, emitir `System_Fracture_Detected` (`process_name` = proceso vivo, `error_trace` = mensaje de abort, `attempted_action` = `workspace-init`).
2. El `Err` de fase **sigue** abortando el ciclo; el evento no sustituye el aborto.
3. Test: tempdir + dirty path fuera de scope → fichero pending de fractura (hash de contenido, no overwrite).

## Fase L2 — Digest en Core (CA9, L-DIGEST-A-B)

Archivos nuevos/extendidos bajo `SddIA/engine/execute-process/src/` (p. ej. `engine/capsule_digest.rs`). **No** duplicar lógica solo en bash.

1. Política A: crate + path-deps locales transitivos + recorte de lockfile.
2. Fallback B si `cargo metadata` falla: `SddIA/Cargo.toml` + `Cargo.lock` + crate (paridad bundle actual).
3. Tests: dos crates en tempdir; mutar A no cambia digest B (A); B documentado como fallback.
4. Después: alinear `_sddia_source_digest` en `build-release-bundle.sh` al mismo contrato (llamar binario o documentar paridad de bytes en `execution.md` si el shell permanece como espejo).

## Fase L3 — Testigo ELF (CA8 parcial)

1. Helper Rust: escribir `{elf}.sha256` (`source_sha256` + `elf_sha256`).
2. Ampliar `CONSUMER_BINS` / lista de build para incluir `iota-immutable-publisher` y el resto de nativos indexados usados por runtime.
3. `start-sddia.sh`: no solo `-p execute-process`; construir + testigo de la lista. Fallo de build aborta ignición.

## Fase L4 — Aduana `resolve_capsule_*` (CA5/CA6/CA10/CA11)

Archivo: `capsule_paths.rs` (+ caché `.SddIA/` CA10).

1. Retirar «primer fichero existente».
2. Contrato spec §2. `profiles` = búsqueda.
3. Caché invalidada por `(path, mtime, len)` del ELF.
4. Tests tempdir (a)–(e) del PBI CA11. Comando: `cargo test -p execute-process capsule_paths`.

**No activar** esta función en el path de invocación de producción hasta L7 (flag interno `SDDIA_CAPSULE_ANCHOR=off` por defecto **o** merge ordenado: primitivas verdes, wire-up en L7). Diseño preferido: código de aduana compilado desde L4, **gate de invocación** (`capsules.rs`) se enciende en L7 tras CA14.

## Fase L5 — Genoma `source_sha256` (CA4/CA7)

```text
./sddia-run.sh --process entity-manager --inputs '{… update tool|skill|daemon|… source_sha256 …}'
```

1. Backfill de cápsulas indexadas con digest A (o B si metadata falla, anotado).
2. Prohibido `Write` sobre `SddIA/tools/*.md` etc.
3. Contrato de acción: si `entity-manager` no admite el campo, forjar schema vía EM, no a mano.

## Fase L6 — Call sites, traza, log (CA12/CA13)

`capsules.rs`, `route_domain_core.rs`:

1. Propagar `capsule-stale-hash`.
2. Matriz `friction_id` del spec.
3. Borrar `#region agent log` (ruta absoluta).
4. Actualizar tests que asuman relay ante causa genérica.

## Fase tests unitarios

Antes de L7: `workspace_init` + `capsule_digest` + `capsule_paths` + tests de fractura batch. Suite: `cargo test -p execute-process` de esos módulos.

## Fase L7 — Rebuild luego fallo duro (CA14, L-ATTACK-ORDER)

Ops documentadas en `execution.md` (no son el sello de Diseño):

1. Inventario por cápsula: digest genoma, testigo, veredicto aduana **en dry-run**.
2. `cargo build --release` del parque indexado; escribir testigos; EM actualiza genomas.
3. Encender wire-up de aduana.
4. Repro PBI §1.1 sobre `release` de `iota-immutable-publisher` → OK.

## Fase L8 — DLT + arranque (CA15/CA16)

1. Drenar o cerrar `.SddIA/dlt/reanchor-queue/` (instancia; evidencia en `execution.md`).
2. Smoke pre-sellado batch (`SDDIA_LAB_SIMULATE_IOTA=1` admisible).
3. Verificar `start-sddia.sh` / bundle no omiten publisher.

## Cierre documental (post-Argos, misma rama)

1. `implementation.md` + `execution.md`.
2. `validacion.md` APTO, `pbi_archived: true` (CA17).
3. PBI R1 → `docs/todos/done/`; anotar padre (CA18).
4. `delivery-close-cycle` `source_process: bug-fix`.

## Delegación

| Fase proceso | Quién | Artefacto |
|--------------|-------|-----------|
| Diseño | Dedalo (este sello, relevo local) | `spec.md`, `plan.md` |
| Ejecución | Tekton | código + `implementation.md` + `execution.md` |
| Verificación | Argos | `validacion.md` |
| Cierre | `delivery-close-cycle` | PR único |

Git: `skill:git-manager` / `./sddia-run.sh --tool git-manager`. KM `docs/todos/`: Cúmulo / Kaizen — Tekton no siembra TODOs nuevos.
